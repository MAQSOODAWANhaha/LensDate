use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait, Order,
    PaginatorTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use sea_orm::prelude::Expr;

use crate::entity::{demand_attachments, demands};

pub struct DemandListFilter {
    pub city_id: Option<i64>,
    pub demand_type: Option<String>,
    pub status: Option<String>,
    pub schedule_start: Option<chrono::DateTime<chrono::Utc>>,
    pub schedule_end: Option<chrono::DateTime<chrono::Utc>>,
    pub min_budget: Option<sea_orm::prelude::Decimal>,
    pub max_budget: Option<sea_orm::prelude::Decimal>,
    pub style_tag: Option<String>,
    pub is_merchant: Option<bool>,
    pub mine: bool,
    pub sort: Option<String>,
}

pub async fn create_demand(
    orm: &DatabaseConnection,
    model: demands::ActiveModel,
) -> anyhow::Result<demands::Model> {
    Ok(model.insert(orm).await?)
}

pub async fn create_attachment(
    orm: &DatabaseConnection,
    model: demand_attachments::ActiveModel,
) -> anyhow::Result<demand_attachments::Model> {
    Ok(model.insert(orm).await?)
}

pub async fn list_demands(
    orm: &DatabaseConnection,
    user_id: i64,
    filter: DemandListFilter,
    page: u64,
    page_size: u64,
) -> anyhow::Result<(Vec<demands::Model>, u64)> {
    let offset = (page - 1) * page_size;

    let mut cond = Condition::all();
    if let Some(city_id) = filter.city_id {
        cond = cond.add(demands::Column::CityId.eq(city_id));
    }
    if let Some(demand_type) = filter.demand_type {
        cond = cond.add(demands::Column::Type.eq(demand_type));
    }
    if let Some(status) = filter.status {
        cond = cond.add(demands::Column::Status.eq(status));
    }
    if let Some(start) = filter.schedule_start {
        cond = cond.add(demands::Column::ScheduleStart.gte(start));
    }
    if let Some(end) = filter.schedule_end {
        cond = cond.add(demands::Column::ScheduleEnd.lte(end));
    }
    if let Some(min_budget) = filter.min_budget {
        let budget_cond = Condition::any()
            .add(demands::Column::BudgetMin.gte(min_budget))
            .add(demands::Column::BudgetMax.gte(min_budget));
        cond = cond.add(budget_cond);
    }
    if let Some(max_budget) = filter.max_budget {
        let budget_cond = Condition::any()
            .add(demands::Column::BudgetMin.lte(max_budget))
            .add(demands::Column::BudgetMax.lte(max_budget));
        cond = cond.add(budget_cond);
    }
    if let Some(is_merchant) = filter.is_merchant {
        cond = cond.add(demands::Column::IsMerchant.eq(is_merchant));
    }
    if let Some(tag) = filter.style_tag.as_deref() {
        let expr = Expr::cust(format!("style_tags::text LIKE '%\"{}\"%'", tag));
        cond = cond.add(expr);
    }
    if filter.mine {
        cond = cond.add(demands::Column::UserId.eq(user_id));
    }

    let mut query = demands::Entity::find().filter(cond);
    query = match filter.sort.as_deref() {
        Some("time_asc") => query.order_by_asc(demands::Column::CreatedAt),
        Some("time_desc") | Some("latest") => query.order_by_desc(demands::Column::CreatedAt),
        Some("budget_asc") => query
            .order_by(
                Expr::cust("COALESCE(budget_min, budget_max, 999999999)"),
                Order::Asc,
            )
            .order_by_desc(demands::Column::CreatedAt),
        Some("budget_desc") => query
            .order_by(
                Expr::cust("COALESCE(budget_max, budget_min, 0)"),
                Order::Desc,
            )
            .order_by_desc(demands::Column::CreatedAt),
        _ => query.order_by_desc(demands::Column::CreatedAt),
    };

    let total = query.clone().count(orm).await?;
    if total == 0 {
        return Ok((Vec::new(), 0));
    }

    let rows = query.limit(page_size).offset(offset).all(orm).await?;
    Ok((rows, total))
}

pub async fn find_demand_by_id(
    orm: &DatabaseConnection,
    id: i64,
) -> anyhow::Result<Option<demands::Model>> {
    Ok(demands::Entity::find_by_id(id).one(orm).await?)
}

pub async fn list_attachments_by_demand(
    orm: &DatabaseConnection,
    demand_id: i64,
) -> anyhow::Result<Vec<demand_attachments::Model>> {
    Ok(demand_attachments::Entity::find()
        .filter(demand_attachments::Column::DemandId.eq(demand_id))
        .all(orm)
        .await?)
}

pub async fn update_demand_status(
    orm: &DatabaseConnection,
    demand: demands::Model,
    status: String,
) -> anyhow::Result<demands::Model> {
    let mut model: demands::ActiveModel = demand.into();
    model.status = Set(status);
    Ok(model.update(orm).await?)
}
