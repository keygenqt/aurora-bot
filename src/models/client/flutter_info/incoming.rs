use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::ClientMethodsKey;
use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::state_message::outgoing::StateMessageOutgoing;
use crate::models::flutter_installed::model::FlutterInstalledModel;
use crate::models::flutter_installed::select::FlutterInstalledModelSelect;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::tr;

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

    fn select(&self, id: String) -> FlutterInfoIncoming {
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

impl TraitIncoming for FlutterInfoIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = FlutterInfoIncoming::name();
        let models: Vec<FlutterInstalledModel> =
            FlutterInstalledModelSelect::search(&self.id, tr!("получаем информацию о Flutter SDK"), &send_type);
        // Select
        match models.iter().count() {
            1 => FlutterInfoOutgoing::new(models.first().unwrap().clone()),
            0 => StateMessageOutgoing::new_info(tr!("Flutter SDK не найдены")),
            _ => Box::new(FlutterInstalledModelSelect::select(key, models, |id| self.select(id))),
        }
    }
}
