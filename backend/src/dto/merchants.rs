use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct CreateMerchantReq {
    pub name: String,
    pub logo_url: Option<String>,
    pub brand_color: Option<String>,
    pub contact_user_id: i64,
}

#[derive(Serialize)]
pub struct MerchantResp {
    pub id: i64,
    pub name: String,
    pub status: String,
}

#[derive(Serialize)]
pub struct MerchantListItem {
    pub id: i64,
    pub name: String,
    pub status: String,
    pub role: String,
    pub logo_url: Option<String>,
    pub brand_color: Option<String>,
}

#[derive(Deserialize)]
pub struct CreateLocationReq {
    pub name: String,
    pub address: Option<String>,
    pub city_id: Option<i64>,
}

#[derive(Deserialize)]
pub struct UpdateLocationReq {
    pub name: String,
    pub address: Option<String>,
    pub city_id: Option<i64>,
}

#[derive(Serialize)]
pub struct MerchantLocationResp {
    pub id: i64,
    pub merchant_id: i64,
    pub name: String,
    pub address: Option<String>,
    pub city_id: Option<i64>,
}

#[derive(Deserialize)]
pub struct AddMerchantMemberReq {
    pub user_id: i64,
    pub role: Option<String>,
}

#[derive(Serialize)]
pub struct MerchantMemberResp {
    pub merchant_id: i64,
    pub user_id: i64,
    pub role: String,
}

#[derive(Deserialize)]
pub struct TemplateItemReq {
    pub name: String,
    pub quantity: i32,
    pub price: f64,
}

#[derive(Deserialize)]
pub struct CreateTemplateReq {
    pub merchant_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub delivery_requirements: Option<serde_json::Value>,
    pub items: Vec<TemplateItemReq>,
}

#[derive(Serialize)]
pub struct TemplateResp {
    pub id: i64,
    pub merchant_id: i64,
    pub name: String,
}

#[derive(Serialize)]
pub struct TemplateItemResp {
    pub name: String,
    pub quantity: i32,
    pub price: f64,
}

#[derive(Serialize)]
pub struct TemplateDetailResp {
    pub id: i64,
    pub merchant_id: i64,
    pub name: String,
    pub description: Option<String>,
    pub delivery_requirements: Option<serde_json::Value>,
    pub created_at: String,
    pub items: Vec<TemplateItemResp>,
}

#[derive(Deserialize)]
pub struct TemplateListQuery {
    pub merchant_id: Option<i64>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Deserialize)]
pub struct CreateApprovalReq {
    pub demand_id: i64,
    pub merchant_id: i64,
    pub status: String,
    pub comment: Option<String>,
}

#[derive(Serialize)]
pub struct ApprovalResp {
    pub id: i64,
    pub status: String,
}

#[derive(Serialize)]
pub struct ApprovalListItem {
    pub id: i64,
    pub demand_id: i64,
    pub merchant_id: i64,
    pub status: String,
    pub approver_id: Option<i64>,
    pub comment: Option<String>,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct ApprovalListQuery {
    pub merchant_id: Option<i64>,
    pub status: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Deserialize)]
pub struct CreateContractReq {
    pub order_id: i64,
    pub terms: serde_json::Value,
    pub version: Option<i32>,
}

#[derive(Serialize)]
pub struct ContractResp {
    pub id: i64,
    pub order_id: i64,
    pub version: i32,
}

#[derive(Serialize)]
pub struct ContractListItem {
    pub id: i64,
    pub order_id: i64,
    pub version: i32,
    pub created_at: String,
    pub terms: serde_json::Value,
}

#[derive(Deserialize)]
pub struct ContractListQuery {
    pub merchant_id: Option<i64>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Deserialize)]
pub struct CreateInvoiceReq {
    pub merchant_id: i64,
    pub order_id: Option<i64>,
    pub title: String,
    pub tax_no: Option<String>,
    pub amount: f64,
}

#[derive(Serialize)]
pub struct InvoiceResp {
    pub id: i64,
    pub merchant_id: i64,
    pub status: String,
}

#[derive(Serialize)]
pub struct InvoiceListItem {
    pub id: i64,
    pub merchant_id: i64,
    pub order_id: Option<i64>,
    pub title: String,
    pub tax_no: Option<String>,
    pub amount: f64,
    pub status: String,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct InvoiceListQuery {
    pub merchant_id: Option<i64>,
    pub status: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Serialize)]
pub struct MerchantOrderListItem {
    pub id: i64,
    pub status: String,
    pub total_amount: f64,
    pub demand_id: Option<i64>,
    pub created_at: String,
}

#[derive(Deserialize)]
pub struct MerchantOrderReportQuery {
    pub merchant_id: Option<i64>,
    pub status: Option<String>,
    pub start_date: Option<String>,
    pub end_date: Option<String>,
    pub limit: Option<u64>,
    pub format: Option<String>,
}

#[derive(Serialize)]
pub struct MerchantOrderReportItem {
    pub id: i64,
    pub demand_id: Option<i64>,
    pub status: String,
    pub total_amount: f64,
    pub paid_amount: f64,
    pub refund_amount: f64,
    pub created_at: String,
}

#[derive(Serialize)]
pub struct MerchantOrderReportResp {
    pub format: String,
    pub generated_at: String,
    pub total: u64,
    pub items: Vec<MerchantOrderReportItem>,
    pub csv: Option<String>,
}

#[derive(Serialize)]
pub struct MerchantOrderItem {
    pub name: String,
    pub price: f64,
    pub quantity: i32,
}

#[derive(Serialize)]
pub struct MerchantOrderDetail {
    pub id: i64,
    pub status: String,
    pub pay_type: String,
    pub total_amount: f64,
    pub service_fee: f64,
    pub schedule_start: Option<String>,
    pub schedule_end: Option<String>,
    pub user_id: i64,
    pub user_phone: String,
    pub demand_id: Option<i64>,
    pub created_at: String,
    pub items: Vec<MerchantOrderItem>,
}

#[derive(Deserialize)]
pub struct MerchantOrderQuery {
    pub merchant_id: Option<i64>,
    pub status: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Deserialize)]
pub struct CreateMerchantAssetReq {
    pub name: String,
    pub asset_type: String,
    pub status: Option<String>,
    pub payload: Option<serde_json::Value>,
}

#[derive(Deserialize)]
pub struct MerchantAssetListQuery {
    pub asset_type: Option<String>,
    pub status: Option<String>,
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Serialize)]
pub struct MerchantAssetResp {
    pub id: i64,
    pub merchant_id: i64,
    pub asset_type: String,
    pub name: String,
    pub status: String,
    pub latest_version: Option<i32>,
    pub latest_payload: Option<serde_json::Value>,
    pub updated_at: String,
}

#[derive(Deserialize)]
pub struct MerchantAssetVersionListQuery {
    pub page: Option<u64>,
    pub page_size: Option<u64>,
}

#[derive(Deserialize)]
pub struct CreateMerchantAssetVersionReq {
    pub payload: serde_json::Value,
}

#[derive(Serialize)]
pub struct MerchantAssetVersionResp {
    pub id: i64,
    pub asset_id: i64,
    pub version: i32,
    pub payload: serde_json::Value,
    pub created_by: i64,
    pub created_at: String,
}
