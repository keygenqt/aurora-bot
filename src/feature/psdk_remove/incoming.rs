use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::selector::selects::select_psdk_installed::PsdkInstalledModelSelect;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::models::psdk_installed::model::PsdkInstalledModel;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::print_debug;
use crate::tools::macros::tr;

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkRemoveIncoming {
    id: Option<String>,
}

impl PsdkRemoveIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::PsdkRemove)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<PsdkRemoveIncoming> {
        print_debug!("> {}: new()", Self::name());
        Box::new(Self { id: None })
    }

    pub fn new_id(id: String) -> Box<PsdkRemoveIncoming> {
        print_debug!("> {}: new_id(id: {})", Self::name(), id);
        Box::new(Self { id: Some(id) })
    }

    fn select(&self, id: String) -> PsdkRemoveIncoming {
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
        model: PsdkInstalledModel,
        send_type: &OutgoingType,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        Ok(StateMessageOutgoing::new_info(tr!("@todo")))
    }
}

impl TraitIncoming for PsdkRemoveIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = PsdkRemoveIncoming::name();
        let models = PsdkInstalledModelSelect::search(&self.id, tr!("получаем информацию о Platform SDK"), &send_type);
        // Select
        match models.iter().count() {
            1 => match Self::run(models.first().unwrap().clone(), &send_type) {
                Ok(result) => result,
                Err(_) => StateMessageOutgoing::new_error(tr!("произошла ошибка при удалении Platform SDK")),
            },
            0 => StateMessageOutgoing::new_info(tr!("Platform SDK не найдены")),
            _ => match PsdkInstalledModelSelect::select(key, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить Platform SDK")),
            },
        }
    }
}
