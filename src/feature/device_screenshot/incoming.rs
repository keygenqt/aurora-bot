use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::selector::selects::select_device::DeviceModelSelect;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::models::device::model::DeviceModel;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::print_debug;
use crate::tools::macros::tr;
use crate::tools::utils;

use super::outgoing::DeviceScreenshotOutgoing;

#[derive(Serialize, Deserialize, Clone)]
pub struct DeviceScreenshotIncoming {
    id: Option<String>,
}

impl DeviceScreenshotIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::DeviceScreenshot)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<DeviceScreenshotIncoming> {
        print_debug!("> {}: new()", Self::name());
        Box::new(Self { id: None })
    }

    pub fn new_id(id: String) -> Box<DeviceScreenshotIncoming> {
        print_debug!("> {}: new_id(id: {})", Self::name(), id);
        Box::new(Self { id: Some(id) })
    }

    fn select(&self, id: String) -> DeviceScreenshotIncoming {
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

    fn run(model: DeviceModel) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        let session = model.session_user()?;
        let path = session.take_screenshot()?;
        let path = path.to_string_lossy().to_string();
        Ok(DeviceScreenshotOutgoing::new(
            path.clone(),
            utils::file_to_base64_by_path(Some(path.as_str())),
        ))
    }
}

impl TraitIncoming for DeviceScreenshotIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = DeviceScreenshotIncoming::name();
        let models = DeviceModelSelect::search(&self.id, tr!("получаем информацию об устройствах"), &send_type);
        // Select
        match models.iter().count() {
            1 => match Self::run(models.first().unwrap().clone()) {
                Ok(result) => result,
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось сделать скриншот")),
            },
            0 => StateMessageOutgoing::new_info(tr!("устройства не найдены")),
            _ => match DeviceModelSelect::select(key, &send_type, models, |id| self.select(id)) {
                Ok(value) => Box::new(value),
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось получить устройство")),
            },
        }
    }
}
