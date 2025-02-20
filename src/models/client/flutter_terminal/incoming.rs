use dbus_crossroads::IfaceBuilder;
use maplit::hashmap;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::state_message::outgoing::StateMessageOutgoing;
use crate::models::client::ClientMethodsKey;
use crate::models::flutter::model::FlutterModel;
use crate::models::flutter::select::FlutterModelSelect;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::tr;
use crate::tools::terminal;

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterTerminalIncoming {
    id: Option<String>,
}

impl FlutterTerminalIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::FlutterTerminal)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<FlutterTerminalIncoming> {
        Box::new(Self { id: None })
    }

    pub fn new_id(id: String) -> Box<FlutterTerminalIncoming> {
        Box::new(Self { id: Some(id) })
    }

    fn select(&self, id: String) -> FlutterTerminalIncoming {
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

impl TraitIncoming for FlutterTerminalIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = FlutterTerminalIncoming::name();
        let models: Vec<FlutterModel> =
            FlutterModelSelect::search(&self.id, tr!("ищем Flutter SDK для открытия терминала"), &send_type);
        // Exec fun
        fn _run(model: FlutterModel) -> Box<dyn TraitOutgoing> {
            let command = terminal::command_aliases(hashmap! {
                "flutter" => model.flutter,
                "dart" => model.dart,
            });
            if let Ok(command) = command {
                terminal::open(command)
            } else {
                StateMessageOutgoing::new_error(tr!("не удалось открыть терминал"))
            }
        }
        // Select
        match models.iter().count() {
            1 => _run(models.first().unwrap().clone()),
            0 => StateMessageOutgoing::new_info(tr!("Flutter SDK не найдены")),
            _ => Box::new(FlutterModelSelect::select(key, models, |id| self.select(id))),
        }
    }
}
