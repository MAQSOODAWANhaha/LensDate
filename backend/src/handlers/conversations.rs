use axum::Json;

use crate::middleware::auth::AuthUser;
use crate::common::ApiResponse;
use crate::dto::conversations::{CreateConversationReq, ConversationResp, ListConversationsQuery};
use crate::dto::pagination::Paged;
use crate::error::ApiResult;
use crate::services::conversations_service;
use crate::state::AppState;

pub async fn create_conversation(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<CreateConversationReq>,
) -> ApiResult<ConversationResp> {
    let data = conversations_service::create_conversation(&state, user_id, req).await?;
    Ok(Json(ApiResponse::ok(data)))
}

pub async fn list_conversations(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(query): axum::extract::Query<ListConversationsQuery>,
) -> ApiResult<Paged<ConversationResp>> {
    let data = conversations_service::list_conversations(&state, user_id, query).await?;
    Ok(Json(ApiResponse::ok(data)))
}
