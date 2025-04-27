use std::time::SystemTime;

use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::selector::selects::select_sdk_installed::SdkInstalledModelSelect;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::models::configuration::Config;
use crate::models::configuration::sdk::SdkConfig;
use crate::models::sdk_installed::model::SdkInstalledModel;
use crate::service::command::exec;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::print_debug;
use crate::tools::macros::tr;

#[derive(Serialize, Deserialize, Clone)]
pub struct SdkUninstallIncoming {
    id: Option<String>,
}

impl SdkUninstallIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::SdkUninstall)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<SdkUninstallIncoming> {
        print_debug!("> {}: new()", Self::name());
        Box::new(Self { id: None })
    }

    pub fn new_id(id: String) -> Box<SdkUninstallIncoming> {
        print_debug!("> {}: new_id(id: {})", Self::name(), id);
        Box::new(Self { id: Some(id) })
    }

    fn select(&self, id: String) -> SdkUninstallIncoming {
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
        model: SdkInstalledModel,
        send_type: &OutgoingType,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        // Time start
        let start = SystemTime::now();
        ////////////
        // UNINSTALL
        StateMessageOutgoing::new_state(tr!("открываем Maintenance tools")).send(send_type);
        exec::exec_wait(&model.tools)?;

        //////////
        // SYNC
        StateMessageOutgoing::new_state(tr!("запуск синхронизации Аврора SDK")).send(send_type);
        match Config::save_sdk(SdkConfig::search()) {
            true => {
                // Time end
                let end = SystemTime::now();
                let duration = end.duration_since(start).unwrap();
                let seconds = duration.as_secs();
                // Result
                Ok(StateMessageOutgoing::new_success(tr!(
                    "удаление Аврора SDK успешно выполнено ({}s)",
                    seconds
                )))
            }
            false => Ok(StateMessageOutgoing::new_warning(tr!(
                "конфигурация не была обновлена, вы точно удалили Аврора SDK?"
            ))),
        }
    }
}

impl TraitIncoming for SdkUninstallIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = SdkUninstallIncoming::name();
        let models = SdkInstalledModelSelect::search(&self.id, tr!("получаем информацию о Аврора SDK"), &send_type);
        // Select
        match models.iter().count() {
            1 => match Self::run(models.first().unwrap().clone(), &send_type) {
                Ok(result) => result,
                Err(_) => StateMessageOutgoing::new_error(tr!("произошла ошибка при удалении Аврора SDK")),
            },
            0 => StateMessageOutgoing::new_info(tr!("Аврора SDK не найдены")),
            _ => match SdkInstalledModelSelect::select(key, &send_type, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить Аврора SDK")),
            },
        }
    }
}
