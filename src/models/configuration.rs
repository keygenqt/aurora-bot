use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ConfigurationModel {
    pub name: String,
}
