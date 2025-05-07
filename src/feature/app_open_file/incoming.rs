use std::path::PathBuf;

use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::service::command::exec;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::print_debug;
use crate::tools::macros::tr;
use crate::tools::programs;
use crate::tools::utils;

#[derive(Serialize, Deserialize, Clone)]
pub struct AppOpenFileIncoming {
    path: String,
}

impl AppOpenFileIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::AppOpenFile)
            .unwrap()
            .to_string()
    }

    pub fn new(path: String) -> Box<AppOpenFileIncoming> {
        print_debug!("> {}: new(path: {})", Self::name(), path);
        Box::new(Self { path })
    }

    pub fn dbus_method_run(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            Self::name(),
            ("path",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (path,): (String,)| async move {
                let outgoing = Self::new(path).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    fn exec(path: &String) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        let path = if path.contains("/") {
            PathBuf::from(path)
        } else {
            utils::get_file_save_path(path)
        };
        let path = match utils::path_to_absolute(&path) {
            Some(value) => value,
            None => Err(tr!("проверьте путь к файлу"))?,
        };
        // @todo run with nohup or another thread
        let program = programs::get_xdg_open()?;
        let output = exec::exec_wait_args(&program, [path])?;
        if !output.status.success() {
            Err(tr!("не удалось открыть файл"))?
        }
        Ok(StateMessageOutgoing::new_success(tr!(
            "файл успешно открыт"
        )))
    }
}

impl TraitIncoming for AppOpenFileIncoming {
    fn run(&self, _: OutgoingType) -> Box<dyn TraitOutgoing> {
        match Self::exec(&self.path) {
            Ok(result) => result,
            Err(e) => StateMessageOutgoing::new_error(tr!("{}", e)),
        }
    }
}
