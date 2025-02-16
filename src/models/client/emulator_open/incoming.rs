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
pub struct EmulatorOpenIncoming {
    id: Option<String>,
}

impl EmulatorOpenIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::EmulatorOpen)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<EmulatorOpenIncoming> {
        Box::new(Self { id: None })
    }

    pub fn new_id(id: String) -> Box<EmulatorOpenIncoming> {
        Box::new(Self { id: Some(id) })
    }

    pub fn dbus_method_run(builder: &mut IfaceBuilder<IfaceData>) {
        builder.method_with_cr_async(
            Self::name(),
            (),
            ("result",),
            move |mut ctx: dbus_crossroads::Context, _, (): ()| async move {
                let outgoing = Self::new().run(OutgoingType::Dbus);
                ctx.reply(Ok((outgoing.to_string(),)))
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
                ctx.reply(Ok((outgoing.to_string(),)))
            },
        );
    }
}

impl TraitIncoming for EmulatorOpenIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        // Search
        let key = EmulatorOpenIncoming::name();
        let models: Vec<EmulatorModel> = EmulatorModelSelect::search(&self.id, &send_type, Some(false));
        // Exec fun
        fn _run(
            emulator: EmulatorModel,
            send_type: &OutgoingType,
        ) -> Result<Box<dyn TraitOutgoing>, Box<dyn std::error::Error>> {
            if !emulator.is_running {
                StateMessageOutgoing::new_state(tr!("открываем эмулятор")).send(send_type);
                emulator.start()?;
            }
            StateMessageOutgoing::new_state(tr!("соединение с эмулятором")).send(send_type);
            // Get emulator connect session
            let emulator = emulator.session_user()?;
            // Close connect
            emulator.close()?;
            // Done
            Ok(StateMessageOutgoing::new_success(tr!(
                "эмулятор {} готов к работе",
                emulator.os_name
            )))
        }
        // Select
        match models.iter().count() {
            1 => match _run(models.first().unwrap().clone(), &send_type) {
                Ok(result) => result,
                Err(_) => StateMessageOutgoing::new_error(tr!("не удалось открыть эмулятор")),
            },
            0 => StateMessageOutgoing::new_info(tr!("эмуляторы не найдены")),
            _ => Box::new(EmulatorModelSelect::select(key, models, |id| {
                *EmulatorOpenIncoming::new_id(id)
            })),
        }
    }
}
