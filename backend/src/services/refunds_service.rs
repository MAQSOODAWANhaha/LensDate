use std::str::FromStr;

use crate::dto::refunds::{CreateRefundReq, RefundResp};
use crate::errors::{DomainError, ServiceResult};
use crate::repositories::orders_repo;
use crate::state::AppState;

pub async fn create_refund(
    state: &AppState,
    user_id: i64,
    req: CreateRefundReq,
) -> ServiceResult<RefundResp> {
    if req.amount <= 0.0 {
        return Err(DomainError::InvalidAmount.into());
    }
    if let Some(party) = req.responsible_party.as_deref()
        && !matches!(party, "user" | "photographer" | "merchant")
    {
        return Err(DomainError::BadRequest("invalid_responsible_party".to_string()).into());
    }

    let order = orders_repo::find_order_by_id(&state.orm, req.order_id)
        .await?
        .ok_or(DomainError::NotFound)?;

    let photographer_user = match order.photographer_id {
        Some(pid) => orders_repo::find_photographer_user_id(&state.orm, pid).await?,
        None => None,
    };

    if order.user_id != user_id && photographer_user != Some(user_id) {
        return Err(DomainError::Forbidden.into());
    }

    let refund = crate::entity::refunds::ActiveModel {
        order_id: sea_orm::ActiveValue::Set(req.order_id),
        applicant_id: sea_orm::ActiveValue::Set(user_id),
        amount: sea_orm::ActiveValue::Set(decimal_from_f64(req.amount)),
        status: sea_orm::ActiveValue::Set("pending".to_string()),
        responsible_party: sea_orm::ActiveValue::Set(req.responsible_party),
        reason: sea_orm::ActiveValue::Set(req.reason),
        proof_url: sea_orm::ActiveValue::Set(req.proof_url),
        ..Default::default()
    };

    let inserted = orders_repo::create_refund(&state.orm, refund).await?;

    Ok(RefundResp {
        id: inserted.id,
        order_id: inserted.order_id,
        amount: decimal_to_f64(inserted.amount),
        status: inserted.status,
        responsible_party: inserted.responsible_party,
    })
}

fn decimal_to_f64(v: sea_orm::prelude::Decimal) -> f64 {
    v.to_string().parse::<f64>().unwrap_or(0.0)
}

fn decimal_from_f64(v: f64) -> sea_orm::prelude::Decimal {
    sea_orm::prelude::Decimal::from_str(&v.to_string())
        .unwrap_or(sea_orm::prelude::Decimal::ZERO)
}
