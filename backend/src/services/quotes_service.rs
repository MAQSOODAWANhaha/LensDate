use std::collections::HashMap;
use std::str::FromStr;

use chrono::{Duration, Utc};
use sea_orm::TransactionTrait;
use serde_json::json;

use crate::dto::quotes::{
    AcceptQuoteResp, CreateQuoteReq, MyQuoteListItem, MyQuoteListQuery, QuoteDetailResp,
    QuoteItemResp, QuoteListItem, QuoteListQuery, QuoteResp, QuoteVersionItem, UpdateQuoteReq,
};
use crate::dto::pagination::{normalize_pagination, Paged};
use crate::errors::{DomainError, ServiceResult};
use crate::repositories::quotes_repo;
use crate::state::AppState;

const QUOTE_EXPIRE_DAYS: i64 = 7;

pub async fn list_quotes(
    state: &AppState,
    user_id: i64,
    query: QuoteListQuery,
) -> ServiceResult<Paged<QuoteListItem>> {
    let demand_id = query
        .demand_id
        .ok_or_else(|| DomainError::BadRequest("demand_id_required".to_string()))?;

    let demand = quotes_repo::find_demand_by_id(&state.orm, demand_id)
        .await?
        .ok_or(DomainError::NotFound)?;
    if demand.user_id != user_id {
        return Err(DomainError::Forbidden.into());
    }

    quotes_repo::expire_quotes_for_demand(&state.orm, demand_id).await?;

    let (page, page_size) = normalize_pagination(query.page, query.page_size);
    let (rows, total) =
        quotes_repo::list_quotes_by_demand(&state.orm, demand_id, page, page_size).await?;
    if rows.is_empty() {
        return Ok(Paged::new(Vec::new(), total, page, page_size));
    }

    let quote_ids: Vec<i64> = rows.iter().map(|q| q.id).collect();
    let items = quotes_repo::list_quote_items_by_quote_ids(&state.orm, quote_ids).await?;

    let mut items_map: HashMap<i64, Vec<QuoteItemResp>> = HashMap::new();
    for item in items {
        items_map.entry(item.quote_id).or_default().push(QuoteItemResp {
            name: item.name,
            price: decimal_to_f64(item.price),
            quantity: item.quantity,
        });
    }

    let items = rows
        .into_iter()
        .map(|q| QuoteListItem {
            id: q.id,
            demand_id: q.demand_id,
            status: q.status,
            total_price: decimal_to_f64(q.total_price),
            photographer_id: q.photographer_id,
            team_id: q.team_id,
            version: q.version,
            expires_at: q.expires_at.map(|v| v.to_rfc3339()),
            items: items_map.remove(&q.id).unwrap_or_default(),
        })
        .collect();

    Ok(Paged::new(items, total, page, page_size))
}

pub async fn get_quote(
    state: &AppState,
    user_id: i64,
    quote_id: i64,
) -> ServiceResult<QuoteDetailResp> {
    quotes_repo::expire_quote_by_id(&state.orm, quote_id).await?;

    let quote = quotes_repo::find_quote_by_id(&state.orm, quote_id)
        .await?
        .ok_or(DomainError::NotFound)?;

    let demand = quotes_repo::find_demand_by_id(&state.orm, quote.demand_id)
        .await?
        .ok_or(DomainError::NotFound)?;
    let is_owner = demand.user_id == user_id;
    let is_photographer = match quote.photographer_id {
        Some(pid) => {
            let owner = quotes_repo::find_photographer_by_id(&state.orm, pid).await?;
            owner.map(|o| o.user_id == user_id).unwrap_or(false)
        }
        None => false,
    };
    if !is_owner && !is_photographer {
        return Err(DomainError::Forbidden.into());
    }

    let items = quotes_repo::list_quote_items_by_quote_id(&state.orm, quote_id)
        .await?
        .into_iter()
        .map(|it| QuoteItemResp {
            name: it.name,
            price: decimal_to_f64(it.price),
            quantity: it.quantity,
        })
        .collect();

    let order = quotes_repo::find_order_by_quote_id(&state.orm, quote_id).await?;
    let (order_id, order_status) = match order {
        Some((id, status)) => (Some(id), Some(status)),
        None => (None, None),
    };

    Ok(QuoteDetailResp {
        id: quote.id,
        demand_id: quote.demand_id,
        status: quote.status,
        total_price: decimal_to_f64(quote.total_price),
        photographer_id: quote.photographer_id,
        team_id: quote.team_id,
        version: quote.version,
        expires_at: quote.expires_at.map(|v| v.to_rfc3339()),
        order_id,
        order_status,
        items,
    })
}

