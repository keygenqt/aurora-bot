use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::selector::selects::select_emulator::EmulatorModelSelect;
use crate::feature::selector::selects::select_emulator_packages::EmulatorPackageSelect;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::models::emulator::model::EmulatorModel;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::tr;

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorPackageRunIncoming {
    id: Option<String>,
    package: Option<String>,
    is_listen: bool,
}

impl EmulatorPackageRunIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::EmulatorPackageRun)
            .unwrap()
            .to_string()
    }

    pub fn new(is_listen: bool) -> Box<EmulatorPackageRunIncoming> {
        Box::new(Self {
            id: None,
            package: None,
            is_listen,
        })
    }

    pub fn new_id(is_listen: bool, id: String) -> Box<EmulatorPackageRunIncoming> {
        Box::new(Self {
            id: Some(id),
            package: None,
            is_listen,
        })
    }

    pub fn new_package(is_listen: bool, package: String) -> Box<EmulatorPackageRunIncoming> {
        Box::new(Self {
            id: None,
            package: Some(package),
            is_listen,
        })
    }

    pub fn new_package_id(is_listen: bool, package: String, id: String) -> Box<EmulatorPackageRunIncoming> {
        Box::new(Self {
            id: Some(id),
            package: Some(package),
            is_listen,
        })
    }

    fn select(&self, id: String) -> EmulatorPackageRunIncoming {
        let mut select = self.clone();
        select.id = Some(id);
        select
    }

    fn select_package(&self, id: String, package: String) -> EmulatorPackageRunIncoming {
        let mut select = self.clone();
        select.id = Some(id);
        select.package = Some(package);
        select
    }

    pub fn dbus_method_run(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            Self::name(),
            ("is_listen",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (is_listen,): (bool,)| async move {
                let outgoing = Self::new(is_listen).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    pub fn dbus_method_run_by_id(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "ById"),
            ("is_listen", "id"),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (is_listen, id): (bool, String)| async move {
                let outgoing = Self::new_id(is_listen, id).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    pub fn dbus_method_run_package(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "Package"),
            ("is_listen", "package"),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (is_listen, package): (bool, String)| async move {
                let outgoing = Self::new_package(is_listen, package).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    pub fn dbus_method_run_package_by_id(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "PackageById"),
            ("is_listen", "package", "id"),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (is_listen, package, id): (bool, String, String)| async move {
                let outgoing = Self::new_package_id(is_listen, package, id).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    fn run(
        model: EmulatorModel,
        package: String,
        is_listen: bool,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        if !model.is_running {
            return Ok(StateMessageOutgoing::new_info(tr!("эмулятор должен быть запущен")));
        }
        let result = if is_listen {
            model.session_user()?.run_package_listen(package)
        } else {
            model.session_user()?.run_package(package)
        };
        if result.is_err() {
            Err(tr!("не удалось запустить приложение"))?
        }
        if is_listen {
            Ok(StateMessageOutgoing::new_success(tr!("приложение остановлено")))
        } else {
            Ok(StateMessageOutgoing::new_success(tr!("отправлена команда на запуск")))
        }
    }
}

impl TraitIncoming for EmulatorPackageRunIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = EmulatorPackageRunIncoming::name();
        let models = EmulatorModelSelect::search(&self.id, &send_type, tr!("ищем запущенный эмулятор"), Some(true));
        // Select
        match models.iter().count() {
            1 => {
                if let Some(model) = models.first() {
                    if let Some(package) = self.package.clone() {
                        match Self::run(model.clone(), package, self.is_listen) {
                            Ok(result) => result,
                            Err(error) => StateMessageOutgoing::new_error(tr!("{}", error)),
                        }
                    } else {
                        match EmulatorPackageSelect::select(key, &send_type, model, |id, package| {
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
            0 => StateMessageOutgoing::new_info(tr!("запущенные эмуляторы не найдены")),
            _ => match EmulatorModelSelect::select(key, &send_type, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить эмулятор")),
            },
        }
    }
}
