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

use super::outgoing::EmulatorRecordOutgoing;

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorRecordIncoming {
    id: Option<String>,
    enable: bool,
}

impl EmulatorRecordIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::EmulatorRecord)
            .unwrap()
            .to_string()
    }

    pub fn new(enable: bool) -> Box<EmulatorRecordIncoming> {
        Box::new(Self { id: None, enable })
    }

    pub fn new_id(id: String, enable: bool) -> Box<EmulatorRecordIncoming> {
        Box::new(Self { id: Some(id), enable })
    }

    fn select(&self, id: String) -> EmulatorRecordIncoming {
        let mut select = self.clone();
        select.id = Some(id);
        select
    }

    pub fn dbus_method_run(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            Self::name(),
            ("enable",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (enable,): (bool,)| async move {
                let outgoing = Self::new(enable).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    pub fn dbus_method_run_by_id(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "ById"),
            ("id", "enable"),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (id, enable): (String, bool)| async move {
                let outgoing = Self::new_id(id, enable).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    fn action_enable(emulator: EmulatorModel) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        if !emulator.is_running {
            return Ok(StateMessageOutgoing::new_info(tr!("эмулятор должен быть запущен")));
        }
        if emulator.is_recording() {
            return Ok(StateMessageOutgoing::new_info(tr!("запись видео уже активирована")));
        }
        let uuid = emulator.uuid.as_str();
        let program = programs::get_vboxmanage()?;
        let output = exec::exec_wait_args(&program, ["controlvm", uuid, "recording", "on"])?;
        if !output.status.success() {
            Err("не удалось активировать запись видео")?
        }
        Ok(StateMessageOutgoing::new_success(tr!("запись видео активирована")))
    }

    fn action_disable(emulator: EmulatorModel) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
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
        // Crop, convert to mp4, gen gif preview
        let outgoing = match ffmpeg_utils::ffmpeg_webm_convert(&path_raw) {
            Ok(values) => {
                EmulatorRecordOutgoing::new(values.0.to_string_lossy().to_string(), Some(values.1))
            },
            Err(_) => {
                EmulatorRecordOutgoing::new(path_raw.to_string_lossy().to_string(), None)
            },
        };
        Ok(outgoing)
    }
}

impl TraitIncoming for EmulatorRecordIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = EmulatorRecordIncoming::name();
        let text = if self.enable {
            tr!("ищем эмулятор для включения записи видео")
        } else {
            tr!("ищем эмулятор для остановки записи видео")
        };
        let models: Vec<EmulatorModel> = EmulatorModelSelect::search(&self.id, &send_type, text, Some(true));
        // Select
        match models.iter().count() {
            1 => {
                if self.enable {
                    match Self::action_enable(models.first().unwrap().clone()) {
                        Ok(result) => result,
                        Err(_) => StateMessageOutgoing::new_error(tr!("не удалось активировать запись видео")),
                    }
                } else {
                    match Self::action_disable(models.first().unwrap().clone()) {
                        Ok(result) => result,
                        Err(_) => StateMessageOutgoing::new_error(tr!("не удалось остановить запись видео")),
                    }
                }
            }
            0 => StateMessageOutgoing::new_info(tr!("запущенные эмуляторы не найдены")),
            _ => Box::new(EmulatorModelSelect::select(key, models, |id| self.select(id))),
        }
    }
}
