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

    fn action_disable(
        emulator: EmulatorModel,
        send_type: &OutgoingType,
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

        // Callback
        let fun: fn(usize) = match send_type {
            OutgoingType::Cli => |index| {
                if index == 0 {
                    StateMessageOutgoing::new_state(tr!("получение данных...")).send(&OutgoingType::Cli);
                } else if index == 1 {
                    StateMessageOutgoing::new_state(tr!("причесываем данные...")).send(&OutgoingType::Cli);
                } else if index == 2 {
                    StateMessageOutgoing::new_state(tr!("начинаем конвертацию")).send(&OutgoingType::Cli);
                } else {
                    if index % 10 == 0 {
                        StateMessageOutgoing::new_state(tr!("конвертация файла {}%", index)).send(&OutgoingType::Cli);
                    }
                }
            },
            OutgoingType::Dbus => |index| {
                if index == 0 {
                    StateMessageOutgoing::new_state(tr!("получение данных...")).send(&OutgoingType::Dbus);
                } else if index == 1 {
                    StateMessageOutgoing::new_state(tr!("причесываем данные...")).send(&OutgoingType::Dbus);
                } else if index == 2 {
                    StateMessageOutgoing::new_state(tr!("начинаем конвертацию")).send(&OutgoingType::Dbus);
                } else {
                    StateMessageOutgoing::new_state(tr!("конвертация файла {}%", index)).send(&OutgoingType::Dbus);
                }
            },
            OutgoingType::Websocket => |index| {
                if index == 0 {
                    StateMessageOutgoing::new_state(tr!("получение данных...")).send(&OutgoingType::Websocket);
                } else if index == 1 {
                    StateMessageOutgoing::new_state(tr!("причесываем данные...")).send(&OutgoingType::Websocket);
                } else if index == 2 {
                    StateMessageOutgoing::new_state(tr!("начинаем конвертацию")).send(&OutgoingType::Websocket);
                } else {
                    if index % 25 == 0 {
                        StateMessageOutgoing::new_state(tr!("конвертация файла {}%", index))
                            .send(&OutgoingType::Websocket);
                    }
                }
            },
        };
        // @todo: modes: gif or mp4
        let outgoing = match ffmpeg_utils::webm_to_mp4(&path_raw, fun) {
            Ok(value) => EmulatorRecordOutgoing::new(value.to_string_lossy().to_string(), value.to_str()),
            Err(_) => EmulatorRecordOutgoing::new(path_raw.to_string_lossy().to_string(), None),
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
        let models: Vec<EmulatorModel> = EmulatorModelSelect::search(&self.id, &send_type, text, Some(false));
        // Select
        match models.iter().count() {
            1 => {
                if self.enable {
                    match Self::action_enable(models.first().unwrap().clone()) {
                        Ok(result) => result,
                        Err(_) => StateMessageOutgoing::new_error(tr!("не удалось активировать запись видео")),
                    }
                } else {
                    match Self::action_disable(models.first().unwrap().clone(), &send_type) {
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
