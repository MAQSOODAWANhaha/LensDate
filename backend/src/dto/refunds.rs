use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateRefundReq {
    pub order_id: i64,
    pub amount: f64,
    pub reason: Option<String>,
    pub proof_url: Option<String>,
    pub responsible_party: Option<String>,
}

#[derive(Serialize)]
pub struct RefundResp {
    pub id: i64,
    pub order_id: i64,
    pub amount: f64,
    pub status: String,
    pub responsible_party: Option<String>,
}
