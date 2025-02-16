use serde::Deserialize;
use serde::Serialize;

#[derive(Deserialize, Serialize)]
pub struct CommonResponse {
    pub code: u32,
    pub message: String,
}
