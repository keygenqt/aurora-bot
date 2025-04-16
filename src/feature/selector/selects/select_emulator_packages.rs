use serde::Serialize;

use crate::feature::incoming::TraitIncoming;
use crate::feature::selector::outgoing::incoming::SelectorIncoming;
use crate::feature::selector::outgoing::outgoing::SelectorOutgoing;
use crate::models::TraitModel;
use crate::models::emulator::model::EmulatorModel;
use crate::tools::macros::tr;

pub struct EmulatorPackageSelect {}

impl EmulatorPackageSelect {
    pub fn select<T: TraitIncoming + Serialize + Clone, F: Fn(String, String) -> T>(
        key: String,
        model: &EmulatorModel,
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
