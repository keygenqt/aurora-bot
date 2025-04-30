use std::path::PathBuf;
use std::time::SystemTime;

use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::selector::selects::select_psdk_installed::PsdkInstalledModelSelect;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::models::configuration::Config;
use crate::models::configuration::psdk::PsdkConfig;
use crate::models::psdk_installed::model::PsdkInstalledModel;
use crate::service::command;
use crate::service::command::exec;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::print_debug;
use crate::tools::macros::tr;
use crate::tools::programs;

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkUninstallIncoming {
    id: Option<String>,
}

impl PsdkUninstallIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::PsdkUninstall)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<PsdkUninstallIncoming> {
        print_debug!("> {}: new()", Self::name());
        Box::new(Self { id: None })
    }

    pub fn new_id(id: String) -> Box<PsdkUninstallIncoming> {
        print_debug!("> {}: new_id(id: {})", Self::name(), id);
        Box::new(Self { id: Some(id) })
    }

    fn select(&self, id: String) -> PsdkUninstallIncoming {
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
        model: PsdkInstalledModel,
        send_type: &OutgoingType,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        // Time start
        let start = SystemTime::now();
        ////////////
        // UNINSTALL
        // Get remove folder
        let folder_remove = &PathBuf::from(model.dir.replace("/sdks/aurora_psdk", ""));
        // Remove folder
        if folder_remove.exists() {
            StateMessageOutgoing::new_state(tr!("удаление директории: {}", folder_remove.to_string_lossy()))
                .send(send_type);
            let sudo = programs::get_sudo()?;
            let _ = exec::exec_wait_args(&sudo, ["rm", "-rf", &folder_remove.to_string_lossy().to_string()])?;
        }

        //////////
        // SUDOERS
        StateMessageOutgoing::new_state(tr!("обновление записи sudoers Platform SDK")).send(send_type);
        let models = PsdkInstalledModel::search_full_without_targets()?;
        command::psdk::add_sudoers_chroot_access(&models)?;

        //////////
        // SYNC
        StateMessageOutgoing::new_state(tr!("запуск синхронизации Platform SDK")).send(send_type);
        let _ = Config::save_psdk(PsdkConfig::search());

        ///////
        // DONE
        // Time end
        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        let seconds = duration.as_secs();
        Ok(StateMessageOutgoing::new_success(tr!(
            "удаление Platform SDK успешно выполнено ({}s)",
            seconds
        )))
    }
}

impl TraitIncoming for PsdkUninstallIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = PsdkUninstallIncoming::name();
        let models = PsdkInstalledModelSelect::search(&self.id, tr!("получаем информацию о Platform SDK"), &send_type);
        // Select
        match models.iter().count() {
            1 => match Self::run(models.first().unwrap().clone(), &send_type) {
                Ok(result) => result,
                Err(_) => StateMessageOutgoing::new_error(tr!("произошла ошибка при удалении Platform SDK")),
            },
            0 => StateMessageOutgoing::new_info(tr!("Platform SDK не найдены")),
            _ => match PsdkInstalledModelSelect::select(key, &send_type, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить Platform SDK")),
            },
        }
    }
}
