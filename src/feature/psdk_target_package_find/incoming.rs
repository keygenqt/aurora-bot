use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::selector::selects::select_psdk_installed::PsdkInstalledModelSelect;
use crate::feature::selector::selects::select_psdk_target::PsdkTargetModelSelect;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::models::psdk_installed::model::PsdkInstalledModel;
use crate::models::psdk_target::model::PsdkTargetModel;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::print_debug;
use crate::tools::macros::tr;

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkTargetPackageFindIncoming {
    id: Option<String>,
    target_id: Option<String>,
    package: String,
}

impl PsdkTargetPackageFindIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::PsdkTargetPackageFind)
            .unwrap()
            .to_string()
    }

    pub fn new_package(package: String) -> Box<PsdkTargetPackageFindIncoming> {
        print_debug!("> {}: new_package(package: {})", Self::name(), package);
        Box::new(Self {
            id: None,
            target_id: None,
            package,
        })
    }

    pub fn new_package_id(id: String, package: String) -> Box<PsdkTargetPackageFindIncoming> {
        print_debug!("> {}: new_package_id(id: {}, package: {})", Self::name(), id, package);
        Box::new(Self {
            id: Some(id),
            target_id: None,
            package,
        })
    }

    pub fn new_package_target(target_id: String, package: String) -> Box<PsdkTargetPackageFindIncoming> {
        print_debug!(
            "> {}: new_package_target(target_id: {}, package: {})",
            Self::name(),
            target_id,
            package
        );
        Box::new(Self {
            id: None,
            target_id: Some(target_id),
            package,
        })
    }

    pub fn new_package_target_id(id: String, target_id: String, package: String) -> Box<PsdkTargetPackageFindIncoming> {
        print_debug!(
            "> {}: new_package_target_id(id: {}, target_id: {}, package: {})",
            Self::name(),
            id,
            target_id,
            package
        );
        Box::new(Self {
            id: Some(id),
            target_id: Some(target_id),
            package,
        })
    }

    fn select(&self, id: String) -> PsdkTargetPackageFindIncoming {
        let mut select = self.clone();
        select.id = Some(id);
        select
    }

    fn select_with_target(&self, id: String, target_id: String) -> PsdkTargetPackageFindIncoming {
        let mut select = self.clone();
        select.id = Some(id);
        select.target_id = Some(target_id);
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

    pub fn dbus_method_run_package_target(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "Target"),
            ("target_id", "package"),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (target_id, package): (String, String)| async move {
                let outgoing = Self::new_package_target(target_id, package).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    pub fn dbus_method_run_package_target_by_id(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "TargetById"),
            ("id", "target_id", "package"),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (id, target_id, package): (String, String, String)| async move {
                let outgoing = Self::new_package_target_id(id, target_id, package).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    // @todo
    #[allow(unused_variables)]
    fn run(
        model: PsdkInstalledModel,
        target: PsdkTargetModel,
        package: String,
        send_type: &OutgoingType,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        // @todo

        Ok(StateMessageOutgoing::new_info(tr!(
            "{}\n{}\n{}",
            model.version_id,
            target.arch,
            package
        )))
    }
}

impl TraitIncoming for PsdkTargetPackageFindIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Find psdk
        let key = PsdkTargetPackageFindIncoming::name();
        let models = PsdkInstalledModelSelect::search(&self.id, tr!("получаем информацию о Platform SDK"), &send_type);
        // Select psdk
        match models.iter().count() {
            1 => {
                // Find psdk targets
                let model = models.first().unwrap().clone();
                let targets = PsdkTargetModelSelect::search(
                    &self.target_id,
                    tr!("получаем информацию о Platform Target"),
                    &send_type,
                    model.targets,
                );
                // Select psdk targets
                match targets.iter().count() {
                    1 => match Self::run(
                        models.first().unwrap().clone(),
                        targets.first().unwrap().clone(),
                        self.package.clone(),
                        &send_type,
                    ) {
                        Ok(result) => result,
                        Err(error) => StateMessageOutgoing::new_error(tr!("{}", error)),
                    },
                    0 => StateMessageOutgoing::new_info(tr!("Platform Target не найдены")),
                    _ => match PsdkTargetModelSelect::select(key, &send_type, targets, |id| {
                        self.select_with_target(model.id.clone(), id)
                    }) {
                        Ok(value) => Box::new(value),
                        Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить Platform Target")),
                    },
                }
            }
            0 => StateMessageOutgoing::new_info(tr!("Platform SDK не найдены")),
            _ => match PsdkInstalledModelSelect::select(key, &send_type, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить Platform SDK")),
            },
        }
    }
}
