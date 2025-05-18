use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::service::dbus::server::IfaceData;

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
        Box::new(Self {})
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

impl TraitIncoming for AppInfoIncoming {
    fn run(&self, _: OutgoingType) -> Box<dyn TraitOutgoing> {
        AppInfoOutgoing::new()
    }
}
