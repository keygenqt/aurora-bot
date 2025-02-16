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
        emulator::{model::EmulatorModel, select::EmulatorModelSelect},
    },
    service::dbus::server::IfaceData,
    tools::macros::tr,
};

use super::outgoing::EmulatorInfoOutgoing;

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorInfoIncoming {
    id: Option<String>,
}

impl EmulatorInfoIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::EmulatorInfo)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<EmulatorInfoIncoming> {
        Box::new(Self { id: None })
    }

    pub fn new_id(id: String) -> Box<EmulatorInfoIncoming> {
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

impl TraitIncoming for EmulatorInfoIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = EmulatorInfoIncoming::name();
        let models: Vec<EmulatorModel> = EmulatorModelSelect::search(&self.id, &send_type, None);
        // Select
        match models.iter().count() {
            1 => EmulatorInfoOutgoing::new(models.first().unwrap().clone()),
            0 => StateMessageOutgoing::new_info(tr!("эмуляторы не найдены")),
            _ => Box::new(EmulatorModelSelect::select(key, models, |id| {
                *EmulatorInfoIncoming::new_id(id)
            })),
        }
    }
}
