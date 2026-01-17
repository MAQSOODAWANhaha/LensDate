use axum::{routing::get, Json, Router};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde::{Deserialize, Serialize};

use crate::middleware::auth::AuthUser;
use crate::error::{ApiError, ApiResult};
use crate::state::AppState;
use crate::entity::{user_profiles, users};

#[derive(Serialize)]
struct MeResp {
    id: i64,
    phone: String,
    status: String,
    profile: ProfileResp,
}

#[derive(Serialize, Default)]
struct ProfileResp {
    nickname: Option<String>,
    avatar_url: Option<String>,
    gender: Option<String>,
    city_id: Option<i64>,
    bio: Option<String>,
}

#[derive(Deserialize)]
struct UpdateProfileReq {
    nickname: Option<String>,
    avatar_url: Option<String>,
    gender: Option<String>,
    city_id: Option<i64>,
    bio: Option<String>,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/me", get(get_me).put(update_me))
}

async fn get_me(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
) -> ApiResult<MeResp> {
    let user = users::Entity::find_by_id(user_id)
        .one(&state.orm)
        .await?
        .ok_or_else(ApiError::not_found)?;

    let profile = user_profiles::Entity::find()
        .filter(user_profiles::Column::UserId.eq(user_id))
        .one(&state.orm)
        .await?;

    let profile_resp = profile.map(|p| ProfileResp {
        nickname: p.nickname,
        avatar_url: p.avatar_url,
        gender: p.gender,
        city_id: p.city_id,
        bio: p.bio,
    }).unwrap_or_default();

    Ok(Json(crate::common::ApiResponse::ok(MeResp {
        id: user.id,
        phone: user.phone,
        status: user.status,
        profile: profile_resp,
    })))
}

async fn update_me(
    AuthUser { user_id }: AuthUser,
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<UpdateProfileReq>,
) -> ApiResult<ProfileResp> {
    if let Some(nickname) = &req.nickname
        && (nickname.len() < 2 || nickname.len() > 20)
    {
        return Err(ApiError::bad_request("invalid_nickname"));
    }

    let existing = user_profiles::Entity::find()
        .filter(user_profiles::Column::UserId.eq(user_id))
        .one(&state.orm)
        .await?;

    let mut model = match existing {
        Some(m) => user_profiles::ActiveModel { user_id: Set(m.user_id), ..Default::default() },
        None => user_profiles::ActiveModel { user_id: Set(user_id), ..Default::default() },
    };

    model.nickname = Set(req.nickname.clone());
    model.avatar_url = Set(req.avatar_url.clone());
    model.gender = Set(req.gender.clone());
    model.city_id = Set(req.city_id);
    model.bio = Set(req.bio.clone());
    model.updated_at = Set(chrono::Utc::now().into());

    model.save(&state.orm).await?;

    Ok(Json(crate::common::ApiResponse::ok(ProfileResp {
        nickname: req.nickname,
        avatar_url: req.avatar_url,
        gender: req.gender,
        city_id: req.city_id,
        bio: req.bio,
    })))
}
