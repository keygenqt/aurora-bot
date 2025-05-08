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

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorCloseIncoming {
    id: Option<String>,
}

impl EmulatorCloseIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::EmulatorClose)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<EmulatorCloseIncoming> {
        Box::new(Self { id: None })
    }

    pub fn new_id(id: String) -> Box<EmulatorCloseIncoming> {
        Box::new(Self { id: Some(id) })
    }

    fn select(&self, id: String) -> EmulatorCloseIncoming {
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

    fn run(model: EmulatorModel) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        model.close()?;
        Ok(StateMessageOutgoing::new_success(tr!("эмулятор закрыт успешно")))
    }
}

impl TraitIncoming for EmulatorCloseIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = EmulatorCloseIncoming::name();
        let models = EmulatorModelSelect::search(&self.id, &send_type, tr!("ищем чего бы остановить"), Some(true));
        // Select
        match models.iter().count() {
            1 => match Self::run(models.first().unwrap().clone()) {
                Ok(result) => result,
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось закрыть эмулятор")),
            },
            0 => StateMessageOutgoing::new_info(tr!("запущенные эмуляторы не найдены")),
            _ => match EmulatorModelSelect::select(key, &send_type, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить эмулятор")),
            },
        }
    }
}
