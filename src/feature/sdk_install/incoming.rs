use std::time::SystemTime;

use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::selector::selects::select_sdk_available::SdkAvailableModelSelect;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::models::configuration::Config;
use crate::models::configuration::sdk::SdkConfig;
use crate::models::sdk_available::model::SdkAvailableModel;
use crate::models::sdk_available::model::SdkInstallType;
use crate::service::command::exec;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::print_debug;
use crate::tools::macros::tr;
use crate::tools::single;
use crate::tools::utils;

#[derive(Serialize, Deserialize, Clone)]
pub struct SdkInstallIncoming {
    id: Option<String>,
}

impl SdkInstallIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::SdkInstall)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<SdkInstallIncoming> {
        print_debug!("> {}: new()", Self::name());
        Box::new(Self { id: None })
    }

    pub fn new_id(id: String) -> Box<SdkInstallIncoming> {
        print_debug!("> {}: new_id(id: {})", Self::name(), id);
        Box::new(Self { id: Some(id) })
    }

    fn select(&self, id: String) -> SdkInstallIncoming {
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
        model: SdkAvailableModel,
        send_type: &OutgoingType,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        ///////////
        // DOWNLOAD
        StateMessageOutgoing::new_state(tr!("начинаем загрузку...")).send(send_type);
        // Time start
        let start = SystemTime::now();
        // Download
        let url = model.url;
        let path = if model.install_type == SdkInstallType::Offline {
            single::get_request().download_file(url, StateMessageOutgoing::get_state_callback_file_big(send_type))?
        } else {
            single::get_request().download_file(url, StateMessageOutgoing::get_state_callback_file_small(send_type))?
        };
        let downloads = utils::move_to_downloads(vec![path])?;
        // Time end
        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        let seconds = duration.as_secs();
        // Download done
        StateMessageOutgoing::new_info(tr!("загрузка успешно выполнена ({}s)", seconds)).send(send_type);

        //////////
        // INSTALL
        StateMessageOutgoing::new_info(tr!("запуск установки Аврора SDK, нажмите там далее, далее, далее..."))
            .send(send_type);
        let sdk_run = downloads.first().unwrap();
        let _ = exec::exec_wait_args("chmod", ["+x", &sdk_run.to_string_lossy().to_string()])?;
        exec::exec_wait(&sdk_run.to_string_lossy().to_string())?;

        //////////
        // SYNC
        // Time start
        let start = SystemTime::now();
        StateMessageOutgoing::new_state(tr!("запуск синхронизации Аврора SDK")).send(send_type);
        match Config::save_sdk(SdkConfig::search()) {
            true => {
                // Time end
                let end = SystemTime::now();
                let duration = end.duration_since(start).unwrap();
                let seconds = duration.as_secs();
                StateMessageOutgoing::new_info(tr!("конфигурация успешно обновлена ({}s)", seconds)).send(send_type);
                // Result
                Ok(StateMessageOutgoing::new_success(tr!(
                    "уставка Аврора SDK выполнена успешно"
                )))
            }
            false => Ok(StateMessageOutgoing::new_warning(tr!(
                "конфигурация не была обновлена, вы точно нажали далее, далее, далее?"
            ))),
        }
    }
}

impl TraitIncoming for SdkInstallIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = SdkInstallIncoming::name();
        let models = SdkAvailableModelSelect::search(&self.id, tr!("получаем список..."), &send_type);
        // Select
        match models.iter().count() {
            1 => match Self::run(models.first().unwrap().clone(), &send_type) {
                Ok(result) => result,
                Err(error) => StateMessageOutgoing::new_error(tr!("{}", error)),
            },
            0 => StateMessageOutgoing::new_info(tr!("не удалось получить данные")),
            _ => match SdkAvailableModelSelect::select(key, &send_type, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить Аврора SDK")),
            },
        }
    }
}
