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
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::print_debug;
use crate::tools::macros::tr;
use crate::tools::single;
use crate::tools::utils;

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorUploadIncoming {
    id: Option<String>,
    path: Option<PathBuf>,
    url: Option<String>,
}

impl EmulatorUploadIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::EmulatorUpload)
            .unwrap()
            .to_string()
    }

    pub fn new_path(path: PathBuf) -> Box<EmulatorUploadIncoming> {
        print_debug!("> {}: new_path(path: {})", Self::name(), path.to_string_lossy());
        Box::new(Self {
            id: None,
            path: Some(path),
            url: None,
        })
    }

    pub fn new_path_id(id: String, path: PathBuf) -> Box<EmulatorUploadIncoming> {
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

    pub fn new_url(url: String) -> Box<EmulatorUploadIncoming> {
        print_debug!("> {}: new_url(url: {})", Self::name(), url);
        Box::new(Self {
            id: None,
            path: None,
            url: Some(url),
        })
    }

    pub fn new_url_id(id: String, url: String) -> Box<EmulatorUploadIncoming> {
        print_debug!("> {}: new_url_id(id: {}, url: {})", Self::name(), id, url);
        Box::new(Self {
            id: Some(id),
            path: None,
            url: Some(url),
        })
    }

    fn select(&self, id: String) -> EmulatorUploadIncoming {
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

    fn upload_by_path(
        emulator: &EmulatorModel,
        send_type: &OutgoingType,
        path: &PathBuf,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        StateMessageOutgoing::new_state(tr!("начинаем загрузку...")).send(send_type);
        emulator
            .session_user()?
            .file_upload(path, StateMessageOutgoing::get_state_callback_file_small(send_type))?;
        Ok(StateMessageOutgoing::new_success(tr!("файл успешно загружен")))
    }

    fn upload_by_url(
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
        StateMessageOutgoing::new_state(tr!("начинаем загрузку...")).send(send_type);
        emulator
            .session_user()?
            .file_upload(&path, StateMessageOutgoing::get_state_callback_file_small(send_type))?;
        Ok(StateMessageOutgoing::new_success(tr!("файл успешно загружен")))
    }
}

impl TraitIncoming for EmulatorUploadIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = EmulatorUploadIncoming::name();
        let models: Vec<EmulatorModel> = EmulatorModelSelect::search(
            &self.id,
            &send_type,
            tr!("ищем запущенный эмулятор для загрузки"),
            Some(true),
        );
        if self.path.as_ref().is_none() && self.url.as_ref().is_none() {
            return StateMessageOutgoing::new_error(tr!("нужно указать путь или url к файлу"));
        }
        // Select
        match models.iter().count() {
            1 => {
                let emulator = models.last().unwrap();
                if !emulator.is_running {
                    StateMessageOutgoing::new_info(tr!("эмулятор должен быть запущен"))
                } else {
                    if self.path.as_ref().is_some() {
                        match Self::upload_by_path(emulator, &send_type, self.path.as_ref().unwrap()) {
                            Ok(value) => value,
                            Err(error) => StateMessageOutgoing::new_error(tr!("{}", error)),
                        }
                    } else {
                        match Self::upload_by_url(emulator, &send_type, self.url.as_ref().unwrap()) {
                            Ok(value) => value,
                            Err(error) => StateMessageOutgoing::new_error(tr!("{}", error)),
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
