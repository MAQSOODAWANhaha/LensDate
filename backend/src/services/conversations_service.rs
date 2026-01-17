use crate::dto::conversations::{CreateConversationReq, ConversationResp, ListConversationsQuery};
use crate::dto::pagination::{normalize_pagination, Paged};
use crate::errors::{DomainError, ServiceResult};
use crate::repositories::conversations_repo;
use crate::state::AppState;

pub async fn create_conversation(
    state: &AppState,
    user_id: i64,
    req: CreateConversationReq,
) -> ServiceResult<ConversationResp> {
    if !matches!(req.r#type.as_str(), "order" | "chat") {
        return Err(DomainError::BadRequest("invalid_type".to_string()).into());
    }

    if let Some(order_id) = req.order_id {
        ensure_order_participant(state, user_id, order_id).await?;
        let existing = conversations_repo::find_conversation_by_order_id(&state.orm, order_id)
            .await?;
        if let Some(conv) = existing {
            return Ok(ConversationResp {
                id: conv.id,
                r#type: conv.r#type,
                order_id: conv.order_id,
            });
        }
    }

    let model = crate::entity::conversations::ActiveModel {
        r#type: sea_orm::ActiveValue::Set(req.r#type),
        order_id: sea_orm::ActiveValue::Set(req.order_id),
        ..Default::default()
    };
    let inserted = conversations_repo::create_conversation(&state.orm, model).await?;

    Ok(ConversationResp {
        id: inserted.id,
        r#type: inserted.r#type,
        order_id: inserted.order_id,
    })
}

pub async fn list_conversations(
    state: &AppState,
    user_id: i64,
    query: ListConversationsQuery,
) -> ServiceResult<Paged<ConversationResp>> {
    let (page, page_size) = normalize_pagination(query.page, query.page_size);

    let (rows, total) = if let Some(order_id) = query.order_id {
        ensure_order_participant(state, user_id, order_id).await?;
        conversations_repo::list_conversations_by_order_id(&state.orm, order_id, page, page_size)
            .await?
    } else {
        let order_ids = load_order_ids_for_user(state, user_id).await?;
        if order_ids.is_empty() {
            return Ok(Paged::new(Vec::new(), 0, page, page_size));
        }
        conversations_repo::list_conversations_by_order_ids(&state.orm, order_ids, page, page_size)
            .await?
    };

    let items = rows
        .into_iter()
        .map(|c| ConversationResp {
            id: c.id,
            r#type: c.r#type,
            order_id: c.order_id,
        })
        .collect();

    Ok(Paged::new(items, total, page, page_size))
}

async fn ensure_order_participant(
    state: &AppState,
    user_id: i64,
    order_id: i64,
) -> ServiceResult<()> {
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

async fn load_order_ids_for_user(state: &AppState, user_id: i64) -> ServiceResult<Vec<i64>> {
    let mut ids = conversations_repo::list_order_ids_by_user(&state.orm, user_id).await?;

    let photographer = conversations_repo::find_photographer_by_user(&state.orm, user_id).await?;
    if let Some(p) = photographer {
        let mut photo_orders = conversations_repo::list_order_ids_by_photographer(&state.orm, p.id)
            .await?;
        ids.append(&mut photo_orders);
    }

    ids.sort_unstable();
    ids.dedup();
    Ok(ids)
}