pub async fn list_my_quotes(
    state: &AppState,
    user_id: i64,
    query: MyQuoteListQuery,
) -> ServiceResult<Paged<MyQuoteListItem>> {
    let photographer = quotes_repo::find_photographer_by_user(&state.orm, user_id)
        .await?
        .ok_or(DomainError::NotFound)?;

    quotes_repo::expire_quotes_for_photographer(&state.orm, photographer.id).await?;

    let (page, page_size) = normalize_pagination(query.page, query.page_size);

    let (rows, total) = quotes_repo::list_quotes_by_photographer(
        &state.orm,
        photographer.id,
        query.status,
        query.demand_id,
        page,
        page_size,
    )
    .await?;

    if rows.is_empty() {
        return Ok(Paged::new(Vec::new(), total, page, page_size));
    }

    let quote_ids: Vec<i64> = rows.iter().map(|q| q.id).collect();
    let items = quotes_repo::list_quote_items_by_quote_ids(&state.orm, quote_ids.clone()).await?;

    let mut items_map: HashMap<i64, Vec<QuoteItemResp>> = HashMap::new();
    for item in items {
        items_map.entry(item.quote_id).or_default().push(QuoteItemResp {
            name: item.name,
            price: decimal_to_f64(item.price),
            quantity: item.quantity,
        });
    }

    let order_rows = quotes_repo::list_orders_by_quote_ids(&state.orm, quote_ids).await?;
    let mut order_map: HashMap<i64, (i64, String)> = HashMap::new();
    for (order_id, quote_id, status) in order_rows {
        if let Some(qid) = quote_id {
            order_map.insert(qid, (order_id, status));
        }
    }

    let items = rows
        .into_iter()
        .map(|q| MyQuoteListItem {
            id: q.id,
            demand_id: q.demand_id,
            status: q.status,
            total_price: decimal_to_f64(q.total_price),
            created_at: q.created_at.to_rfc3339(),
            version: q.version,
            expires_at: q.expires_at.map(|v| v.to_rfc3339()),
            order_id: order_map.get(&q.id).map(|(id, _)| *id),
            order_status: order_map.get(&q.id).map(|(_, status)| status.clone()),
            items: items_map.remove(&q.id).unwrap_or_default(),
        })
        .collect();

    Ok(Paged::new(items, total, page, page_size))
}

pub async fn withdraw_quote(
    state: &AppState,
    user_id: i64,
    quote_id: i64,
) -> ServiceResult<QuoteResp> {
    let quote = quotes_repo::find_quote_by_id(&state.orm, quote_id)
        .await?
        .ok_or(DomainError::NotFound)?;

    if quote.status != "pending" {
        return Err(DomainError::InvalidStatus.into());
    }

    let photographer_id = quote.photographer_id.ok_or(DomainError::Forbidden)?;
    let photographer = quotes_repo::find_photographer_by_id(&state.orm, photographer_id)
        .await?
        .ok_or(DomainError::NotFound)?;
    if photographer.user_id != user_id {
        return Err(DomainError::Forbidden.into());
    }

    let updated = quotes_repo::update_quote_status(&state.orm, quote, "expired".to_string()).await?;

    Ok(QuoteResp {
        id: updated.id,
        demand_id: updated.demand_id,
        status: updated.status,
    })
}

