use axum::Json;

use crate::middleware::auth::AuthUser;
use crate::common::ApiResponse;
use crate::dto::messages::{CreateMessageReq, ListMessagesQuery, MessageItem, MessageResp};
use crate::dto::pagination::Paged;
use crate::error::ApiResult;
use crate::services::messages_service;
use crate::state::AppState;

pub async fn send_message(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<CreateMessageReq>,
) -> ApiResult<MessageResp> {
    let data = messages_service::send_message(&state, user_id, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn list_messages(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(query): axum::extract::Query<ListMessagesQuery>,
) -> ApiResult<Paged<MessageItem>> {
    let data = messages_service::list_messages(&state, user_id, query).await?;
    Ok(Json(ApiResponse::ok(data)))
}
