use std::path::PathBuf;

use colored::Colorize;
use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::selector::selects::select_psdk_installed::PsdkInstalledModelSelect;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::models::psdk_installed::model::PsdkInstalledModel;
use crate::models::psdk_target::model::PsdkTargetModel;
use crate::models::psdk_target_package::model::PsdkTargetPackageModel;
use crate::service::command;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::print_debug;
use crate::tools::macros::tr;
use crate::tools::utils;

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkTargetPackageInstallIncoming {
    id: Option<String>,
    path: PathBuf,
}

impl PsdkTargetPackageInstallIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::PsdkTargetPackageInstall)
            .unwrap()
            .to_string()
    }
    pub fn new_path(path: PathBuf) -> Box<PsdkTargetPackageInstallIncoming> {
        print_debug!("> {}: new_path(path: {})", Self::name(), path.to_string_lossy());
        Box::new(Self { id: None, path })
    }

    pub fn new_path_id(id: String, path: PathBuf) -> Box<PsdkTargetPackageInstallIncoming> {
        print_debug!(
            "> {}: new_path_id(id: {}, path: {})",
            Self::name(),
            id,
            path.to_string_lossy()
        );
        Box::new(Self { id: Some(id), path })
    }

    fn select(&self, id: String) -> PsdkTargetPackageInstallIncoming {
        let mut select = self.clone();
        select.id = Some(id);
        select
    }

    pub fn dbus_method_run_path(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            Self::name(),
            ("path",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (path,): (String,)| async move {
                let outgoing = match utils::path_to_absolute(&PathBuf::from(path)) {
                    Some(path) => Self::new_path(path).run(OutgoingType::Dbus),
                    None => StateMessageOutgoing::new_error(tr!("проверьте путь к файлу")),
                };
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    pub fn dbus_method_run_path_by_id(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "ById"),
            ("id", "path"),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (id, path): (String, String)| async move {
                let outgoing = match utils::path_to_absolute(&PathBuf::from(path)) {
                    Some(path) => Self::new_path_id(id, path).run(OutgoingType::Dbus),
                    None => StateMessageOutgoing::new_error(tr!("проверьте путь к файлу")),
                };
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    fn run(
        model: PsdkInstalledModel,
        target: PsdkTargetModel,
        path: &PathBuf,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        if !path.is_file() {
            Err(tr!("необходимо указать путь к файлу"))?
        }
        let package_name = match utils::get_package_name(path) {
            Some(value) => value,
            None => Err(tr!("необходимо указать путь к RPM пакету"))?,
        };
        // Search package
        let packages = PsdkTargetPackageModel::search_local(&model.chroot, &target.full_name, &package_name, true)?;
        if !packages.is_empty() {
            return Ok(StateMessageOutgoing::new_info(tr!(
                "пакет {} уже установлен",
                package_name.bold()
            )));
        }
        // Install package
        command::psdk::target_package_install(&model.chroot, &path, &target)?;
        // Success
        Ok(StateMessageOutgoing::new_success(tr!(
            "пакет {} успешно установлен",
            package_name.bold()
        )))
    }
}

impl TraitIncoming for PsdkTargetPackageInstallIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Find psdk
        let key = PsdkTargetPackageInstallIncoming::name();
        let models = PsdkInstalledModelSelect::search(&self.id, tr!("получаем информацию о Platform SDK"), &send_type);
        // Select psdk
        match models.iter().count() {
            1 => {
                let model = models.first().unwrap().clone();
                if !self.path.is_file() {
                    return StateMessageOutgoing::new_error(tr!("необходимо указать путь к файлу"));
                }
                let package_arch = match utils::get_package_arch(&self.path) {
                    Some(value) => value,
                    None => return StateMessageOutgoing::new_error(tr!("необходимо указать путь к RPM пакету")),
                };
                match model
                    .targets
                    .iter()
                    .filter(|e| e.arch == package_arch)
                    .cloned()
                    .collect::<Vec<PsdkTargetModel>>()
                    .first()
                {
                    Some(target) => match Self::run(models.first().unwrap().clone(), target.clone(), &self.path) {
                        Ok(result) => result,
                        Err(error) => StateMessageOutgoing::new_error(tr!("{}", error)),
                    },
                    None => StateMessageOutgoing::new_error(tr!(
                        "Platform Target с архитектурой {} не найден",
                        package_arch
                    )),
                }
            }
            0 => StateMessageOutgoing::new_info(tr!("Platform SDK не найдены")),
            _ => match PsdkInstalledModelSelect::select(key, &send_type, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить Platform SDK")),
            },
        }
    }
}
