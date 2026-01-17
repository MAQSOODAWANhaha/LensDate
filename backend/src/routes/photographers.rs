use axum::{routing::{get, post}, Json, Router};
use sea_orm::{
    ActiveModelTrait, ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, Set,
};
use serde::{Deserialize, Serialize};

use crate::middleware::auth::AuthUser;
use crate::error::{ApiError, ApiResult};
use crate::state::AppState;
use crate::entity::{order_items, orders, photographers, portfolio_items, portfolios, users};
use crate::dto::pagination::{normalize_pagination, Paged};
use crate::handlers::photographers as photographer_handlers;

#[derive(Deserialize)]
struct CreatePhotographerReq {
    r#type: String,
    city_id: i64,
    service_area: Option<String>,
}

#[derive(Serialize)]
struct PhotographerResp {
    id: i64,
    user_id: i64,
    r#type: String,
    status: String,
    city_id: Option<i64>,
    service_area: Option<String>,
}

#[derive(Deserialize)]
struct CreatePortfolioReq {
    photographer_id: i64,
    title: String,
}

#[derive(Serialize)]
struct PortfolioResp {
    id: i64,
    photographer_id: i64,
    title: String,
    status: String,
}

#[derive(Deserialize)]
struct CreatePortfolioItemReq {
    url: String,
    tags: Option<Vec<String>>,
    cover_flag: Option<bool>,
}

#[derive(Serialize)]
struct PortfolioItemResp {
    id: i64,
    portfolio_id: i64,
    url: String,
}

#[derive(Serialize)]
struct PhotographerOrderListItem {
    id: i64,
    status: String,
    total_amount: f64,
    demand_id: Option<i64>,
    created_at: String,
}

#[derive(Serialize)]
struct PhotographerOrderItem {
    name: String,
    price: f64,
    quantity: i32,
}

#[derive(Serialize)]
struct PhotographerOrderDetail {
    id: i64,
    status: String,
    pay_type: String,
    total_amount: f64,
    service_fee: f64,
    schedule_start: Option<String>,
    schedule_end: Option<String>,
    user_id: i64,
    user_phone: Option<String>,
    demand_id: Option<i64>,
    created_at: String,
    items: Vec<PhotographerOrderItem>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_photographer).get(photographer_handlers::list_photographers))
        .route("/me", get(get_my_photographer))
        .route("/me/orders", get(list_my_orders))
        .route("/me/orders/:id", get(get_my_order))
        .route("/:id", get(get_photographer))
}

pub fn portfolio_router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_portfolio).get(list_portfolios))
        .route("/:id/items", post(add_portfolio_item).get(list_portfolio_items))
}

async fn create_photographer(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<CreatePhotographerReq>,
) -> ApiResult<PhotographerResp> {
    if req.r#type != "individual" && req.r#type != "team" {
        return Err(ApiError::bad_request("invalid_type"));
    }

    let model = photographers::ActiveModel {
        user_id: Set(user_id),
        r#type: Set(req.r#type),
        status: Set("pending".to_string()),
        city_id: Set(Some(req.city_id)),
        service_area: Set(req.service_area),
        ..Default::default()
    };

    let inserted = model.insert(&state.orm).await?;

    Ok(Json(crate::common::ApiResponse::ok(PhotographerResp {
        id: inserted.id,
        user_id: inserted.user_id,
        r#type: inserted.r#type,
        status: inserted.status,
        city_id: inserted.city_id,
        service_area: inserted.service_area,
    })))
}

