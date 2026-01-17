use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct OrderListQuery {
    pub status: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub keyword: Option<String>,
    pub sort: Option<String>,
    pub min_amount: Option<f64>,
    pub max_amount: Option<f64>,
    pub start_time: Option<String>,
    pub end_time: Option<String>,
}

#[derive(Serialize)]
pub struct OrderItemResp {
    pub name: String,
    pub price: f64,
    pub quantity: i32,
}

#[derive(Serialize)]
pub struct OrderResp {
    pub id: i64,
    pub user_id: i64,
    pub status: String,
    pub pay_type: String,
    pub total_amount: f64,
    pub service_fee: f64,
    pub items: Vec<OrderItemResp>,
}

#[derive(Serialize)]
pub struct RefundPreviewResp {
    pub order_id: i64,
    pub paid_amount: f64,
    pub refund_ratio: f64,
    pub refund_amount: f64,
    pub responsible_party: String,
    pub rule: String,
}

#[derive(Deserialize)]
pub struct CancelOrderReq {
    pub reason: Option<String>,
}

#[derive(Serialize)]
pub struct CancelOrderResp {
    pub order_id: i64,
    pub status: String,
    pub refund_id: Option<i64>,
    pub refund_amount: f64,
}

#[derive(Serialize)]
pub struct OrderListItem {
    pub id: i64,
    pub status: String,
    pub total_amount: f64,
}
