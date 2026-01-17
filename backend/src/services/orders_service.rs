use std::str::FromStr;

use sea_orm::TransactionTrait;

use crate::dto::orders::{
    CancelOrderReq, CancelOrderResp, OrderItemResp, OrderListItem, OrderListQuery, OrderResp,
    RefundPreviewResp,
};
use crate::dto::pagination::{normalize_pagination, Paged};
use crate::entity::orders;
use crate::errors::{DomainError, ServiceResult};
use crate::repositories::orders_repo;
use crate::state::AppState;

pub async fn list_orders(
    state: &AppState,
    user_id: i64,
    query: OrderListQuery,
) -> ServiceResult<Paged<OrderListItem>> {
    let (page, page_size) = normalize_pagination(query.page, query.page_size);

    let keyword_id = match query.keyword.as_deref() {
        Some(keyword) if !keyword.trim().is_empty() => keyword
            .trim()
            .parse::<i64>()
            .map(Some)
            .map_err(|_| DomainError::BadRequest("invalid_keyword".to_string()))?,
        _ => None,
    };

    if let (Some(min_amount), Some(max_amount)) = (query.min_amount, query.max_amount)
        && min_amount > max_amount
    {
        return Err(DomainError::BadRequest("invalid_amount_range".to_string()).into());
    }

    let min_amount = query.min_amount.map(decimal_from_f64);
    let max_amount = query.max_amount.map(decimal_from_f64);

    let start_time = match query.start_time.as_deref() {
        Some(value) => Some(parse_datetime(value)?),
        None => None,
    };
    let end_time = match query.end_time.as_deref() {
        Some(value) => Some(parse_datetime(value)?),
        None => None,
    };

    let filter = orders_repo::OrderListFilter {
        status: query.status,
        keyword_id,
        min_amount,
        max_amount,
        start_time,
        end_time,
        sort: query.sort,
    };

    let (rows, total) =
        orders_repo::list_orders_by_user(&state.orm, user_id, filter, page, page_size).await?;

    let items = rows
        .into_iter()
        .map(|r| OrderListItem {
            id: r.id,
            status: r.status,
            total_amount: decimal_to_f64(r.total_amount),
        })
        .collect();

    Ok(Paged::new(items, total, page, page_size))
}

pub async fn get_order(
    state: &AppState,
    user_id: i64,
    order_id: i64,
) -> ServiceResult<OrderResp> {
    let order = orders_repo::find_order_by_id(&state.orm, order_id)
        .await?
        .ok_or(DomainError::NotFound)?;

    if order.user_id != user_id {
        return Err(DomainError::Forbidden.into());
    }

    let items = orders_repo::list_order_items(&state.orm, order_id)
        .await?
        .into_iter()
        .map(|it| OrderItemResp {
            name: it.name,
            price: decimal_to_f64(it.price),
            quantity: it.quantity,
        })
        .collect();

    Ok(OrderResp {
        id: order.id,
        user_id: order.user_id,
        status: order.status,
        pay_type: order.pay_type,
        total_amount: decimal_to_f64(order.total_amount),
        service_fee: decimal_to_f64(order.service_fee),
        items,
    })
}

pub async fn refund_preview(
    state: &AppState,
    user_id: i64,
    order_id: i64,
) -> ServiceResult<RefundPreviewResp> {
    let order = orders_repo::find_order_by_id(&state.orm, order_id)
        .await?
        .ok_or(DomainError::NotFound)?;

    let cancel_by = resolve_cancel_role(&state.orm, &order, user_id).await?;
    ensure_cancellable(&order)?;

    let paid_amount = calc_paid_amount(&state.orm, order_id).await?;
    let (ratio, rule) = compute_refund_ratio(&order, &cancel_by, paid_amount);
    let refund_amount = (paid_amount * ratio).max(0.0);

    Ok(RefundPreviewResp {
        order_id,
        paid_amount,
        refund_ratio: ratio,
        refund_amount,
        responsible_party: cancel_by,
        rule,
    })
}