async fn get_photographer(
    axum::extract::Path(id): axum::extract::Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<PhotographerResp> {
    let row = photographers::Entity::find_by_id(id)
        .one(&state.orm)
        .await?
        .ok_or_else(ApiError::not_found)?;

    Ok(Json(crate::common::ApiResponse::ok(PhotographerResp {
        id: row.id,
        user_id: row.user_id,
        r#type: row.r#type,
        status: row.status,
        city_id: row.city_id,
        service_area: row.service_area,
    })))
}

async fn get_my_photographer(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<PhotographerResp> {
    let row = photographers::Entity::find()
        .filter(photographers::Column::UserId.eq(user_id))
        .one(&state.orm)
        .await?
        .ok_or_else(ApiError::not_found)?;

    Ok(Json(crate::common::ApiResponse::ok(PhotographerResp {
        id: row.id,
        user_id: row.user_id,
        r#type: row.r#type,
        status: row.status,
        city_id: row.city_id,
        service_area: row.service_area,
    })))
}

#[derive(Deserialize)]
struct PhotographerOrderQuery {
    status: Option<String>,
    page: Option<u64>,
    page_size: Option<u64>,
}

async fn list_my_orders(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(q): axum::extract::Query<PhotographerOrderQuery>,
) -> ApiResult<Paged<PhotographerOrderListItem>> {
    let photographer = photographers::Entity::find()
        .filter(photographers::Column::UserId.eq(user_id))
        .one(&state.orm)
        .await?
        .ok_or_else(ApiError::not_found)?;

    let (page, page_size) = normalize_pagination(q.page, q.page_size);
    let offset = (page - 1) * page_size;

    let mut query = orders::Entity::find()
        .filter(orders::Column::PhotographerId.eq(photographer.id));
    if let Some(status) = q.status.as_deref() {
        query = query.filter(orders::Column::Status.eq(status));
    }

    let total = query.clone().count(&state.orm).await?;
    if total == 0 {
        return Ok(Json(crate::common::ApiResponse::ok(Paged::new(
            Vec::new(),
            0,
            page,
            page_size,
        ))));
    }

    let rows = query
        .order_by_desc(orders::Column::CreatedAt)
        .limit(page_size)
        .offset(offset)
        .all(&state.orm)
        .await?;

    let items = rows
        .into_iter()
        .map(|o| PhotographerOrderListItem {
            id: o.id,
            status: o.status,
            total_amount: decimal_to_f64(o.total_amount),
            demand_id: o.demand_id,
            created_at: o.created_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(crate::common::ApiResponse::ok(Paged::new(
        items, total, page, page_size,
    ))))
}

async fn get_my_order(
    AuthUser { user_id }: AuthUser,
    axum::extract::Path(order_id): axum::extract::Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<PhotographerOrderDetail> {
    let photographer = photographers::Entity::find()
        .filter(photographers::Column::UserId.eq(user_id))
        .one(&state.orm)
        .await?
        .ok_or_else(ApiError::not_found)?;

    let order = orders::Entity::find_by_id(order_id)
        .one(&state.orm)
        .await?
        .ok_or_else(ApiError::not_found)?;

    if order.photographer_id != Some(photographer.id) {
        return Err(ApiError::forbidden());
    }

    let items = order_items::Entity::find()
        .filter(order_items::Column::OrderId.eq(order_id))
        .all(&state.orm)
        .await?
        .into_iter()
        .map(|it| PhotographerOrderItem {
            name: it.name,
            price: decimal_to_f64(it.price),
            quantity: it.quantity,
        })
        .collect();

    let user_phone = users::Entity::find_by_id(order.user_id)
        .select_only()
        .column(users::Column::Phone)
        .into_tuple::<String>()
        .one(&state.orm)
        .await?;

    Ok(Json(crate::common::ApiResponse::ok(PhotographerOrderDetail {
        id: order.id,
        status: order.status,
        pay_type: order.pay_type,
        total_amount: decimal_to_f64(order.total_amount),
        service_fee: decimal_to_f64(order.service_fee),
        schedule_start: order.schedule_start.map(|d| d.to_rfc3339()),
        schedule_end: order.schedule_end.map(|d| d.to_rfc3339()),
        user_id: order.user_id,
        user_phone,
        demand_id: order.demand_id,
        created_at: order.created_at.to_rfc3339(),
        items,
    })))
}

async fn create_portfolio(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<CreatePortfolioReq>,
) -> ApiResult<PortfolioResp> {
    let photographer = photographers::Entity::find_by_id(req.photographer_id)
        .one(&state.orm)
        .await?
        .ok_or_else(ApiError::not_found)?;

    if photographer.user_id != user_id {
        return Err(ApiError::forbidden());
    }

    let model = portfolios::ActiveModel {
        photographer_id: Set(req.photographer_id),
        title: Set(req.title),
        status: Set("pending".to_string()),
        ..Default::default()
    };

    let inserted = model.insert(&state.orm).await?;

    Ok(Json(crate::common::ApiResponse::ok(PortfolioResp {
        id: inserted.id,
        photographer_id: inserted.photographer_id,
        title: inserted.title,
        status: inserted.status,
    })))
}

async fn add_portfolio_item(
    AuthUser { user_id }: AuthUser,
    axum::extract::Path(portfolio_id): axum::extract::Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<CreatePortfolioItemReq>,
) -> ApiResult<PortfolioItemResp> {
    let portfolio = portfolios::Entity::find_by_id(portfolio_id)
        .one(&state.orm)
        .await?
        .ok_or_else(ApiError::not_found)?;

    let photographer = photographers::Entity::find_by_id(portfolio.photographer_id)
        .one(&state.orm)
        .await?
        .ok_or_else(ApiError::not_found)?;

    if photographer.user_id != user_id {
        return Err(ApiError::forbidden());
    }

    let item = portfolio_items::ActiveModel {
        portfolio_id: Set(portfolio_id),
        url: Set(req.url),
        tags: Set(req.tags.map(|v| serde_json::json!(v))),
        cover_flag: Set(req.cover_flag.unwrap_or(false)),
        ..Default::default()
    };

    let inserted = item.insert(&state.orm).await?;

    Ok(Json(crate::common::ApiResponse::ok(PortfolioItemResp {
        id: inserted.id,
        portfolio_id: inserted.portfolio_id,
        url: inserted.url,
    })))
}

#[derive(Deserialize)]
struct PortfolioListQuery {
    photographer_id: Option<i64>,
}

async fn list_portfolios(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(q): axum::extract::Query<PortfolioListQuery>,
) -> ApiResult<Vec<PortfolioResp>> {
    let photographer_id = match q.photographer_id {
        Some(id) => id,
        None => {
            let photographer = photographers::Entity::find()
                .filter(photographers::Column::UserId.eq(user_id))
                .one(&state.orm)
                .await?
                .ok_or_else(ApiError::not_found)?;
            photographer.id
        }
    };

    let photographer = photographers::Entity::find_by_id(photographer_id)
        .one(&state.orm)
        .await?
        .ok_or_else(ApiError::not_found)?;
    if photographer.user_id != user_id {
        return Err(ApiError::forbidden());
    }

    let rows = portfolios::Entity::find()
        .filter(portfolios::Column::PhotographerId.eq(photographer_id))
        .order_by_desc(portfolios::Column::CreatedAt)
        .all(&state.orm)
        .await?;

    let items = rows
        .into_iter()
        .map(|p| PortfolioResp {
            id: p.id,
            photographer_id: p.photographer_id,
            title: p.title,
            status: p.status,
        })
        .collect();

    Ok(Json(crate::common::ApiResponse::ok(items)))
}

async fn list_portfolio_items(
    AuthUser { user_id }: AuthUser,
    axum::extract::Path(portfolio_id): axum::extract::Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<Vec<PortfolioItemResp>> {
    let portfolio = portfolios::Entity::find_by_id(portfolio_id)
        .one(&state.orm)
        .await?
        .ok_or_else(ApiError::not_found)?;

    let photographer = photographers::Entity::find_by_id(portfolio.photographer_id)
        .one(&state.orm)
        .await?
        .ok_or_else(ApiError::not_found)?;
    if photographer.user_id != user_id {
        return Err(ApiError::forbidden());
    }

    let rows = portfolio_items::Entity::find()
        .filter(portfolio_items::Column::PortfolioId.eq(portfolio_id))
        .order_by_desc(portfolio_items::Column::CreatedAt)
        .all(&state.orm)
        .await?;

    let items = rows
        .into_iter()
        .map(|item| PortfolioItemResp {
            id: item.id,
            portfolio_id: item.portfolio_id,
            url: item.url,
        })
        .collect();

    Ok(Json(crate::common::ApiResponse::ok(items)))
}

fn decimal_to_f64(v: sea_orm::prelude::Decimal) -> f64 {
    v.to_string().parse::<f64>().unwrap_or(0.0)
}
