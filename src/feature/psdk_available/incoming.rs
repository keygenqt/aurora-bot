use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::selector::selects::select_psdk_available::PsdkAvailableModelSelect;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::tr;

use super::outgoing::PsdkAvailableOutgoing;

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkAvailableIncoming {
    id: Option<String>,
}

impl PsdkAvailableIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::PsdkAvailable)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<PsdkAvailableIncoming> {
        Box::new(Self { id: None })
    }

    pub fn new_id(id: String) -> Box<PsdkAvailableIncoming> {
        Box::new(Self { id: Some(id) })
    }

    fn select(&self, id: String) -> PsdkAvailableIncoming {
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
}

impl TraitIncoming for PsdkAvailableIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = PsdkAvailableIncoming::name();
        let models = PsdkAvailableModelSelect::search(&self.id, tr!("получаем список..."), &send_type);
        // Select
        match models.iter().count() {
            1 => PsdkAvailableOutgoing::new(models.first().unwrap().clone()),
            0 => StateMessageOutgoing::new_info(tr!("не удалось получить данные")),
            _ => match PsdkAvailableModelSelect::select(key, &send_type, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить Platform SDK")),
            },
        }
    }
}
