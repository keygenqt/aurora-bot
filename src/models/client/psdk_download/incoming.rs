use std::time::SystemTime;

use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::ClientMethodsKey;
use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::psdk_available::outgoing::PsdkAvailableItemOutgoing;
use crate::models::client::psdk_available::select::PsdkAvailableSelect;
use crate::models::client::state_message::outgoing::StateMessageOutgoing;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::tr;
use crate::tools::single;
use crate::tools::utils;

#[derive(Serialize, Deserialize, Clone)]
pub struct PsdkDownloadIncoming {
    id: Option<String>,
}

// @todo to server
impl PsdkDownloadIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::PsdkDownload)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<PsdkDownloadIncoming> {
        Box::new(Self { id: None })
    }

    pub fn new_id(id: String) -> Box<PsdkDownloadIncoming> {
        Box::new(Self { id: Some(id) })
    }

    fn select(&self, id: String) -> PsdkDownloadIncoming {
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
}

impl TraitIncoming for PsdkDownloadIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = PsdkDownloadIncoming::name();
        let models: Vec<PsdkAvailableItemOutgoing> = PsdkAvailableSelect::search(
            &self.id,
            &send_type,
            tr!("получаем список..."),
            tr!("получаем данные..."),
        );
        // Exec fun
        fn _run(
            model: PsdkAvailableItemOutgoing,
            send_type: &OutgoingType,
        ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
            StateMessageOutgoing::new_state(tr!("начинаем загрузку...")).send(send_type);
            // Time start
            let start = SystemTime::now();
            // Download
            let urls = model.urls;
            let paths = single::get_request()
                .download_files(urls, StateMessageOutgoing::get_state_callback_file_big(send_type))?;
            let _ = utils::move_to_downloads(paths)?;
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
        // Select
        match models.iter().count() {
            1 => match _run(models.first().unwrap().clone(), &send_type) {
                Ok(value) => value,
                Err(error) => StateMessageOutgoing::new_error(tr!("{}", error)),
            },
            0 => StateMessageOutgoing::new_info(tr!("Platform SDK не найдено")),
            _ => Box::new(PsdkAvailableSelect::select(key, models, |id| self.select(id))),
        }
    }
}
