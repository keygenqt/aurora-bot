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
use crate::tools::macros::tr;
use crate::tools::programs;

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorOpenVncIncoming {
    id: Option<String>,
    password: String,
    port: String,
}

// @todo Add to server
impl EmulatorOpenVncIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::EmulatorOpenVnc)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<EmulatorOpenVncIncoming> {
        Box::new(Self {
            id: None,
            password: "00000".to_string(),
            port: "3389".to_string(),
        })
    }

    pub fn new_id(id: String) -> Box<EmulatorOpenVncIncoming> {
        Box::new(Self {
            id: Some(id),
            password: "00000".to_string(),
            port: "3389".to_string(),
        })
    }

    pub fn dbus_method_run(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            Self::name(),
            (),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (): ()| async move {
                let outgoing = Self::new().run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_string(),)))
            },
        );
    }

    pub fn dbus_method_run_by_id(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "ById"),
            ("id",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (id,): (String,)| async move {
                let outgoing = Self::new_id(id).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_string(),)))
            },
        );
    }
}

impl TraitIncoming for EmulatorOpenVncIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = EmulatorOpenVncIncoming::name();
        let models: Vec<EmulatorModel> = EmulatorModelSelect::search(&self.id, &send_type, Some(false));
        // Exec fun
        fn _run(
            emulator: EmulatorModel,
            password: &String,
            port: &String,
        ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
            if emulator.is_running {
                Ok(StateMessageOutgoing::new_info(tr!("эмулятор уже запущен")))
            } else {
                let uuid = emulator.uuid.as_str();
                let program = programs::get_vboxmanage()?;
                let output = exec::exec_wait_args(&program, ["setproperty", "vrdeextpack", "VNC"])?;
                if !output.status.success() {
                    Err("не удалось изменить настройки")?
                }
                let output = exec::exec_wait_args(
                    &program,
                    ["modifyvm", uuid, "--vrdeproperty", &format!("VNCPassword={}", password)],
                )?;
                if !output.status.success() {
                    Err(format!("не удалось установить пароль"))?
                }
                let output = exec::exec_wait_args(&program, ["modifyvm", uuid, "--vrde-port", &port])?;
                if !output.status.success() {
                    Err(format!("не удалось установить порт {}", port))?
                }
                let output = exec::exec_wait_args(&program, ["modifyvm", uuid, "--vrde", "on"])?;
                if !output.status.success() {
                    Err("не удалось включить vrde")?
                }
                let output = exec::exec_wait_args(&program, ["startvm", uuid, "--type", "headless"])?;
                if !output.status.success() {
                    Err("не удалось запустить эмулятор headless")?
                }
                Ok(StateMessageOutgoing::new_success(tr!("эмулятор успешно запущен")))
            }
        }
        // Select
        match models.iter().count() {
            1 => match _run(models.first().unwrap().clone(), &self.password, &self.port) {
                Ok(result) => result,
                Err(error) => StateMessageOutgoing::new_error(tr!("{}", error)),
            },
            0 => StateMessageOutgoing::new_info(tr!("эмуляторы не найдены")),
            _ => Box::new(EmulatorModelSelect::select(key, models, |id| {
                *EmulatorOpenVncIncoming::new_id(id)
            })),
        }
    }
}
