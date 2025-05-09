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
use crate::models::psdk_target_package::model::PsdkTargetPackageModel;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::tr;

use super::outgoing::PsdkTargetPackageFindOutgoing;

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

    pub fn new(package: String) -> Box<PsdkTargetPackageFindIncoming> {
        Box::new(Self {
            id: None,
            target_id: None,
            package,
        })
    }

    pub fn new_id(package: String, id: String) -> Box<PsdkTargetPackageFindIncoming> {
        Box::new(Self {
            id: Some(id),
            target_id: None,
            package,
        })
    }

    pub fn new_target(package: String, target_id: String) -> Box<PsdkTargetPackageFindIncoming> {
        Box::new(Self {
            id: None,
            target_id: Some(target_id),
            package,
        })
    }

    pub fn new_target_id(package: String, target_id: String, id: String) -> Box<PsdkTargetPackageFindIncoming> {
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

    pub fn dbus_method_run(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            Self::name(),
            ("package",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (package,): (String,)| async move {
                let outgoing = Self::new(package).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    pub fn dbus_method_run_by_id(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "ById"),
            ("package", "id",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (package, id,): (String, String,)| async move {
                let outgoing = Self::new_id(package, id).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    pub fn dbus_method_run_target(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "Target"),
            ("package", "target_id",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (package, target_id,): (String, String,)| async move {
                let outgoing = Self::new_target(package, target_id).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    pub fn dbus_method_run_target_by_id(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "TargetById"),
            ("package", "target_id", "id"),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (package, target_id, id,): (String, String, String,)| async move {
                let outgoing = Self::new_target_id(package, target_id, id).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    fn run(
        model: PsdkInstalledModel,
        target: PsdkTargetModel,
        package: String,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        let packages = PsdkTargetPackageModel::search_local(&model.chroot, &target.full_name, &package, false)?;
        Ok(PsdkTargetPackageFindOutgoing::new(packages))
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
