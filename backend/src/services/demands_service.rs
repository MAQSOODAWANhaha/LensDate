use std::str::FromStr;

use crate::dto::demands::{
    AttachmentResp, CreateDemandReq, DemandDetail, DemandListItem, DemandListQuery,
    DemandMerchantAssetItem, DemandMerchantAssetQuery, DemandResp,
};
use crate::dto::pagination::{normalize_pagination, Paged};
use crate::errors::{DomainError, ServiceResult};
use sea_orm::{ColumnTrait, Condition, EntityTrait, QueryFilter};

use crate::repositories::demands_repo;
use crate::repositories::merchants_repo;
use crate::repositories::teams_repo;
use crate::state::AppState;

pub async fn create_demand(
    state: &AppState,
    user_id: i64,
    req: CreateDemandReq,
) -> ServiceResult<DemandResp> {
    if req.r#type.is_empty() {
        return Err(DomainError::BadRequest("invalid_type".to_string()).into());
    }
    if req.is_merchant.unwrap_or(false) && req.merchant_id.is_none() {
        return Err(DomainError::BadRequest("merchant_id_required".to_string()).into());
    }

    let start = parse_datetime(&req.schedule_start)?;
    let end = parse_datetime(&req.schedule_end)?;
    if start >= end {
        return Err(DomainError::BadRequest("invalid_schedule".to_string()).into());
    }

    let model = crate::entity::demands::ActiveModel {
        user_id: sea_orm::ActiveValue::Set(user_id),
        r#type: sea_orm::ActiveValue::Set(req.r#type),
        city_id: sea_orm::ActiveValue::Set(Some(req.city_id)),
        location: sea_orm::ActiveValue::Set(req.location),
        schedule_start: sea_orm::ActiveValue::Set(Some(start.into())),
        schedule_end: sea_orm::ActiveValue::Set(Some(end.into())),
        budget_min: sea_orm::ActiveValue::Set(req.budget_min.map(decimal_from_f64)),
        budget_max: sea_orm::ActiveValue::Set(req.budget_max.map(decimal_from_f64)),
        people_count: sea_orm::ActiveValue::Set(req.people_count),
        style_tags: sea_orm::ActiveValue::Set(req.style_tags.map(|v| serde_json::json!(v))),
        status: sea_orm::ActiveValue::Set("open".to_string()),
        is_merchant: sea_orm::ActiveValue::Set(req.is_merchant.unwrap_or(false)),
        merchant_id: sea_orm::ActiveValue::Set(req.merchant_id),
        ..Default::default()
    };

    let inserted = demands_repo::create_demand(&state.orm, model).await?;

    if let Some(attachments) = req.attachments {
        for att in attachments {
            let a = crate::entity::demand_attachments::ActiveModel {
                demand_id: sea_orm::ActiveValue::Set(inserted.id),
                file_url: sea_orm::ActiveValue::Set(att.file_url),
                file_type: sea_orm::ActiveValue::Set(att.file_type),
                ..Default::default()
            };
            demands_repo::create_attachment(&state.orm, a).await?;
        }
    }

    Ok(DemandResp {
        id: inserted.id,
        user_id: inserted.user_id,
        r#type: inserted.r#type,
        status: inserted.status,
    })
}

pub async fn list_demands(
    state: &AppState,
    user_id: i64,
    query: DemandListQuery,
) -> ServiceResult<Paged<DemandListItem>> {
    let (page, page_size) = normalize_pagination(query.page, query.page_size);

    let schedule_start = match query.schedule_start.as_deref() {
        Some(value) => Some(parse_datetime(value)?),
        None => None,
    };
    let schedule_end = match query.schedule_end.as_deref() {
        Some(value) => Some(parse_datetime(value)?),
        None => None,
    };

    if let Some(tag) = query.style_tag.as_deref() {
        let tag = tag.trim();
        if !tag.is_empty() && tag.contains('\'') {
            return Err(DomainError::BadRequest("invalid_style_tag".to_string()).into());
        }
    }

    let filter = demands_repo::DemandListFilter {
        city_id: query.city_id,
        demand_type: query.r#type,
        status: query.status,
        schedule_start,
        schedule_end,
        min_budget: query.min_budget.map(decimal_from_f64),
        max_budget: query.max_budget.map(decimal_from_f64),
        style_tag: query.style_tag.and_then(|s| {
            let trimmed = s.trim().to_string();
            if trimmed.is_empty() { None } else { Some(trimmed) }
        }),
        is_merchant: query.is_merchant,
        mine: query.mine.unwrap_or(false),
        sort: query.sort,
    };

    let (rows, total) =
        demands_repo::list_demands(&state.orm, user_id, filter, page, page_size).await?;

    let items = rows
        .into_iter()
        .map(|r| DemandListItem {
            id: r.id,
            r#type: r.r#type,
            city_id: r.city_id,
            status: r.status,
            schedule_start: r.schedule_start.map(|d| d.to_rfc3339()),
        })
        .collect();

    Ok(Paged::new(items, total, page, page_size))
}