pub async fn create_quote(
    state: &AppState,
    user_id: i64,
    req: CreateQuoteReq,
) -> ServiceResult<QuoteResp> {
    if req.items.is_empty() {
        return Err(DomainError::ItemsRequired.into());
    }
    if req.photographer_id.is_none() == req.team_id.is_none() {
        return Err(DomainError::BadRequest("photographer_or_team_required".to_string()).into());
    }

    if let Some(pid) = req.photographer_id {
        let owner = quotes_repo::find_photographer_by_id(&state.orm, pid).await?;
        if owner.map(|o| o.user_id) != Some(user_id) {
            return Err(DomainError::Forbidden.into());
        }
    }

    let expires_at = Utc::now() + Duration::days(QUOTE_EXPIRE_DAYS);
    let items_json = json!(req
        .items
        .iter()
        .map(|item| json!({
            "name": item.name,
            "price": item.price,
            "quantity": item.quantity
        }))
        .collect::<Vec<_>>());

    let txn = state.orm.begin().await?;
    let q = crate::entity::quotes::ActiveModel {
        demand_id: sea_orm::ActiveValue::Set(req.demand_id),
        photographer_id: sea_orm::ActiveValue::Set(req.photographer_id),
        team_id: sea_orm::ActiveValue::Set(req.team_id),
        total_price: sea_orm::ActiveValue::Set(decimal_from_f64(req.total_price)),
        status: sea_orm::ActiveValue::Set("pending".to_string()),
        version: sea_orm::ActiveValue::Set(1),
        expires_at: sea_orm::ActiveValue::Set(Some(expires_at.into())),
        ..Default::default()
    };

    let quote = quotes_repo::create_quote(&txn, q).await?;

    for item in req.items {
        let qi = crate::entity::quote_items::ActiveModel {
            quote_id: sea_orm::ActiveValue::Set(quote.id),
            name: sea_orm::ActiveValue::Set(item.name),
            price: sea_orm::ActiveValue::Set(decimal_from_f64(item.price)),
            quantity: sea_orm::ActiveValue::Set(item.quantity),
            ..Default::default()
        };
        quotes_repo::create_quote_item(&txn, qi).await?;
    }

    let version = crate::entity::quote_versions::ActiveModel {
        quote_id: sea_orm::ActiveValue::Set(quote.id),
        version: sea_orm::ActiveValue::Set(1),
        total_price: sea_orm::ActiveValue::Set(decimal_from_f64(req.total_price)),
        items: sea_orm::ActiveValue::Set(items_json),
        note: sea_orm::ActiveValue::Set(req.note),
        created_by: sea_orm::ActiveValue::Set(user_id),
        ..Default::default()
    };
    quotes_repo::create_quote_version(&txn, version).await?;

    txn.commit().await?;

    Ok(QuoteResp {
        id: quote.id,
        demand_id: quote.demand_id,
        status: quote.status,
    })
}

pub async fn update_quote(
    state: &AppState,
    user_id: i64,
    quote_id: i64,
    req: UpdateQuoteReq,
) -> ServiceResult<QuoteResp> {
    if req.items.is_empty() {
        return Err(DomainError::ItemsRequired.into());
    }

    let items_json = json!(req
        .items
        .iter()
        .map(|item| json!({
            "name": item.name,
            "price": item.price,
            "quantity": item.quantity
        }))
        .collect::<Vec<_>>());

    let txn = state.orm.begin().await?;
    let quote = quotes_repo::find_quote_by_id(&txn, quote_id)
        .await?
        .ok_or(DomainError::NotFound)?;

    if quote.status != "pending" {
        return Err(DomainError::InvalidStatus.into());
    }

    let photographer_id = quote.photographer_id.ok_or(DomainError::Forbidden)?;
    let photographer = quotes_repo::find_photographer_by_id(&txn, photographer_id)
        .await?
        .ok_or(DomainError::NotFound)?;
    if photographer.user_id != user_id {
        return Err(DomainError::Forbidden.into());
    }

    let new_version = quote.version + 1;
    let expires_at = Utc::now() + Duration::days(QUOTE_EXPIRE_DAYS);

    let updated = quotes_repo::update_quote(
        &txn,
        quote,
        decimal_from_f64(req.total_price),
        new_version,
        expires_at,
    )
    .await?;

    quotes_repo::delete_quote_items(&txn, quote_id).await?;

    for item in req.items {
        let qi = crate::entity::quote_items::ActiveModel {
            quote_id: sea_orm::ActiveValue::Set(updated.id),
            name: sea_orm::ActiveValue::Set(item.name),
            price: sea_orm::ActiveValue::Set(decimal_from_f64(item.price)),
            quantity: sea_orm::ActiveValue::Set(item.quantity),
            ..Default::default()
        };
        quotes_repo::create_quote_item(&txn, qi).await?;
    }

    let version = crate::entity::quote_versions::ActiveModel {
        quote_id: sea_orm::ActiveValue::Set(updated.id),
        version: sea_orm::ActiveValue::Set(new_version),
        total_price: sea_orm::ActiveValue::Set(decimal_from_f64(req.total_price)),
        items: sea_orm::ActiveValue::Set(items_json),
        note: sea_orm::ActiveValue::Set(req.note),
        created_by: sea_orm::ActiveValue::Set(user_id),
        ..Default::default()
    };
    quotes_repo::create_quote_version(&txn, version).await?;

    txn.commit().await?;

    Ok(QuoteResp {
        id: updated.id,
        demand_id: updated.demand_id,
        status: updated.status,
    })
}

