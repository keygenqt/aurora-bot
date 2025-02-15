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
        flutter::{model::FlutterModel, select::FlutterModelSelect},
    },
    service::dbus::server::IfaceData,
    tools::macros::tr,
};

use super::outgoing::FlutterInfoOutgoing;

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterInfoIncoming {
    id: Option<String>,
}

impl FlutterInfoIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::FlutterInfo)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<FlutterInfoIncoming> {
        Box::new(Self { id: None })
    }

    pub fn new_id(id: String) -> Box<FlutterInfoIncoming> {
        Box::new(Self { id: Some(id) })
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

    pub fn dbus_method_run_by_id(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "ById"),
            ("id",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (id,): (String,)| async move {
                let outgoing = Self::new_id(id).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_string(),)))
            },
        );
    }
}

impl TraitIncoming for FlutterInfoIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = FlutterInfoIncoming::name();
        let models: Vec<FlutterModel> = FlutterModelSelect::search(&self.id, &send_type);
        // Select
        match models.iter().count() {
            1 => FlutterInfoOutgoing::new(models.first().unwrap().clone()),
            0 => StateMessageOutgoing::new_info(tr!("Flutter SDK не найдены")),
            _ => Box::new(FlutterModelSelect::select(key, models, |id| {
                *FlutterInfoIncoming::new_id(id)
            })),
        }
    }
}
