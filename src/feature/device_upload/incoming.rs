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
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::print_debug;
use crate::tools::macros::tr;
use crate::tools::single;
use crate::tools::utils;

#[derive(Serialize, Deserialize, Clone)]
pub struct DeviceUploadIncoming {
    id: Option<String>,
    path: Option<PathBuf>,
    url: Option<String>,
}

impl DeviceUploadIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::DeviceUpload)
            .unwrap()
            .to_string()
    }

    pub fn new_path(path: PathBuf) -> Box<DeviceUploadIncoming> {
        print_debug!("> {}: new_path(path: {})", Self::name(), path.to_string_lossy());
        Box::new(Self {
            id: None,
            path: Some(path),
            url: None,
        })
    }

    pub fn new_path_id(id: String, path: PathBuf) -> Box<DeviceUploadIncoming> {
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
        })
    }

    pub fn new_url(url: String) -> Box<DeviceUploadIncoming> {
        print_debug!("> {}: new_url(url: {})", Self::name(), url);
        Box::new(Self {
            id: None,
            path: None,
            url: Some(url),
        })
    }

    pub fn new_url_id(id: String, url: String) -> Box<DeviceUploadIncoming> {
        print_debug!("> {}: new_url_id(id: {}, url: {})", Self::name(), id, url);
        Box::new(Self {
            id: Some(id),
            path: None,
            url: Some(url),
        })
    }

    fn select(&self, id: String) -> DeviceUploadIncoming {
        let mut select = self.clone();
        select.id = Some(id);
        select
    }

    pub fn dbus_method_run_path(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "ByPath"),
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
            format!("{}{}", Self::name(), "ByPathId"),
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

    fn run_by_path(
        model: &DeviceModel,
        send_type: &OutgoingType,
        path: &PathBuf,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        StateMessageOutgoing::new_state(tr!("начинаем загрузку...")).send(send_type);
        model
            .session_user()?
            .file_upload(path, StateMessageOutgoing::get_state_callback_file_small(send_type))?;
        Ok(StateMessageOutgoing::new_success(tr!("файл успешно загружен")))
    }

    fn run_by_url(
        model: &DeviceModel,
        send_type: &OutgoingType,
        url: &String,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        let session = model.session_user()?;
        let url = match utils::get_https_url(url.to_string()) {
            Some(url) => url,
            None => Err(tr!("не удалось скачать файл"))?,
        };
        StateMessageOutgoing::new_state(tr!("скачиваем файл...")).send(send_type);
        let path = single::get_request().download_file(
            url.to_string(),
            StateMessageOutgoing::get_state_callback_file_small(send_type),
        )?;
        StateMessageOutgoing::new_state(tr!("начинаем загрузку...")).send(send_type);
        session.file_upload(&path, StateMessageOutgoing::get_state_callback_file_small(send_type))?;
        Ok(StateMessageOutgoing::new_success(tr!("файл успешно загружен")))
    }
}

impl TraitIncoming for DeviceUploadIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = DeviceUploadIncoming::name();
        let models = DeviceModelSelect::search(&self.id, tr!("получаем информацию об устройствах"), &send_type);
        if self.path.as_ref().is_none() && self.url.as_ref().is_none() {
            return StateMessageOutgoing::new_error(tr!("нужно указать путь или url к файлу"));
        }
        // Select
        match models.iter().count() {
            1 => {
                let emulator = models.last().unwrap();
                if self.path.as_ref().is_some() {
                    match Self::run_by_path(emulator, &send_type, self.path.as_ref().unwrap()) {
                        Ok(value) => value,
                        Err(error) => StateMessageOutgoing::new_error(tr!("{}", error)),
                    }
                } else {
                    match Self::run_by_url(emulator, &send_type, self.url.as_ref().unwrap()) {
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
