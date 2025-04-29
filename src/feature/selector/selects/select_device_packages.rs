use serde::Serialize;

use crate::feature::incoming::TraitIncoming;
use crate::feature::outgoing::OutgoingType;
use crate::feature::selector::outgoing::incoming::SelectorIncoming;
use crate::feature::selector::outgoing::outgoing::SelectorOutgoing;
use crate::models::TraitModel;
use crate::models::device::model::DeviceModel;
use crate::tools::macros::tr;

pub struct DevicePackageSelect {}

impl DevicePackageSelect {
    pub fn select<T: TraitIncoming + Serialize + Clone, F: Fn(String, String) -> T>(
        key: String,
        send_type: &OutgoingType,
        model: &DeviceModel,
        incoming: F,
    ) -> Result<SelectorOutgoing<T>, Box<dyn std::error::Error>> {
        let session = match model.session_user() {
            Ok(value) => value,
            Err(_) => Err(tr!("не удалось получить доступ"))?,
        };
        let packages = match session.get_install_packages() {
            Ok(value) => value,
            Err(error) => Err(format!("{}", error))?,
        };
        Ok(SelectorOutgoing {
            key,
            send_type: send_type.clone(),
            variants: packages
                .iter()
                .map(|e| SelectorIncoming {
                    name: tr!("Пакет: {}", e),
                    incoming: incoming(model.get_id(), e.to_string()),
                })
                .collect::<Vec<SelectorIncoming<T>>>(),
        })
    }
}
