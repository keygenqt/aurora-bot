use std::path::PathBuf;

use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::ClientMethodsKey;
use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::selector::selects::select_emulator::EmulatorModelSelect;
use crate::models::client::state_message::outgoing::StateMessageOutgoing;
use crate::models::emulator::model::EmulatorModel;
use crate::models::psdk_installed::model::PsdkInstalledModel;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::tr;
use crate::tools::utils;

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorPackageInstallIncoming {
    id: Option<String>,
    path: Option<PathBuf>,
}

// @todo add to server
impl EmulatorPackageInstallIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::EmulatorPackageInstall)
            .unwrap()
            .to_string()
    }

    pub fn new(path: PathBuf) -> Box<EmulatorPackageInstallIncoming> {
        Box::new(Self {
            id: None,
            path: Some(path),
        })
    }

    pub fn new_id(id: String, path: PathBuf) -> Box<EmulatorPackageInstallIncoming> {
        Box::new(Self {
            id: Some(id),
            path: Some(path),
        })
    }

    fn select(&self, id: String) -> EmulatorPackageInstallIncoming {
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
                    Some(path) => Self::new(path).run(OutgoingType::Dbus),
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
                    Some(path) => Self::new_id(id, path).run(OutgoingType::Dbus),
                    None => StateMessageOutgoing::new_error(tr!("проверьте путь к файлу")),
                };
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    fn run_install(
        emulator: &EmulatorModel,
        send_type: &OutgoingType,
        path: &PathBuf,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        if !emulator.is_running {
            return Ok(StateMessageOutgoing::new_info(tr!("эмулятор должен быть запущен")));
        }
        // Check and sign package
        let psdk = match PsdkInstalledModel::get_latest() {
            Some(value) => value,
            None => Err(tr!("для проверки и подписи пакете необходим установить Platform SDK"))?,
        };
        if !psdk.package_is_sign(path) && !psdk.package_sign(path) {
            Err(tr!("не удалось провалидировать подпись"))?;
        }
        // Get package name from rpm
        let package_name = utils::get_package_name(path);
        if package_name.is_none() {
            Err("не удалось получить название пакета")?;
        }
        // Get session
        let session = emulator.session_user()?;
        // Upload file
        StateMessageOutgoing::new_state(tr!("загружаем пакет...")).send(send_type);
        let path_remote = &session.file_upload(path, StateMessageOutgoing::get_state_callback_file_small(send_type))?;
        // Install by apm
        StateMessageOutgoing::new_state(tr!("установка пакета")).send(send_type);
        session.install_package(path_remote.clone(), package_name)?;
        // Success result
        Ok(StateMessageOutgoing::new_success(tr!("пакет успешно установлен")))
    }
}

impl TraitIncoming for EmulatorPackageInstallIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = EmulatorPackageInstallIncoming::name();
        let models: Vec<EmulatorModel> = EmulatorModelSelect::search(
            &self.id,
            &send_type,
            tr!("ищем запущенный эмулятор для загрузки"),
            Some(true),
        );
        if self.path.as_ref().is_none() {
            return StateMessageOutgoing::new_error(tr!("нужно указать путь к файлу"));
        }
        // Select
        match models.iter().count() {
            1 => match Self::run_install(models.last().unwrap(), &send_type, self.path.as_ref().unwrap()) {
                Ok(value) => value,
                Err(error) => StateMessageOutgoing::new_error(tr!("{}", error)),
            },
            0 => StateMessageOutgoing::new_info(tr!("запущенные эмуляторы не найдены")),
            _ => match EmulatorModelSelect::select(key, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить эмулятор")),
            },
        }
    }
}
