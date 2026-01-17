use axum::{extract::Path, Json};

use crate::middleware::auth::AuthUser;
use crate::common::ApiResponse;
use crate::dto::orders::{
    CancelOrderReq, CancelOrderResp, OrderListItem, OrderListQuery, OrderResp, RefundPreviewResp,
};
use crate::dto::pagination::Paged;
use crate::error::ApiResult;
use crate::services::orders_service;
use crate::state::AppState;

pub async fn list_orders(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(query): axum::extract::Query<OrderListQuery>,
) -> ApiResult<Paged<OrderListItem>> {
    let data = orders_service::list_orders(&state, user_id, query).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn get_order(
    AuthUser { user_id }: AuthUser,
    Path(order_id): Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<OrderResp> {
    let data = orders_service::get_order(&state, user_id, order_id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn refund_preview(
    AuthUser { user_id }: AuthUser,
    Path(order_id): Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<RefundPreviewResp> {
    let data = orders_service::refund_preview(&state, user_id, order_id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn cancel_order(
    AuthUser { user_id }: AuthUser,
    Path(order_id): Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<CancelOrderReq>,
) -> ApiResult<CancelOrderResp> {
    let data = orders_service::cancel_order(&state, user_id, order_id, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}
