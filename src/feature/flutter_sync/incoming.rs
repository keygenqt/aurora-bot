use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::models::configuration::Config;
use crate::models::configuration::flutter::FlutterConfig;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::tr;

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterSyncIncoming {}

impl FlutterSyncIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::FlutterSync)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<FlutterSyncIncoming> {
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

impl TraitIncoming for FlutterSyncIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        StateMessageOutgoing::new_state(tr!("запуск синхронизации Flutter SDK")).send(&send_type);
        if Config::save_flutter(FlutterConfig::search()) {
            StateMessageOutgoing::new_success(tr!("конфигурация Flutter SDK обновлена"))
        } else {
            StateMessageOutgoing::new_info(tr!("конфигурация не требует обновления"))
        }
    }
}
