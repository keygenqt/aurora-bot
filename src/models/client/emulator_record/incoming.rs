use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::models::client::incoming::TraitIncoming;
use crate::models::client::outgoing::OutgoingType;
use crate::models::client::outgoing::TraitOutgoing;
use crate::models::client::state_message::outgoing::StateMessageOutgoing;
use crate::models::client::ClientMethodsKey;
use crate::models::emulator::model::EmulatorModel;
use crate::models::emulator::select::EmulatorModelSelect;
use crate::service::dbus::server::IfaceData;
use crate::tools::macros::tr;

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorRecordIncoming {
    id: Option<String>,
    enable: bool,
}

// @todo Add to server
impl EmulatorRecordIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::EmulatorRecord)
            .unwrap()
            .to_string()
    }

    pub fn new(enable: bool) -> Box<EmulatorRecordIncoming> {
        Box::new(Self { id: None, enable })
    }

    pub fn new_id(id: String, enable: bool) -> Box<EmulatorRecordIncoming> {
        Box::new(Self { id: Some(id), enable })
    }

    fn select(&self, id: String) -> EmulatorRecordIncoming {
        let mut select = self.clone();
        select.id = Some(id);
        select
    }

    pub fn dbus_method_run(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            Self::name(),
            ("enable",),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (enable,): (bool,)| async move {
                let outgoing = Self::new(enable).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    pub fn dbus_method_run_by_id(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            format!("{}{}", Self::name(), "ById"),
            ("id", "enable"),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (id, enable): (String, bool)| async move {
                let outgoing = Self::new_id(id, enable).run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_json(),)))
            },
        );
    }

    fn action_enable(_: EmulatorModel, _: &OutgoingType) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        Ok(StateMessageOutgoing::new_info(tr!("@todo enable")))
    }

    fn action_disable(
        _: EmulatorModel,
        _: &OutgoingType,
    ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
        Ok(StateMessageOutgoing::new_info(tr!("@todo disable")))
    }
}

impl TraitIncoming for EmulatorRecordIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = EmulatorRecordIncoming::name();
        let models: Vec<EmulatorModel> = EmulatorModelSelect::search(&self.id, &send_type, Some(false));
        // Select
        match models.iter().count() {
            1 => {
                if self.enable {
                    match Self::action_enable(models.first().unwrap().clone(), &send_type) {
                        Ok(result) => result,
                        Err(_) => StateMessageOutgoing::new_error(tr!("не активировать запись видео")),
                    }
                } else {
                    match Self::action_disable(models.first().unwrap().clone(), &send_type) {
                        Ok(result) => result,
                        Err(_) => StateMessageOutgoing::new_error(tr!("не удалось остановить запись видео")),
                    }
                }
            }
            0 => StateMessageOutgoing::new_info(tr!("эмуляторы не найдены")),
            _ => Box::new(EmulatorModelSelect::select(key, models, |id| self.select(id))),
        }
    }
}
