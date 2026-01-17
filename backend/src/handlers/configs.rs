use axum::{extract::Path, Json};

use crate::middleware::auth::AuthUser;
use crate::common::ApiResponse;
use crate::dto::configs::{ConfigResp, UpdateConfigReq};
use crate::error::ApiResult;
use crate::services::configs_service;
use crate::state::AppState;

pub async fn get_config(
    AuthUser { .. }: AuthUser,
    Path(key): Path<String>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<ConfigResp> {
    let data = configs_service::get_config(&state, key).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn upsert_config(
    AuthUser { .. }: AuthUser,
    Path(key): Path<String>,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<UpdateConfigReq>,
) -> ApiResult<ConfigResp> {
    let data = configs_service::upsert_config(&state, key, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}
