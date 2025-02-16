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
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::tr;
use crate::tools::terminal;

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorTerminalIncoming {
    id: Option<String>,
}

impl EmulatorTerminalIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::EmulatorTerminal)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<EmulatorTerminalIncoming> {
        Box::new(Self { id: None })
    }

    pub fn new_id(id: String) -> Box<EmulatorTerminalIncoming> {
        Box::new(Self { id: Some(id) })
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

impl TraitIncoming for EmulatorTerminalIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = EmulatorTerminalIncoming::name();
        let models: Vec<EmulatorModel> = EmulatorModelSelect::search(&self.id, &send_type, Some(true));
        // Exec fun
        fn _run(emulator: EmulatorModel) -> Box<dyn TraitOutgoing> {
            if !emulator.is_running {
                return StateMessageOutgoing::new_info(tr!("эмулятор должен быть запущен"));
            } else {
                // Run command
                let command = format!(
                    "ssh -o 'ConnectTimeout=2' -o 'StrictHostKeyChecking=no' defaultuser@localhost -p 2223 -i {}",
                    emulator.key
                );
                // Try run terminal
                terminal::open(command)
            }
        }
        // Select
        match models.iter().count() {
            1 => _run(models.first().unwrap().clone()),
            0 => StateMessageOutgoing::new_info(tr!("запущенные эмуляторы не найдены")),
            _ => Box::new(EmulatorModelSelect::select(key, models, |id| {
                *EmulatorTerminalIncoming::new_id(id)
            })),
        }
    }
}
