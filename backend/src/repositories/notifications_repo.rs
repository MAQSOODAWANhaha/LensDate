use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set};
use sea_orm::sea_query::Expr;

use crate::entity::notifications;

pub async fn list_notifications(
    orm: &DatabaseConnection,
    user_id: i64,
    read_status: Option<String>,
    page: u64,
    page_size: u64,
) -> anyhow::Result<(Vec<notifications::Model>, u64)> {
    let offset = (page - 1) * page_size;
    let mut query = notifications::Entity::find().filter(notifications::Column::UserId.eq(user_id));
    if let Some(read_status) = read_status.as_deref() {
        match read_status {
            "unread" => {
                query = query.filter(notifications::Column::ReadAt.is_null());
            }
            "read" => {
                query = query.filter(notifications::Column::ReadAt.is_not_null());
            }
            _ => {}
        }
    }

    let total = query.clone().count(orm).await?;
    if total == 0 {
        return Ok((Vec::new(), 0));
    }

    let rows = query
        .order_by_desc(notifications::Column::CreatedAt)
        .limit(page_size)
        .offset(offset)
        .all(orm)
        .await?;
    Ok((rows, total))
}

pub async fn count_unread(orm: &DatabaseConnection, user_id: i64) -> anyhow::Result<u64> {
    Ok(notifications::Entity::find()
        .filter(notifications::Column::UserId.eq(user_id))
        .filter(notifications::Column::ReadAt.is_null())
        .count(orm)
        .await?)
}

pub async fn mark_all_read(
    orm: &DatabaseConnection,
    user_id: i64,
) -> anyhow::Result<u64> {
    let now = chrono::Utc::now();
    let result = notifications::Entity::update_many()
        .col_expr(notifications::Column::ReadAt, Expr::value(now))
        .filter(notifications::Column::UserId.eq(user_id))
        .filter(notifications::Column::ReadAt.is_null())
        .exec(orm)
        .await?;
    Ok(result.rows_affected)
}

pub async fn find_notification_by_id(
    orm: &DatabaseConnection,
    id: i64,
) -> anyhow::Result<Option<notifications::Model>> {
    Ok(notifications::Entity::find_by_id(id).one(orm).await?)
}

pub async fn update_notification_read(
    orm: &DatabaseConnection,
    row: notifications::Model,
) -> anyhow::Result<notifications::Model> {
    let read_at = row.read_at;
    let mut model: notifications::ActiveModel = row.into();
    if read_at.is_none() {
        model.read_at = Set(Some(chrono::Utc::now().into()));
    }
    Ok(model.update(orm).await?)
}
