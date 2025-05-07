use dbus_crossroads::IfaceBuilder;
use serde::Deserialize;
use serde::Serialize;

use crate::feature::ClientMethodsKey;
use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::outgoing::TraitOutgoing;
use crate::feature::state_message::outgoing::StateMessageOutgoing;
use crate::models::configuration::Config;
use crate::models::configuration::device::DeviceConfig;
use crate::service::dbus::server::IfaceData;
use crate::tools::constants;
use crate::tools::macros::print_debug;
use crate::tools::macros::tr;
use crate::tools::utils;

#[derive(Serialize, Deserialize, Clone)]
pub struct DeviceSyncIncoming {}

impl DeviceSyncIncoming {
    pub fn name() -> String {
        serde_variant::to_variant_name(&ClientMethodsKey::DeviceSync)
            .unwrap()
            .to_string()
    }

    pub fn new() -> Box<DeviceSyncIncoming> {
        print_debug!("> {}: new()", Self::name());
        Box::new(Self {})
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
}

impl TraitIncoming for DeviceSyncIncoming {
    fn run(&self, send_type: OutgoingType) -> Box<dyn TraitOutgoing> {
        StateMessageOutgoing::new_state(tr!("запуск синхронизации устройств")).send(&send_type);
        // Check config
        let config_path = utils::get_file_save_path(constants::DEVICES_CONFIGURATION_FILE);
        let path_str = config_path.to_string_lossy();
        if !config_path.exists() {
            StateMessageOutgoing::new_state(tr!("создан конфигурационный файл устройств: {}", path_str))
                .send(&send_type);
        } else {
            StateMessageOutgoing::new_state(tr!("поиск активных устройств согласно конфигурации: {}", path_str))
                .send(&send_type);
        }
        // Search devices
        let devices = DeviceConfig::search();
        if devices.is_empty() {
            Config::save_device(devices);
            return StateMessageOutgoing::new_info(tr!("ни одного устройства в сети не найдено"));
        } else {
            StateMessageOutgoing::new_state(tr!("найдено устройств: {}", devices.len())).send(&send_type);
        }
        // Update config application
        if Config::save_device(devices) {
            StateMessageOutgoing::new_success(tr!("конфигурация устройств обновлена"))
        } else {
            StateMessageOutgoing::new_info(tr!("конфигурация не требует обновления"))
        }
    }
}
