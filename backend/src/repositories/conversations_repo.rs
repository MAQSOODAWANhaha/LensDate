use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect,
};

use crate::entity::{conversations, orders, photographers};

pub async fn find_conversation_by_order_id(
    orm: &DatabaseConnection,
    order_id: i64,
) -> anyhow::Result<Option<conversations::Model>> {
    Ok(conversations::Entity::find()
        .filter(conversations::Column::OrderId.eq(order_id))
        .one(orm)
        .await?)
}

pub async fn create_conversation(
    orm: &DatabaseConnection,
    model: conversations::ActiveModel,
) -> anyhow::Result<conversations::Model> {
    Ok(model.insert(orm).await?)
}

pub async fn list_conversations_by_order_ids(
    orm: &DatabaseConnection,
    order_ids: Vec<i64>,
    page: u64,
    page_size: u64,
) -> anyhow::Result<(Vec<conversations::Model>, u64)> {
    if order_ids.is_empty() {
        return Ok((Vec::new(), 0));
    }

    let offset = (page - 1) * page_size;
    let mut query = conversations::Entity::find()
        .filter(conversations::Column::OrderId.is_in(order_ids));

    let total = query.clone().count(orm).await?;
    if total == 0 {
        return Ok((Vec::new(), 0));
    }

    query = query.order_by_desc(conversations::Column::CreatedAt);
    let rows = query.limit(page_size).offset(offset).all(orm).await?;
    Ok((rows, total))
}

pub async fn list_conversations_by_order_id(
    orm: &DatabaseConnection,
    order_id: i64,
    page: u64,
    page_size: u64,
) -> anyhow::Result<(Vec<conversations::Model>, u64)> {
    let offset = (page - 1) * page_size;
    let mut query = conversations::Entity::find()
        .filter(conversations::Column::OrderId.eq(order_id));

    let total = query.clone().count(orm).await?;
    if total == 0 {
        return Ok((Vec::new(), 0));
    }

    query = query.order_by_desc(conversations::Column::CreatedAt);
    let rows = query.limit(page_size).offset(offset).all(orm).await?;
    Ok((rows, total))
}

pub async fn find_conversation_by_id(
    orm: &DatabaseConnection,
    conversation_id: i64,
) -> anyhow::Result<Option<conversations::Model>> {
    Ok(conversations::Entity::find_by_id(conversation_id)
        .one(orm)
        .await?)
}

pub async fn find_order_by_id(
    orm: &DatabaseConnection,
    order_id: i64,
) -> anyhow::Result<Option<orders::Model>> {
    Ok(orders::Entity::find_by_id(order_id).one(orm).await?)
}

pub async fn find_photographer_by_id(
    orm: &DatabaseConnection,
    photographer_id: i64,
) -> anyhow::Result<Option<photographers::Model>> {
    Ok(photographers::Entity::find_by_id(photographer_id)
        .one(orm)
        .await?)
}

pub async fn list_order_ids_by_user(
    orm: &DatabaseConnection,
    user_id: i64,
) -> anyhow::Result<Vec<i64>> {
    Ok(orders::Entity::find()
        .select_only()
        .column(orders::Column::Id)
        .filter(orders::Column::UserId.eq(user_id))
        .into_tuple::<i64>()
        .all(orm)
        .await?)
}

pub async fn find_photographer_by_user(
    orm: &DatabaseConnection,
    user_id: i64,
) -> anyhow::Result<Option<photographers::Model>> {
    Ok(photographers::Entity::find()
        .filter(photographers::Column::UserId.eq(user_id))
        .one(orm)
        .await?)
}

pub async fn list_order_ids_by_photographer(
    orm: &DatabaseConnection,
    photographer_id: i64,
) -> anyhow::Result<Vec<i64>> {
    Ok(orders::Entity::find()
        .select_only()
        .column(orders::Column::Id)
        .filter(orders::Column::PhotographerId.eq(photographer_id))
        .into_tuple::<i64>()
        .all(orm)
        .await?)
}
