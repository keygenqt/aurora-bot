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
use crate::tools::macros::tr;
use crate::tools::programs;

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorOpenIncoming {
    id: Option<String>,
    is_vnc: bool,
    password: Option<String>,
    port: Option<u64>,
}

impl EmulatorOpenIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::EmulatorOpen)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<EmulatorOpenIncoming> {
        Box::new(Self {
            id: None,
            is_vnc: false,
            password: None,
            port: None,
        })
    }

    pub fn new_id(id: String) -> Box<EmulatorOpenIncoming> {
        Box::new(Self {
            id: Some(id),
            is_vnc: false,
            password: None,
            port: None,
        })
    }

    pub fn new_vnc(password: String, port: u64) -> Box<EmulatorOpenIncoming> {
        Box::new(Self {
            id: None,
            is_vnc: true,
            password: Some(password),
            port: Some(port),
        })
    }

    pub fn new_vnc_id(password: String, port: u64, id: String) -> Box<EmulatorOpenIncoming> {
        Box::new(Self {
            id: Some(id),
            is_vnc: true,
            password: Some(password),
            port: Some(port),
        })
    }

    fn select(&self, id: String) -> EmulatorOpenIncoming {
        let mut select = self.clone();
        select.id = Some(id);
        select
    }

    pub fn dbus_method_run(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            Self::name(),
            (),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (): ()| async move {
                let outgoing = Self::new().run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
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
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    pub fn dbus_method_run_vnc(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "Vnc"),
            ("password", "port"),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (password, port): (String, u64)| async move {
                let outgoing = Self::new_vnc(password, port).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    pub fn dbus_method_run_vnc_by_id(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "VncById"),
            ("password", "port", "id"),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (password, port, id): (String, u64, String)| async move {
                let outgoing = Self::new_vnc_id(password, port, id).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    fn run(
        model: EmulatorModel,
        send_type: &OutgoingType,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        if !model.is_running {
            StateMessageOutgoing::new_state(tr!("открываем эмулятор")).send(send_type);
            model.start()?;
        }
        StateMessageOutgoing::new_state(tr!("соединение с эмулятором")).send(send_type);
        // Get emulator connect session
        let model = model.session_user()?;
        // Close connect
        model.close()?;
        // Done
        Ok(StateMessageOutgoing::new_success(tr!(
            "эмулятор {} готов к работе",
            model.os_name
        )))
    }

    fn run_vnc(
        model: EmulatorModel,
        send_type: &OutgoingType,
        password: Option<String>,
        port: Option<u64>,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        if model.is_running {
            Ok(StateMessageOutgoing::new_info(tr!("эмулятор уже запущен")))
        } else {
            StateMessageOutgoing::new_state(tr!("открываем эмулятор")).send(send_type);

            let uuid = model.uuid.as_str();
            let program = programs::get_vboxmanage()?;
            let output = exec::exec_wait_args(&program, ["setproperty", "vrdeextpack", "VNC"])?;
            if !output.status.success() {
                Err(tr!("не удалось изменить настройки"))?
            } else {
                StateMessageOutgoing::new_state(tr!("включен VirtualBox VNC")).send(send_type);
            }
            let password = password.unwrap_or_else(|| "00000".to_string());
            let output = exec::exec_wait_args(
                &program,
                ["modifyvm", uuid, "--vrdeproperty", &format!("VNCPassword={}", password)],
            )?;
            if !output.status.success() {
                Err(tr!("не удалось установить пароль"))?
            } else {
                StateMessageOutgoing::new_info(tr!("установлен пароль: <code>{}</code>", password)).send(send_type);
            }
            let port = &port.unwrap_or_else(|| 3389).to_string();
            let output = exec::exec_wait_args(&program, ["modifyvm", uuid, "--vrde-port", port])?;
            if !output.status.success() {
                Err(tr!("не удалось установить порт"))?
            } else {
                StateMessageOutgoing::new_info(tr!("установлен порт: <code>{}</code>", port)).send(send_type);
            }
            let output = exec::exec_wait_args(&program, ["modifyvm", uuid, "--vrde", "on"])?;
            if !output.status.success() {
                Err(tr!("не удалось включить vrde"))?
            }
            let output = exec::exec_wait_args(&program, ["startvm", uuid, "--type", "headless"])?;
            if !output.status.success() {
                Err(tr!("не удалось запустить эмулятор headless"))?
            }
            Ok(StateMessageOutgoing::new_success(tr!("эмулятор успешно запущен")))
        }
    }
}

impl TraitIncoming for EmulatorOpenIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = EmulatorOpenIncoming::name();
        let models = EmulatorModelSelect::search(
            &self.id,
            &send_type,
            tr!("ищем эмулятор который можно открыть"),
            Some(false),
        );
        // Select
        match models.iter().count() {
            1 => {
                if self.is_vnc {
                    match Self::run_vnc(
                        models.first().unwrap().clone(),
                        &send_type,
                        self.password.clone(),
                        self.port,
                    ) {
                        Ok(result) => result,
                        Err(_) => StateMessageOutgoing::new_error(tr!("не удалось открыть эмулятор")),
                    }
                } else {
                    match Self::run(models.first().unwrap().clone(), &send_type) {
                        Ok(result) => result,
                        Err(_) => StateMessageOutgoing::new_error(tr!("не удалось открыть эмулятор")),
                    }
                }
            }
            0 => StateMessageOutgoing::new_info(tr!("эмуляторы не найдены")),
            _ => match EmulatorModelSelect::select(key, &send_type, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить эмулятор")),
            },
        }
    }
}
