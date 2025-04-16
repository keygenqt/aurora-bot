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

    fn run(
        model: SdkInstalledModel,
        send_type: &OutgoingType,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        StateMessageOutgoing::new_state(tr!("открываем Maintenance tools")).send(send_type);
        model.start_tools()?;
        Ok(StateMessageOutgoing::new_success(tr!(
            "Maintenance tools успешно запущено"
        )))
    }
}

impl TraitIncoming for SdkToolsIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = SdkInfoIncoming::name();
        let models = SdkInstalledModelSelect::search(
            &self.id,
            tr!("ищем Аврора SDK для открытия Maintenance tools"),
            &send_type,
        );
        // Select
        match models.iter().count() {
            1 => match Self::run(models.first().unwrap().clone(), &send_type) {
                Ok(result) => result,
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось открыть Maintenance tools")),
            },
            0 => StateMessageOutgoing::new_info(tr!("Аврора SDK не найдены")),
            _ => match SdkInstalledModelSelect::select(key, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить Аврора SDK")),
            },
        }
    }
}
