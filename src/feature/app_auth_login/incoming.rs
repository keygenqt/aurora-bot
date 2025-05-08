use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::tr;
use crate::tools::single;

#[derive(Serialize, Deserialize, Clone)]
pub struct AppAuthLoginIncoming {
    token: String,
}

impl AppAuthLoginIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::AppAuthLogin)
            .unwrap()
            .to_string()
    }

    pub fn new(token: String) -> Box<AppAuthLoginIncoming> {
        Box::new(Self { token })
    }

    pub fn dbus_method_run(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            Self::name(),
            ("token",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (token,): (String,)| async move {
                let outgoing = AppAuthLoginIncoming::new(token).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }
}

impl TraitIncoming for AppAuthLoginIncoming {
    fn run(&self, _: OutgoingType) -> Box<dyn TraitOutgoing> {
        match single::get_request().auth_ping_token(self.token.clone()) {
            Ok(_) => StateMessageOutgoing::new_success(tr!("авторизация прошла успешно")),
            Err(_) => StateMessageOutgoing::new_error(tr!("авторизация завершилась неудачей")),
        }
    }
}
