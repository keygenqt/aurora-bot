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
    is_root: bool,
}

impl EmulatorTerminalIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::EmulatorTerminal)
            .unwrap()
            .to_string()
    }

    pub fn new(is_root: bool) -> Box<EmulatorTerminalIncoming> {
        Box::new(Self { id: None, is_root })
    }

    pub fn new_id(id: String, is_root: bool) -> Box<EmulatorTerminalIncoming> {
        Box::new(Self { id: Some(id), is_root })
    }

    fn select(&self, id: String) -> EmulatorTerminalIncoming {
        let mut select = self.clone();
        select.id = Some(id);
        select
    }

    pub fn dbus_method_run(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            Self::name(),
            ("is_root",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (is_root,): (bool,)| async move {
                let outgoing = Self::new(is_root).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    pub fn dbus_method_run_by_id(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "ById"),
            ("id", "is_root"),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (id, is_root): (String, bool)| async move {
                let outgoing = Self::new_id(id, is_root).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }
}

impl TraitIncoming for EmulatorTerminalIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = EmulatorTerminalIncoming::name();
        let models: Vec<EmulatorModel> = EmulatorModelSelect::search(
            &self.id,
            &send_type,
            tr!("ищем запущенный эмулятор для открытия терминала"),
            Some(true),
        );
        // Exec fun
        fn _run(emulator: EmulatorModel, is_root: bool) -> Box<dyn TraitOutgoing> {
            if !emulator.is_running {
                return StateMessageOutgoing::new_info(tr!("эмулятор должен быть запущен"));
            } else {
                let user = if is_root { "root" } else { "defaultuser" };
                // Run command
                let command = format!(
                    "ssh -o 'ConnectTimeout=2' -o 'StrictHostKeyChecking=no' {}@localhost -p 2223 -i {}",
                    user, emulator.key
                );
                // Try run terminal
                terminal::open(command)
            }
        }
        // Select
        match models.iter().count() {
            1 => _run(models.first().unwrap().clone(), self.is_root),
            0 => StateMessageOutgoing::new_info(tr!("запущенные эмуляторы не найдены")),
            _ => Box::new(EmulatorModelSelect::select(key, models, |id| self.select(id))),
        }
    }
}
