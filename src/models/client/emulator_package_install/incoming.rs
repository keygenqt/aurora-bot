use std::path::PathBuf;

use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::ClientMethodsKey;
use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::selector::selects::select_demo_app::DemoAppModelSelect;
use crate::models::client::selector::selects::select_emulator::EmulatorModelSelect;
use crate::models::client::state_message::outgoing::StateMessageOutgoing;
use crate::models::demo_app::model::DemoAppModel;
use crate::models::emulator::model::EmulatorModel;
use crate::models::psdk_installed::model::PsdkInstalledModel;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::print_debug;
use crate::tools::macros::tr;
use crate::tools::single;
use crate::tools::utils;

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorPackageInstallIncoming {
    id: Option<String>,
    path: Option<PathBuf>,
    url: Option<String>,
    is_demo: bool,
}

impl EmulatorPackageInstallIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::EmulatorPackageInstall)
            .unwrap()
            .to_string()
    }

    pub fn new_path(path: PathBuf) -> Box<EmulatorPackageInstallIncoming> {
        print_debug!("> {}: new_path(path: {})", Self::name(), path.to_string_lossy());
        Box::new(Self {
            id: None,
            path: Some(path),
            url: None,
            is_demo: false,
        })
    }

    pub fn new_path_id(id: String, path: PathBuf) -> Box<EmulatorPackageInstallIncoming> {
        print_debug!(
            "> {}: new_path_id(id: {}, path: {})",
            Self::name(),
            id,
            path.to_string_lossy()
        );
        Box::new(Self {
            id: Some(id),
            path: Some(path),
            url: None,
            is_demo: false,
        })
    }

    pub fn new_url(url: String) -> Box<EmulatorPackageInstallIncoming> {
        print_debug!("> {}: new_url(url: {})", Self::name(), url);
        Box::new(Self {
            id: None,
            path: None,
            url: Some(url),
            is_demo: false,
        })
    }

    pub fn new_url_id(id: String, url: String) -> Box<EmulatorPackageInstallIncoming> {
        print_debug!("> {}: new_url_id(id: {}, url: {})", Self::name(), id, url);
        Box::new(Self {
            id: Some(id),
            path: None,
            url: Some(url),
            is_demo: false,
        })
    }

    pub fn new_demo() -> Box<EmulatorPackageInstallIncoming> {
        print_debug!("> {}: new_demo()", Self::name());
        Box::new(Self {
            id: None,
            path: None,
            url: None,
            is_demo: true,
        })
    }

    fn select(&self, id: String) -> EmulatorPackageInstallIncoming {
        let mut select = self.clone();
        select.id = Some(id);
        select
    }

    fn select_url(&self, url: String) -> EmulatorPackageInstallIncoming {
        let mut select = self.clone();
        select.url = Some(url);
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

    pub fn dbus_method_run_url(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "ByUrl"),
            ("url",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (url,): (String,)| async move {
                let outgoing = Self::new_url(url).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    pub fn dbus_method_run_url_by_id(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "ByUrlId"),
            ("id", "url"),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (id, url): (String, String)| async move {
                let outgoing = Self::new_url_id(id, url).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    pub fn dbus_method_run_demo(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "Demo"),
            (),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (): ()| async move {
                let outgoing = Self::new_demo().run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    fn run_install_by_path(
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
            Err(tr!("не удалось получить название пакета"))?;
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

    fn run_install_by_url(
        emulator: &EmulatorModel,
        send_type: &OutgoingType,
        url: &String,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        let url = match utils::get_https_url(url.to_string()) {
            Some(url) => url,
            None => Err(tr!("не удалось скачать файл"))?,
        };
        StateMessageOutgoing::new_state(tr!("скачиваем файл...")).send(send_type);
        let path = single::get_request().download_file(
            url.to_string(),
            StateMessageOutgoing::get_state_callback_file_small(send_type),
        )?;
        // Move to download - psdk not mount /temp
        let rpm_path_download = match utils::move_to_downloads(vec![path]) {
            Ok(value) => value.first().unwrap().clone(),
            Err(_) => Err(tr!("не удалось сохранить файл"))?,
        };
        // Run install
        Self::run_install_by_path(emulator, send_type, &rpm_path_download)
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
        if self.is_demo == false && self.path.as_ref().is_none() && self.url.as_ref().is_none() {
            return StateMessageOutgoing::new_error(tr!("нужно указать путь или url к файлу"));
        }
        // Select
        match models.iter().count() {
            1 => {
                let emulator = models.last().unwrap();
                if !emulator.is_running {
                    StateMessageOutgoing::new_info(tr!("эмулятор должен быть запущен"))
                } else {
                    if self.is_demo && self.url.as_ref().is_none() {
                        let packages: Vec<DemoAppModel> =
                            DemoAppModelSelect::search(&self.id, tr!("ищем доступные приложения"), &send_type);
                        match DemoAppModelSelect::select(key, packages, |url| self.select_url(url)) {
                            Ok(value) => Box::new(value),
                            Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить эмулятор")),
                        }
                    } else {
                        if self.path.as_ref().is_some() {
                            match Self::run_install_by_path(emulator, &send_type, self.path.as_ref().unwrap()) {
                                Ok(value) => value,
                                Err(error) => StateMessageOutgoing::new_error(tr!("{}", error)),
                            }
                        } else {
                            match Self::run_install_by_url(emulator, &send_type, self.url.as_ref().unwrap()) {
                                Ok(value) => value,
                                Err(error) => StateMessageOutgoing::new_error(tr!("{}", error)),
                            }
                        }
                    }
                }
            }
            0 => StateMessageOutgoing::new_info(tr!("запущенные эмуляторы не найдены")),
            _ => match EmulatorModelSelect::select(key, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить эмулятор")),
            },
        }
    }
}
