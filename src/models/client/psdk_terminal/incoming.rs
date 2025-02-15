use dbus_crossroads::IfaceBuilder;
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        client::{
            incoming::TraitIncoming,
            outgoing::{OutgoingType, TraitOutgoing},
            psdk_info::incoming::PsdkInfoIncoming,
            state_message::outgoing::StateMessageOutgoing,
            ClientMethodsKey,
        },
        psdk::{model::PsdkModel, select::PsdkModelSelect},
    },
    service::dbus::server::IfaceData,
    tools::{macros::tr, terminal},
};

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkTerminalIncoming {
    id: Option<String>,
}

impl PsdkTerminalIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::PsdkTerminal)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<PsdkTerminalIncoming> {
        Box::new(Self { id: None })
    }

    pub fn new_id(id: String) -> Box<PsdkTerminalIncoming> {
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

impl TraitIncoming for PsdkTerminalIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = PsdkInfoIncoming::name();
        let models: Vec<PsdkModel> = PsdkModelSelect::search(&self.id, &send_type);
        // Exec fun
        fn _run(model: PsdkModel) -> Box<dyn TraitOutgoing> {
            terminal::open(model.chroot)
        }
        // Select
        match models.iter().count() {
            1 => _run(models.first().unwrap().clone()),
            0 => StateMessageOutgoing::new_info(tr!("Platform SDK не найдены")),
            _ => Box::new(PsdkModelSelect::select(key, models, |id| {
                *PsdkInfoIncoming::new_id(id)
            })),
        }
    }
}
