use crate::app::api::enums::CommandKey;

use super::models::{ApiInfoIncoming, AppInfoIncoming, EmulatorStartIncoming, Incoming};

impl Incoming {
    pub fn app_info() -> Incoming {
        Incoming::AppInfo(AppInfoIncoming {
            key: CommandKey::AppInfo,
            message: "версия приложения".into(),
        })
    }

    pub fn emulator_start() -> Incoming {
        Incoming::EmulatorStart(EmulatorStartIncoming {
            key: CommandKey::EmulatorStart,
            message: "запуск эмулятора".into(),
        })
    }

    pub fn api_info() -> Incoming {
        Incoming::ApiInfo(ApiInfoIncoming {
            key: CommandKey::ApiInfo,
            message: "версия dbus интерфейса".into(),
        })
    }
}
