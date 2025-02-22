use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::ClientMethodsKey;
use crate::service::dbus::server::IfaceData;

use super::outgoing::SdkAvailableOutgoing;

#[derive(Serialize, Deserialize, Clone)]
pub struct SdkAvailableIncoming {}

impl SdkAvailableIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::SdkAvailable)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<SdkAvailableIncoming> {
        Box::new(Self { })
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
}

impl TraitIncoming for SdkAvailableIncoming {
    fn run(&self, _: OutgoingType) -> Box<dyn TraitOutgoing> {
        // @todo sdk available
        SdkAvailableOutgoing::new(vec![])
    }
}
