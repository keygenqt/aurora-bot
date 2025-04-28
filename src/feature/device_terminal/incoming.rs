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
use crate::tools::programs;
use crate::tools::terminal;

#[derive(Serialize, Deserialize, Clone)]
pub struct DeviceTerminalIncoming {
    id: Option<String>,
}

impl DeviceTerminalIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::DeviceTerminal)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<DeviceTerminalIncoming> {
        print_debug!("> {}: new()", Self::name());
        Box::new(Self { id: None })
    }

    pub fn new_id(id: String) -> Box<DeviceTerminalIncoming> {
        print_debug!("> {}: new_id(id: {})", Self::name(), id);
        Box::new(Self { id: Some(id) })
    }

    fn select(&self, id: String) -> DeviceTerminalIncoming {
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

    fn run(model: DeviceModel, send_type: &OutgoingType) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        if let Some(path) = model.path {
            let command = format!(
                "ssh -o 'ConnectTimeout=2' -o 'StrictHostKeyChecking=no' defaultuser@{} -p {} -i {}",
                model.port, model.host, path,
            );
            // Try run terminal
            return Ok(terminal::open(command));
        }
        if let Some(pass) = model.pass {
            let sshpass = programs::get_sshpass();
            let command = if sshpass.is_err() {
                StateMessageOutgoing::new_warning(tr!(
                    "установите пакет sshpass для входе без пароля, или используйте ключ для авторизации"
                ))
                .send(send_type);
                format!(
                    "ssh -o 'ConnectTimeout=2' -o 'StrictHostKeyChecking=no' defaultuser@{} -p {}",
                    model.host, model.port,
                )
            } else {
                format!(
                    "sshpass -p '{}' ssh -o 'ConnectTimeout=2' -o 'StrictHostKeyChecking=no' defaultuser@{} -p {}",
                    pass, model.host, model.port,
                )
            };
            // Try run terminal
            return Ok(terminal::open(command));
        }
        Err("не удалось найти тип соединения")?
    }
}

impl TraitIncoming for DeviceTerminalIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = DeviceTerminalIncoming::name();
        let models = DeviceModelSelect::search(&self.id, tr!("получаем список устройств"), &send_type);
        // Select
        match models.iter().count() {
            1 => match Self::run(models.first().unwrap().clone(), &send_type) {
                Ok(value) => value,
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось запустить терминал")),
            },
            0 => StateMessageOutgoing::new_info(tr!("активные устройства не найдены")),
            _ => match DeviceModelSelect::select(key, &send_type, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить устройство")),
            },
        }
    }
}
