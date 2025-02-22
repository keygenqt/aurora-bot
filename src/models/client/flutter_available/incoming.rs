use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::ClientMethodsKey;
use crate::service::dbus::server::IfaceData;

use super::outgoing::FlutterAvailableOutgoing;

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterAvailableIncoming {}

impl FlutterAvailableIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::FlutterAvailable)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<FlutterAvailableIncoming> {
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

impl TraitIncoming for FlutterAvailableIncoming {
    fn run(&self, _: OutgoingType) -> Box<dyn TraitOutgoing> {
        // @todo flutter available
        FlutterAvailableOutgoing::new(vec![])
    }
}