pub async fn cancel_order(
    state: &AppState,
    user_id: i64,
    order_id: i64,
    req: CancelOrderReq,
) -> ServiceResult<CancelOrderResp> {
    let txn = state.orm.begin().await?;

    let order = orders_repo::find_order_by_id(&txn, order_id)
        .await?
        .ok_or(DomainError::NotFound)?;

    let cancel_by = resolve_cancel_role(&txn, &order, user_id).await?;
    ensure_cancellable(&order)?;
    if order.status == "cancelled" {
        return Err(DomainError::BadRequest("already_cancelled".to_string()).into());
    }

    let paid_amount = calc_paid_amount(&txn, order_id).await?;
    let (ratio, _rule) = compute_refund_ratio(&order, &cancel_by, paid_amount);
    let refund_amount = (paid_amount * ratio).max(0.0);

    let refund_id = if refund_amount > 0.0 {
        let refund = crate::entity::refunds::ActiveModel {
            order_id: sea_orm::ActiveValue::Set(order_id),
            applicant_id: sea_orm::ActiveValue::Set(user_id),
            amount: sea_orm::ActiveValue::Set(decimal_from_f64(refund_amount)),
            status: sea_orm::ActiveValue::Set("pending".to_string()),
            responsible_party: sea_orm::ActiveValue::Set(Some(cancel_by.clone())),
            reason: sea_orm::ActiveValue::Set(req.reason),
            proof_url: sea_orm::ActiveValue::Set(None),
            ..Default::default()
        };
        Some(orders_repo::create_refund(&txn, refund).await?.id)
    } else {
        None
    };

    orders_repo::update_order_status_cancelled(&txn, order, chrono::Utc::now()).await?;

    txn.commit().await?;

    Ok(CancelOrderResp {
        order_id,
        status: "cancelled".to_string(),
        refund_id,
        refund_amount,
    })
}

async fn resolve_cancel_role<C: sea_orm::ConnectionTrait>(
    conn: &C,
    order: &orders::Model,
    user_id: i64,
) -> ServiceResult<String> {
    if order.user_id == user_id {
        return Ok("user".to_string());
    }

    let photographer_user = match order.photographer_id {
        Some(pid) => orders_repo::find_photographer_user_id(conn, pid).await?,
        None => None,
    };

    if photographer_user == Some(user_id) {
        return Ok("photographer".to_string());
    }

    Err(DomainError::Forbidden.into())
}

fn ensure_cancellable(order: &orders::Model) -> ServiceResult<()> {
    if matches!(order.status.as_str(), "completed" | "reviewed") {
        return Err(DomainError::BadRequest("order_not_cancellable".to_string()).into());
    }
    Ok(())
}

fn compute_refund_ratio(order: &orders::Model, cancel_by: &str, paid: f64) -> (f64, String) {
    if cancel_by == "photographer" {
        return (1.0, "photographer_full_refund".to_string());
    }
    if paid <= 0.0 {
        return (0.0, "unpaid_no_refund".to_string());
    }
    let start_at = order
        .schedule_start
        .map(|t| t.with_timezone(&chrono::Utc))
        .unwrap_or_else(|| order.created_at.with_timezone(&chrono::Utc));
    let now = chrono::Utc::now();
    let diff_days = (start_at - now).num_days();
    let is_deposit = order.pay_type == "deposit";
    let ratio = if diff_days >= 7 {
        if is_deposit { 0.8 } else { 0.9 }
    } else if diff_days >= 3 {
        if is_deposit { 0.5 } else { 0.7 }
    } else if is_deposit {
        0.0
    } else {
        0.5
    };
    (ratio, "time_based_default".to_string())
}

async fn calc_paid_amount<C: sea_orm::ConnectionTrait>(
    conn: &C,
    order_id: i64,
) -> ServiceResult<f64> {
    let rows = orders_repo::list_success_payments(conn, order_id).await?;
    let total = rows
        .into_iter()
        .fold(0.0, |acc, p| acc + decimal_to_f64(p.amount));
    Ok(total)
}

fn decimal_to_f64(v: sea_orm::prelude::Decimal) -> f64 {
    v.to_string().parse::<f64>().unwrap_or(0.0)
}

fn decimal_from_f64(v: f64) -> sea_orm::prelude::Decimal {
    sea_orm::prelude::Decimal::from_str(&v.to_string())
        .unwrap_or(sea_orm::prelude::Decimal::ZERO)
}

fn parse_datetime(input: &str) -> Result<chrono::DateTime<chrono::Utc>, DomainError> {
    chrono::DateTime::parse_from_rfc3339(input)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .map_err(|_| DomainError::BadRequest("invalid_datetime".to_string()))
}
