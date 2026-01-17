use axum::{extract::Path, Json};

use crate::middleware::auth::AuthUser;
use crate::common::ApiResponse;
use crate::dto::merchants::{
    AddMerchantMemberReq, ApprovalListItem, ApprovalListQuery, ApprovalResp, ContractListItem,
    ContractListQuery, ContractResp, CreateApprovalReq, CreateContractReq, CreateInvoiceReq,
    CreateLocationReq, CreateMerchantAssetReq, CreateMerchantAssetVersionReq, CreateMerchantReq,
    CreateTemplateReq, InvoiceListItem, InvoiceListQuery, InvoiceResp, MerchantAssetListQuery,
    MerchantAssetResp, MerchantAssetVersionListQuery, MerchantAssetVersionResp, MerchantListItem,
    MerchantLocationResp, MerchantMemberResp, MerchantOrderDetail, MerchantOrderListItem,
    MerchantOrderQuery, MerchantOrderReportQuery, MerchantOrderReportResp, MerchantResp,
    TemplateDetailResp, TemplateListQuery, TemplateResp, UpdateLocationReq,
};
use crate::dto::pagination::Paged;
use crate::error::ApiResult;
use crate::services::merchants_service;
use crate::state::AppState;

pub async fn create_merchant(
    AuthUser { .. }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<CreateMerchantReq>,
) -> ApiResult<MerchantResp> {
    let data = merchants_service::create_merchant(&state, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn list_my_merchants(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<Vec<MerchantListItem>> {
    let data = merchants_service::list_my_merchants(&state, user_id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn create_location(
    AuthUser { user_id }: AuthUser,
    Path(merchant_id): Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<CreateLocationReq>,
) -> ApiResult<MerchantLocationResp> {
    let data = merchants_service::create_location(&state, user_id, merchant_id, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn list_locations(
    AuthUser { user_id }: AuthUser,
    Path(merchant_id): Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<Vec<MerchantLocationResp>> {
    let data = merchants_service::list_locations(&state, user_id, merchant_id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn update_location(
    AuthUser { user_id }: AuthUser,
    Path((merchant_id, location_id)): Path<(i64, i64)>,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<UpdateLocationReq>,
) -> ApiResult<MerchantLocationResp> {
    let data =
        merchants_service::update_location(&state, user_id, merchant_id, location_id, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn delete_location(
    AuthUser { user_id }: AuthUser,
    Path((merchant_id, location_id)): Path<(i64, i64)>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<MerchantLocationResp> {
    let data =
        merchants_service::delete_location(&state, user_id, merchant_id, location_id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn add_member(
    AuthUser { user_id }: AuthUser,
    Path(merchant_id): Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<AddMerchantMemberReq>,
) -> ApiResult<MerchantMemberResp> {
    let data = merchants_service::add_member(&state, user_id, merchant_id, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn list_members(
    AuthUser { user_id }: AuthUser,
    Path(merchant_id): Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<Vec<MerchantMemberResp>> {
    let data = merchants_service::list_members(&state, user_id, merchant_id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn remove_member(
    AuthUser { user_id }: AuthUser,
    Path((merchant_id, user_id_param)): Path<(i64, i64)>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<MerchantMemberResp> {
    let data = merchants_service::remove_member(&state, user_id, merchant_id, user_id_param).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn create_template(
    AuthUser { .. }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<CreateTemplateReq>,
) -> ApiResult<TemplateResp> {
    let data = merchants_service::create_template(&state, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn list_templates(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(query): axum::extract::Query<TemplateListQuery>,
) -> ApiResult<Paged<TemplateDetailResp>> {
    let data = merchants_service::list_templates(&state, user_id, query).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn create_approval(
    AuthUser { .. }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<CreateApprovalReq>,
) -> ApiResult<ApprovalResp> {
    let data = merchants_service::create_approval(&state, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn list_approvals(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(query): axum::extract::Query<ApprovalListQuery>,
) -> ApiResult<Paged<ApprovalListItem>> {
    let data = merchants_service::list_approvals(&state, user_id, query).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn create_contract(
    AuthUser { .. }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<CreateContractReq>,
) -> ApiResult<ContractResp> {
    let data = merchants_service::create_contract(&state, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn list_contracts(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(query): axum::extract::Query<ContractListQuery>,
) -> ApiResult<Paged<ContractListItem>> {
    let data = merchants_service::list_contracts(&state, user_id, query).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn create_invoice(
    AuthUser { .. }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<CreateInvoiceReq>,
) -> ApiResult<InvoiceResp> {
    let data = merchants_service::create_invoice(&state, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn list_invoices(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(query): axum::extract::Query<InvoiceListQuery>,
) -> ApiResult<Paged<InvoiceListItem>> {
    let data = merchants_service::list_invoices(&state, user_id, query).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn create_merchant_asset(
    AuthUser { user_id }: AuthUser,
    Path(merchant_id): Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<CreateMerchantAssetReq>,
) -> ApiResult<MerchantAssetResp> {
    let data = merchants_service::create_merchant_asset(&state, user_id, merchant_id, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn list_merchant_assets(
    AuthUser { user_id }: AuthUser,
    Path(merchant_id): Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(query): axum::extract::Query<MerchantAssetListQuery>,
) -> ApiResult<Paged<MerchantAssetResp>> {
    let data = merchants_service::list_merchant_assets(&state, user_id, merchant_id, query).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn list_merchant_asset_versions(
    AuthUser { user_id }: AuthUser,
    Path(asset_id): Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(query): axum::extract::Query<MerchantAssetVersionListQuery>,
) -> ApiResult<Paged<MerchantAssetVersionResp>> {
    let data = merchants_service::list_merchant_asset_versions(&state, user_id, asset_id, query)
        .await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn create_merchant_asset_version(
    AuthUser { user_id }: AuthUser,
    Path(asset_id): Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<CreateMerchantAssetVersionReq>,
) -> ApiResult<MerchantAssetVersionResp> {
    let data = merchants_service::create_merchant_asset_version(&state, user_id, asset_id, req)
        .await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn list_merchant_orders(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(query): axum::extract::Query<MerchantOrderQuery>,
) -> ApiResult<Paged<MerchantOrderListItem>> {
    let data = merchants_service::list_merchant_orders(&state, user_id, query).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn get_merchant_order(
    AuthUser { user_id }: AuthUser,
    Path(order_id): Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<MerchantOrderDetail> {
    let data = merchants_service::get_merchant_order(&state, user_id, order_id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn export_merchant_orders_report(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(query): axum::extract::Query<MerchantOrderReportQuery>,
) -> ApiResult<MerchantOrderReportResp> {
    let data = merchants_service::export_merchant_orders_report(&state, user_id, query).await?;
    Ok(Json(ApiResponse::ok(data)))
}
