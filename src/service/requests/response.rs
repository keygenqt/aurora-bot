use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
pub struct ApiResponse {
    pub code: u32,
    pub message: String,
}

#[derive(Deserialize, Serialize, Debug)]
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

#[derive(Deserialize, Serialize, Debug)]
pub struct FaqResponse {
    hash: String,
    title: String,
    text: String,
    fname: String,
    lname: String,
    timestamp: u64,
    image: Option<String>,
}
