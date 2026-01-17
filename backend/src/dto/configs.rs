use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UpdateConfigReq {
    pub value: serde_json::Value,
}

#[derive(Serialize)]
pub struct ConfigResp {
    pub id: i64,
    pub key: String,
    pub value: serde_json::Value,
}
