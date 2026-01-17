use axum::{routing::post, Json, Router};
use sea_orm::{ActiveModelTrait, EntityTrait, Set};
use serde::{Deserialize, Serialize};

use crate::middleware::auth::AuthUser;
use crate::error::{ApiError, ApiResult};
use crate::state::AppState;
use crate::entity::{orders, photographers, reviews};

#[derive(Deserialize)]
struct CreateReviewReq {
    order_id: i64,
    score: i32,
    tags: Option<Vec<String>>,
    comment: Option<String>,
}

#[derive(Serialize)]
struct ReviewResp {
    id: i64,
    order_id: i64,
    score: i32,
}

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(create_review))
}

async fn create_review(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<CreateReviewReq>,
) -> ApiResult<ReviewResp> {
    if !(1..=5).contains(&req.score) {
        return Err(ApiError::bad_request("invalid_score"));
    }

    let order = orders::Entity::find_by_id(req.order_id)
        .one(&state.orm)
        .await?
        .ok_or_else(ApiError::not_found)?;

    if order.user_id != user_id {
        return Err(ApiError::forbidden());
    }

    let photographer_user = match order.photographer_id {
        Some(pid) => photographers::Entity::find_by_id(pid)
            .one(&state.orm)
            .await?
            .map(|p| p.user_id),
        None => None,
    };

    let model = reviews::ActiveModel {
        order_id: Set(req.order_id),
        rater_id: Set(user_id),
        ratee_id: Set(photographer_user.unwrap_or(user_id)),
        score: Set(req.score),
        tags: Set(req.tags.map(|v| serde_json::json!(v))),
        comment: Set(req.comment),
        ..Default::default()
    };

    let inserted = model.insert(&state.orm).await?;

    let mut order_model: orders::ActiveModel = order.into();
    order_model.status = Set("reviewed".to_string());
    order_model.update(&state.orm).await?;

    Ok(Json(crate::common::ApiResponse::ok(ReviewResp {
        id: inserted.id,
        order_id: inserted.order_id,
        score: inserted.score,
    })))
}
