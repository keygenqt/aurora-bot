use std::fs;
use std::path::PathBuf;
use std::time::SystemTime;

use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::selector::selects::select_flutter_installed::FlutterInstalledModelSelect;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::models::configuration::Config;
use crate::models::configuration::flutter::FlutterConfig;
use crate::models::flutter_installed::model::FlutterInstalledModel;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::print_debug;
use crate::tools::macros::tr;

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterUninstallIncoming {
    id: Option<String>,
}

impl FlutterUninstallIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::FlutterUninstall)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<FlutterUninstallIncoming> {
        print_debug!("> {}: new()", Self::name());
        Box::new(Self { id: None })
    }

    pub fn new_id(id: String) -> Box<FlutterUninstallIncoming> {
        print_debug!("> {}: new_id(id: {})", Self::name(), id);
        Box::new(Self { id: Some(id) })
    }

    fn select(&self, id: String) -> FlutterUninstallIncoming {
        let mut select = self.clone();
        select.id = Some(id);
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

    fn run(
        model: &FlutterInstalledModel,
        send_type: &OutgoingType,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        // Time start
        let start = SystemTime::now();
        ////////////
        // UNINSTALL
        // Check parent nested
        let mut parts = model.dir.split("/").collect::<Vec<&str>>();
        parts.reverse();
        let folder_remove = if parts[1].contains("flutter") && parts[2] == "opt" {
            &PathBuf::from(model.dir.clone()).parent().unwrap().to_path_buf()
        } else {
            &PathBuf::from(model.dir.clone())
        };
        // Remove folder
        StateMessageOutgoing::new_state(tr!("удаление директории: {}", folder_remove.to_string_lossy()))
            .send(send_type);
        fs::remove_dir_all(folder_remove)?;

        //////////
        // SYNC
        StateMessageOutgoing::new_state(tr!("запуск синхронизации Flutter SDK")).send(send_type);
        let _ = Config::save_flutter(FlutterConfig::search());

        ///////
        // DONE
        // Time end
        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        let seconds = duration.as_secs();
        Ok(StateMessageOutgoing::new_success(tr!(
            "удаление Flutter SDK выполнено успешно ({}s)",
            seconds
        )))
    }
}

impl TraitIncoming for FlutterUninstallIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = FlutterUninstallIncoming::name();
        let models =
            FlutterInstalledModelSelect::search(&self.id, tr!("получаем информацию о Flutter SDK"), &send_type);
        // Select
        match models.iter().count() {
            1 => match Self::run(&models.first().unwrap().clone(), &send_type) {
                Ok(result) => result,
                Err(_) => StateMessageOutgoing::new_error(tr!("произошла ошибка при удалении Flutter SDK")),
            },
            0 => StateMessageOutgoing::new_info(tr!("Flutter SDK не найдены")),
            _ => match FlutterInstalledModelSelect::select(key, &send_type, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить Flutter SDK")),
            },
        }
    }
}
