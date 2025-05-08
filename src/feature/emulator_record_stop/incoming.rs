use std::path::Path;

use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::selector::selects::select_emulator::EmulatorModelSelect;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::models::emulator::model::EmulatorModel;
use crate::service::command::exec;
use crate::service::dbus::server::IfaceData;
use crate::tools::ffmpeg_utils;
use crate::tools::macros::tr;
use crate::tools::programs;
use crate::tools::utils;

use super::outgoing::EmulatorRecordStopOutgoing;

/// Common state client
#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
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

    pub fn new_id(stop_type: EmulatorRecordStopType, id: String) -> Box<EmulatorRecordStopIncoming> {
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
            ("stop_type", "id",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (stop_type, id,): (String, String,)| async move {
                let outgoing = match serde_json::from_str::<EmulatorRecordStopType>(&stop_type) {
                    Ok(value) => Self::new_id(value, id).run(OutgoingType::Dbus),
                    Err(_) => StateMessageOutgoing::new_error(tr!("указан не верный тип: Raw, Mp4, Gif")),
                };
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    fn run(
        model: EmulatorModel,
        send_type: &OutgoingType,
        stop_type: &EmulatorRecordStopType,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        if !model.is_running {
            return Ok(StateMessageOutgoing::new_info(tr!("эмулятор должен быть запущен")));
        }
        if !model.is_recording() {
            return Ok(StateMessageOutgoing::new_info(tr!("запись видео не активна")));
        }
        let uuid = model.uuid.as_str();
        let program = programs::get_vboxmanage()?;
        let output = exec::exec_wait_args(&program, ["controlvm", uuid, "recording", "off"])?;
        if !output.status.success() {
            Err(tr!("не удалось остановить запись видео"))?
        }
        let name = model.name;
        let path_raw = Path::new(&model.dir)
            .join("emulator")
            .join(&name)
            .join(&name)
            .join(format!("{}-screen0.webm", &name));
        let outgoing = match stop_type {
            EmulatorRecordStopType::Raw => {
                EmulatorRecordStopOutgoing::new(path_raw.to_string_lossy().to_string(), None)
            }
            EmulatorRecordStopType::Mp4 => {
                match ffmpeg_utils::webm_to_mp4(
                    &path_raw,
                    StateMessageOutgoing::get_state_callback_file_small(&send_type),
                ) {
                    Ok(value) => EmulatorRecordStopOutgoing::new(
                        value.to_string_lossy().to_string(),
                        utils::file_to_base64_by_path(value.to_str()),
                    ),
                    Err(_) => EmulatorRecordStopOutgoing::new(path_raw.to_string_lossy().to_string(), None),
                }
            }
            EmulatorRecordStopType::Gif => {
                match ffmpeg_utils::webm_to_gif(
                    &path_raw,
                    StateMessageOutgoing::get_state_callback_file_small(&send_type),
                ) {
                    Ok(value) => EmulatorRecordStopOutgoing::new(value.to_string_lossy().to_string(), None),
                    Err(_) => EmulatorRecordStopOutgoing::new(path_raw.to_string_lossy().to_string(), None),
                }
            }
        };
        Ok(outgoing)
    }
}

impl TraitIncoming for EmulatorRecordStopIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = EmulatorRecordStopIncoming::name();
        let models = EmulatorModelSelect::search(
            &self.id,
            &send_type,
            tr!("ищем эмулятор для остановки записи видео"),
            Some(true),
        );
        // Select
        match models.iter().count() {
            1 => match Self::run(models.first().unwrap().clone(), &send_type, &self.stop_type) {
                Ok(result) => result,
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось остановить запись видео")),
            },
            0 => StateMessageOutgoing::new_info(tr!("запущенные эмуляторы не найдены")),
            _ => match EmulatorModelSelect::select(key, &send_type, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить эмулятор")),
            },
        }
    }
}
