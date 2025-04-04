use std::path::PathBuf;

use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::ClientMethodsKey;
use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::state_message::outgoing::StateMessageOutgoing;
use crate::service::command::exec;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::print_debug;
use crate::tools::macros::tr;
use crate::tools::programs;
use crate::tools::utils;

#[derive(Serialize, Deserialize, Clone)]
pub struct AppOpenDirIncoming {
    path: String,
}

impl AppOpenDirIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::AppOpenDir)
            .unwrap()
            .to_string()
    }

    pub fn new(path: String) -> Box<AppOpenDirIncoming> {
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
        let path = PathBuf::from(path);
        let path = match utils::path_to_absolute(&path) {
            Some(value) => value,
            None => Err(tr!("проверьте путь к директории"))?
        };
        let program = programs::get_xdg_open()?;
        let output = exec::exec_wait_args(&program, [path])?;
        if !output.status.success() {
            Err(tr!("не удалось активировать запись видео"))?
        }
        Ok(StateMessageOutgoing::new_success(tr!("файловый менеджер открыт успешно")))
    }
}

impl TraitIncoming for AppOpenDirIncoming {
    fn run(&self, _: OutgoingType) -> Box<dyn TraitOutgoing> {
        match Self::exec(&self.path) {
            Ok(result) => result,
            Err(e) => StateMessageOutgoing::new_error(tr!("{}", e)),
        }
    }
}
