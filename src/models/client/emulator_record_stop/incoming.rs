use std::path::Path;

use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::state_message::outgoing::StateMessageOutgoing;
use crate::models::client::ClientMethodsKey;
use crate::models::emulator::model::EmulatorModel;
use crate::models::emulator::select::EmulatorModelSelect;
use crate::service::command::exec;
use crate::service::dbus::server::IfaceData;
use crate::tools::ffmpeg_utils;
use crate::tools::macros::tr;
use crate::tools::programs;
use crate::tools::utils;

use super::outgoing::EmulatorRecordStopOutgoing;

/// Common state client
#[derive(Deserialize, Serialize, Clone, PartialEq)]
pub enum EmulatorRecordStopType {
    Raw,
    Mp4,
    Gif,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorRecordStopIncoming {
    id: Option<String>,
    stop_type: EmulatorRecordStopType,
}

impl EmulatorRecordStopIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::EmulatorRecordStop)
            .unwrap()
            .to_string()
    }

    pub fn new(stop_type: EmulatorRecordStopType) -> Box<EmulatorRecordStopIncoming> {
        Box::new(Self { id: None, stop_type })
    }

    pub fn new_id(id: String, stop_type: EmulatorRecordStopType) -> Box<EmulatorRecordStopIncoming> {
        Box::new(Self {
            id: Some(id),
            stop_type,
        })
    }

    fn select(&self, id: String) -> EmulatorRecordStopIncoming {
        let mut select = self.clone();
        select.id = Some(id);
        select
    }

    pub fn dbus_method_run(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            Self::name(),
            ("stop_type",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (stop_type,): (String,)| async move {
                let outgoing = match serde_json::from_str::<EmulatorRecordStopType>(&stop_type) {
                    Ok(value) => Self::new(value).run(OutgoingType::Dbus),
                    Err(_) => StateMessageOutgoing::new_error(tr!("указан не верный тип: Raw, Mp4, Gif")),
                };
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    pub fn dbus_method_run_by_id(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "ById"),
            ("id", "stop_type"),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (id, stop_type): (String, String)| async move {
                let outgoing = match serde_json::from_str::<EmulatorRecordStopType>(&stop_type) {
                    Ok(value) => Self::new_id(id, value).run(OutgoingType::Dbus),
                    Err(_) => StateMessageOutgoing::new_error(tr!("указан не верный тип: Raw, Mp4, Gif")),
                };
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    fn stop(
        emulator: EmulatorModel,
        send_type: &OutgoingType,
        stop_type: &EmulatorRecordStopType,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        if !emulator.is_running {
            return Ok(StateMessageOutgoing::new_info(tr!("эмулятор должен быть запущен")));
        }
        if !emulator.is_recording() {
            return Ok(StateMessageOutgoing::new_info(tr!("запись видео не активна")));
        }
        let uuid = emulator.uuid.as_str();
        let program = programs::get_vboxmanage()?;
        let output = exec::exec_wait_args(&program, ["controlvm", uuid, "recording", "off"])?;
        if !output.status.success() {
            Err("не удалось остановить запись видео")?
        }
        let name = emulator.name;
        let path_raw = Path::new(&emulator.dir)
            .join("emulator")
            .join(&name)
            .join(&name)
            .join(format!("{}-screen0.webm", &name));
        let outgoing = match stop_type {
            EmulatorRecordStopType::Raw => {
                EmulatorRecordStopOutgoing::new(path_raw.to_string_lossy().to_string(), None)
            }
            EmulatorRecordStopType::Mp4 => {
                match ffmpeg_utils::webm_to_mp4(&path_raw, Self::get_state_callback(&send_type)) {
                    Ok(value) => EmulatorRecordStopOutgoing::new(
                        value.to_string_lossy().to_string(),
                        utils::file_to_base64_by_path(value.to_str()),
                    ),
                    Err(_) => EmulatorRecordStopOutgoing::new(path_raw.to_string_lossy().to_string(), None),
                }
            }
            EmulatorRecordStopType::Gif => {
                match ffmpeg_utils::webm_to_gif(&path_raw, Self::get_state_callback(&send_type)) {
                    Ok(value) => EmulatorRecordStopOutgoing::new(value.to_string_lossy().to_string(), None),
                    Err(_) => EmulatorRecordStopOutgoing::new(path_raw.to_string_lossy().to_string(), None),
                }
            }
        };
        Ok(outgoing)
    }

    fn get_state_callback(send_type: &OutgoingType) -> fn(i32) {
        match send_type {
            OutgoingType::Cli => |progress| {
                if progress < 0 {
                    match progress {
                        -1 => StateMessageOutgoing::new_state(tr!("получение данных...")).send(&OutgoingType::Cli),
                        -2 => StateMessageOutgoing::new_state(tr!("причесываем данные...")).send(&OutgoingType::Cli),
                        -3 => StateMessageOutgoing::new_state(tr!("начинаем конвертацию")).send(&OutgoingType::Cli),
                        _ => {}
                    }
                } else {
                    StateMessageOutgoing::new_progress(progress.to_string()).send(&OutgoingType::Cli);
                }
            },
            OutgoingType::Dbus => |progress| {
                if progress < 0 {
                    match progress {
                        -1 => StateMessageOutgoing::new_state(tr!("получение данных...")).send(&OutgoingType::Dbus),
                        -2 => StateMessageOutgoing::new_state(tr!("причесываем данные...")).send(&OutgoingType::Dbus),
                        -3 => StateMessageOutgoing::new_state(tr!("начинаем конвертацию")).send(&OutgoingType::Dbus),
                        _ => {}
                    }
                } else {
                    StateMessageOutgoing::new_progress(progress.to_string()).send(&OutgoingType::Dbus);
                }
            },
            OutgoingType::Websocket => {
                |progress| {
                    if progress < 0 {
                        match progress {
                            -1 => StateMessageOutgoing::new_state(tr!("получение данных..."))
                                .send(&OutgoingType::Websocket),
                            -2 => StateMessageOutgoing::new_state(tr!("причесываем данные..."))
                                .send(&OutgoingType::Websocket),
                            -3 => StateMessageOutgoing::new_state(tr!("начинаем конвертацию"))
                                .send(&OutgoingType::Websocket),
                            _ => {}
                        }
                    } else {
                        if progress % 10 == 0 {
                            StateMessageOutgoing::new_progress(progress.to_string()).send(&OutgoingType::Websocket);
                        }
                    }
                }
            }
        }
    }
}

impl TraitIncoming for EmulatorRecordStopIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = EmulatorRecordStopIncoming::name();
        let models: Vec<EmulatorModel> = EmulatorModelSelect::search(
            &self.id,
            &send_type,
            tr!("ищем эмулятор для остановки записи видео"),
            Some(true),
        );
        // Select
        match models.iter().count() {
            1 => match Self::stop(models.first().unwrap().clone(), &send_type, &self.stop_type) {
                Ok(result) => result,
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось остановить запись видео")),
            },
            0 => StateMessageOutgoing::new_info(tr!("запущенные эмуляторы не найдены")),
            _ => Box::new(EmulatorModelSelect::select(key, models, |id| self.select(id))),
        }
    }
}