pub async fn list_quote_versions(
    state: &AppState,
    user_id: i64,
    quote_id: i64,
) -> ServiceResult<Vec<QuoteVersionItem>> {
    let quote = quotes_repo::find_quote_by_id(&state.orm, quote_id)
        .await?
        .ok_or(DomainError::NotFound)?;
    let demand = quotes_repo::find_demand_by_id(&state.orm, quote.demand_id)
        .await?
        .ok_or(DomainError::NotFound)?;
    let is_owner = demand.user_id == user_id;
    let is_photographer = match quote.photographer_id {
        Some(pid) => {
            let owner = quotes_repo::find_photographer_by_id(&state.orm, pid).await?;
            owner.map(|o| o.user_id == user_id).unwrap_or(false)
        }
        None => false,
    };
    if !is_owner && !is_photographer {
        return Err(DomainError::Forbidden.into());
    }

    let rows = quotes_repo::list_quote_versions(&state.orm, quote_id).await?;

    Ok(rows
        .into_iter()
        .map(|v| QuoteVersionItem {
            id: v.id,
            version: v.version,
            total_price: decimal_to_f64(v.total_price),
            items: v.items,
            note: v.note,
            created_by: v.created_by,
            created_at: v.created_at.to_rfc3339(),
        })
        .collect())
}

pub async fn accept_quote(
    state: &AppState,
    user_id: i64,
    quote_id: i64,
) -> ServiceResult<AcceptQuoteResp> {
    let txn = state.orm.begin().await?;

    let quote = quotes_repo::find_quote_by_id(&txn, quote_id)
        .await?
        .ok_or(DomainError::NotFound)?;

    let demand = quotes_repo::find_demand_by_id(&state.orm, quote.demand_id)
        .await?
        .ok_or(DomainError::NotFound)?;

    if demand.user_id != user_id {
        return Err(DomainError::Forbidden.into());
    }
    if quote.status != "pending" {
        return Err(DomainError::BadRequest("quote_not_pending".to_string()).into());
    }

    let existing = quotes_repo::find_order_by_quote_id(&txn, quote_id).await?;
    if existing.is_some() {
        return Err(DomainError::Conflict("order_exists".to_string()).into());
    }

    let total_price = quote.total_price;
    let photographer_id = quote.photographer_id;
    let team_id = quote.team_id;
    quotes_repo::update_quote_status(&txn, quote, "accepted".to_string()).await?;

    let order = crate::entity::orders::ActiveModel {
        user_id: sea_orm::ActiveValue::Set(user_id),
        demand_id: sea_orm::ActiveValue::Set(Some(demand.id)),
        quote_id: sea_orm::ActiveValue::Set(Some(quote_id)),
        photographer_id: sea_orm::ActiveValue::Set(photographer_id),
        team_id: sea_orm::ActiveValue::Set(team_id),
        status: sea_orm::ActiveValue::Set("confirmed".to_string()),
        pay_type: sea_orm::ActiveValue::Set("deposit".to_string()),
        deposit_amount: sea_orm::ActiveValue::Set(sea_orm::prelude::Decimal::ZERO),
        total_amount: sea_orm::ActiveValue::Set(total_price),
        service_fee: sea_orm::ActiveValue::Set(sea_orm::prelude::Decimal::ZERO),
        ..Default::default()
    };

    let created = quotes_repo::create_order(&txn, order).await?;
    txn.commit().await?;

    Ok(AcceptQuoteResp { order_id: created.id })
}

fn decimal_from_f64(v: f64) -> sea_orm::prelude::Decimal {
    sea_orm::prelude::Decimal::from_str(&v.to_string())
        .unwrap_or(sea_orm::prelude::Decimal::ZERO)
}

fn decimal_to_f64(v: sea_orm::prelude::Decimal) -> f64 {
    v.to_string().parse::<f64>().unwrap_or(0.0)
}
