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

    pub fn new_id(is_root: bool, id: String) -> Box<EmulatorTerminalIncoming> {
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
            ("is_root", "id",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (is_root, id,): (bool, String,)| async move {
                let outgoing = Self::new_id(is_root, id).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    fn run(model: EmulatorModel, is_root: bool) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        if !model.is_running {
            Ok(StateMessageOutgoing::new_info(tr!("эмулятор должен быть запущен")))
        } else {
            let user = if is_root { "root" } else { "defaultuser" };
            // Run command
            let command = format!(
                "ssh -o 'ConnectTimeout=2' -o 'StrictHostKeyChecking=no' {}@localhost -p 2223 -i {}",
                user, model.key
            );
            // Try run terminal
            Ok(terminal::open(command))
        }
    }
}

impl TraitIncoming for EmulatorTerminalIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = EmulatorTerminalIncoming::name();
        let models = EmulatorModelSelect::search(
            &self.id,
            &send_type,
            tr!("ищем запущенный эмулятор для открытия терминала"),
            Some(true),
        );
        // Select
        match models.iter().count() {
            1 => match Self::run(models.first().unwrap().clone(), self.is_root) {
                Ok(value) => value,
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось запустить терминал")),
            },
            0 => StateMessageOutgoing::new_info(tr!("запущенные эмуляторы не найдены")),
            _ => match EmulatorModelSelect::select(key, &send_type, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить эмулятор")),
            },
        }
    }
}
