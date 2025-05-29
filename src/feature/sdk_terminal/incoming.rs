use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::sdk_info::incoming::SdkInfoIncoming;
use crate::feature::selector::selects::select_sdk_installed::SdkInstalledModelSelect;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::models::sdk_available::model::SdkBuildType;
use crate::models::sdk_installed::model::SdkInstalledModel;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::tr;
use crate::tools::terminal;

#[derive(Serialize, Deserialize, Clone)]
pub struct SdkTerminalIncoming {
    id: Option<String>,
}

impl SdkTerminalIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::SdkTerminal)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<SdkTerminalIncoming> {
        Box::new(Self { id: None })
    }

    pub fn new_id(id: String) -> Box<SdkTerminalIncoming> {
        Box::new(Self { id: Some(id) })
    }

    fn select(&self, id: String) -> SdkTerminalIncoming {
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

    fn run(
        model: SdkInstalledModel,
        send_type: &OutgoingType,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        let mut engine = model.get_sdk_engine()?;
        if !engine.is_running {
            StateMessageOutgoing::new_state(tr!("запускаем engine...")).send(send_type);
            engine.start()?;
            // Get engine connect session
            let model = engine.session()?;
            // Close connect
            model.close()?;
        }
        // Run command
        let command = format!(
            "ssh -o 'ConnectTimeout=30' -o 'StrictHostKeyChecking=no' mersdk@localhost -p 2222 -i {}",
            engine.key
        );
        // Try run terminal
        Ok(terminal::open(command))
    }
}

impl TraitIncoming for SdkTerminalIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = SdkInfoIncoming::name();
        let models: Vec<SdkInstalledModel> =
            SdkInstalledModelSelect::search(&self.id, tr!("ищем Аврора SDK (MB2)"), &send_type)
                .iter()
                .filter(|e| e.build_type == SdkBuildType::MB2)
                .map(|e| e.clone())
                .collect();
        // Select
        match models.iter().count() {
            1 => match Self::run(models.first().unwrap().clone(), &send_type) {
                Ok(result) => result,
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось открыть терминал")),
            },
            0 => StateMessageOutgoing::new_info(tr!("Аврора SDK не найдены")),
            _ => match SdkInstalledModelSelect::select(key, &send_type, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить Аврора SDK")),
            },
        }
    }
}
