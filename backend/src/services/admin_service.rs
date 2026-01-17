use axum::Json;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, Condition, EntityTrait, PaginatorTrait, QueryFilter, QueryOrder,
    QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::{HashMap, HashSet};
use chrono::{Duration, NaiveDate, TimeZone, Utc};

use crate::middleware::auth::AuthUser;
use crate::dto::pagination::{normalize_pagination, Paged};
use crate::error::{ApiError, ApiResult};
use crate::state::AppState;
use crate::entity::{
    audit_logs, deliveries, delivery_items, dispute_evidence, disputes, merchant_approvals, merchants,
    merchant_template_items, merchant_templates, merchant_users, order_items, orders, payments,
    photographers, portfolios, refunds, reviews, user_profiles, users,
};

#[derive(Deserialize)]
pub struct CreateAuditReq {
    action: String,
    target_type: Option<String>,
    target_id: Option<i64>,
    detail: Option<serde_json::Value>,
}

#[derive(Serialize)]
pub struct AuditResp {
    id: i64,
    action: String,
}

pub async fn create_audit(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<CreateAuditReq>,
) -> ApiResult<AuditResp> {
    ensure_role_access(&state.orm, user_id, &["admin"]).await?;

    let model = audit_logs::ActiveModel {
        admin_id: Set(user_id),
        action: Set(req.action),
        target_type: Set(req.target_type),
        target_id: Set(req.target_id),
        detail: Set(req.detail),
        ..Default::default()
    };

    let inserted = model.insert(&state.orm).await?;

    Ok(Json(crate::common::ApiResponse::ok(AuditResp {
        id: inserted.id,
        action: inserted.action,
    })))
}

#[derive(Deserialize)]
pub struct AuditListQuery {
    action: Option<String>,
    page: Option<u64>,
    page_size: Option<u64>,
}

#[derive(Serialize)]
pub struct AuditListItem {
    id: i64,
    action: String,
    target_type: Option<String>,
    target_id: Option<i64>,
    admin_id: i64,
    admin_phone: Option<String>,
    detail: Option<serde_json::Value>,
    created_at: String,
}

