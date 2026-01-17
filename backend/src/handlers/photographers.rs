use axum::Json;

use crate::common::ApiResponse;
use crate::dto::pagination::Paged;
use crate::dto::photographers::{PhotographerListItem, PhotographerListQuery};
use crate::error::ApiResult;
use crate::middleware::auth::AuthUser;
use crate::services::photographers_service;
use crate::state::AppState;

pub async fn list_photographers(
    AuthUser { .. }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(query): axum::extract::Query<PhotographerListQuery>,
) -> ApiResult<Paged<PhotographerListItem>> {
    let data = photographers_service::list_photographers(&state, query).await?;
    Ok(Json(ApiResponse::ok(data)))
}
