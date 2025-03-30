use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::ClientMethodsKey;
use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::state_message::outgoing::StateMessageOutgoing;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::print_debug;
use crate::tools::macros::tr;
use crate::tools::single;

#[derive(Serialize, Deserialize, Clone)]
pub struct AppAuthLogoutIncoming {}

impl AppAuthLogoutIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::AppAuthLogout)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<AppAuthLogoutIncoming> {
        print_debug!("> {}: new()", Self::name());
        Box::new(Self {})
    }

    pub fn dbus_method_run(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            Self::name(),
            (),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (): ()| async move {
                let outgoing = AppAuthLogoutIncoming::new().run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }
}

impl TraitIncoming for AppAuthLogoutIncoming {
    fn run(&self, _: OutgoingType) -> Box<dyn TraitOutgoing> {
        match single::get_request().logout() {
            Ok(_) => StateMessageOutgoing::new_success(tr!("сессия удалена успешно")),
            Err(_) => StateMessageOutgoing::new_error(tr!("сессия не найдена")),
        }
    }
}
