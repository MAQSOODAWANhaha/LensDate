use sea_orm::{
    ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, PaginatorTrait, QueryFilter,
    QueryOrder, QuerySelect,
};

use crate::entity::messages;

pub async fn create_message(
    orm: &DatabaseConnection,
    model: messages::ActiveModel,
) -> anyhow::Result<messages::Model> {
    Ok(model.insert(orm).await?)
}

pub async fn list_messages(
    orm: &DatabaseConnection,
    conversation_id: i64,
    page: u64,
    page_size: u64,
) -> anyhow::Result<(Vec<messages::Model>, u64)> {
    let offset = (page - 1) * page_size;
    let mut query = messages::Entity::find()
        .filter(messages::Column::ConversationId.eq(conversation_id));

    let total = query.clone().count(orm).await?;
    if total == 0 {
        return Ok((Vec::new(), 0));
    }

    query = query.order_by_desc(messages::Column::SentAt);
    let rows = query.limit(page_size).offset(offset).all(orm).await?;
    Ok((rows, total))
}
