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
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // @todo sdk available
        StateMessageOutgoing::new_state(tr!("получение данных с репозитория")).send(&send_type);
        match single::get_request().get_repo_url_sdk() {
            Ok(url_files) => {
                for url in url_files {
                    // @todo
                    println!("{}", url)
                }
                SdkAvailableOutgoing::new(vec![])
            },
            Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить данные")),
        }
    }
}