pub async fn list_audits(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(q): axum::extract::Query<AuditListQuery>,
) -> ApiResult<Paged<AuditListItem>> {
    ensure_role_access(&state.orm, user_id, &["admin"]).await?;

    let (page, page_size) = normalize_pagination(q.page, q.page_size);
    let offset = (page - 1) * page_size;

    let mut query = audit_logs::Entity::find();
    if let Some(action) = &q.action {
        query = query.filter(audit_logs::Column::Action.eq(action));
    }

    let total = query.clone().count(&state.orm).await?;
    if total == 0 {
        return Ok(Json(crate::common::ApiResponse::ok(Paged {
            items: Vec::new(),
            page,
            page_size,
            total,
        })));
    }

    let rows = query
        .order_by_desc(audit_logs::Column::CreatedAt)
        .limit(page_size)
        .offset(offset)
        .all(&state.orm)
        .await?;

    let admin_ids: Vec<i64> = rows.iter().map(|a| a.admin_id).collect();
    let admin_rows = users::Entity::find()
        .filter(users::Column::Id.is_in(admin_ids))
        .all(&state.orm)
        .await?;
    let admin_map: HashMap<i64, String> =
        admin_rows.into_iter().map(|u| (u.id, u.phone)).collect();

    let items = rows
        .into_iter()
        .map(|a| AuditListItem {
            id: a.id,
            action: a.action,
            target_type: a.target_type,
            target_id: a.target_id,
            admin_id: a.admin_id,
            admin_phone: admin_map.get(&a.admin_id).cloned(),
            detail: a.detail,
            created_at: a.created_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(crate::common::ApiResponse::ok(Paged {
        items,
        page,
        page_size,
        total,
    })))
}

#[derive(Deserialize)]
pub struct MetricsQuery {
    days: Option<i64>,
}

#[derive(Serialize)]
pub struct AdminMetricsResp {
    period_days: i64,
    users_total: u64,
    orders_total: u64,
    orders_period: u64,
    orders_today: u64,
    disputes_open: u64,
    disputes_period: u64,
    pending_photographers: u64,
    pending_merchant_approvals: u64,
    revenue_today: f64,
    revenue_period: f64,
}

#[derive(Serialize)]
pub struct AdminTrendPoint {
    date: String,
    orders: u64,
    disputes: u64,
    revenue: f64,
}

#[derive(Serialize)]
pub struct AdminTrendResp {
    days: i64,
    items: Vec<AdminTrendPoint>,
}

#[derive(Deserialize)]
pub struct OrdersReportQuery {
    start_date: Option<String>,
    end_date: Option<String>,
    status: Option<String>,
    limit: Option<u64>,
    format: Option<String>,
}

#[derive(Serialize)]
pub struct OrderReportItem {
    id: i64,
    user_id: i64,
    photographer_id: Option<i64>,
    status: String,
    pay_type: String,
    total_amount: f64,
    created_at: String,
}

#[derive(Serialize)]
pub struct OrderReportResp {
    format: String,
    generated_at: String,
    total: u64,
    items: Vec<OrderReportItem>,
    csv: Option<String>,
}

pub async fn get_metrics(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(q): axum::extract::Query<MetricsQuery>,
) -> ApiResult<AdminMetricsResp> {
    ensure_role_access(&state.orm, user_id, &["admin", "ops", "manager"]).await?;

    let period_days = normalize_days(q.days, 7, 90);
    let now = Utc::now();
    let today_start = day_start(now.date_naive());
    let period_start = day_start(
        (now - Duration::days(period_days.saturating_sub(1))).date_naive(),
    );

    let users_total = users::Entity::find().count(&state.orm).await?;
    let orders_total = orders::Entity::find().count(&state.orm).await?;
    let orders_period = orders::Entity::find()
        .filter(orders::Column::CreatedAt.gte(period_start))
        .count(&state.orm)
        .await?;
    let orders_today = orders::Entity::find()
        .filter(orders::Column::CreatedAt.gte(today_start))
        .count(&state.orm)
        .await?;

    let disputes_open = disputes::Entity::find()
        .filter(dispute_open_condition())
        .count(&state.orm)
        .await?;
    let disputes_period = disputes::Entity::find()
        .filter(disputes::Column::CreatedAt.gte(period_start))
        .count(&state.orm)
        .await?;

    let pending_photographers = photographers::Entity::find()
        .filter(photographers::Column::Status.eq("pending"))
        .count(&state.orm)
        .await?;
    let pending_merchant_approvals = merchant_approvals::Entity::find()
        .filter(merchant_approvals::Column::Status.eq("pending"))
        .count(&state.orm)
        .await?;

    let payments_rows = payments::Entity::find()
        .filter(payments::Column::Status.eq("success"))
        .filter(payments::Column::PaidAt.is_not_null())
        .filter(payments::Column::PaidAt.gte(period_start))
        .all(&state.orm)
        .await?;

    let mut revenue_period = 0.0;
    let mut revenue_today = 0.0;
    for payment in payments_rows {
        let amount = decimal_to_f64(payment.amount);
        revenue_period += amount;
        if let Some(paid_at) = payment.paid_at
            && paid_at.with_timezone(&Utc) >= today_start
        {
            revenue_today += amount;
        }
    }

    Ok(Json(crate::common::ApiResponse::ok(AdminMetricsResp {
        period_days,
        users_total,
        orders_total,
        orders_period,
        orders_today,
        disputes_open,
        disputes_period,
        pending_photographers,
        pending_merchant_approvals,
        revenue_today: round_currency(revenue_today),
        revenue_period: round_currency(revenue_period),
    })))
}

pub async fn get_metrics_trends(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(q): axum::extract::Query<MetricsQuery>,
) -> ApiResult<AdminTrendResp> {
    ensure_role_access(&state.orm, user_id, &["admin", "ops", "manager"]).await?;

    let days = normalize_days(q.days, 7, 90);
    let today = Utc::now().date_naive();
    let start_date = today - Duration::days(days.saturating_sub(1));
    let start_dt = day_start(start_date);

    let mut stats: HashMap<NaiveDate, (u64, u64, f64)> = HashMap::new();
    let mut date = start_date;
    while date <= today {
        stats.insert(date, (0, 0, 0.0));
        date = date.succ_opt().unwrap_or(today + Duration::days(1));
    }

    let order_rows = orders::Entity::find()
        .filter(orders::Column::CreatedAt.gte(start_dt))
        .all(&state.orm)
        .await?;
    for order in order_rows {
        let date = order.created_at.with_timezone(&Utc).date_naive();
        if let Some(entry) = stats.get_mut(&date) {
            entry.0 = entry.0.saturating_add(1);
        }
    }

    let dispute_rows = disputes::Entity::find()
        .filter(disputes::Column::CreatedAt.gte(start_dt))
        .all(&state.orm)
        .await?;
    for dispute in dispute_rows {
        let date = dispute.created_at.with_timezone(&Utc).date_naive();
        if let Some(entry) = stats.get_mut(&date) {
            entry.1 = entry.1.saturating_add(1);
        }
    }

    let payment_rows = payments::Entity::find()
        .filter(payments::Column::Status.eq("success"))
        .filter(payments::Column::PaidAt.is_not_null())
        .filter(payments::Column::PaidAt.gte(start_dt))
        .all(&state.orm)
        .await?;
    for payment in payment_rows {
        if let Some(paid_at) = payment.paid_at {
            let date = paid_at.with_timezone(&Utc).date_naive();
            if let Some(entry) = stats.get_mut(&date) {
                entry.2 += decimal_to_f64(payment.amount);
            }
        }
    }

    let mut items = Vec::new();
    let mut date = start_date;
    while date <= today {
        let (orders_count, dispute_count, revenue) = stats.get(&date).cloned().unwrap_or((0, 0, 0.0));
        items.push(AdminTrendPoint {
            date: date.format("%Y-%m-%d").to_string(),
            orders: orders_count,
            disputes: dispute_count,
            revenue: round_currency(revenue),
        });
        date = date.succ_opt().unwrap_or(today + Duration::days(1));
    }

    Ok(Json(crate::common::ApiResponse::ok(AdminTrendResp { days, items })))
}

pub async fn export_orders_report(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(q): axum::extract::Query<OrdersReportQuery>,
) -> ApiResult<OrderReportResp> {
    ensure_role_access(&state.orm, user_id, &["admin", "ops", "manager"]).await?;

    let limit = q.limit.unwrap_or(500).min(5000);
    let mut query = orders::Entity::find();
    if let Some(status) = &q.status {
        query = query.filter(orders::Column::Status.eq(status));
    }
    if let Some(start) = q.start_date.as_deref() {
        let dt = parse_date(start)?;
        query = query.filter(orders::Column::CreatedAt.gte(dt));
    }
    if let Some(end) = q.end_date.as_deref() {
        let dt = parse_date(end)?;
        query = query.filter(orders::Column::CreatedAt.lt(dt + Duration::days(1)));
    }

    let total = query.clone().count(&state.orm).await?;
    let rows = query
        .order_by_desc(orders::Column::CreatedAt)
        .limit(limit)
        .all(&state.orm)
        .await?;

    let items: Vec<OrderReportItem> = rows
        .into_iter()
        .map(|o| OrderReportItem {
            id: o.id,
            user_id: o.user_id,
            photographer_id: o.photographer_id,
            status: o.status,
            pay_type: o.pay_type,
            total_amount: decimal_to_f64(o.total_amount),
            created_at: o.created_at.to_rfc3339(),
        })
        .collect();

    let format = q.format.unwrap_or_else(|| "json".to_string());
    let csv = if format == "csv" {
        Some(render_orders_csv(&items))
    } else {
        None
    };

    Ok(Json(crate::common::ApiResponse::ok(OrderReportResp {
        format,
        generated_at: Utc::now().to_rfc3339(),
        total,
        items,
        csv,
    })))
}

#[derive(Deserialize)]
pub struct AdminUserListQuery {
    role: Option<String>,
    status: Option<String>,
    keyword: Option<String>,
    page: Option<u64>,
    page_size: Option<u64>,
}

#[derive(Serialize)]
pub struct AdminUserListItem {
    id: i64,
    phone: String,
    status: String,
    role: String,
    nickname: Option<String>,
    city_id: Option<i64>,
    updated_at: String,
    photographer_id: Option<i64>,
    photographer_status: Option<String>,
}

pub async fn list_admin_users(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(q): axum::extract::Query<AdminUserListQuery>,
) -> ApiResult<Paged<AdminUserListItem>> {
    ensure_role_access(&state.orm, user_id, &["admin", "ops", "manager"]).await?;

    let (page, page_size) = normalize_pagination(q.page, q.page_size);
    let offset = (page - 1) * page_size;

    let mut cond = Condition::all();
    if let Some(status) = &q.status {
        cond = cond.add(users::Column::Status.eq(status));
    }

    if let Some(keyword) = &q.keyword {
        let like = format!("%{}%", keyword);
        let mut keyword_cond = Condition::any().add(users::Column::Phone.like(like.clone()));
        let nickname_ids = user_profiles::Entity::find()
            .filter(user_profiles::Column::Nickname.like(like))
            .select_only()
            .column(user_profiles::Column::UserId)
            .into_tuple::<i64>()
            .all(&state.orm)
            .await?;
        if !nickname_ids.is_empty() {
            keyword_cond = keyword_cond.add(users::Column::Id.is_in(nickname_ids));
        }
        cond = cond.add(keyword_cond);
    }

    let role_filter = q.role.clone();
    if let Some(role) = q.role.as_deref() {
        let user_ids = match role {
            "photographer" => load_photographer_user_ids(&state.orm).await?,
            "merchant" => load_merchant_user_ids(&state.orm).await?,
            "user" => HashSet::new(),
            _ => return Err(ApiError::bad_request("invalid_role")),
        };
        if role != "user" && user_ids.is_empty() {
            return Ok(Json(crate::common::ApiResponse::ok(Paged {
                items: Vec::new(),
                page,
                page_size,
                total: 0,
            })));
        }
        if role != "user" {
            cond = cond.add(users::Column::Id.is_in(user_ids.into_iter().collect::<Vec<_>>()));
        } else {
            let photographer_ids = load_photographer_user_ids(&state.orm).await?;
            let merchant_ids = load_merchant_user_ids(&state.orm).await?;
            let mut exclude_ids: Vec<i64> = photographer_ids.into_iter().collect();
            exclude_ids.extend(merchant_ids);
            if !exclude_ids.is_empty() {
                cond = cond.add(users::Column::Id.is_not_in(exclude_ids));
            }
        }
    }

    let total = users::Entity::find()
        .filter(cond.clone())
        .count(&state.orm)
        .await?;

    if total == 0 {
        return Ok(Json(crate::common::ApiResponse::ok(Paged {
            items: Vec::new(),
            page,
            page_size,
            total,
        })));
    }

    let rows = users::Entity::find()
        .filter(cond)
        .order_by_desc(users::Column::UpdatedAt)
        .limit(page_size)
        .offset(offset)
        .all(&state.orm)
        .await?;

    if rows.is_empty() {
        return Ok(Json(crate::common::ApiResponse::ok(Paged {
            items: Vec::new(),
            page,
            page_size,
            total,
        })));
    }

    let ids: Vec<i64> = rows.iter().map(|u| u.id).collect();
    let profiles = user_profiles::Entity::find()
        .filter(user_profiles::Column::UserId.is_in(ids.clone()))
        .all(&state.orm)
        .await?;
    let profile_map: HashMap<i64, user_profiles::Model> =
        profiles.into_iter().map(|p| (p.user_id, p)).collect();

    let photographer_rows = photographers::Entity::find()
        .filter(photographers::Column::UserId.is_in(ids.clone()))
        .all(&state.orm)
        .await?;
    let photographer_set: HashSet<i64> = photographer_rows.iter().map(|p| p.user_id).collect();
    let photographer_map: HashMap<i64, (i64, String)> = photographer_rows
        .into_iter()
        .map(|p| (p.user_id, (p.id, p.status)))
        .collect();

    let merchant_set = load_merchant_user_ids_by_ids(&state.orm, &ids).await?;

    let items: Vec<AdminUserListItem> = rows
        .into_iter()
        .filter_map(|u| {
            let profile = profile_map.get(&u.id);
            let role = if photographer_set.contains(&u.id) {
                "photographer"
            } else if merchant_set.contains(&u.id) {
                "merchant"
            } else {
                "user"
            };
            if let Some(filter) = role_filter.as_deref()
                && filter != "all"
                && filter != role
            {
                return None;
            }
            let photographer_info = photographer_map.get(&u.id);
            Some(AdminUserListItem {
                id: u.id,
                phone: u.phone,
                status: u.status,
                role: role.to_string(),
                nickname: profile.and_then(|p| p.nickname.clone()),
                city_id: profile.and_then(|p| p.city_id),
                updated_at: u.updated_at.to_rfc3339(),
                photographer_id: photographer_info.map(|(id, _)| *id),
                photographer_status: photographer_info.map(|(_, status)| status.clone()),
            })
        })
        .collect();

    Ok(Json(crate::common::ApiResponse::ok(Paged {
        items,
        page,
        page_size,
        total,
    })))
}

#[derive(Deserialize)]
pub struct AdminOrderListQuery {
    status: Option<String>,
    page: Option<u64>,
    page_size: Option<u64>,
}

#[derive(Serialize)]
pub struct AdminOrderListItem {
    id: i64,
    user_id: i64,
    user_phone: Option<String>,
    photographer_id: Option<i64>,
    photographer_phone: Option<String>,
    status: String,
    pay_type: String,
    total_amount: f64,
    created_at: String,
}

pub async fn list_admin_orders(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(q): axum::extract::Query<AdminOrderListQuery>,
) -> ApiResult<Paged<AdminOrderListItem>> {
    ensure_role_access(&state.orm, user_id, &["admin", "ops", "manager"]).await?;

    let (page, page_size) = normalize_pagination(q.page, q.page_size);
    let offset = (page - 1) * page_size;

    let mut query = orders::Entity::find();
    if let Some(status) = &q.status {
        query = query.filter(orders::Column::Status.eq(status));
    }

    let total = query.clone().count(&state.orm).await?;
    if total == 0 {
        return Ok(Json(crate::common::ApiResponse::ok(Paged {
            items: Vec::new(),
            page,
            page_size,
            total,
        })));
    }

    let rows = query
        .order_by_desc(orders::Column::CreatedAt)
        .limit(page_size)
        .offset(offset)
        .all(&state.orm)
        .await?;

    if rows.is_empty() {
        return Ok(Json(crate::common::ApiResponse::ok(Paged {
            items: Vec::new(),
            page,
            page_size,
            total,
        })));
    }

    let user_ids: HashSet<i64> = rows.iter().map(|o| o.user_id).collect();
    let photographer_ids: Vec<i64> = rows.iter().filter_map(|o| o.photographer_id).collect();

    let photographer_rows = if photographer_ids.is_empty() {
        Vec::new()
    } else {
        photographers::Entity::find()
            .filter(photographers::Column::Id.is_in(photographer_ids.clone()))
            .all(&state.orm)
            .await?
    };
    let photographer_user_map: HashMap<i64, i64> = photographer_rows
        .into_iter()
        .map(|p| (p.id, p.user_id))
        .collect();

    let mut all_user_ids: HashSet<i64> = user_ids.clone();
    for uid in photographer_user_map.values() {
        all_user_ids.insert(*uid);
    }

    let user_rows = users::Entity::find()
        .filter(users::Column::Id.is_in(all_user_ids.into_iter().collect::<Vec<_>>()))
        .all(&state.orm)
        .await?;
    let user_phone_map: HashMap<i64, String> = user_rows
        .into_iter()
        .map(|u| (u.id, u.phone))
        .collect();

    let items: Vec<AdminOrderListItem> = rows
        .into_iter()
        .map(|o| {
            let photographer_phone = o
                .photographer_id
                .and_then(|pid| photographer_user_map.get(&pid))
                .and_then(|uid| user_phone_map.get(uid))
                .cloned();
            AdminOrderListItem {
                id: o.id,
                user_id: o.user_id,
                user_phone: user_phone_map.get(&o.user_id).cloned(),
                photographer_id: o.photographer_id,
                photographer_phone,
                status: o.status,
                pay_type: o.pay_type,
                total_amount: decimal_to_f64(o.total_amount),
                created_at: o.created_at.to_rfc3339(),
            }
        })
        .collect();

    Ok(Json(crate::common::ApiResponse::ok(Paged {
        items,
        page,
        page_size,
        total,
    })))
}

#[derive(Deserialize)]
pub struct AdminDisputeListQuery {
    status: Option<String>,
    page: Option<u64>,
    page_size: Option<u64>,
}

#[derive(Serialize)]
pub struct AdminDisputeListItem {
    id: i64,
    order_id: i64,
    initiator_id: i64,
    initiator_phone: Option<String>,
    status: String,
    reason: Option<String>,
    updated_at: String,
}

pub async fn list_admin_disputes(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(q): axum::extract::Query<AdminDisputeListQuery>,
) -> ApiResult<Paged<AdminDisputeListItem>> {
    ensure_role_access(&state.orm, user_id, &["admin", "ops", "manager"]).await?;

    let (page, page_size) = normalize_pagination(q.page, q.page_size);
    let offset = (page - 1) * page_size;

    let mut query = disputes::Entity::find();
    if let Some(status) = &q.status {
        query = query.filter(disputes::Column::Status.eq(status));
    }

    let total = query.clone().count(&state.orm).await?;
    if total == 0 {
        return Ok(Json(crate::common::ApiResponse::ok(Paged {
            items: Vec::new(),
            page,
            page_size,
            total,
        })));
    }

    let rows = query
        .order_by_desc(disputes::Column::UpdatedAt)
        .limit(page_size)
        .offset(offset)
        .all(&state.orm)
        .await?;

    if rows.is_empty() {
        return Ok(Json(crate::common::ApiResponse::ok(Paged {
            items: Vec::new(),
            page,
            page_size,
            total,
        })));
    }

    let initiator_ids: Vec<i64> = rows.iter().map(|d| d.initiator_id).collect();
    let initiators = users::Entity::find()
        .filter(users::Column::Id.is_in(initiator_ids))
        .all(&state.orm)
        .await?;
    let initiator_map: HashMap<i64, String> =
        initiators.into_iter().map(|u| (u.id, u.phone)).collect();

    let items: Vec<AdminDisputeListItem> = rows
        .into_iter()
        .map(|d| AdminDisputeListItem {
            id: d.id,
            order_id: d.order_id,
            initiator_id: d.initiator_id,
            initiator_phone: initiator_map.get(&d.initiator_id).cloned(),
            status: d.status,
            reason: d.reason,
            updated_at: d.updated_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(crate::common::ApiResponse::ok(Paged {
        items,
        page,
        page_size,
        total,
    })))
}

#[derive(Deserialize)]
pub struct AdminPortfolioListQuery {
    status: Option<String>,
    photographer_id: Option<i64>,
    page: Option<u64>,
    page_size: Option<u64>,
}

#[derive(Serialize)]
pub struct AdminPortfolioListItem {
    id: i64,
    photographer_id: i64,
    photographer_phone: Option<String>,
    title: String,
    status: String,
    created_at: String,
    updated_at: String,
}

pub async fn list_admin_portfolios(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(q): axum::extract::Query<AdminPortfolioListQuery>,
) -> ApiResult<Paged<AdminPortfolioListItem>> {
    ensure_role_access(&state.orm, user_id, &["admin", "ops"]).await?;

    let (page, page_size) = normalize_pagination(q.page, q.page_size);
    let offset = (page - 1) * page_size;

    let mut query = portfolios::Entity::find();
    if let Some(status) = &q.status {
        query = query.filter(portfolios::Column::Status.eq(status));
    }
    if let Some(photographer_id) = q.photographer_id {
        query = query.filter(portfolios::Column::PhotographerId.eq(photographer_id));
    }

    let total = query.clone().count(&state.orm).await?;
    if total == 0 {
        return Ok(Json(crate::common::ApiResponse::ok(Paged {
            items: Vec::new(),
            page,
            page_size,
            total,
        })));
    }

    let rows = query
        .order_by_desc(portfolios::Column::CreatedAt)
        .limit(page_size)
        .offset(offset)
        .all(&state.orm)
        .await?;

    if rows.is_empty() {
        return Ok(Json(crate::common::ApiResponse::ok(Paged {
            items: Vec::new(),
            page,
            page_size,
            total,
        })));
    }

    let photographer_ids: Vec<i64> = rows.iter().map(|p| p.photographer_id).collect();
    let photographer_rows = photographers::Entity::find()
        .filter(photographers::Column::Id.is_in(photographer_ids.clone()))
        .all(&state.orm)
        .await?;
    let photographer_user_map: HashMap<i64, i64> = photographer_rows
        .into_iter()
        .map(|p| (p.id, p.user_id))
        .collect();

    let user_ids: Vec<i64> = photographer_user_map.values().cloned().collect();
    let user_rows = if user_ids.is_empty() {
        Vec::new()
    } else {
        users::Entity::find()
            .filter(users::Column::Id.is_in(user_ids))
            .all(&state.orm)
            .await?
    };
    let user_phone_map: HashMap<i64, String> = user_rows
        .into_iter()
        .map(|u| (u.id, u.phone))
        .collect();

    let items: Vec<AdminPortfolioListItem> = rows
        .into_iter()
        .map(|p| {
            let photographer_phone = photographer_user_map
                .get(&p.photographer_id)
                .and_then(|uid| user_phone_map.get(uid))
                .cloned();
            AdminPortfolioListItem {
                id: p.id,
                photographer_id: p.photographer_id,
                photographer_phone,
                title: p.title,
                status: p.status,
                created_at: p.created_at.to_rfc3339(),
                updated_at: p.updated_at.to_rfc3339(),
            }
        })
        .collect();

    Ok(Json(crate::common::ApiResponse::ok(Paged {
        items,
        page,
        page_size,
        total,
    })))
}

pub async fn ensure_role_access(
    db: &sea_orm::DatabaseConnection,
    user_id: i64,
    allowed: &[&str],
) -> Result<(), ApiError> {
    let user = users::Entity::find_by_id(user_id)
        .one(db)
        .await?
        .ok_or_else(ApiError::unauthorized)?;
    let roles = resolve_roles(&user.phone);
    if roles.iter().any(|role| allowed.contains(&role.as_str())) {
        Ok(())
    } else {
        Err(ApiError::forbidden())
    }
}

fn resolve_roles(phone: &str) -> Vec<String> {
    let mut roles = Vec::new();
    if env_list_contains("ADMIN_PHONES", phone) {
        roles.push("admin".to_string());
    }
    if env_list_contains("OPS_PHONES", phone) {
        roles.push("ops".to_string());
    }
    if env_list_contains("MANAGER_PHONES", phone) {
        roles.push("manager".to_string());
    }
    if roles.is_empty() {
        roles.push("user".to_string());
    }
    roles
}

fn env_list_contains(key: &str, phone: &str) -> bool {
    std::env::var(key)
        .ok()
        .map(|v| {
            v.split(',')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .any(|s| s == phone)
        })
        .unwrap_or(false)
}

fn normalize_days(input: Option<i64>, default: i64, max: i64) -> i64 {
    let value = input.unwrap_or(default);
    let value = if value <= 0 { default } else { value };
    value.min(max)
}

fn day_start(date: NaiveDate) -> chrono::DateTime<Utc> {
    let naive = date.and_hms_opt(0, 0, 0).unwrap();
    Utc.from_utc_datetime(&naive)
}

fn parse_date(input: &str) -> Result<chrono::DateTime<Utc>, ApiError> {
    let date = NaiveDate::parse_from_str(input, "%Y-%m-%d")
        .map_err(|_| ApiError::bad_request("invalid_date"))?;
    Ok(day_start(date))
}

fn dispute_open_condition() -> Condition {
    Condition::all()
        .add(disputes::Column::Status.ne("resolved"))
        .add(disputes::Column::Status.ne("rejected"))
}

fn round_currency(value: f64) -> f64 {
    (value * 100.0).round() / 100.0
}

fn render_orders_csv(items: &[OrderReportItem]) -> String {
    let mut out = String::from("id,user_id,photographer_id,status,pay_type,total_amount,created_at\n");
    for item in items {
        let photographer = item.photographer_id.map(|v| v.to_string()).unwrap_or_default();
        let row = [
            item.id.to_string(),
            item.user_id.to_string(),
            photographer,
            item.status.clone(),
            item.pay_type.clone(),
            item.total_amount.to_string(),
            item.created_at.clone(),
        ];
        out.push_str(&row.iter().map(|v| escape_csv(v)).collect::<Vec<_>>().join(","));
        out.push('\n');
    }
    out
}

fn escape_csv(input: &str) -> String {
    if input.contains(',') || input.contains('"') || input.contains('\n') {
        let escaped = input.replace('"', "\"\"");
        format!("\"{}\"", escaped)
    } else {
        input.to_string()
    }
}

pub async fn load_photographer_user_ids(
    db: &sea_orm::DatabaseConnection,
) -> Result<HashSet<i64>, ApiError> {
    let ids = photographers::Entity::find()
        .select_only()
        .column(photographers::Column::UserId)
        .into_tuple::<i64>()
        .all(db)
        .await?;
    Ok(ids.into_iter().collect())
}

pub async fn load_merchant_user_ids(
    db: &sea_orm::DatabaseConnection,
) -> Result<HashSet<i64>, ApiError> {
    let contact_ids = merchants::Entity::find()
        .select_only()
        .column(merchants::Column::ContactUserId)
        .into_tuple::<Option<i64>>()
        .all(db)
        .await?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    let member_ids = merchant_users::Entity::find()
        .select_only()
        .column(merchant_users::Column::UserId)
        .into_tuple::<i64>()
        .all(db)
        .await?;

    let mut set: HashSet<i64> = contact_ids.into_iter().collect();
    set.extend(member_ids);
    Ok(set)
}

pub async fn load_merchant_user_ids_by_ids(
    db: &sea_orm::DatabaseConnection,
    ids: &[i64],
) -> Result<HashSet<i64>, ApiError> {
    let contact_ids = merchants::Entity::find()
        .filter(merchants::Column::ContactUserId.is_in(ids.to_vec()))
        .select_only()
        .column(merchants::Column::ContactUserId)
        .into_tuple::<Option<i64>>()
        .all(db)
        .await?
        .into_iter()
        .flatten()
        .collect::<Vec<_>>();

    let member_ids = merchant_users::Entity::find()
        .filter(merchant_users::Column::UserId.is_in(ids.to_vec()))
        .select_only()
        .column(merchant_users::Column::UserId)
        .into_tuple::<i64>()
        .all(db)
        .await?;

    let mut set: HashSet<i64> = contact_ids.into_iter().collect();
    set.extend(member_ids);
    Ok(set)
}

fn decimal_to_f64(v: sea_orm::prelude::Decimal) -> f64 {
    v.to_string().parse::<f64>().unwrap_or(0.0)
}

#[derive(Serialize)]
pub struct AdminOrderItemResp {
    name: String,
    price: f64,
    quantity: i32,
}

#[derive(Serialize)]
pub struct AdminPaymentResp {
    id: i64,
    amount: f64,
    status: String,
    pay_channel: String,
    paid_at: Option<String>,
    proof_url: Option<String>,
}

#[derive(Serialize)]
pub struct AdminRefundResp {
    id: i64,
    amount: f64,
    status: String,
    reason: Option<String>,
    proof_url: Option<String>,
    created_at: String,
}

#[derive(Serialize)]
pub struct AdminDeliveryItemResp {
    id: i64,
    file_url: String,
    version: Option<String>,
    note: Option<String>,
}

#[derive(Serialize)]
pub struct AdminDeliveryResp {
    id: i64,
    status: String,
    submitted_at: Option<String>,
    accepted_at: Option<String>,
    items: Vec<AdminDeliveryItemResp>,
}

#[derive(Serialize)]
pub struct AdminReviewResp {
    score: i32,
    tags: Option<Vec<String>>,
    comment: Option<String>,
    created_at: String,
}

#[derive(Serialize)]
pub struct AdminOrderDetailResp {
    id: i64,
    status: String,
    pay_type: String,
    total_amount: f64,
    deposit_amount: f64,
    service_fee: f64,
    schedule_start: Option<String>,
    schedule_end: Option<String>,
    created_at: String,
    updated_at: String,
    user_id: i64,
    user_phone: Option<String>,
    photographer_id: Option<i64>,
    photographer_phone: Option<String>,
    items: Vec<AdminOrderItemResp>,
    payments: Vec<AdminPaymentResp>,
    refunds: Vec<AdminRefundResp>,
    deliveries: Vec<AdminDeliveryResp>,
    review: Option<AdminReviewResp>,
}

pub async fn get_admin_order_detail(
    AuthUser { user_id }: AuthUser,
    axum::extract::Path(order_id): axum::extract::Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<AdminOrderDetailResp> {
    ensure_role_access(&state.orm, user_id, &["admin", "ops", "manager"]).await?;

    let order = orders::Entity::find_by_id(order_id)
        .one(&state.orm)
        .await?
        .ok_or_else(ApiError::not_found)?;

    let user_phone = users::Entity::find_by_id(order.user_id)
        .one(&state.orm)
        .await?
        .map(|u| u.phone);

    let (photographer_id, photographer_phone) = if let Some(pid) = order.photographer_id {
        let photographer = photographers::Entity::find_by_id(pid)
            .one(&state.orm)
            .await?;
        let phone = if let Some(p) = photographer {
            users::Entity::find_by_id(p.user_id)
                .one(&state.orm)
                .await?
                .map(|u| u.phone)
        } else {
            None
        };
        (Some(pid), phone)
    } else {
        (None, None)
    };

    let items = order_items::Entity::find()
        .filter(order_items::Column::OrderId.eq(order_id))
        .all(&state.orm)
        .await?
        .into_iter()
        .map(|it| AdminOrderItemResp {
            name: it.name,
            price: decimal_to_f64(it.price),
            quantity: it.quantity,
        })
        .collect::<Vec<_>>();

    let payments = payments::Entity::find()
        .filter(payments::Column::OrderId.eq(order_id))
        .order_by_desc(payments::Column::CreatedAt)
        .all(&state.orm)
        .await?
        .into_iter()
        .map(|p| AdminPaymentResp {
            id: p.id,
            amount: decimal_to_f64(p.amount),
            status: p.status,
            pay_channel: p.pay_channel,
            paid_at: p.paid_at.map(|d| d.to_rfc3339()),
            proof_url: p.proof_url,
        })
        .collect::<Vec<_>>();

    let refunds = refunds::Entity::find()
        .filter(refunds::Column::OrderId.eq(order_id))
        .order_by_desc(refunds::Column::CreatedAt)
        .all(&state.orm)
        .await?
        .into_iter()
        .map(|r| AdminRefundResp {
            id: r.id,
            amount: decimal_to_f64(r.amount),
            status: r.status,
            reason: r.reason,
            proof_url: r.proof_url,
            created_at: r.created_at.to_rfc3339(),
        })
        .collect::<Vec<_>>();

    let delivery_rows = deliveries::Entity::find()
        .filter(deliveries::Column::OrderId.eq(order_id))
        .all(&state.orm)
        .await?;
    let delivery_ids: Vec<i64> = delivery_rows.iter().map(|d| d.id).collect();
    let delivery_items_rows = if delivery_ids.is_empty() {
        Vec::new()
    } else {
        delivery_items::Entity::find()
            .filter(delivery_items::Column::DeliveryId.is_in(delivery_ids.clone()))
            .all(&state.orm)
            .await?
    };
    let mut delivery_item_map: HashMap<i64, Vec<AdminDeliveryItemResp>> = HashMap::new();
    for item in delivery_items_rows {
        delivery_item_map
            .entry(item.delivery_id)
            .or_default()
            .push(AdminDeliveryItemResp {
                id: item.id,
                file_url: item.file_url,
                version: item.version,
                note: item.note,
            });
    }
    let deliveries = delivery_rows
        .into_iter()
        .map(|d| AdminDeliveryResp {
            id: d.id,
            status: d.status,
            submitted_at: d.submitted_at.map(|t| t.to_rfc3339()),
            accepted_at: d.accepted_at.map(|t| t.to_rfc3339()),
            items: delivery_item_map.remove(&d.id).unwrap_or_default(),
        })
        .collect::<Vec<_>>();

    let review = reviews::Entity::find()
        .filter(reviews::Column::OrderId.eq(order_id))
        .one(&state.orm)
        .await?
        .map(|r| AdminReviewResp {
            score: r.score,
            tags: r.tags.and_then(|v| serde_json::from_value(v).ok()),
            comment: r.comment,
            created_at: r.created_at.to_rfc3339(),
        });

    Ok(Json(crate::common::ApiResponse::ok(AdminOrderDetailResp {
        id: order.id,
        status: order.status,
        pay_type: order.pay_type,
        total_amount: decimal_to_f64(order.total_amount),
        deposit_amount: decimal_to_f64(order.deposit_amount),
        service_fee: decimal_to_f64(order.service_fee),
        schedule_start: order.schedule_start.map(|d| d.to_rfc3339()),
        schedule_end: order.schedule_end.map(|d| d.to_rfc3339()),
        created_at: order.created_at.to_rfc3339(),
        updated_at: order.updated_at.to_rfc3339(),
        user_id: order.user_id,
        user_phone,
        photographer_id,
        photographer_phone,
        items,
        payments,
        refunds,
        deliveries,
        review,
    })))
}

#[derive(Serialize)]
pub struct AdminDisputeEvidenceResp {
    id: i64,
    file_url: String,
    note: Option<String>,
    created_at: String,
}

#[derive(Serialize)]
pub struct AdminDisputeDetailResp {
    id: i64,
    order_id: i64,
    order_status: Option<String>,
    initiator_id: i64,
    initiator_phone: Option<String>,
    status: String,
    reason: Option<String>,
    resolution: Option<String>,
    created_at: String,
    updated_at: String,
    evidence: Vec<AdminDisputeEvidenceResp>,
}

pub async fn get_admin_dispute_detail(
    AuthUser { user_id }: AuthUser,
    axum::extract::Path(dispute_id): axum::extract::Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<AdminDisputeDetailResp> {
    ensure_role_access(&state.orm, user_id, &["admin", "ops", "manager"]).await?;

    let dispute = disputes::Entity::find_by_id(dispute_id)
        .one(&state.orm)
        .await?
        .ok_or_else(ApiError::not_found)?;

    let order_status = orders::Entity::find_by_id(dispute.order_id)
        .one(&state.orm)
        .await?
        .map(|o| o.status);

    let initiator_phone = users::Entity::find_by_id(dispute.initiator_id)
        .one(&state.orm)
        .await?
        .map(|u| u.phone);

    let evidence = dispute_evidence::Entity::find()
        .filter(dispute_evidence::Column::DisputeId.eq(dispute_id))
        .order_by_desc(dispute_evidence::Column::CreatedAt)
        .all(&state.orm)
        .await?
        .into_iter()
        .map(|e| AdminDisputeEvidenceResp {
            id: e.id,
            file_url: e.file_url,
            note: e.note,
            created_at: e.created_at.to_rfc3339(),
        })
        .collect::<Vec<_>>();

    Ok(Json(crate::common::ApiResponse::ok(AdminDisputeDetailResp {
        id: dispute.id,
        order_id: dispute.order_id,
        order_status,
        initiator_id: dispute.initiator_id,
        initiator_phone,
        status: dispute.status,
        reason: dispute.reason,
        resolution: dispute.resolution,
        created_at: dispute.created_at.to_rfc3339(),
        updated_at: dispute.updated_at.to_rfc3339(),
        evidence,
    })))
}

#[derive(Deserialize)]
pub struct MerchantApprovalListQuery {
    status: Option<String>,
    page: Option<u64>,
    page_size: Option<u64>,
}

#[derive(Serialize)]
pub struct MerchantApprovalListItem {
    id: i64,
    merchant_id: i64,
    merchant_name: String,
    demand_id: i64,
    status: String,
    comment: Option<String>,
    created_at: String,
}

pub async fn list_merchant_approvals(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(q): axum::extract::Query<MerchantApprovalListQuery>,
) -> ApiResult<Paged<MerchantApprovalListItem>> {
    ensure_role_access(&state.orm, user_id, &["admin", "ops"]).await?;

    let (page, page_size) = normalize_pagination(q.page, q.page_size);
    let offset = (page - 1) * page_size;

    let mut query = merchant_approvals::Entity::find();
    if let Some(status) = &q.status {
        query = query.filter(merchant_approvals::Column::Status.eq(status));
    }

    let total = query.clone().count(&state.orm).await?;
    if total == 0 {
        return Ok(Json(crate::common::ApiResponse::ok(Paged {
            items: Vec::new(),
            page,
            page_size,
            total,
        })));
    }

    let rows = query
        .order_by_desc(merchant_approvals::Column::CreatedAt)
        .limit(page_size)
        .offset(offset)
        .all(&state.orm)
        .await?;

    let merchant_ids: Vec<i64> = rows.iter().map(|r| r.merchant_id).collect();
    let merchant_rows = merchants::Entity::find()
        .filter(merchants::Column::Id.is_in(merchant_ids))
        .all(&state.orm)
        .await?;
    let merchant_map: HashMap<i64, String> =
        merchant_rows.into_iter().map(|m| (m.id, m.name)).collect();

    let items = rows
        .into_iter()
        .map(|r| MerchantApprovalListItem {
            id: r.id,
            merchant_id: r.merchant_id,
            merchant_name: merchant_map.get(&r.merchant_id).cloned().unwrap_or_default(),
            demand_id: r.demand_id,
            status: r.status,
            comment: r.comment,
            created_at: r.created_at.to_rfc3339(),
        })
        .collect();

    Ok(Json(crate::common::ApiResponse::ok(Paged {
        items,
        page,
        page_size,
        total,
    })))
}

#[derive(Deserialize)]
pub struct ReviewMerchantApprovalReq {
    status: String,
    comment: Option<String>,
}

#[derive(Serialize)]
pub struct ReviewMerchantApprovalResp {
    id: i64,
    status: String,
}

pub async fn review_merchant_approval(
    AuthUser { user_id }: AuthUser,
    axum::extract::Path(approval_id): axum::extract::Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<ReviewMerchantApprovalReq>,
) -> ApiResult<ReviewMerchantApprovalResp> {
    ensure_role_access(&state.orm, user_id, &["admin", "ops"]).await?;
    if !matches!(req.status.as_str(), "approved" | "rejected") {
        return Err(ApiError::bad_request("invalid_status"));
    }

    let approval = merchant_approvals::Entity::find_by_id(approval_id)
        .one(&state.orm)
        .await?
        .ok_or_else(ApiError::not_found)?;

    let mut model: merchant_approvals::ActiveModel = approval.into();
    model.status = Set(req.status.clone());
    model.approver_id = Set(Some(user_id));
    model.comment = Set(req.comment.clone());
    let updated = model.update(&state.orm).await?;

    let audit = audit_logs::ActiveModel {
        admin_id: Set(user_id),
        action: Set("merchant_approval_review".to_string()),
        target_type: Set(Some("merchant_approval".to_string())),
        target_id: Set(Some(approval_id)),
        detail: Set(Some(json!({ "status": req.status, "comment": req.comment }))),
        ..Default::default()
    };
    audit.insert(&state.orm).await?;

    Ok(Json(crate::common::ApiResponse::ok(ReviewMerchantApprovalResp {
        id: updated.id,
        status: updated.status,
    })))
}

#[derive(Deserialize)]
pub struct MerchantTemplateListQuery {
    merchant_id: Option<i64>,
    page: Option<u64>,
    page_size: Option<u64>,
}

#[derive(Serialize)]
pub struct MerchantTemplateItemResp {
    name: String,
    quantity: i32,
    price: f64,
}

#[derive(Serialize)]
pub struct MerchantTemplateListItem {
    id: i64,
    merchant_id: i64,
    merchant_name: String,
    name: String,
    description: Option<String>,
    created_at: String,
    items: Vec<MerchantTemplateItemResp>,
}

pub async fn list_merchant_templates(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(q): axum::extract::Query<MerchantTemplateListQuery>,
) -> ApiResult<Paged<MerchantTemplateListItem>> {
    ensure_role_access(&state.orm, user_id, &["admin", "ops"]).await?;

    let (page, page_size) = normalize_pagination(q.page, q.page_size);
    let offset = (page - 1) * page_size;

    let mut query = merchant_templates::Entity::find();
    if let Some(merchant_id) = q.merchant_id {
        query = query.filter(merchant_templates::Column::MerchantId.eq(merchant_id));
    }

    let total = query.clone().count(&state.orm).await?;
    if total == 0 {
        return Ok(Json(crate::common::ApiResponse::ok(Paged {
            items: Vec::new(),
            page,
            page_size,
            total,
        })));
    }

    let rows = query
        .order_by_desc(merchant_templates::Column::CreatedAt)
        .limit(page_size)
        .offset(offset)
        .all(&state.orm)
        .await?;

    let merchant_ids: Vec<i64> = rows.iter().map(|t| t.merchant_id).collect();
    let merchant_rows = merchants::Entity::find()
        .filter(merchants::Column::Id.is_in(merchant_ids))
        .all(&state.orm)
        .await?;
    let merchant_map: HashMap<i64, String> =
        merchant_rows.into_iter().map(|m| (m.id, m.name)).collect();

    let template_ids: Vec<i64> = rows.iter().map(|t| t.id).collect();
    let item_rows = merchant_template_items::Entity::find()
        .filter(merchant_template_items::Column::TemplateId.is_in(template_ids.clone()))
        .all(&state.orm)
        .await?;
    let mut items_map: HashMap<i64, Vec<MerchantTemplateItemResp>> = HashMap::new();
    for item in item_rows {
        items_map
            .entry(item.template_id)
            .or_default()
            .push(MerchantTemplateItemResp {
                name: item.name,
                quantity: item.quantity,
                price: decimal_to_f64(item.price),
            });
    }

    let items = rows
        .into_iter()
        .map(|t| MerchantTemplateListItem {
            id: t.id,
            merchant_id: t.merchant_id,
            merchant_name: merchant_map.get(&t.merchant_id).cloned().unwrap_or_default(),
            name: t.name,
            description: t.description,
            created_at: t.created_at.to_rfc3339(),
            items: items_map.remove(&t.id).unwrap_or_default(),
        })
        .collect();

    Ok(Json(crate::common::ApiResponse::ok(Paged {
        items,
        page,
        page_size,
        total,
    })))
}

#[derive(Deserialize)]
pub struct ReviewPhotographerReq {
    status: String,
    comment: Option<String>,
}

#[derive(Serialize)]
pub struct ReviewPhotographerResp {
    id: i64,
    status: String,
}

pub async fn review_photographer(
    AuthUser { user_id }: AuthUser,
    axum::extract::Path(photographer_id): axum::extract::Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<ReviewPhotographerReq>,
) -> ApiResult<ReviewPhotographerResp> {
    ensure_role_access(&state.orm, user_id, &["admin"]).await?;
    if !matches!(req.status.as_str(), "approved" | "rejected") {
        return Err(ApiError::bad_request("invalid_status"));
    }

    let photographer = photographers::Entity::find_by_id(photographer_id)
        .one(&state.orm)
        .await?
        .ok_or_else(ApiError::not_found)?;

    let mut model: photographers::ActiveModel = photographer.into();
    model.status = Set(req.status.clone());
    let updated = model.update(&state.orm).await?;

    let audit = audit_logs::ActiveModel {
        admin_id: Set(user_id),
        action: Set("photographer_review".to_string()),
        target_type: Set(Some("photographer".to_string())),
        target_id: Set(Some(photographer_id)),
        detail: Set(Some(json!({ "status": req.status, "comment": req.comment }))),
        ..Default::default()
    };
    audit.insert(&state.orm).await?;

    Ok(Json(crate::common::ApiResponse::ok(ReviewPhotographerResp {
        id: updated.id,
        status: updated.status,
    })))
}

#[derive(Deserialize)]
pub struct ReviewPortfolioReq {
    status: String,
    comment: Option<String>,
}

#[derive(Serialize)]
pub struct ReviewPortfolioResp {
    id: i64,
    status: String,
}

pub async fn review_portfolio(
    AuthUser { user_id }: AuthUser,
    axum::extract::Path(portfolio_id): axum::extract::Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<ReviewPortfolioReq>,
) -> ApiResult<ReviewPortfolioResp> {
    ensure_role_access(&state.orm, user_id, &["admin", "ops"]).await?;
    if !matches!(req.status.as_str(), "approved" | "rejected") {
        return Err(ApiError::bad_request("invalid_status"));
    }

    let portfolio = portfolios::Entity::find_by_id(portfolio_id)
        .one(&state.orm)
        .await?
        .ok_or_else(ApiError::not_found)?;

    let mut model: portfolios::ActiveModel = portfolio.into();
    model.status = Set(req.status.clone());
    let updated = model.update(&state.orm).await?;

    let audit = audit_logs::ActiveModel {
        admin_id: Set(user_id),
        action: Set("portfolio_review".to_string()),
        target_type: Set(Some("portfolio".to_string())),
        target_id: Set(Some(portfolio_id)),
        detail: Set(Some(json!({ "status": req.status, "comment": req.comment }))),
        ..Default::default()
    };
    audit.insert(&state.orm).await?;

    Ok(Json(crate::common::ApiResponse::ok(ReviewPortfolioResp {
        id: updated.id,
        status: updated.status,
    })))
}

#[derive(Deserialize)]
pub struct FreezeOrderReq {
    reason: Option<String>,
}

#[derive(Serialize)]
pub struct FreezeOrderResp {
    id: i64,
    status: String,
}

pub async fn freeze_order(
    AuthUser { user_id }: AuthUser,
    axum::extract::Path(order_id): axum::extract::Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<FreezeOrderReq>,
) -> ApiResult<FreezeOrderResp> {
    ensure_role_access(&state.orm, user_id, &["admin", "ops"]).await?;

    let order = orders::Entity::find_by_id(order_id)
        .one(&state.orm)
        .await?
        .ok_or_else(ApiError::not_found)?;

    let mut model: orders::ActiveModel = order.into();
    model.status = Set("frozen".to_string());
    let updated = model.update(&state.orm).await?;

    let audit = audit_logs::ActiveModel {
        admin_id: Set(user_id),
        action: Set("order_freeze".to_string()),
        target_type: Set(Some("order".to_string())),
        target_id: Set(Some(order_id)),
        detail: Set(Some(json!({ "reason": req.reason }))),
        ..Default::default()
    };
    audit.insert(&state.orm).await?;

    Ok(Json(crate::common::ApiResponse::ok(FreezeOrderResp {
        id: updated.id,
        status: updated.status,
    })))
}

#[derive(Deserialize)]
pub struct ResolveDisputeReq {
    resolution: String,
    status: Option<String>,
}

#[derive(Serialize)]
pub struct ResolveDisputeResp {
    id: i64,
    status: String,
}

pub async fn resolve_dispute(
    AuthUser { user_id }: AuthUser,
    axum::extract::Path(dispute_id): axum::extract::Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<ResolveDisputeReq>,
) -> ApiResult<ResolveDisputeResp> {
    ensure_role_access(&state.orm, user_id, &["admin", "ops"]).await?;
    if req.resolution.trim().is_empty() {
        return Err(ApiError::bad_request("resolution_required"));
    }
    if let Some(status) = req.status.as_deref()
        && !matches!(status, "resolved" | "rejected" | "processing")
    {
        return Err(ApiError::bad_request("invalid_status"));
    }

    let dispute = disputes::Entity::find_by_id(dispute_id)
        .one(&state.orm)
        .await?
        .ok_or_else(ApiError::not_found)?;

    let mut model: disputes::ActiveModel = dispute.into();
    model.resolution = Set(Some(req.resolution.clone()));
    model.status = Set(req.status.unwrap_or_else(|| "resolved".to_string()));
    let updated = model.update(&state.orm).await?;

    let audit = audit_logs::ActiveModel {
        admin_id: Set(user_id),
        action: Set("dispute_resolve".to_string()),
        target_type: Set(Some("dispute".to_string())),
        target_id: Set(Some(dispute_id)),
        detail: Set(Some(json!({ "status": updated.status, "resolution": req.resolution }))),
        ..Default::default()
    };
    audit.insert(&state.orm).await?;

    Ok(Json(crate::common::ApiResponse::ok(ResolveDisputeResp {
        id: updated.id,
        status: updated.status,
    })))
}
