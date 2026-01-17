use std::str::FromStr;

use sea_orm::TransactionTrait;

use crate::dto::payments::{CreatePaymentReq, PaymentResp};
use crate::errors::{DomainError, ServiceResult};
use crate::repositories::orders_repo;
use crate::state::AppState;

pub async fn create_payment(
    state: &AppState,
    user_id: i64,
    req: CreatePaymentReq,
) -> ServiceResult<PaymentResp> {
    if req.amount <= 0.0 {
        return Err(DomainError::InvalidAmount.into());
    }
    if !matches!(req.pay_channel.as_str(), "wx" | "alipay" | "bank") {
        return Err(DomainError::BadRequest("invalid_channel".to_string()).into());
    }
    if req.proof_url.is_empty() {
        return Err(DomainError::BadRequest("proof_required".to_string()).into());
    }
    if let Some(stage) = req.stage.as_deref()
        && !matches!(stage, "deposit" | "mid" | "final")
    {
        return Err(DomainError::BadRequest("invalid_stage".to_string()).into());
    }

    let txn = state.orm.begin().await?;
    let order = orders_repo::find_order_by_id(&txn, req.order_id)
        .await?
        .ok_or(DomainError::NotFound)?;

    if order.user_id != user_id {
        return Err(DomainError::Forbidden.into());
    }

    let payee_id = match order.photographer_id {
        Some(pid) => orders_repo::find_photographer_user_id(&txn, pid)
            .await?
            .unwrap_or(user_id),
        None => user_id,
    };

    if order.pay_type == "phase" && req.stage.is_none() {
        return Err(DomainError::BadRequest("stage_required".to_string()).into());
    }

    let payment = crate::entity::payments::ActiveModel {
        order_id: sea_orm::ActiveValue::Set(req.order_id),
        payer_id: sea_orm::ActiveValue::Set(user_id),
        payee_id: sea_orm::ActiveValue::Set(payee_id),
        amount: sea_orm::ActiveValue::Set(decimal_from_f64(req.amount)),
        status: sea_orm::ActiveValue::Set("success".to_string()),
        pay_channel: sea_orm::ActiveValue::Set(req.pay_channel),
        stage: sea_orm::ActiveValue::Set(req.stage.clone()),
        proof_url: sea_orm::ActiveValue::Set(Some(req.proof_url)),
        paid_at: sea_orm::ActiveValue::Set(Some(chrono::Utc::now().into())),
        ..Default::default()
    };

    let inserted = orders_repo::create_payment(&txn, payment).await?;

    let should_mark_paid = match req.stage.as_deref() {
        Some("final") => true,
        Some(_) => false,
        None => true,
    };
    if should_mark_paid {
        orders_repo::update_order_status(&txn, order, "paid".to_string()).await?;
    }

    txn.commit().await?;

    Ok(PaymentResp {
        id: inserted.id,
        order_id: inserted.order_id,
        amount: decimal_to_f64(inserted.amount),
        status: inserted.status,
        stage: inserted.stage,
    })
}

fn decimal_to_f64(v: sea_orm::prelude::Decimal) -> f64 {
    v.to_string().parse::<f64>().unwrap_or(0.0)
}

fn decimal_from_f64(v: f64) -> sea_orm::prelude::Decimal {
    sea_orm::prelude::Decimal::from_str(&v.to_string())
        .unwrap_or(sea_orm::prelude::Decimal::ZERO)
}
