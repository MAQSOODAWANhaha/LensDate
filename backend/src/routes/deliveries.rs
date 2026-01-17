use axum::{routing::post, Json, Router};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set, TransactionTrait};
use serde::{Deserialize, Serialize};

use crate::middleware::auth::AuthUser;
use crate::error::{ApiError, ApiResult};
use crate::state::AppState;
use crate::entity::{deliveries, delivery_items, orders, photographers};

#[derive(Deserialize)]
struct DeliveryItemReq {
    file_url: String,
    version: Option<String>,
    note: Option<String>,
}

#[derive(Deserialize)]
struct CreateDeliveryReq {
    order_id: i64,
    items: Vec<DeliveryItemReq>,
}

#[derive(Serialize)]
struct DeliveryResp {
    id: i64,
    order_id: i64,
    status: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_delivery).get(list_deliveries))
        .route("/:id/accept", post(accept_delivery))
}

#[derive(Deserialize)]
struct DeliveryListQuery {
    order_id: i64,
}

#[derive(Serialize)]
struct DeliveryItemResp {
    id: i64,
    file_url: String,
    version: Option<String>,
    note: Option<String>,
}

#[derive(Serialize)]
struct DeliveryDetailResp {
    id: i64,
    order_id: i64,
    status: String,
    submitted_at: Option<String>,
    accepted_at: Option<String>,
    items: Vec<DeliveryItemResp>,
}

async fn list_deliveries(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    axum::extract::Query(q): axum::extract::Query<DeliveryListQuery>,
) -> ApiResult<Vec<DeliveryDetailResp>> {
    let order = orders::Entity::find_by_id(q.order_id)
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

    let rows = deliveries::Entity::find()
        .filter(deliveries::Column::OrderId.eq(q.order_id))
        .order_by_desc(deliveries::Column::SubmittedAt)
        .all(&state.orm)
        .await?;
    if rows.is_empty() {
        return Ok(Json(crate::common::ApiResponse::ok(Vec::new())));
    }

    let ids: Vec<i64> = rows.iter().map(|d| d.id).collect();
    let item_rows = delivery_items::Entity::find()
        .filter(delivery_items::Column::DeliveryId.is_in(ids.clone()))
        .order_by_desc(delivery_items::Column::CreatedAt)
        .all(&state.orm)
        .await?;
    let mut items_map: std::collections::HashMap<i64, Vec<DeliveryItemResp>> =
        std::collections::HashMap::new();
    for item in item_rows {
        items_map
            .entry(item.delivery_id)
            .or_default()
            .push(DeliveryItemResp {
                id: item.id,
                file_url: item.file_url,
                version: item.version,
                note: item.note,
            });
    }

    let list = rows
        .into_iter()
        .map(|d| DeliveryDetailResp {
            id: d.id,
            order_id: d.order_id,
            status: d.status,
            submitted_at: d.submitted_at.map(|t| t.to_rfc3339()),
            accepted_at: d.accepted_at.map(|t| t.to_rfc3339()),
            items: items_map.remove(&d.id).unwrap_or_default(),
        })
        .collect();

    Ok(Json(crate::common::ApiResponse::ok(list)))
}

async fn create_delivery(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<CreateDeliveryReq>,
) -> ApiResult<DeliveryResp> {
    if req.items.is_empty() {
        return Err(ApiError::bad_request("items_required"));
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

    if photographer_user != Some(user_id) {
        return Err(ApiError::forbidden());
    }

    let txn = state.orm.begin().await?;

    let delivery = deliveries::ActiveModel {
        order_id: Set(req.order_id),
        status: Set("submitted".to_string()),
        submitted_at: Set(Some(chrono::Utc::now().into())),
        ..Default::default()
    };
    let inserted = delivery.insert(&txn).await?;

    for item in req.items {
        let di = delivery_items::ActiveModel {
            delivery_id: Set(inserted.id),
            file_url: Set(item.file_url),
            version: Set(item.version),
            note: Set(item.note),
            ..Default::default()
        };
        di.insert(&txn).await?;
    }

    let mut order_model: orders::ActiveModel = order.into();
    order_model.status = Set("ongoing".to_string());
    order_model.update(&txn).await?;

    txn.commit().await?;

    Ok(Json(crate::common::ApiResponse::ok(DeliveryResp {
        id: inserted.id,
        order_id: inserted.order_id,
        status: inserted.status,
    })))
}

async fn accept_delivery(
    AuthUser { user_id }: AuthUser,
    axum::extract::Path(delivery_id): axum::extract::Path<i64>,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<DeliveryResp> {
    let delivery = deliveries::Entity::find_by_id(delivery_id)
        .one(&state.orm)
        .await?
        .ok_or_else(ApiError::not_found)?;

    let order = orders::Entity::find_by_id(delivery.order_id)
        .one(&state.orm)
        .await?
        .ok_or_else(ApiError::not_found)?;

    if order.user_id != user_id {
        return Err(ApiError::forbidden());
    }
    if delivery.status != "submitted" {
        return Err(ApiError::bad_request("invalid_status"));
    }

    let txn = state.orm.begin().await?;

    let mut delivery_model: deliveries::ActiveModel = delivery.into();
    delivery_model.status = Set("accepted".to_string());
    delivery_model.accepted_at = Set(Some(chrono::Utc::now().into()));
    let updated = delivery_model.update(&txn).await?;

    let mut order_model: orders::ActiveModel = order.into();
    order_model.status = Set("completed".to_string());
    order_model.update(&txn).await?;

    txn.commit().await?;

    Ok(Json(crate::common::ApiResponse::ok(DeliveryResp {
        id: updated.id,
        order_id: updated.order_id,
        status: updated.status,
    })))
}