pub async fn get_demand(
    state: &AppState,
    demand_id: i64,
) -> ServiceResult<DemandDetail> {
    let row = demands_repo::find_demand_by_id(&state.orm, demand_id)
        .await?
        .ok_or(DomainError::NotFound)?;

    let attachments = demands_repo::list_attachments_by_demand(&state.orm, demand_id)
        .await?
        .into_iter()
        .map(|a| AttachmentResp {
            id: a.id,
            file_url: a.file_url,
            file_type: a.file_type,
        })
        .collect();

    Ok(DemandDetail {
        id: row.id,
        user_id: row.user_id,
        r#type: row.r#type,
        status: row.status,
        city_id: row.city_id,
        location: row.location,
        schedule_start: row.schedule_start.map(|d| d.to_rfc3339()),
        schedule_end: row.schedule_end.map(|d| d.to_rfc3339()),
        budget_min: row.budget_min.map(decimal_to_f64),
        budget_max: row.budget_max.map(decimal_to_f64),
        people_count: row.people_count,
        style_tags: row.style_tags.and_then(|v| serde_json::from_value(v).ok()),
        is_merchant: row.is_merchant,
        merchant_id: row.merchant_id,
        attachments,
    })
}

pub async fn list_demand_merchant_assets(
    state: &AppState,
    user_id: i64,
    demand_id: i64,
    query: DemandMerchantAssetQuery,
) -> ServiceResult<Paged<DemandMerchantAssetItem>> {
    let demand = demands_repo::find_demand_by_id(&state.orm, demand_id)
        .await?
        .ok_or(DomainError::NotFound)?;
    if !demand.is_merchant {
        return Err(DomainError::BadRequest("not_merchant_demand".to_string()).into());
    }
    let merchant_id = demand
        .merchant_id
        .ok_or(DomainError::BadRequest("merchant_id_required".to_string()))?;

    ensure_demand_asset_access(state, user_id, &demand, merchant_id).await?;

    let asset_type = match query.asset_type {
        Some(value) => Some(normalize_asset_type(value)?),
        None => None,
    };

    let (page, page_size) = normalize_pagination(query.page, query.page_size);
    let (rows, total) = merchants_repo::list_merchant_assets(
        &state.orm,
        merchant_id,
        asset_type,
        Some("active".to_string()),
        page,
        page_size,
    )
    .await?;

    if rows.is_empty() {
        return Ok(Paged::new(Vec::new(), total, page, page_size));
    }

    let asset_ids: Vec<i64> = rows.iter().map(|row| row.id).collect();
    let latest_rows =
        merchants_repo::list_latest_asset_versions_by_asset_ids(&state.orm, asset_ids).await?;
    let mut latest_map: std::collections::HashMap<i64, crate::entity::merchant_asset_versions::Model> =
        std::collections::HashMap::new();
    for row in latest_rows {
        latest_map.entry(row.asset_id).or_insert(row);
    }

    let items = rows
        .into_iter()
        .map(|row| {
            let latest = latest_map.get(&row.id);
            DemandMerchantAssetItem {
                id: row.id,
                merchant_id: row.merchant_id,
                asset_type: row.asset_type,
                name: row.name,
                latest_version: latest.map(|v| v.version),
                latest_payload: latest.map(|v| v.payload.clone()),
                updated_at: row.updated_at.to_rfc3339(),
            }
        })
        .collect();

    Ok(Paged::new(items, total, page, page_size))
}

