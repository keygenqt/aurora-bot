use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::ClientMethodsKey;
use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::selector::selects::select_emulator::EmulatorModelSelect;
use crate::models::client::selector::selects::select_emulator_packages::EmulatorPackageSelect;
use crate::models::client::state_message::outgoing::StateMessageOutgoing;
use crate::models::emulator::model::EmulatorModel;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::tr;

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorPackageUninstallIncoming {
    id: Option<String>,
    package: Option<String>,
}

impl EmulatorPackageUninstallIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::EmulatorPackageUninstall)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<EmulatorPackageUninstallIncoming> {
        Box::new(Self {
            id: None,
            package: None,
        })
    }

    pub fn new_id(id: String) -> Box<EmulatorPackageUninstallIncoming> {
        Box::new(Self {
            id: Some(id),
            package: None,
        })
    }

    pub fn new_package(package: String) -> Box<EmulatorPackageUninstallIncoming> {
        Box::new(Self {
            id: None,
            package: Some(package),
        })
    }

    pub fn new_id_package(id: String, package: String) -> Box<EmulatorPackageUninstallIncoming> {
        Box::new(Self {
            id: Some(id),
            package: Some(package),
        })
    }

    fn select(&self, id: String) -> EmulatorPackageUninstallIncoming {
        let mut select = self.clone();
        select.id = Some(id);
        select
    }

    fn select_package(&self, id: String, package: String) -> EmulatorPackageUninstallIncoming {
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

    pub fn dbus_method_run_by_package(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "ByPackage"),
            ("package",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (package,): (String,)| async move {
                let outgoing = Self::new_package(package).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    pub fn dbus_method_run_by_id_package(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "ByIdPackage"),
            ("id", "package"),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (id, package): (String, String)| async move {
                let outgoing = Self::new_id_package(id, package).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    fn run_uninstall(
        emulator: EmulatorModel,
        package_name: String,
        send_type: &OutgoingType,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        if !emulator.is_running {
            return Ok(StateMessageOutgoing::new_info(tr!("эмулятор должен быть запущен")));
        }
        // Get session
        let session = emulator.session_user()?;
        // Remove by apm
        StateMessageOutgoing::new_state(tr!("удаление пакета")).send(send_type);
        session.remove_package(package_name, true)?;
        // Success result
        Ok(StateMessageOutgoing::new_success(tr!("пакет удален")))
    }
}

impl TraitIncoming for EmulatorPackageUninstallIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = EmulatorPackageUninstallIncoming::name();
        let models: Vec<EmulatorModel> =
            EmulatorModelSelect::search(&self.id, &send_type, tr!("ищем запущенный эмулятор"), Some(true));
        // Select
        match models.iter().count() {
            1 => {
                if let Some(model) = models.first() {
                    if let Some(package) = self.package.clone() {
                        match Self::run_uninstall(model.clone(), package, &send_type) {
                            Ok(result) => result,
                            Err(error) => StateMessageOutgoing::new_error(tr!("{}", error)),
                        }
                    } else {
                        match EmulatorPackageSelect::select(key, model, |id, package| self.select_package(id, package))
                        {
                            Ok(value) => Box::new(value),
                            Err(_) => StateMessageOutgoing::new_error(tr!("не удалось найти пакеты")),
                        }
                    }
                } else {
                    panic!("ошибка получения данных")
                }
            }
            0 => StateMessageOutgoing::new_info(tr!("запущенные эмуляторы не найдены")),
            _ => match EmulatorModelSelect::select(key, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить эмулятор")),
            },
        }
    }
}
