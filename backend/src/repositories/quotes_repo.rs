use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DatabaseConnection, DbErr, EntityTrait,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use sea_orm::prelude::Expr;

use crate::entity::{demands, orders, quote_items, quote_versions, quotes, photographers};

pub async fn expire_quotes_for_demand(
    orm: &DatabaseConnection,
    demand_id: i64,
) -> Result<(), DbErr> {
    let now = chrono::Utc::now();
    quotes::Entity::update_many()
        .col_expr(quotes::Column::Status, Expr::value("expired"))
        .filter(quotes::Column::DemandId.eq(demand_id))
        .filter(quotes::Column::Status.eq("pending"))
        .filter(quotes::Column::ExpiresAt.lte(now))
        .exec(orm)
        .await?;
    Ok(())
}

pub async fn expire_quotes_for_photographer(
    orm: &DatabaseConnection,
    photographer_id: i64,
) -> Result<(), DbErr> {
    let now = chrono::Utc::now();
    quotes::Entity::update_many()
        .col_expr(quotes::Column::Status, Expr::value("expired"))
        .filter(quotes::Column::PhotographerId.eq(photographer_id))
        .filter(quotes::Column::Status.eq("pending"))
        .filter(quotes::Column::ExpiresAt.lte(now))
        .exec(orm)
        .await?;
    Ok(())
}

pub async fn expire_quote_by_id(orm: &DatabaseConnection, quote_id: i64) -> Result<(), DbErr> {
    let now = chrono::Utc::now();
    quotes::Entity::update_many()
        .col_expr(quotes::Column::Status, Expr::value("expired"))
        .filter(quotes::Column::Id.eq(quote_id))
        .filter(quotes::Column::Status.eq("pending"))
        .filter(quotes::Column::ExpiresAt.lte(now))
        .exec(orm)
        .await?;
    Ok(())
}

pub async fn list_quotes_by_demand(
    orm: &DatabaseConnection,
    demand_id: i64,
    page: u64,
    page_size: u64,
) -> anyhow::Result<(Vec<quotes::Model>, u64)> {
    let offset = (page - 1) * page_size;
    let mut query = quotes::Entity::find()
        .filter(quotes::Column::DemandId.eq(demand_id));

    let total = query.clone().count(orm).await?;
    if total == 0 {
        return Ok((Vec::new(), 0));
    }

    query = query.order_by_desc(quotes::Column::CreatedAt);
    let rows = query.limit(page_size).offset(offset).all(orm).await?;
    Ok((rows, total))
}

pub async fn list_quote_items_by_quote_ids(
    orm: &DatabaseConnection,
    quote_ids: Vec<i64>,
) -> anyhow::Result<Vec<quote_items::Model>> {
    if quote_ids.is_empty() {
        return Ok(Vec::new());
    }
    Ok(quote_items::Entity::find()
        .filter(quote_items::Column::QuoteId.is_in(quote_ids))
        .all(orm)
        .await?)
}

pub async fn list_quote_items_by_quote_id(
    orm: &DatabaseConnection,
    quote_id: i64,
) -> anyhow::Result<Vec<quote_items::Model>> {
    Ok(quote_items::Entity::find()
        .filter(quote_items::Column::QuoteId.eq(quote_id))
        .all(orm)
        .await?)
}

pub async fn find_quote_by_id<C: ConnectionTrait>(
    orm: &C,
    quote_id: i64,
) -> anyhow::Result<Option<quotes::Model>> {
    Ok(quotes::Entity::find_by_id(quote_id).one(orm).await?)
}

pub async fn find_demand_by_id(
    orm: &DatabaseConnection,
    demand_id: i64,
) -> anyhow::Result<Option<demands::Model>> {
    Ok(demands::Entity::find_by_id(demand_id).one(orm).await?)
}