pub async fn close_demand(
    state: &AppState,
    user_id: i64,
    demand_id: i64,
) -> ServiceResult<DemandResp> {
    let demand = demands_repo::find_demand_by_id(&state.orm, demand_id)
        .await?
        .ok_or(DomainError::NotFound)?;

    if demand.user_id != user_id || demand.status != "open" {
        return Err(DomainError::BadRequest("cannot_close".to_string()).into());
    }

    let updated = demands_repo::update_demand_status(&state.orm, demand, "closed".to_string())
        .await?;

    Ok(DemandResp {
        id: updated.id,
        user_id: updated.user_id,
        r#type: updated.r#type,
        status: updated.status,
    })
}

fn parse_datetime(input: &str) -> Result<chrono::DateTime<chrono::Utc>, DomainError> {
    chrono::DateTime::parse_from_rfc3339(input)
        .map(|dt| dt.with_timezone(&chrono::Utc))
        .map_err(|_| DomainError::BadRequest("invalid_datetime".to_string()))
}

fn decimal_to_f64(v: sea_orm::prelude::Decimal) -> f64 {
    v.to_string().parse::<f64>().unwrap_or(0.0)
}

fn decimal_from_f64(v: f64) -> sea_orm::prelude::Decimal {
    sea_orm::prelude::Decimal::from_str(&v.to_string())
        .unwrap_or(sea_orm::prelude::Decimal::ZERO)
}

fn normalize_asset_type(value: String) -> Result<String, DomainError> {
    let value = value.to_lowercase();
    if matches!(value.as_str(), "logo" | "brand" | "style" | "reference") {
        Ok(value)
    } else {
        Err(DomainError::BadRequest("invalid_asset_type".to_string()))
    }
}

async fn ensure_demand_asset_access(
    state: &AppState,
    user_id: i64,
    demand: &crate::entity::demands::Model,
    merchant_id: i64,
) -> ServiceResult<()> {
    if demand.user_id == user_id {
        return Ok(());
    }

    let merchant = merchants_repo::find_merchant_by_id(&state.orm, merchant_id)
        .await?
        .ok_or(DomainError::NotFound)?;
    if merchant.contact_user_id == Some(user_id) {
        return Ok(());
    }
    if merchants_repo::find_merchant_user(&state.orm, merchant_id, user_id)
        .await?
        .is_some()
    {
        return Ok(());
    }

    let photographer = crate::entity::photographers::Entity::find()
        .filter(crate::entity::photographers::Column::UserId.eq(user_id))
        .one(&state.orm)
        .await?;
    let photographer_id = photographer.map(|p| p.id);

    let mut team_ids = teams_repo::list_owned_team_ids(&state.orm, user_id).await?;
    let members = teams_repo::list_team_members_by_user(&state.orm, user_id).await?;
    team_ids.extend(members.into_iter().map(|m| m.team_id));
    team_ids.sort_unstable();
    team_ids.dedup();

    let mut quote_cond = Condition::any();
    let mut has_quote_cond = false;
    if let Some(pid) = photographer_id {
        quote_cond = quote_cond.add(crate::entity::quotes::Column::PhotographerId.eq(pid));
        has_quote_cond = true;
    }
    if !team_ids.is_empty() {
        quote_cond = quote_cond.add(crate::entity::quotes::Column::TeamId.is_in(team_ids.clone()));
        has_quote_cond = true;
    }
    if has_quote_cond {
        let quote = crate::entity::quotes::Entity::find()
            .filter(crate::entity::quotes::Column::DemandId.eq(demand.id))
            .filter(quote_cond)
            .one(&state.orm)
            .await?;
        if quote.is_some() {
            return Ok(());
        }
    }

    let mut order_cond = Condition::any();
    let mut has_order_cond = false;
    if let Some(pid) = photographer_id {
        order_cond = order_cond.add(crate::entity::orders::Column::PhotographerId.eq(pid));
        has_order_cond = true;
    }
    if !team_ids.is_empty() {
        order_cond = order_cond.add(crate::entity::orders::Column::TeamId.is_in(team_ids));
        has_order_cond = true;
    }
    if has_order_cond {
        let order = crate::entity::orders::Entity::find()
            .filter(crate::entity::orders::Column::DemandId.eq(demand.id))
            .filter(order_cond)
            .one(&state.orm)
            .await?;
        if order.is_some() {
            return Ok(());
        }
    }

    Err(DomainError::Forbidden.into())
}
