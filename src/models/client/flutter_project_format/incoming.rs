use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::ClientMethodsKey;
use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::selector::selects::select_flutter_installed::FlutterInstalledModelSelect;
use crate::models::client::state_message::outgoing::StateMessageOutgoing;
use crate::models::flutter_installed::model::FlutterInstalledModel;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::print_debug;
use crate::tools::macros::tr;

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterProjectFormatIncoming {
    id: Option<String>,
}

impl FlutterProjectFormatIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::FlutterProjectFormat)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<FlutterProjectFormatIncoming> {
        print_debug!("> {}: new()", Self::name());
        Box::new(Self { id: None })
    }

    pub fn new_id(id: String) -> Box<FlutterProjectFormatIncoming> {
        print_debug!("> {}: new_id(id: {})", Self::name(), id);
        Box::new(Self { id: Some(id) })
    }

    fn select(&self, id: String) -> FlutterProjectFormatIncoming {
        let mut select = self.clone();
        select.id = Some(id);
        select
    }

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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

    #[allow(unused_variables)]
    fn run(
        model: FlutterInstalledModel,
        send_type: &OutgoingType,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        Ok(StateMessageOutgoing::new_info(tr!("@todo")))
    }
}

impl TraitIncoming for FlutterProjectFormatIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = FlutterProjectFormatIncoming::name();
        let models: Vec<FlutterInstalledModel> =
            FlutterInstalledModelSelect::search(&self.id, tr!("получаем информацию о Flutter SDK"), &send_type);
        // Select
        match models.iter().count() {
            1 => match Self::run(models.first().unwrap().clone(), &send_type) {
                Ok(result) => result,
                Err(_) => StateMessageOutgoing::new_error(tr!("@todo")),
            },
            0 => StateMessageOutgoing::new_info(tr!("Flutter SDK не найдены")),
            _ => match FlutterInstalledModelSelect::select(key, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("@todo")),
            },
        }
    }
}
