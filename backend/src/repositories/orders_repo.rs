use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, EntityTrait, PaginatorTrait,
    QueryFilter, QueryOrder, QuerySelect, Set,
};

use crate::entity::{order_items, orders, payments, photographers, refunds};

pub struct OrderListFilter {
    pub status: Option<String>,
    pub keyword_id: Option<i64>,
    pub min_amount: Option<sea_orm::prelude::Decimal>,
    pub max_amount: Option<sea_orm::prelude::Decimal>,
    pub start_time: Option<chrono::DateTime<chrono::Utc>>,
    pub end_time: Option<chrono::DateTime<chrono::Utc>>,
    pub sort: Option<String>,
}

pub async fn list_orders_by_user(
    orm: &DatabaseConnection,
    user_id: i64,
    filter: OrderListFilter,
    page: u64,
    page_size: u64,
) -> anyhow::Result<(Vec<orders::Model>, u64)> {
    let offset = (page - 1) * page_size;
    let mut query = orders::Entity::find().filter(orders::Column::UserId.eq(user_id));

    if let Some(status) = filter.status {
        query = query.filter(orders::Column::Status.eq(status));
    }
    if let Some(keyword_id) = filter.keyword_id {
        query = query.filter(orders::Column::Id.eq(keyword_id));
    }
    if let Some(min_amount) = filter.min_amount {
        query = query.filter(orders::Column::TotalAmount.gte(min_amount));
    }
    if let Some(max_amount) = filter.max_amount {
        query = query.filter(orders::Column::TotalAmount.lte(max_amount));
    }
    if let Some(start_time) = filter.start_time {
        query = query.filter(orders::Column::CreatedAt.gte(start_time));
    }
    if let Some(end_time) = filter.end_time {
        query = query.filter(orders::Column::CreatedAt.lte(end_time));
    }

    query = match filter.sort.as_deref() {
        Some("amount_desc") => query.order_by_desc(orders::Column::TotalAmount),
        Some("amount_asc") => query.order_by_asc(orders::Column::TotalAmount),
        Some("time_asc") => query.order_by_asc(orders::Column::CreatedAt),
        Some("time_desc") | Some("latest") => query.order_by_desc(orders::Column::CreatedAt),
        _ => query.order_by_desc(orders::Column::CreatedAt),
    };

    let total = query.clone().count(orm).await?;
    if total == 0 {
        return Ok((Vec::new(), 0));
    }

    let rows = query.limit(page_size).offset(offset).all(orm).await?;
    Ok((rows, total))
}

pub async fn find_order_by_id<C: ConnectionTrait>(
    orm: &C,
    order_id: i64,
) -> anyhow::Result<Option<orders::Model>> {
    Ok(orders::Entity::find_by_id(order_id).one(orm).await?)
}

pub async fn list_order_items<C: ConnectionTrait>(
    orm: &C,
    order_id: i64,
) -> anyhow::Result<Vec<order_items::Model>> {
    Ok(order_items::Entity::find()
        .filter(order_items::Column::OrderId.eq(order_id))
        .all(orm)
        .await?)
}

pub async fn list_success_payments<C: ConnectionTrait>(
    orm: &C,
    order_id: i64,
) -> anyhow::Result<Vec<payments::Model>> {
    Ok(payments::Entity::find()
        .filter(payments::Column::OrderId.eq(order_id))
        .filter(payments::Column::Status.eq("success"))
        .all(orm)
        .await?)
}

pub async fn find_photographer_user_id<C: ConnectionTrait>(
    orm: &C,
    photographer_id: i64,
) -> anyhow::Result<Option<i64>> {
    Ok(photographers::Entity::find_by_id(photographer_id)
        .one(orm)
        .await?
        .map(|p| p.user_id))
}

pub async fn create_refund<C: ConnectionTrait>(
    orm: &C,
    refund: refunds::ActiveModel,
) -> anyhow::Result<refunds::Model> {
    Ok(refund.insert(orm).await?)
}

pub async fn update_order_status_cancelled<C: ConnectionTrait>(
    orm: &C,
    order: orders::Model,
    cancelled_at: chrono::DateTime<chrono::Utc>,
) -> anyhow::Result<orders::Model> {
    let mut model: orders::ActiveModel = order.into();
    model.status = Set("cancelled".to_string());
    model.cancelled_at = Set(Some(cancelled_at.into()));
    Ok(model.update(orm).await?)
}

pub async fn update_order_status<C: ConnectionTrait>(
    orm: &C,
    order: orders::Model,
    status: String,
) -> anyhow::Result<orders::Model> {
    let mut model: orders::ActiveModel = order.into();
    model.status = Set(status);
    Ok(model.update(orm).await?)
}

pub async fn create_payment<C: ConnectionTrait>(
    orm: &C,
    payment: payments::ActiveModel,
) -> anyhow::Result<payments::Model> {
    Ok(payment.insert(orm).await?)
}
