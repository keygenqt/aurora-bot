use std::process::Command;

use dbus_crossroads::IfaceBuilder;
use serde::{Deserialize, Serialize};

use crate::{
    models::{
        client::{
            incoming::TraitIncoming,
            outgoing::{OutgoingType, TraitOutgoing},
            sdk_info::incoming::SdkInfoIncoming,
            state_message::outgoing::StateMessageOutgoing,
            ClientMethodsKey,
        },
        sdk::{model::SdkModel, select::SdkModelSelect},
    },
    service::dbus::server::IfaceData,
    tools::macros::tr,
};

#[derive(Serialize, Deserialize, Clone)]
pub struct SdkToolsIncoming {
    id: Option<String>,
}

impl SdkToolsIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::SdkTools)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<SdkToolsIncoming> {
        Box::new(Self { id: None })
    }

    pub fn new_id(id: String) -> Box<SdkToolsIncoming> {
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

impl TraitIncoming for SdkToolsIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = SdkInfoIncoming::name();
        let models: Vec<SdkModel> = SdkModelSelect::search(&self.id, &send_type);
        // Exec fun
        fn _run(model: SdkModel) -> Box<dyn TraitOutgoing> {
            if let Ok(_) = Command::new(model.tools).spawn() {
                StateMessageOutgoing::new_success(tr!("Аврора SDK Tools запущено"))
            } else {
                StateMessageOutgoing::new_success(tr!("ошибка при запуске Аврора SDK Tools"))
            }
        }
        // Select
        match models.iter().count() {
            1 => _run(models.first().unwrap().clone()),
            0 => StateMessageOutgoing::new_info(tr!("Аврора SDK не найдены")),
            _ => Box::new(SdkModelSelect::select(key, models, |id| {
                *SdkInfoIncoming::new_id(id)
            })),
        }
    }
}
