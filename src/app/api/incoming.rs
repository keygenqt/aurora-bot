use crate::{
    app::api::enums::CommandKey,
    models::incoming::{
        ApiInfoIncoming, AppInfoIncoming, ConnectionIncoming, EmulatorStartIncoming,
    },
};

#[allow(dead_code)]
#[derive(Clone)]
pub enum Incoming {
    // Common
    AppInfo(AppInfoIncoming),
    EmulatorStart(EmulatorStartIncoming),
    // Websocket
    Connection(ConnectionIncoming),
    // D-Bus
    ApiInfo(ApiInfoIncoming),
}

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
