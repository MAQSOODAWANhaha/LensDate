use axum::{routing::post, Json, Router};
use sea_orm::{ActiveModelTrait, EntityTrait, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::middleware::auth::AuthUser;
use crate::error::{ApiError, ApiResult};
use crate::state::AppState;
use crate::entity::{dispute_evidence, disputes, orders, photographers};

#[derive(Deserialize)]
struct EvidenceReq {
    file_url: String,
    note: Option<String>,
}

#[derive(Deserialize)]
struct CreateDisputeReq {
    order_id: i64,
    reason: String,
    evidence: Option<Vec<EvidenceReq>>,
}

#[derive(Serialize)]
struct DisputeResp {
    id: i64,
    order_id: i64,
    status: String,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(create_dispute))
}

async fn create_dispute(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<CreateDisputeReq>,
) -> ApiResult<DisputeResp> {
    if req.reason.is_empty() {
        return Err(ApiError::bad_request("invalid_reason"));
    }

    let order = orders::Entity::find_by_id(req.order_id)
        .one(&state.orm)
        .await?
        .ok_or_else(ApiError::not_found)?;

    let photographer_user = match order.photographer_id {
        Some(pid) => photographers::Entity::find_by_id(pid)
            .one(&state.orm)
            .await?
            .map(|p| p.user_id),
        None => None,
    };

    if order.user_id != user_id && photographer_user != Some(user_id) {
        return Err(ApiError::forbidden());
    }

    let txn = state.orm.begin().await?;

    let dispute = disputes::ActiveModel {
        order_id: Set(req.order_id),
        initiator_id: Set(user_id),
        status: Set("submitted".to_string()),
        reason: Set(Some(req.reason)),
        ..Default::default()
    };

    let inserted = dispute.insert(&txn).await?;

    if let Some(evidence) = req.evidence {
        for e in evidence {
            let ev = dispute_evidence::ActiveModel {
                dispute_id: Set(inserted.id),
                file_url: Set(e.file_url),
                note: Set(e.note),
                ..Default::default()
            };
            ev.insert(&txn).await?;
        }
    }

    txn.commit().await?;

    Ok(Json(crate::common::ApiResponse::ok(DisputeResp {
        id: inserted.id,
        order_id: inserted.order_id,
        status: inserted.status,
    })))
}
