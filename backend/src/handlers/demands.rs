use axum::{extract::Path, Json};

use crate::middleware::auth::AuthUser;
use crate::common::ApiResponse;
use crate::dto::demands::{
    CreateDemandReq, DemandDetail, DemandListItem, DemandListQuery, DemandMerchantAssetItem,
    DemandMerchantAssetQuery, DemandResp,
};
use crate::dto::pagination::Paged;
use crate::error::ApiResult;
use crate::services::demands_service;
use crate::state::AppState;

pub async fn create_demand(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<CreateDemandReq>,
) -> ApiResult<DemandResp> {
    let data = demands_service::create_demand(&state, user_id, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn list_demands(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(query): axum::extract::Query<DemandListQuery>,
) -> ApiResult<Paged<DemandListItem>> {
    let data = demands_service::list_demands(&state, user_id, query).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn get_demand(
    Path(demand_id): Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<DemandDetail> {
    let data = demands_service::get_demand(&state, demand_id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn list_demand_merchant_assets(
    AuthUser { user_id }: AuthUser,
    Path(demand_id): Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(query): axum::extract::Query<DemandMerchantAssetQuery>,
) -> ApiResult<Paged<DemandMerchantAssetItem>> {
    let data =
        demands_service::list_demand_merchant_assets(&state, user_id, demand_id, query).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn close_demand(
    AuthUser { user_id }: AuthUser,
    Path(demand_id): Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<DemandResp> {
    let data = demands_service::close_demand(&state, user_id, demand_id).await?;
    Ok(Json(ApiResponse::ok(data)))
}
