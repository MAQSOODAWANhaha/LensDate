use axum::Json;

use crate::middleware::auth::AuthUser;
use crate::common::ApiResponse;
use crate::dto::refunds::{CreateRefundReq, RefundResp};
use crate::error::ApiResult;
use crate::services::refunds_service;
use crate::state::AppState;

pub async fn create_refund(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<CreateRefundReq>,
) -> ApiResult<RefundResp> {
    let data = refunds_service::create_refund(&state, user_id, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}
