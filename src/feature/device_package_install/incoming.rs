use std::fs;
use std::path::PathBuf;

use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::selector::selects::select_device::DeviceModelSelect;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::models::device::model::DeviceModel;
use crate::models::psdk_installed::model::PsdkInstalledModel;
use crate::service::command;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::tr;
use crate::tools::single;
use crate::tools::utils;

#[derive(Serialize, Deserialize, Clone)]
pub struct DevicePackageInstallIncoming {
    id: Option<String>,
    path: Option<PathBuf>,
    urls: Option<Vec<String>>,
}

impl DevicePackageInstallIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::DevicePackageInstall)
            .unwrap()
            .to_string()
    }

    pub fn new_path(path: PathBuf) -> Box<DevicePackageInstallIncoming> {
        Box::new(Self {
            id: None,
            path: Some(path),
            urls: None,
        })
    }

    pub fn new_path_id(path: PathBuf, id: String) -> Box<DevicePackageInstallIncoming> {
        Box::new(Self {
            id: Some(id),
            path: Some(path),
            urls: None,
        })
    }

    pub fn new_urls(urls: Vec<String>) -> Box<DevicePackageInstallIncoming> {
        Box::new(Self {
            id: None,
            path: None,
            urls: Some(urls),
        })
    }

    pub fn new_urls_id(urls: Vec<String>, id: String) -> Box<DevicePackageInstallIncoming> {
        Box::new(Self {
            id: Some(id),
            path: None,
            urls: Some(urls),
        })
    }

    fn select(&self, id: String) -> DevicePackageInstallIncoming {
        let mut select = self.clone();
        select.id = Some(id);
        select
    }

    pub fn dbus_method_run_path(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "Path"),
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
            format!("{}{}", Self::name(), "PathById"),
            ("path", "id"),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (path, id): (String, String)| async move {
                let outgoing = match utils::path_to_absolute(&PathBuf::from(path)) {
                    Some(path) => Self::new_path_id(path, id).run(OutgoingType::Dbus),
                    None => StateMessageOutgoing::new_error(tr!("проверьте путь к файлу")),
                };
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    pub fn dbus_method_run_urls(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "Urls"),
            ("urls",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (urls,): (Vec<String>,)| async move {
                let outgoing = Self::new_urls(urls).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    pub fn dbus_method_run_urls_by_id(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "UrlsById"),
            ("urls", "id"),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (urls, id): (Vec<String>, String)| async move {
                let outgoing = Self::new_urls_id(urls, id).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    fn run_install_by_path(
        model: &DeviceModel,
        send_type: &OutgoingType,
        path: &PathBuf,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        // Check and sign package
        let psdk = match PsdkInstalledModel::get_latest() {
            Some(value) => value,
            None => Err(tr!("для проверки и подписи пакета необходим установить Platform SDK"))?,
        };
        if !command::psdk::rpm_is_sign(&psdk.chroot, path) && !command::psdk::rpm_sign(&psdk.chroot, path) {
            Err(tr!("валидация пакета не удалось"))?;
        }
        // Get package name from rpm
        let package_name = utils::get_package_name(path);
        if package_name.is_none() {
            Err(tr!("не удалось получить название пакета"))?;
        }
        // Get package name from rpm
        if let Some(package_arch) = utils::get_package_arch(path) {
            if package_arch != model.arch.to_string() {
                Err(tr!("неверная архитектура пакета"))?;
            }
        } else {
            Err(tr!("не удалось получить архитектуру пакета"))?;
        }
        // Get session
        let session = model.session_user()?;
        // Upload file
        StateMessageOutgoing::new_state(tr!("загружаем пакет...")).send(send_type);
        let path_remote = &session.file_upload(path, StateMessageOutgoing::get_state_callback_file_small(send_type))?;
        // Install by apm
        StateMessageOutgoing::new_state(tr!("установка пакета")).send(send_type);
        session.install_package(path_remote.clone(), package_name)?;
        // Remove if temp for psdk
        if path.to_string_lossy().contains("~temp_") {
            let _ = fs::remove_file(path);
        }
        // Success result
        Ok(StateMessageOutgoing::new_success(tr!("пакет успешно установлен")))
    }

    fn run_install_by_urls(
        model: &DeviceModel,
        send_type: &OutgoingType,
        urls: &Vec<String>,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        StateMessageOutgoing::new_state(tr!("скачиваем файл...")).send(send_type);
        let url = if urls.len() == 1 {
            urls.first().unwrap()
        } else {
            match urls.iter().filter(|e| e.contains(&model.arch)).next() {
                Some(value) => value,
                None => Err(tr!("не удалось найти ссылку для таргета: {}", model.arch))?,
            }
        };
        let url = match utils::get_https_url(url.to_string()) {
            Some(url) => url,
            None => Err(tr!("не удалось скачать файл"))?,
        };
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
        Self::run_install_by_path(model, send_type, &rpm_path_download)
    }
}

impl TraitIncoming for DevicePackageInstallIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = DevicePackageInstallIncoming::name();
        let models = DeviceModelSelect::search(&self.id, tr!("получаем информацию об устройствах"), &send_type);
        if self.path.as_ref().is_none() && self.urls.as_ref().is_none() {
            return StateMessageOutgoing::new_error(tr!("нужно указать путь или url к файлу"));
        }
        // Select
        match models.iter().count() {
            1 => {
                let emulator = models.last().unwrap();
                if self.path.as_ref().is_some() {
                    match Self::run_install_by_path(emulator, &send_type, self.path.as_ref().unwrap()) {
                        Ok(value) => value,
                        Err(error) => StateMessageOutgoing::new_error(tr!("{}", error)),
                    }
                } else {
                    match Self::run_install_by_urls(emulator, &send_type, self.urls.as_ref().unwrap()) {
                        Ok(value) => value,
                        Err(error) => StateMessageOutgoing::new_error(tr!("{}", error)),
                    }
                }
            }
            0 => StateMessageOutgoing::new_info(tr!("устройства не найдены")),
            _ => match DeviceModelSelect::select(key, &send_type, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить устройство")),
            },
        }
    }
}
