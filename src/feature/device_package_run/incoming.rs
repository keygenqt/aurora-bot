use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::selector::selects::select_device::DeviceModelSelect;
use crate::feature::selector::selects::select_device_packages::DevicePackageSelect;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::models::device::model::DeviceModel;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::tr;

#[derive(Serialize, Deserialize, Clone)]
pub struct DevicePackageRunIncoming {
    id: Option<String>,
    package: Option<String>,
}

impl DevicePackageRunIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::DevicePackageRun)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<DevicePackageRunIncoming> {
        Box::new(Self {
            id: None,
            package: None,
        })
    }

    pub fn new_id(id: String) -> Box<DevicePackageRunIncoming> {
        Box::new(Self {
            id: Some(id),
            package: None,
        })
    }

    pub fn new_package(package: String) -> Box<DevicePackageRunIncoming> {
        Box::new(Self {
            id: None,
            package: Some(package),
        })
    }

    pub fn new_package_id(package: String, id: String) -> Box<DevicePackageRunIncoming> {
        Box::new(Self {
            id: Some(id),
            package: Some(package),
        })
    }

    fn select(&self, id: String) -> DevicePackageRunIncoming {
        let mut select = self.clone();
        select.id = Some(id);
        select
    }

    fn select_package(&self, id: String, package: String) -> DevicePackageRunIncoming {
        let mut select = self.clone();
        select.id = Some(id);
        select.package = Some(package);
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

    pub fn dbus_method_run_package(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "Package"),
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
            format!("{}{}", Self::name(), "PackageById"),
            ("package", "id",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (package, id,): (String, String,)| async move {
                let outgoing = Self::new_package_id(package, id).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    fn run(model: DeviceModel, package: String) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        match model.session_user()?.run_package_listen(package) {
            Ok(_) => Ok(StateMessageOutgoing::new_success(tr!("приложение остановлено"))),
            Err(_) => Err(tr!("не удалось запустить приложение"))?,
        }
    }
}

impl TraitIncoming for DevicePackageRunIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = DevicePackageRunIncoming::name();
        let models = DeviceModelSelect::search(&self.id, tr!("получаем информацию об устройствах"), &send_type);
        // Select
        match models.iter().count() {
            1 => {
                if let Some(model) = models.first() {
                    if let Some(package) = self.package.clone() {
                        match Self::run(model.clone(), package) {
                            Ok(result) => result,
                            Err(error) => StateMessageOutgoing::new_error(tr!("{}", error)),
                        }
                    } else {
                        match DevicePackageSelect::select(key, &send_type, model, |id, package| {
                            self.select_package(id, package)
                        }) {
                            Ok(value) => Box::new(value),
                            Err(_) => StateMessageOutgoing::new_error(tr!("не удалось найти пакеты")),
                        }
                    }
                } else {
                    panic!("ошибка получения данных")
                }
            }
            0 => StateMessageOutgoing::new_info(tr!("устройства не найдены")),
            _ => match DeviceModelSelect::select(key, &send_type, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить устройство")),
            },
        }
    }
}