pub async fn find_photographer_by_id<C: ConnectionTrait>(
    orm: &C,
    photographer_id: i64,
) -> anyhow::Result<Option<photographers::Model>> {
    Ok(photographers::Entity::find_by_id(photographer_id)
        .one(orm)
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

pub async fn list_quotes_by_photographer(
    orm: &DatabaseConnection,
    photographer_id: i64,
    status: Option<String>,
    demand_id: Option<i64>,
    page: u64,
    page_size: u64,
) -> anyhow::Result<(Vec<quotes::Model>, u64)> {
    let offset = (page - 1) * page_size;

    let mut query = quotes::Entity::find()
        .filter(quotes::Column::PhotographerId.eq(photographer_id));
    if let Some(status) = status {
        query = query.filter(quotes::Column::Status.eq(status));
    }
    if let Some(demand_id) = demand_id {
        query = query.filter(quotes::Column::DemandId.eq(demand_id));
    }

    let total = query.clone().count(orm).await?;
    if total == 0 {
        return Ok((Vec::new(), 0));
    }

    let rows = query
        .order_by_desc(quotes::Column::CreatedAt)
        .limit(page_size)
        .offset(offset)
        .all(orm)
        .await?;
    Ok((rows, total))
}

pub async fn list_orders_by_quote_ids(
    orm: &DatabaseConnection,
    quote_ids: Vec<i64>,
) -> anyhow::Result<Vec<(i64, Option<i64>, String)>> {
    if quote_ids.is_empty() {
        return Ok(Vec::new());
    }

    Ok(orders::Entity::find()
        .select_only()
        .column(orders::Column::Id)
        .column(orders::Column::QuoteId)
        .column(orders::Column::Status)
        .filter(orders::Column::QuoteId.is_in(quote_ids))
        .into_tuple::<(i64, Option<i64>, String)>()
        .all(orm)
        .await?)
}

pub async fn find_order_by_quote_id<C: ConnectionTrait>(
    orm: &C,
    quote_id: i64,
) -> anyhow::Result<Option<(i64, String)>> {
    Ok(orders::Entity::find()
        .select_only()
        .column(orders::Column::Id)
        .column(orders::Column::Status)
        .filter(orders::Column::QuoteId.eq(quote_id))
        .into_tuple::<(i64, String)>()
        .one(orm)
        .await?)
}

pub async fn create_quote<C: ConnectionTrait>(
    orm: &C,
    model: quotes::ActiveModel,
) -> anyhow::Result<quotes::Model> {
    Ok(model.insert(orm).await?)
}

pub async fn create_quote_item<C: ConnectionTrait>(
    orm: &C,
    model: quote_items::ActiveModel,
) -> anyhow::Result<quote_items::Model> {
    Ok(model.insert(orm).await?)
}

pub async fn create_quote_version<C: ConnectionTrait>(
    orm: &C,
    model: quote_versions::ActiveModel,
) -> anyhow::Result<quote_versions::Model> {
    Ok(model.insert(orm).await?)
}

pub async fn update_quote<C: ConnectionTrait>(
    orm: &C,
    quote: quotes::Model,
    total_price: sea_orm::prelude::Decimal,
    version: i32,
    expires_at: chrono::DateTime<chrono::Utc>,
) -> anyhow::Result<quotes::Model> {
    let mut model: quotes::ActiveModel = quote.into();
    model.total_price = Set(total_price);
    model.version = Set(version);
    model.expires_at = Set(Some(expires_at.into()));
    model.updated_at = Set(chrono::Utc::now().into());
    Ok(model.update(orm).await?)
}

pub async fn update_quote_status<C: ConnectionTrait>(
    orm: &C,
    quote: quotes::Model,
    status: String,
) -> anyhow::Result<quotes::Model> {
    let mut model: quotes::ActiveModel = quote.into();
    model.status = Set(status);
    Ok(model.update(orm).await?)
}

pub async fn delete_quote_items<C: ConnectionTrait>(
    orm: &C,
    quote_id: i64,
) -> anyhow::Result<()> {
    quote_items::Entity::delete_many()
        .filter(quote_items::Column::QuoteId.eq(quote_id))
        .exec(orm)
        .await?;
    Ok(())
}

pub async fn list_quote_versions(
    orm: &DatabaseConnection,
    quote_id: i64,
) -> anyhow::Result<Vec<quote_versions::Model>> {
    Ok(quote_versions::Entity::find()
        .filter(quote_versions::Column::QuoteId.eq(quote_id))
        .order_by_desc(quote_versions::Column::Version)
        .all(orm)
        .await?)
}

pub async fn create_order<C: ConnectionTrait>(
    orm: &C,
    model: orders::ActiveModel,
) -> anyhow::Result<orders::Model> {
    Ok(model.insert(orm).await?)
}
