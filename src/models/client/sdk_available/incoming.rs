use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::ClientMethodsKey;
use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::sdk_available::outgoing::SdkAvailableItemOutgoing;
use crate::models::client::state_message::outgoing::StateMessageOutgoing;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::tr;

use super::outgoing::SdkAvailableOutgoing;
use super::select::SdkAvailableSelect;

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
        Box::new(Self { id: None })
    }

    pub fn new_id(id: String) -> Box<SdkAvailableIncoming> {
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
            Self::name(),
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
        let models: Vec<SdkAvailableItemOutgoing> =
            SdkAvailableSelect::search(&self.id, &send_type, tr!("получаем список..."), tr!("получаем модель..."));
        // Select
        match models.iter().count() {
            1 => SdkAvailableOutgoing::new(models.first().unwrap().clone()),
            0 => StateMessageOutgoing::new_info(tr!("не удалось получить данные")),
            _ => Box::new(SdkAvailableSelect::select(key, models, |id| self.select(id))),
        }
    }
}
