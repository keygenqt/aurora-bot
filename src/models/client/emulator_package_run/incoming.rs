use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::selector::outgoing::incoming::SelectorIncoming;
use crate::models::client::selector::outgoing::outgoing::SelectorOutgoing;
use crate::models::client::ClientMethodsKey;
use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::state_message::outgoing::StateMessageOutgoing;
use crate::models::emulator::model::EmulatorModel;
use crate::models::emulator::select::EmulatorModelSelect;
use crate::models::TraitModel;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::tr;

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorPackageRunIncoming {
    id: Option<String>,
    package: Option<String>,
}

impl EmulatorPackageRunIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::EmulatorPackageRun)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<EmulatorPackageRunIncoming> {
        Box::new(Self { id: None, package: None })
    }

    pub fn new_id(id: String) -> Box<EmulatorPackageRunIncoming> {
        Box::new(Self { id: Some(id), package: None  })
    }

    pub fn new_id_package(id: String, package: String) -> Box<EmulatorPackageRunIncoming> {
        Box::new(Self { id: Some(id), package: Some(package) })
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

    pub fn dbus_method_run_by_id_package(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "ByIdPackage"),
            ("id", "package",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (id, package,): (String, String,)| async move {
                let outgoing = Self::new_id_package(id, package).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    fn run_package(emulator: EmulatorModel, package: String) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        if !emulator.is_running {
            return Ok(StateMessageOutgoing::new_info(tr!("эмулятор должен быть запущен")));
        }
        // @todo handle errors
        emulator.session_user()?.run_package(package)?;
        Ok(StateMessageOutgoing::new_success(tr!("приложение запущено")))
    }
}

impl TraitIncoming for EmulatorPackageRunIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = EmulatorPackageRunIncoming::name();
        let models: Vec<EmulatorModel> = EmulatorModelSelect::search(&self.id, &send_type, tr!("ищем эмулятор"), Some(true));
        // Select
        match models.iter().count() {
            1 => if let Some(package) = self.package.clone() {
                match Self::run_package(models.first().unwrap().clone(), package) {
                    Ok(result) => result,
                    Err(error) => StateMessageOutgoing::new_error(tr!("{}", error)),
                }
            } else {
                let model = models.first().unwrap();
                Box::new(SelectorOutgoing {
                    key,
                    variants: model.get_install_packages()
                        .iter()
                        .map(|e| SelectorIncoming {
                            name: tr!("Пакет: {}", e),
                            incoming: self.select_package(model.get_id(), e.to_string()),
                        })
                        .collect::<Vec<SelectorIncoming<EmulatorPackageRunIncoming>>>(),
                })
            },
            0 => StateMessageOutgoing::new_info(tr!("запущенные эмуляторы не найдены")),
            _ => Box::new(EmulatorModelSelect::select(key, models, |id| self.select(id))),
        }
    }
}
