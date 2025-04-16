use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::selector::selects::select_sdk_available::SdkAvailableModelSelect;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::print_debug;
use crate::tools::macros::tr;

use super::outgoing::SdkAvailableOutgoing;

#[derive(Serialize, Deserialize, Clone)]
pub struct SdkAvailableIncoming {
    id: Option<String>,
}

impl SdkAvailableIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::SdkAvailable)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<SdkAvailableIncoming> {
        print_debug!("> {}: new()", Self::name());
        Box::new(Self { id: None })
    }

    pub fn new_id(id: String) -> Box<SdkAvailableIncoming> {
        print_debug!("> {}: new_id(id: {})", Self::name(), id);
        Box::new(Self { id: Some(id) })
    }

    fn select(&self, id: String) -> SdkAvailableIncoming {
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

impl TraitIncoming for SdkAvailableIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = SdkAvailableIncoming::name();
        let models = SdkAvailableModelSelect::search(&self.id, tr!("получаем список..."), &send_type);
        // Select
        match models.iter().count() {
            1 => SdkAvailableOutgoing::new(models.first().unwrap().clone()),
            0 => StateMessageOutgoing::new_info(tr!("не удалось получить данные")),
            _ => match SdkAvailableModelSelect::select(key, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить Аврора SDK")),
            },
        }
    }
}
