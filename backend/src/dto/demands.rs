use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateDemandReq {
    pub r#type: String,
    pub city_id: i64,
    pub location: Option<String>,
    pub schedule_start: String,
    pub schedule_end: String,
    pub budget_min: Option<f64>,
    pub budget_max: Option<f64>,
    pub people_count: Option<i32>,
    pub style_tags: Option<Vec<String>>,
    pub attachments: Option<Vec<AttachmentReq>>,
    pub is_merchant: Option<bool>,
    pub merchant_id: Option<i64>,
}

#[derive(Deserialize)]
pub struct AttachmentReq {
    pub file_url: String,
    pub file_type: Option<String>,
}

#[derive(Serialize)]
pub struct DemandResp {
    pub id: i64,
    pub user_id: i64,
    pub r#type: String,
    pub status: String,
}

#[derive(Deserialize)]
pub struct DemandListQuery {
    pub city_id: Option<i64>,
    pub r#type: Option<String>,
    pub status: Option<String>,
    pub schedule_start: Option<String>,
    pub schedule_end: Option<String>,
    pub min_budget: Option<f64>,
    pub max_budget: Option<f64>,
    pub style_tag: Option<String>,
    pub is_merchant: Option<bool>,
    pub sort: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
    pub mine: Option<bool>,
}

#[derive(Serialize)]
pub struct DemandListItem {
    pub id: i64,
    pub r#type: String,
    pub city_id: Option<i64>,
    pub status: String,
    pub schedule_start: Option<String>,
}

#[derive(Serialize)]
pub struct DemandDetail {
    pub id: i64,
    pub user_id: i64,
    pub r#type: String,
    pub status: String,
    pub city_id: Option<i64>,
    pub location: Option<String>,
    pub schedule_start: Option<String>,
    pub schedule_end: Option<String>,
    pub budget_min: Option<f64>,
    pub budget_max: Option<f64>,
    pub people_count: Option<i32>,
    pub style_tags: Option<Vec<String>>,
    pub is_merchant: bool,
    pub merchant_id: Option<i64>,
    pub attachments: Vec<AttachmentResp>,
}

#[derive(Serialize)]
pub struct AttachmentResp {
    pub id: i64,
    pub file_url: String,
    pub file_type: Option<String>,
}

#[derive(Deserialize)]
pub struct DemandMerchantAssetQuery {
    pub asset_type: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Serialize)]
pub struct DemandMerchantAssetItem {
    pub id: i64,
    pub merchant_id: i64,
    pub asset_type: String,
    pub name: String,
    pub latest_version: Option<i32>,
    pub latest_payload: Option<serde_json::Value>,
    pub updated_at: String,
}
