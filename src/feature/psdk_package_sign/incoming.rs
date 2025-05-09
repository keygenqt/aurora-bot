use std::path::PathBuf;

use colored::Colorize;
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
use crate::service::command;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::tr;
use crate::tools::utils;

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkPackageSignIncoming {
    id: Option<String>,
    path: PathBuf,
}

impl PsdkPackageSignIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::PsdkPackageSign)
            .unwrap()
            .to_string()
    }

    pub fn new(path: PathBuf) -> Box<PsdkPackageSignIncoming> {
        Box::new(Self { id: None, path })
    }

    pub fn new_id(path: PathBuf, id: String) -> Box<PsdkPackageSignIncoming> {
        Box::new(Self { id: Some(id), path })
    }

    fn select(&self, id: String) -> PsdkPackageSignIncoming {
        let mut select = self.clone();
        select.id = Some(id);
        select
    }

    pub fn dbus_method_run(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            Self::name(),
            ("path",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (path,): (String,)| async move {
                let outgoing = match utils::path_to_absolute(&PathBuf::from(path)) {
                    Some(path) => Self::new(path).run(OutgoingType::Dbus),
                    None => StateMessageOutgoing::new_error(tr!("проверьте путь к файлу")),
                };
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    pub fn dbus_method_run_by_id(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "ById"),
            ("path", "id"),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (path, id): (String, String)| async move {
                let outgoing = match utils::path_to_absolute(&PathBuf::from(path)) {
                    Some(path) => Self::new_id(path, id).run(OutgoingType::Dbus),
                    None => StateMessageOutgoing::new_error(tr!("проверьте путь к файлу")),
                };
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    fn run(model: PsdkInstalledModel, path: &PathBuf) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        if !path.is_file() {
            Err(tr!("необходимо указать путь к файлу"))?
        }
        let package_name = match utils::get_package_name(path) {
            Some(value) => value,
            None => Err(tr!("необходимо указать путь к RPM пакету"))?,
        };
        if !command::psdk::rpm_sign(&model.chroot, path) {
            Err(tr!("подпись пакета не удалось"))?;
        }
        Ok(StateMessageOutgoing::new_success(tr!(
            "пакет {} успешно подписан",
            package_name.bold()
        )))
    }
}

impl TraitIncoming for PsdkPackageSignIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = PsdkPackageSignIncoming::name();
        let models = PsdkInstalledModelSelect::search(&self.id, tr!("получаем информацию о Platform SDK"), &send_type);
        // Select
        match models.iter().count() {
            1 => match Self::run(models.first().unwrap().clone(), &self.path) {
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
