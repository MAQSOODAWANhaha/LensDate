use serde::{Deserialize, Serialize};

#[derive(Deserialize, Clone)]
pub struct QuoteItemReq {
    pub name: String,
    pub price: f64,
    pub quantity: i32,
}

#[derive(Deserialize)]
pub struct CreateQuoteReq {
    pub demand_id: i64,
    pub photographer_id: Option<i64>,
    pub team_id: Option<i64>,
    pub total_price: f64,
    pub items: Vec<QuoteItemReq>,
    pub note: Option<String>,
}

#[derive(Serialize)]
pub struct QuoteResp {
    pub id: i64,
    pub demand_id: i64,
    pub status: String,
}

#[derive(Serialize)]
pub struct AcceptQuoteResp {
    pub order_id: i64,
}

#[derive(Deserialize)]
pub struct QuoteListQuery {
    pub demand_id: Option<i64>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Serialize)]
pub struct QuoteItemResp {
    pub name: String,
    pub price: f64,
    pub quantity: i32,
}

#[derive(Serialize)]
pub struct QuoteListItem {
    pub id: i64,
    pub demand_id: i64,
    pub status: String,
    pub total_price: f64,
    pub photographer_id: Option<i64>,
    pub team_id: Option<i64>,
    pub version: i32,
    pub expires_at: Option<String>,
    pub items: Vec<QuoteItemResp>,
}

#[derive(Deserialize)]
pub struct MyQuoteListQuery {
    pub status: Option<String>,
    pub demand_id: Option<i64>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Serialize)]
pub struct MyQuoteListItem {
    pub id: i64,
    pub demand_id: i64,
    pub status: String,
    pub total_price: f64,
    pub created_at: String,
    pub version: i32,
    pub expires_at: Option<String>,
    pub order_id: Option<i64>,
    pub order_status: Option<String>,
    pub items: Vec<QuoteItemResp>,
}

#[derive(Serialize)]
pub struct QuoteDetailResp {
    pub id: i64,
    pub demand_id: i64,
    pub status: String,
    pub total_price: f64,
    pub photographer_id: Option<i64>,
    pub team_id: Option<i64>,
    pub version: i32,
    pub expires_at: Option<String>,
    pub order_id: Option<i64>,
    pub order_status: Option<String>,
    pub items: Vec<QuoteItemResp>,
}

#[derive(Deserialize)]
pub struct UpdateQuoteReq {
    pub total_price: f64,
    pub items: Vec<QuoteItemReq>,
    pub note: Option<String>,
}

#[derive(Serialize)]
pub struct QuoteVersionItem {
    pub id: i64,
    pub version: i32,
    pub total_price: f64,
    pub items: serde_json::Value,
    pub note: Option<String>,
    pub created_by: i64,
    pub created_at: String,
}
