use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::state_message::outgoing::StateMessageOutgoing;
use crate::models::client::ClientMethodsKey;
use crate::models::configuration::sdk::SdkConfig;
use crate::models::configuration::Config;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::tr;

#[derive(Serialize, Deserialize, Clone)]
pub struct SdkSyncIncoming {}

impl SdkSyncIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::SdkSync)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<SdkSyncIncoming> {
        Box::new(Self {})
    }

    pub fn dbus_method_run(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            Self::name(),
            (),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (): ()| async move {
                let outgoing = Self::new().run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_string(),)))
            },
        );
    }
}

impl TraitIncoming for SdkSyncIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        StateMessageOutgoing::new_state(tr!("запуск синхронизации Аврора SDK")).send(&send_type);
        if Config::save_sdk(SdkConfig::search()) {
            StateMessageOutgoing::new_success(tr!("конфигурация Аврора SDK обновлена"))
        } else {
            StateMessageOutgoing::new_info(tr!("изменения не найдены"))
        }
    }
}
