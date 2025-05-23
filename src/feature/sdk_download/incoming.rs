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
use crate::models::sdk_available::model::SdkAvailableModel;
use crate::models::sdk_available::model::SdkInstallType;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::tr;
use crate::tools::single;
use crate::tools::utils;

#[derive(Serialize, Deserialize, Clone)]
pub struct SdkDownloadIncoming {
    id: Option<String>,
}

impl SdkDownloadIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::SdkDownload)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<SdkDownloadIncoming> {
        Box::new(Self { id: None })
    }

    pub fn new_id(id: String) -> Box<SdkDownloadIncoming> {
        Box::new(Self { id: Some(id) })
    }

    fn select(&self, id: String) -> SdkDownloadIncoming {
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
        let _ = utils::move_to_downloads(vec![path])?;
        // Time end
        let end = SystemTime::now();
        let duration = end.duration_since(start).unwrap();
        let seconds = duration.as_secs();
        // Done
        Ok(StateMessageOutgoing::new_success(tr!(
            "загрузка успешно выполнена ({}s)",
            seconds
        )))
    }
}

impl TraitIncoming for SdkDownloadIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = SdkDownloadIncoming::name();
        let models = SdkAvailableModelSelect::search(&self.id, tr!("получаем список..."), &send_type);
        // Select
        match models.iter().count() {
            1 => match Self::run(models.first().unwrap().clone(), &send_type) {
                Ok(value) => value,
                Err(error) => StateMessageOutgoing::new_error(tr!("{}", error)),
            },
            0 => StateMessageOutgoing::new_info(tr!("Flutter SDK не найдены")),
            _ => match SdkAvailableModelSelect::select(key, &send_type, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить Аврора SDK")),
            },
        }
    }
}
