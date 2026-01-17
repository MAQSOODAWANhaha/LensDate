use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreatePaymentReq {
    pub order_id: i64,
    pub amount: f64,
    pub pay_channel: String,
    pub proof_url: String,
    pub stage: Option<String>,
}

#[derive(Serialize)]
pub struct PaymentResp {
    pub id: i64,
    pub order_id: i64,
    pub amount: f64,
    pub status: String,
    pub stage: Option<String>,
}
