use std::path::PathBuf;

use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::selector::selects::select_flutter_installed::FlutterInstalledModelSelect;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::models::flutter_installed::model::FlutterInstalledModel;
use crate::service::dbus::server::IfaceData;
use crate::tools::format_utils;
use crate::tools::macros::tr;
use crate::tools::utils;

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterProjectFormatIncoming {
    id: Option<String>,
    path: PathBuf,
}

impl FlutterProjectFormatIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::FlutterProjectFormat)
            .unwrap()
            .to_string()
    }

    pub fn new(path: PathBuf) -> Box<FlutterProjectFormatIncoming> {
        Box::new(Self { id: None, path })
    }

    pub fn new_id(path: PathBuf, id: String) -> Box<FlutterProjectFormatIncoming> {
        Box::new(Self { id: Some(id), path })
    }

    fn select(&self, id: String) -> FlutterProjectFormatIncoming {
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
                    None => StateMessageOutgoing::new_error(tr!("проверьте путь к проекту")),
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
                    None => StateMessageOutgoing::new_error(tr!("проверьте путь к проекту")),
                };
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    fn run(model: FlutterInstalledModel, path: &PathBuf) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        // Format
        let result_cpp = format_utils::cpp_format(path)?;
        let result_dart = format_utils::dart_format(path, &PathBuf::from(model.dart))?;
        // Count
        let count_files = result_cpp.count_files + result_dart.count_files;
        let count_formats = result_cpp.count_formats + result_dart.count_formats;
        let count_exclude = result_cpp.count_exclude + result_dart.count_exclude;
        // Result
        if count_formats == 0 {
            Ok(StateMessageOutgoing::new_info(tr!("проект не требует форматирования")))
        } else {
            if count_exclude == 0 {
                Ok(StateMessageOutgoing::new_success(tr!(
                    "найдено: {}, форматировано: {}",
                    count_files,
                    count_formats,
                )))
            } else {
                Ok(StateMessageOutgoing::new_success(tr!(
                    "найдено: {}, форматировано: {}, исключено: {}",
                    count_files,
                    count_formats,
                    count_exclude,
                )))
            }
        }
    }
}

impl TraitIncoming for FlutterProjectFormatIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = FlutterProjectFormatIncoming::name();
        let models =
            FlutterInstalledModelSelect::search(&self.id, tr!("получаем информацию о Flutter SDK"), &send_type);
        // Select
        match models.iter().count() {
            1 => match Self::run(models.first().unwrap().clone(), &self.path) {
                Ok(result) => result,
                Err(error) => StateMessageOutgoing::new_error(tr!("{}", error)),
            },
            0 => StateMessageOutgoing::new_info(tr!("Flutter SDK не найдены")),
            _ => match FlutterInstalledModelSelect::select(key, &send_type, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить Flutter SDK")),
            },
        }
    }
}
