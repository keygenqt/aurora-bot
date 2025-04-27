use std::fs;
use std::os::linux::fs::MetadataExt;
use std::path::PathBuf;
use std::time::SystemTime;

use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::selector::selects::select_psdk_available::PsdkAvailableModelSelect;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::models::configuration::Config;
use crate::models::configuration::psdk::PsdkConfig;
use crate::models::psdk_available::model::PsdkAvailableModel;
use crate::models::psdk_installed::model::PsdkInstalledModel;
use crate::service::command;
use crate::service::command::exec;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::print_debug;
use crate::tools::macros::tr;
use crate::tools::programs;
use crate::tools::single;
use crate::tools::utils;

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkInstallIncoming {
    id: Option<String>,
}

impl PsdkInstallIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::PsdkInstall)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<PsdkInstallIncoming> {
        print_debug!("> {}: new()", Self::name());
        Box::new(Self { id: None })
    }

    pub fn new_id(id: String) -> Box<PsdkInstallIncoming> {
        print_debug!("> {}: new_id(id: {})", Self::name(), id);
        Box::new(Self { id: Some(id) })
    }

    fn select(&self, id: String) -> PsdkInstallIncoming {
        let mut select = self.clone();
        select.id = Some(id);
        select
    }

    pub fn dbus_method_run(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            Self::name(),
            (),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (): ()| async move {
                let outgoing = Self::new().run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    pub fn dbus_method_run_by_id(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "ById"),
            ("id",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (id,): (String,)| async move {
                let outgoing = Self::new_id(id).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    fn run(
        model: PsdkAvailableModel,
        send_type: &OutgoingType,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        ///////////
        // DOWNLOAD
        StateMessageOutgoing::new_state(tr!("начинаем загрузку...")).send(send_type);
        // Time start
        let start = SystemTime::now();
        // Download
        let urls = model.urls;
        let paths =
            single::get_request().download_files(urls, StateMessageOutgoing::get_state_callback_file_big(send_type))?;
        let downloads = utils::move_to_downloads(paths)?;
        // Time end
        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        let seconds = duration.as_secs();
        // Download done
        StateMessageOutgoing::new_info(tr!("загрузка успешно выполнена ({}s)", seconds)).send(send_type);

        //////////
        // INSTALL
        // Time start
        let start = SystemTime::now();
        // Get features paths
        let path_chroot = downloads
            .iter()
            .filter(|e| {
                let name = e.to_string_lossy();
                name.contains("Chroot") && !name.contains("md5sum")
            })
            .next();
        let path_tooling = downloads
            .iter()
            .filter(|e| {
                let name = e.to_string_lossy();
                name.contains("Tooling") && !name.contains("md5sum")
            })
            .next();
        let path_targets = downloads
            .iter()
            .filter(|e| {
                let name = e.to_string_lossy();
                name.contains("Target") && !name.contains("md5sum")
            })
            .collect::<Vec<&PathBuf>>();

        // Check data
        if path_chroot.as_ref().is_none() {
            Err(tr!("не найден Chroot, установка прервана"))?;
        }
        if path_tooling.as_ref().is_none() {
            Err(tr!("не найден Tooling, установка прервана"))?;
        }
        if path_targets.is_empty() {
            Err(tr!("не найдены Targets, установка прервана"))?;
        }

        // Init data
        let sudo = programs::get_sudo()?;
        let tar = programs::get_tar()?;
        let path_chroot = path_chroot.unwrap();
        let path_tooling = path_tooling.unwrap();
        let version = path_chroot.to_string_lossy().split("-").collect::<Vec<&str>>()[1].to_string();
        // Init folders
        let mut path_unpack = utils::get_home_folder_path();
        path_unpack.push(".local/opt");
        path_unpack.push(format!("PlatformSDK_{}", version));
        // Check exist dir
        if path_unpack.exists() {
            return Ok(StateMessageOutgoing::new_warning(tr!(
                "установка не будет продолжена, найдена директория: {}",
                path_unpack.to_string_lossy()
            )));
        }
        let _ = exec::exec_wait_args(&sudo, ["echo"])?;
        let mut path_unpack_psdk_dir = path_unpack.clone();
        path_unpack_psdk_dir.push("sdks/aurora_psdk");
        let mut path_unpack_toolings = path_unpack.clone();
        path_unpack_toolings.push("toolings");
        let mut path_unpack_tarballs = path_unpack.clone();
        path_unpack_tarballs.push("tarballs");
        let mut path_unpack_targets = path_unpack.clone();
        path_unpack_targets.push("targets");
        // Create folders
        fs::create_dir_all(&path_unpack)?;
        fs::create_dir_all(&path_unpack_psdk_dir)?;
        fs::create_dir_all(&path_unpack_toolings)?;
        fs::create_dir_all(&path_unpack_tarballs)?;
        fs::create_dir_all(&path_unpack_targets)?;
        // Get chroot path
        let mut chroot = path_unpack_psdk_dir.clone();
        chroot.push("sdk-chroot");
        // Install chroot
        StateMessageOutgoing::new_state(tr!("установка Chroot Platform SDK")).send(send_type);
        let size = path_chroot.metadata()?.st_size();
        let count = (size * 130 / 439822186) as i32;
        exec::exec_wait_args_callback(
            &sudo,
            [
                &tar,
                "--numeric-owner",
                "-p",
                "-xjf",
                &path_chroot.to_string_lossy().to_string(),
                "--totals",
                "--checkpoint=1000",
                "--checkpoint-action=echo='#%u'",
                "-C",
                &path_unpack_psdk_dir.to_string_lossy().to_string(),
            ],
            StateMessageOutgoing::get_state_callback_count(count, send_type),
        )?;
        StateMessageOutgoing::new_progress("100".into()).send(send_type);
        // Install tooling
        StateMessageOutgoing::new_state(tr!("установка Tooling Platform SDK")).send(send_type);
        let count = 4;
        exec::exec_wait_args_callback(
            &sudo,
            [
                &chroot.to_string_lossy().to_string(),
                "sdk-assistant",
                "tooling",
                "create",
                "-y",
                &format!("AuroraOS-{}-base", &version),
                &path_tooling.to_string_lossy().to_string(),
            ],
            StateMessageOutgoing::get_state_callback_count(count, send_type),
        )?;
        StateMessageOutgoing::new_progress("100".into()).send(send_type);

        // Install targets
        for path_target in path_targets {
            // Get arch from name
            let name_target = path_target.to_string_lossy().to_string();
            let mut name_parts = name_target.split("-").collect::<Vec<&str>>();
            name_parts.reverse();
            let arch = name_parts
                .first()
                .unwrap()
                .split(".")
                .collect::<Vec<&str>>()
                .first()
                .unwrap()
                .to_string();
            // Install
            StateMessageOutgoing::new_state(tr!("установка Target {} Platform SDK", &arch)).send(send_type);
            let count = 9;
            exec::exec_wait_args_callback(
                &sudo,
                [
                    &chroot.to_string_lossy().to_string(),
                    "sdk-assistant",
                    "target",
                    "create",
                    "-y",
                    &format!("AuroraOS-{}-base-{}", &version, &arch),
                    &path_target.to_string_lossy().to_string(),
                ],
                StateMessageOutgoing::get_state_callback_count(count, send_type),
            )?;
            StateMessageOutgoing::new_progress("100".into()).send(send_type);
        }

        //////////
        // SUDOERS
        StateMessageOutgoing::new_state(tr!("обновление записи sudoers Platform SDK")).send(send_type);
        let models = PsdkInstalledModel::search_full_without_targets()?;
        command::psdk::add_sudoers_chroot_access(&models)?;

        //////////
        // SYNC
        StateMessageOutgoing::new_state(tr!("запуск синхронизации Platform SDK")).send(send_type);
        let _ = Config::save_psdk(PsdkConfig::search());

        ///////
        // DONE
        // Time end
        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        let seconds = duration.as_secs();
        Ok(StateMessageOutgoing::new_success(tr!(
            "уставка Platform SDK успешно выполнена ({}s)",
            seconds
        )))
    }
}

impl TraitIncoming for PsdkInstallIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = PsdkInstallIncoming::name();
        let models = PsdkAvailableModelSelect::search(&self.id, tr!("получаем список..."), &send_type);
        // Select
        match models.iter().count() {
            1 => match Self::run(models.first().unwrap().clone(), &send_type) {
                Ok(result) => result,
                Err(error) => StateMessageOutgoing::new_error(tr!("{}", error)),
            },
            0 => StateMessageOutgoing::new_info(tr!("не удалось получить данные")),
            _ => match PsdkAvailableModelSelect::select(key, &send_type, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить Platform SDK")),
            },
        }
    }
}
