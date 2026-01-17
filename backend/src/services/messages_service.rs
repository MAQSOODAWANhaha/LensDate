use crate::dto::messages::{CreateMessageReq, ListMessagesQuery, MessageItem, MessageResp};
use crate::dto::pagination::{normalize_pagination, Paged};
use crate::errors::{DomainError, ServiceResult};
use crate::repositories::{conversations_repo, messages_repo};
use crate::state::AppState;

pub async fn send_message(
    state: &AppState,
    user_id: i64,
    req: CreateMessageReq,
) -> ServiceResult<MessageResp> {
    if !matches!(req.msg_type.as_str(), "text" | "image" | "file") {
        return Err(DomainError::BadRequest("invalid_type".to_string()).into());
    }

    ensure_conversation_participant(state, user_id, req.conversation_id).await?;

    let model = crate::entity::messages::ActiveModel {
        conversation_id: sea_orm::ActiveValue::Set(req.conversation_id),
        sender_id: sea_orm::ActiveValue::Set(user_id),
        content: sea_orm::ActiveValue::Set(req.content),
        msg_type: sea_orm::ActiveValue::Set(req.msg_type),
        sent_at: sea_orm::ActiveValue::Set(chrono::Utc::now().into()),
        ..Default::default()
    };

    let inserted = messages_repo::create_message(&state.orm, model).await?;

    Ok(MessageResp {
        id: inserted.id,
        conversation_id: inserted.conversation_id,
    })
}

pub async fn list_messages(
    state: &AppState,
    user_id: i64,
    query: ListMessagesQuery,
) -> ServiceResult<Paged<MessageItem>> {
    ensure_conversation_participant(state, user_id, query.conversation_id).await?;

    let (page, page_size) = normalize_pagination(query.page, query.page_size);

    let (rows, total) =
        messages_repo::list_messages(&state.orm, query.conversation_id, page, page_size).await?;

    let items = rows
        .into_iter()
        .map(|m| MessageItem {
            id: m.id,
            conversation_id: m.conversation_id,
            sender_id: m.sender_id,
            content: m.content,
            msg_type: m.msg_type,
            sent_at: m.sent_at.to_rfc3339(),
        })
        .collect();

    Ok(Paged::new(items, total, page, page_size))
}

async fn ensure_conversation_participant(
    state: &AppState,
    user_id: i64,
    conversation_id: i64,
) -> ServiceResult<()> {
    let conv = conversations_repo::find_conversation_by_id(&state.orm, conversation_id)
        .await?
        .ok_or(DomainError::NotFound)?;
    let order_id = conv.order_id.ok_or(DomainError::Forbidden)?;

    let order = conversations_repo::find_order_by_id(&state.orm, order_id)
        .await?
        .ok_or(DomainError::NotFound)?;
    if order.user_id == user_id {
        return Ok(());
    }
    let photographer_user = match order.photographer_id {
        Some(pid) => conversations_repo::find_photographer_by_id(&state.orm, pid)
            .await?
            .map(|p| p.user_id),
        None => None,
    };
    if photographer_user == Some(user_id) {
        Ok(())
    } else {
        Err(DomainError::Forbidden.into())
    }
}
