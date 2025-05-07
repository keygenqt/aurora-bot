use std::fs;
use std::fs::File;
use std::time::SystemTime;

use dbus_crossroads::IfaceBuilder;
use flate2::read::GzDecoder;
use serde::Deserialize;
use serde::Serialize;
use tar::Archive;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::selector::selects::select_flutter_available::FlutterAvailableModelSelect;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::models::configuration::Config;
use crate::models::configuration::flutter::FlutterConfig;
use crate::models::flutter_available::model::FlutterAvailableModel;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::print_debug;
use crate::tools::macros::tr;
use crate::tools::single;
use crate::tools::utils;

#[derive(Serialize, Deserialize, Clone)]
pub struct FlutterInstallIncoming {
    id: Option<String>,
}

impl FlutterInstallIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::FlutterInstall)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<FlutterInstallIncoming> {
        print_debug!("> {}: new()", Self::name());
        Box::new(Self { id: None })
    }

    pub fn new_id(id: String) -> Box<FlutterInstallIncoming> {
        print_debug!("> {}: new_id(id: {})", Self::name(), id);
        Box::new(Self { id: Some(id) })
    }

    fn select(&self, id: String) -> FlutterInstallIncoming {
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
        model: FlutterAvailableModel,
        send_type: &OutgoingType,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        ///////////
        // DOWNLOAD
        // Time start
        let start = SystemTime::now();
        StateMessageOutgoing::new_state(tr!("начинаем загрузку...")).send(send_type);
        // Download
        let url = match model.url_repo {
            Some(url_repo) => url_repo,
            None => model.url_tar_gz,
        };
        let path =
            single::get_request().download_file(url, StateMessageOutgoing::get_state_callback_file_small(send_type))?;
        let downloads = utils::move_to_downloads(vec![path])?;
        // Time end
        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        let seconds = duration.as_secs();
        // Download done
        StateMessageOutgoing::new_info(tr!("загрузка успешно выполнена ({}s)", seconds)).send(send_type);

        //////////
        // INSTALL
        // Time start
        let start = SystemTime::now();
        // Unpack
        StateMessageOutgoing::new_state(tr!("начинаем распаковку...")).send(send_type);
        let flutter_tar = downloads.first().unwrap();
        let folder_name = flutter_tar
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .replace(".tar.gz", "");
        // Unpack folder
        let mut path_unpack = utils::get_home_folder_path();
        path_unpack.push(".local/opt");
        path_unpack.push(folder_name);
        // Check exist dir
        if path_unpack.exists() {
            return Ok(StateMessageOutgoing::new_warning(tr!(
                "установка не будет продолжена, найдена директория: {}",
                path_unpack.to_string_lossy()
            )));
        }
        let tar_gz = File::open(flutter_tar)?;
        let tar = GzDecoder::new(tar_gz);
        let mut archive = Archive::new(tar);
        archive.unpack(&path_unpack)?;
        StateMessageOutgoing::new_state(tr!("распаковка успешно выполнена")).send(send_type);
        // Check .git folder for work flutter
        let flutters_path = utils::search_files_by_path("bin/flutter", &path_unpack);
        let first = flutters_path.first().unwrap().replace("bin/flutter", "");
        fs::create_dir(format!("{}.git", first))?;

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
            "установка Flutter SDK успешно выполнена ({}s)",
            seconds
        )))
    }
}

impl TraitIncoming for FlutterInstallIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = FlutterInstallIncoming::name();
        let models = FlutterAvailableModelSelect::search(&self.id, tr!("получаем список..."), &send_type);
        // Select
        match models.iter().count() {
            1 => match Self::run(models.first().unwrap().clone(), &send_type) {
                Ok(result) => result,
                Err(error) => StateMessageOutgoing::new_error(format!("{}", error)),
            },
            0 => StateMessageOutgoing::new_info(tr!("не удалось получить данные")),
            _ => match FlutterAvailableModelSelect::select(key, &send_type, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить Flutter SDK")),
            },
        }
    }
}
