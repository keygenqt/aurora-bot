use std::path::PathBuf;

use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::models::pubspec::model::PubspecModel;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::print_debug;
use crate::tools::macros::tr;
use crate::tools::utils;

use super::outgoing::FlutterProjectReportOutgoing;

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterProjectReportIncoming {
    id: Option<String>,
    path: PathBuf,
}

impl FlutterProjectReportIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::FlutterProjectReport)
            .unwrap()
            .to_string()
    }

    pub fn new_path(path: PathBuf) -> Box<FlutterProjectReportIncoming> {
        print_debug!("> {}: new_path(path: {})", Self::name(), path.to_string_lossy());
        Box::new(Self { id: None, path })
    }

    pub fn new_path_id(id: String, path: PathBuf) -> Box<FlutterProjectReportIncoming> {
        print_debug!(
            "> {}: new_path_id(id: {}, path: {})",
            Self::name(),
            id,
            path.to_string_lossy()
        );
        Box::new(Self { id: Some(id), path })
    }

    pub fn dbus_method_run_path(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            Self::name(),
            ("path",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (path,): (String,)| async move {
                let outgoing = match utils::path_to_absolute(&PathBuf::from(path)) {
                    Some(path) => Self::new_path(path).run(OutgoingType::Dbus),
                    None => StateMessageOutgoing::new_error(tr!("проверьте путь к pubspec.yaml")),
                };
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    pub fn dbus_method_run_path_by_id(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "ById"),
            ("id", "path"),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (id, path): (String, String)| async move {
                let outgoing = match utils::path_to_absolute(&PathBuf::from(path)) {
                    Some(path) => Self::new_path_id(id, path).run(OutgoingType::Dbus),
                    None => StateMessageOutgoing::new_error(tr!("проверьте путь к pubspec.yaml")),
                };
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    fn run(path: &PathBuf, send_type: &OutgoingType) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        if path.file_name().unwrap().to_str().unwrap() != "pubspec.yaml" {
            Err(tr!("укажите путь к pubspec.yaml"))?;
        }
        // Parse yaml and get all dependency
        StateMessageOutgoing::new_state(tr!("получение данных пакета")).send(send_type);
        let package = match PubspecModel::parse_model(path) {
            Ok(value) => value,
            Err(_) => Err("не удалось прочитать pubspec.yaml")?,
        };
        // Get all dependency
        StateMessageOutgoing::new_state(tr!("получение зависимостей пакета")).send(send_type);
        let dependencies =
            match PubspecModel::search_dependencies(path, StateMessageOutgoing::get_state_callback(send_type)) {
                Ok(value) => value,
                Err(_) => Err("не удалось получить зависимости")?,
            };
        // Gen report
        StateMessageOutgoing::new_state(tr!("генерация отчета")).send(send_type);
        let path = utils::get_report_save_path();
        match PubspecModel::gen_report_pdf(package, dependencies, &path) {
            Ok(_) => {
                let path = path.to_string_lossy().to_string();
                Ok(FlutterProjectReportOutgoing::new(
                    path.clone(),
                    utils::file_to_base64_by_path(Some(path.as_str())),
                ))
            }
            Err(_) => Err("не удалось создать отчет")?,
        }
    }
}

impl TraitIncoming for FlutterProjectReportIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        match Self::run(&self.path, &send_type) {
            Ok(result) => result,
            Err(error) => StateMessageOutgoing::new_error(tr!("{}", error)),
        }
    }
}
