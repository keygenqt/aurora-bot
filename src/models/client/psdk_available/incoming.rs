use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::state_message::outgoing::StateMessageOutgoing;
use crate::models::client::ClientMethodsKey;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::tr;
use crate::tools::single;

use super::outgoing::PsdkAvailableOutgoing;

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkAvailableIncoming {
    is_all: bool
}

impl PsdkAvailableIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::PsdkAvailable)
            .unwrap()
            .to_string()
    }

    pub fn new(is_all: bool) -> Box<PsdkAvailableIncoming> {
        Box::new(Self { is_all })
    }

    pub fn dbus_method_run(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            Self::name(),
            ("is_all", ),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (is_all,): (bool,)| async move {
                let outgoing = Self::new(is_all).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }
}

impl TraitIncoming for PsdkAvailableIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        StateMessageOutgoing::new_state(tr!("получение данных с репозитория")).send(&send_type);
        // @todo psdk available
        let _ = single::get_request().get_repo_url_psdk();
        PsdkAvailableOutgoing::new(vec![])
    }
}
