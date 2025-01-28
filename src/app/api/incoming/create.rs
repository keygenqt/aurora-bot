use crate::app::api::enums::ClientKey;

use super::models::{AppInfoIncoming, EmulatorStartIncoming, Incoming};

impl Incoming {
    pub fn app_info() -> Incoming {
        Incoming::AppInfo(AppInfoIncoming {
            key: ClientKey::AppInfo,
            message: "версия приложения".into(),
        })
    }

    pub fn emulator_start() -> Incoming {
        Incoming::EmulatorStart(EmulatorStartIncoming {
            key: ClientKey::EmulatorStart,
            message: "запуск эмулятора".into(),
        })
    }
}
