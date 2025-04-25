use std::path::PathBuf;

use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::service::dbus::server::IfaceData;
use crate::tools::format_utils;
use crate::tools::macros::print_debug;
use crate::tools::macros::tr;
use crate::tools::utils;

#[derive(Serialize, Deserialize, Clone)]
pub struct SdkProjectFormatIncoming {
    id: Option<String>,
    path: PathBuf,
}

impl SdkProjectFormatIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::SdkProjectFormat)
            .unwrap()
            .to_string()
    }

    pub fn new_path(path: PathBuf) -> Box<SdkProjectFormatIncoming> {
        print_debug!("> {}: new_path(path: {})", Self::name(), path.to_string_lossy());
        Box::new(Self { id: None, path })
    }

    pub fn new_path_id(id: String, path: PathBuf) -> Box<SdkProjectFormatIncoming> {
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
                    None => StateMessageOutgoing::new_error(tr!("проверьте путь к проекту")),
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
                    None => StateMessageOutgoing::new_error(tr!("проверьте путь к проекту")),
                };
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    #[allow(unused_variables)]
    fn run(path: &PathBuf, send_type: &OutgoingType) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        // Format
        let result = format_utils::cpp_format(path)?;
        // Result
        if result.count_formats == 0 {
            Ok(StateMessageOutgoing::new_info(tr!("проект не требует форматирования")))
        } else {
            if result.count_exclude == 0 {
                Ok(StateMessageOutgoing::new_success(tr!(
                    "найдено: {}, форматировано: {}",
                    result.count_files,
                    result.count_formats,
                )))
            } else {
                Ok(StateMessageOutgoing::new_success(tr!(
                    "найдено: {}, форматировано: {}, исключено: {}",
                    result.count_files,
                    result.count_formats,
                    result.count_exclude,
                )))
            }
        }
    }
}

impl TraitIncoming for SdkProjectFormatIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        match Self::run(&self.path, &send_type) {
            Ok(value) => value,
            Err(error) => StateMessageOutgoing::new_error(tr!("{}", error)),
        }
    }
}
