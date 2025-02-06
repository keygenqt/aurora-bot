use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct UserResponse {
    #[serde(alias = "tgId")]
    tg_id: u64,
    #[serde(alias = "firstName")]
    first_name: String,
    #[serde(alias = "lastName")]
    last_name: String,
    #[serde(alias = "userName")]
    user_name: String,
    #[serde(alias = "mode")]
    mode: String,
}
