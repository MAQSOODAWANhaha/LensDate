use axum::Json;

use crate::middleware::auth::AuthUser;
use crate::common::ApiResponse;
use crate::dto::payments::{CreatePaymentReq, PaymentResp};
use crate::error::ApiResult;
use crate::services::payments_service;
use crate::state::AppState;

pub async fn create_payment(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<CreatePaymentReq>,
) -> ApiResult<PaymentResp> {
    let data = payments_service::create_payment(&state, user_id, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}
