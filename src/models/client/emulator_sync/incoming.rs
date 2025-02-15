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
        configuration::{emulator::EmulatorConfig, Config},
    },
    service::dbus::server::IfaceData,
    tools::macros::tr,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorSyncIncoming {}

impl EmulatorSyncIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::EmulatorSync)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<EmulatorSyncIncoming> {
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

impl TraitIncoming for EmulatorSyncIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        StateMessageOutgoing::new_state(tr!("запуск синхронизации эмуляторов")).send(&send_type);
        if Config::save_emulator(EmulatorConfig::search()) {
            StateMessageOutgoing::new_success(tr!("конфигурация эмуляторов обновлена"))
        } else {
            StateMessageOutgoing::new_info(tr!("изменения не найдены"))
        }
    }
}
