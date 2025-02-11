use crate::{
    models::{
        emulator::model::EmulatorModel,
        outgoing::{
            emulator_terminal::EmulatorTerminalOutgoing, Outgoing, OutgoingState, OutgoingType,
        },
    },
    service::command::exec,
    utils::programs,
};

use super::{Incoming, TraitIncoming};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct EmulatorTerminalIncoming {
    is_root: bool,
}

impl EmulatorTerminalIncoming {
    pub fn new_user() -> Incoming {
        Incoming::EmulatorTerminal(Self { is_root: false })
    }

    pub fn new_root() -> Incoming {
        Incoming::EmulatorTerminal(Self { is_root: true })
    }
}

impl TraitIncoming for EmulatorTerminalIncoming {
    fn name() -> String {
        "EmulatorTerminal".into()
    }

    async fn run(&self, _: OutgoingType) -> Outgoing {
        async fn _exec(is_root: bool) -> Result<(), Box<dyn std::error::Error>> {
            let emulators = EmulatorModel::search().await;
            // Get first emulator, multiselect for the future
            if let Some(emulator) = emulators.iter().next() {
                if emulator.is_running {
                    let program = programs::get_gnome_terminal()?;
                    let user = if is_root { "root" } else { "defaultuser" };
                    let command = format!("ssh -o 'ConnectTimeout=2' -o 'StrictHostKeyChecking=no' {}@localhost -p 2223 -i {}", user, emulator.key);
                    let _ = exec::exec_wait_args(&program, ["--", "bash", "-c", &command])?;
                    Ok(())
                } else {
                    Err("эмулятор закрыт")?
                }
            } else {
                Err("что-то пошло не так")?
            }
        }
        match _exec(self.is_root).await {
            Ok(_) => EmulatorTerminalOutgoing::new(OutgoingState::Success),
            Err(_) => EmulatorTerminalOutgoing::new(OutgoingState::Error),
        }
    }
}
