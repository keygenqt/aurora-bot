use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::ClientMethodsKey;
use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::print_debug;

use super::outgoing::AppInfoOutgoing;

#[derive(Serialize, Deserialize, Clone)]
pub struct AppInfoIncoming {}

impl AppInfoIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::AppInfo)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<AppInfoIncoming> {
        print_debug!("> {}: new()", Self::name());
        Box::new(Self {})
    }

    pub fn dbus_method_run(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            Self::name(),
            (),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (): ()| async move {
                let outgoing = AppInfoIncoming::new().run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }
}

impl TraitIncoming for AppInfoIncoming {
    fn run(&self, _: OutgoingType) -> Box<dyn TraitOutgoing> {
        AppInfoOutgoing::new()
    }
}
