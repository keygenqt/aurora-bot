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
pub struct PsdkTargetPackageSearchIncoming {
    id: Option<String>,
    package: String,
}

impl PsdkTargetPackageSearchIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::PsdkTargetPackageSearch)
            .unwrap()
            .to_string()
    }

    pub fn new_package(package: String) -> Box<PsdkTargetPackageSearchIncoming> {
        print_debug!("> {}: new_package(package: {})", Self::name(), package);
        Box::new(Self { id: None, package })
    }

    pub fn new_package_id(id: String, package: String) -> Box<PsdkTargetPackageSearchIncoming> {
        print_debug!("> {}: new_package_id(id: {}, package: {})", Self::name(), id, package,);
        Box::new(Self { id: Some(id), package })
    }

    fn select(&self, id: String) -> PsdkTargetPackageSearchIncoming {
        let mut select = self.clone();
        select.id = Some(id);
        select
    }

    pub fn dbus_method_run_package(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            Self::name(),
            ("package",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (package,): (String,)| async move {
                let outgoing = Self::new_package(package).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    pub fn dbus_method_run_package_by_id(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "ById"),
            ("id", "package"),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (id, package): (String, String)| async move {
                let outgoing = Self::new_package_id(id, package).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    // @todo
    #[allow(unused_variables)]
    fn run(
        model: PsdkInstalledModel,
        send_type: &OutgoingType,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        Ok(StateMessageOutgoing::new_info(tr!("@todo")))
    }
}

impl TraitIncoming for PsdkTargetPackageSearchIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = PsdkTargetPackageSearchIncoming::name();
        let models = PsdkInstalledModelSelect::search(&self.id, tr!("получаем информацию о Platform SDK"), &send_type);
        // Select
        match models.iter().count() {
            1 => match Self::run(models.first().unwrap().clone(), &send_type) {
                Ok(result) => result,
                Err(error) => StateMessageOutgoing::new_error(tr!("{}", error)),
            },
            0 => StateMessageOutgoing::new_info(tr!("Platform SDK не найдены")),
            _ => match PsdkInstalledModelSelect::select(key, &send_type, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить Platform SDK")),
            },
        }
    }
}
