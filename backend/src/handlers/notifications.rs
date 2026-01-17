use axum::{extract::Path, Json};

use crate::middleware::auth::AuthUser;
use crate::common::ApiResponse;
use crate::dto::notifications::{
    ListNotificationsQuery, MarkAllReadResp, NotificationItem, NotificationSummary,
};
use crate::dto::pagination::Paged;
use crate::error::ApiResult;
use crate::services::notifications_service;
use crate::state::AppState;

pub async fn list_notifications(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(query): axum::extract::Query<ListNotificationsQuery>,
) -> ApiResult<Paged<NotificationItem>> {
    let data = notifications_service::list_notifications(&state, user_id, query).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn get_summary(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<NotificationSummary> {
    let data = notifications_service::get_summary(&state, user_id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn mark_all_read(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<MarkAllReadResp> {
    let data = notifications_service::mark_all_read(&state, user_id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn get_notification(
    AuthUser { user_id }: AuthUser,
    Path(notification_id): Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<NotificationItem> {
    let data = notifications_service::get_notification(&state, user_id, notification_id).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn mark_notification_read(
    AuthUser { user_id }: AuthUser,
    Path(notification_id): Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<NotificationItem> {
    let data = notifications_service::mark_notification_read(&state, user_id, notification_id).await?;
    Ok(Json(ApiResponse::ok(data)))
}
