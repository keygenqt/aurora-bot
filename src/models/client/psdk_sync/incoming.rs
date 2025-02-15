use dbus_crossroads::IfaceBuilder;
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        client::{
            incoming::TraitIncoming,
            outgoing::{OutgoingType, TraitOutgoing},
            state_message::outgoing::StateMessageOutgoing,
            ClientMethodsKey,
        },
        configuration::{psdk::PsdkConfig, Config},
    },
    service::dbus::server::IfaceData,
    tools::macros::tr,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkSyncIncoming {}

impl PsdkSyncIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::PsdkSync)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<PsdkSyncIncoming> {
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

impl TraitIncoming for PsdkSyncIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        StateMessageOutgoing::new_state(tr!("запуск синхронизации Platform SDK")).send(&send_type);
        if Config::save_psdk(PsdkConfig::search()) {
            StateMessageOutgoing::new_success(tr!("конфигурация Platform SDK обновлена"))
        } else {
            StateMessageOutgoing::new_info(tr!("изменения не найдены"))
        }
    }
}
