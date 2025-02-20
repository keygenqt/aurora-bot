use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::state_message::outgoing::StateMessageOutgoing;
use crate::models::client::ClientMethodsKey;
use crate::models::psdk::model::PsdkModel;
use crate::models::psdk::select::PsdkModelSelect;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::tr;
use crate::tools::terminal;

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

    fn select(&self, id: String) -> PsdkTerminalIncoming {
        let mut select = self.clone();
        select.id = Some(id);
        select
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

    pub fn dbus_method_run_by_id(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "ById"),
            ("id",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (id,): (String,)| async move {
                let outgoing = Self::new_id(id).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }
}

impl TraitIncoming for PsdkTerminalIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = PsdkTerminalIncoming::name();
        let models: Vec<PsdkModel> =
            PsdkModelSelect::search(&self.id, tr!("ищем Platform SDK для открытия терминала"), &send_type);
        // Exec fun
        fn _run(model: PsdkModel) -> Box<dyn TraitOutgoing> {
            terminal::open(model.chroot)
        }
        // Select
        match models.iter().count() {
            1 => _run(models.first().unwrap().clone()),
            0 => StateMessageOutgoing::new_info(tr!("Platform SDK не найдены")),
            _ => Box::new(PsdkModelSelect::select(key, models, |id| self.select(id))),
        }
    }
}
