use std::process::Command;

use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::ClientMethodsKey;
use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::sdk_info::incoming::SdkInfoIncoming;
use crate::models::client::selector::selects::select_sdk_installed::SdkInstalledModelSelect;
use crate::models::client::state_message::outgoing::StateMessageOutgoing;
use crate::models::sdk_installed::model::SdkInstalledModel;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::print_debug;
use crate::tools::macros::tr;

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
        print_debug!("> {}: new()", Self::name());
        Box::new(Self { id: None })
    }

    pub fn new_id(id: String) -> Box<SdkToolsIncoming> {
        print_debug!("> {}: new_id(id: {})", Self::name(), id);
        Box::new(Self { id: Some(id) })
    }

    fn select(&self, id: String) -> SdkToolsIncoming {
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

impl TraitIncoming for SdkToolsIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = SdkInfoIncoming::name();
        let models: Vec<SdkInstalledModel> = SdkInstalledModelSelect::search(
            &self.id,
            tr!("ищем Аврора SDK для открытия Maintenance tools"),
            &send_type,
        );
        // Exec fun
        fn _run(model: SdkInstalledModel) -> Box<dyn TraitOutgoing> {
            if let Ok(_) = Command::new(model.tools).spawn() {
                StateMessageOutgoing::new_success(tr!("Аврора SDK Maintenance tools запущено"))
            } else {
                StateMessageOutgoing::new_error(tr!("ошибка при запуске Аврора SDK Maintenance tools"))
            }
        }
        // Select
        match models.iter().count() {
            1 => _run(models.first().unwrap().clone()),
            0 => StateMessageOutgoing::new_info(tr!("Аврора SDK не найдены")),
            _ => match SdkInstalledModelSelect::select(key, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить Аврора SDK")),
            },
        }
    }
}
