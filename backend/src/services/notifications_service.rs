use crate::dto::notifications::{
    ListNotificationsQuery, MarkAllReadResp, NotificationItem, NotificationSummary,
};
use crate::dto::pagination::{normalize_pagination, Paged};
use crate::errors::{DomainError, ServiceResult};
use crate::repositories::notifications_repo;
use crate::state::AppState;

pub async fn list_notifications(
    state: &AppState,
    user_id: i64,
    query: ListNotificationsQuery,
) -> ServiceResult<Paged<NotificationItem>> {
    let (page, page_size) = normalize_pagination(query.page, query.page_size);

    if query
        .read_status
        .as_deref()
        .is_some_and(|read_status| !matches!(read_status, "unread" | "read"))
    {
        return Err(DomainError::BadRequest("invalid_read_status".to_string()).into());
    }

    let (rows, total) = notifications_repo::list_notifications(
        &state.orm,
        user_id,
        query.read_status,
        page,
        page_size,
    )
    .await?;

    let items = rows
        .into_iter()
        .map(|n| NotificationItem {
            id: n.id,
            r#type: n.r#type,
            title: n.title,
            content: n.content,
            read_at: n.read_at.map(|t| t.to_rfc3339()),
            created_at: n.created_at.to_rfc3339(),
        })
        .collect();

    Ok(Paged::new(items, total, page, page_size))
}

pub async fn get_summary(state: &AppState, user_id: i64) -> ServiceResult<NotificationSummary> {
    let unread_count = notifications_repo::count_unread(&state.orm, user_id).await?;
    Ok(NotificationSummary { unread_count })
}

pub async fn mark_all_read(
    state: &AppState,
    user_id: i64,
) -> ServiceResult<MarkAllReadResp> {
    let updated = notifications_repo::mark_all_read(&state.orm, user_id).await?;
    Ok(MarkAllReadResp { updated })
}

pub async fn get_notification(
    state: &AppState,
    user_id: i64,
    notification_id: i64,
) -> ServiceResult<NotificationItem> {
    let row = notifications_repo::find_notification_by_id(&state.orm, notification_id)
        .await?
        .ok_or(DomainError::NotFound)?;

    if row.user_id != user_id {
        return Err(DomainError::Forbidden.into());
    }

    Ok(NotificationItem {
        id: row.id,
        r#type: row.r#type,
        title: row.title,
        content: row.content,
        read_at: row.read_at.map(|t| t.to_rfc3339()),
        created_at: row.created_at.to_rfc3339(),
    })
}

pub async fn mark_notification_read(
    state: &AppState,
    user_id: i64,
    notification_id: i64,
) -> ServiceResult<NotificationItem> {
    let row = notifications_repo::find_notification_by_id(&state.orm, notification_id)
        .await?
        .ok_or(DomainError::NotFound)?;

    if row.user_id != user_id {
        return Err(DomainError::Forbidden.into());
    }

    let updated = notifications_repo::update_notification_read(&state.orm, row).await?;

    Ok(NotificationItem {
        id: updated.id,
        r#type: updated.r#type,
        title: updated.title,
        content: updated.content,
        read_at: updated.read_at.map(|t| t.to_rfc3339()),
        created_at: updated.created_at.to_rfc3339(),
    })
}
