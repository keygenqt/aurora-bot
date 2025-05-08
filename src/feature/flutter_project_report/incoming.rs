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
use crate::tools::gen_pdf;
use crate::tools::macros::tr;
use crate::tools::single;
use crate::tools::utils;

use super::outgoing::FlutterProjectReportOutgoing;

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterProjectReportIncoming {
    id: Option<String>,
    path: Option<PathBuf>,
    url: Option<String>,
}

impl FlutterProjectReportIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::FlutterProjectReport)
            .unwrap()
            .to_string()
    }

    pub fn new_path(path: PathBuf) -> Box<FlutterProjectReportIncoming> {
        Box::new(Self {
            id: None,
            path: Some(path),
            url: None,
        })
    }

    pub fn new_path_id(path: PathBuf, id: String) -> Box<FlutterProjectReportIncoming> {
        Box::new(Self {
            id: Some(id),
            path: Some(path),
            url: None,
        })
    }

    pub fn new_url(url: String) -> Box<FlutterProjectReportIncoming> {
        Box::new(Self {
            id: None,
            path: None,
            url: Some(url),
        })
    }

    pub fn new_url_id(url: String, id: String) -> Box<FlutterProjectReportIncoming> {
        Box::new(Self {
            id: Some(id),
            path: None,
            url: Some(url),
        })
    }

    pub fn dbus_method_run_path(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "Path"),
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
            format!("{}{}", Self::name(), "PathById"),
            ("path", "id",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (path, id,): (String, String,)| async move {
                let outgoing = match utils::path_to_absolute(&PathBuf::from(path)) {
                    Some(path) => Self::new_path_id(path, id).run(OutgoingType::Dbus),
                    None => StateMessageOutgoing::new_error(tr!("проверьте путь к pubspec.yaml")),
                };
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    pub fn dbus_method_run_url(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "Url"),
            ("url",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (url,): (String,)| async move {
                let outgoing = Self::new_url(url).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    pub fn dbus_method_run_url_by_id(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "UrlById"),
            ("url", "id"),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (url, id,): (String, String,)| async move {
                let outgoing = Self::new_url_id(url, id).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    fn run_path(
        path: &PathBuf,
        send_type: &OutgoingType,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
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
        match gen_pdf::gen_report_dart_packages(package, dependencies, &path) {
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

    fn run_url(url: &String, send_type: &OutgoingType) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        StateMessageOutgoing::new_state(tr!("скачиваем файл...")).send(send_type);
        let url = match utils::get_https_url(url.to_string()) {
            Some(url) => url,
            None => Err(tr!("не удалось скачать файл"))?,
        };
        let path = single::get_request().download_file(
            url.to_string(),
            StateMessageOutgoing::get_state_callback_file_small(send_type),
        )?;
        // Run gen report
        Self::run_path(&path, send_type)
    }
}

impl TraitIncoming for FlutterProjectReportIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        if let Some(path) = &self.path {
            return match Self::run_path(path, &send_type) {
                Ok(result) => result,
                Err(error) => StateMessageOutgoing::new_error(tr!("{}", error)),
            };
        }
        if let Some(url) = &self.url {
            return match Self::run_url(url, &send_type) {
                Ok(result) => result,
                Err(error) => StateMessageOutgoing::new_error(tr!("{}", error)),
            };
        }
        StateMessageOutgoing::new_error(tr!("нужно указать путь или url к pubspec.yaml"))
    }
}
