use axum::{routing::post, Json, Router};
use bcrypt::verify;
use chrono::{Duration, Utc};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, QueryOrder, Set};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{ApiError, ApiResult};
use crate::state::AppState;
use crate::entity::{sessions, users, verification_codes};

#[derive(Deserialize)]
struct SendCodeReq {
    phone: String,
}

#[derive(Serialize)]
struct SendCodeResp {
    expired_at: i64,
}

#[derive(Deserialize)]
struct LoginReq {
    phone: Option<String>,
    code: Option<String>,
    username: Option<String>,
    password: Option<String>,
}

#[derive(Serialize)]
struct LoginResp {
    token: String,
    user: LoginUser,
    roles: Vec<String>,
}

#[derive(Serialize)]
struct LoginUser {
    id: i64,
    phone: String,
    status: String,
}

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/code", post(send_code))
        .route("/login", post(login))
}

async fn send_code(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<SendCodeReq>,
) -> ApiResult<SendCodeResp> {
    if !is_valid_phone(&req.phone) {
        return Err(ApiError::bad_request("invalid_phone"));
    }

    let code = generate_code();
    let expired_at = Utc::now() + Duration::minutes(5);

    let model = verification_codes::ActiveModel {
        phone: Set(req.phone),
        code: Set(code),
        expired_at: Set(expired_at.into()),
        ..Default::default()
    };

    let inserted = model.insert(&state.orm).await?;

    Ok(Json(crate::common::ApiResponse::ok(SendCodeResp {
        expired_at: inserted.expired_at.timestamp(),
    })))
}

async fn login(
    axum::extract::State(state): axum::extract::State<AppState>,
    Json(req): Json<LoginReq>,
) -> ApiResult<LoginResp> {
    let user = if let Some(code) = req.code.as_deref() {
        let phone = req
            .phone
            .as_deref()
            .ok_or_else(|| ApiError::bad_request("phone_required"))?;
        if !is_valid_phone(phone) {
            return Err(ApiError::bad_request("invalid_phone"));
        }

        let verify_code = verification_codes::Entity::find()
            .filter(verification_codes::Column::Phone.eq(phone))
            .filter(verification_codes::Column::Code.eq(code))
            .filter(verification_codes::Column::ExpiredAt.gt(Utc::now()))
            .order_by_desc(verification_codes::Column::CreatedAt)
            .one(&state.orm)
            .await?;

        if verify_code.is_none() {
            return Err(ApiError::bad_request("invalid_code"));
        }

        ensure_user(&state.orm, phone).await?
    } else if let Some(password) = req.password.as_deref() {
        if req.username.is_none() && req.phone.is_none() {
            return Err(ApiError::bad_request("login_payload_required"));
        }
        let mut user = None;
        if let Some(username) = req.username.as_deref() {
            if !is_valid_username(username) {
                return Err(ApiError::bad_request("invalid_username"));
            }
            user = users::Entity::find()
                .filter(users::Column::Username.eq(username))
                .one(&state.orm)
                .await?;
        } else if let Some(phone) = req.phone.as_deref() {
            if !is_valid_phone(phone) {
                return Err(ApiError::bad_request("invalid_phone"));
            }
            user = users::Entity::find()
                .filter(users::Column::Phone.eq(phone))
                .one(&state.orm)
                .await?;
        }

        let user = user.ok_or_else(|| ApiError::bad_request("invalid_credentials"))?;
        let hash = user
            .password_hash
            .as_deref()
            .ok_or_else(|| ApiError::bad_request("password_not_set"))?;
        let ok = verify(password, hash).map_err(|_| ApiError::internal())?;
        if !ok {
            return Err(ApiError::bad_request("invalid_credentials"));
        }
        user
    } else {
        return Err(ApiError::bad_request("login_payload_required"));
    };
    let token = Uuid::new_v4().to_string();

    let session = sessions::ActiveModel {
        user_id: Set(user.id),
        token: Set(token.clone()),
        expired_at: Set((Utc::now() + Duration::days(7)).into()),
        ..Default::default()
    };

    session.insert(&state.orm).await?;

    let roles = resolve_roles(&user.phone);
    let resp = LoginResp {
        token,
        user: LoginUser { id: user.id, phone: user.phone, status: user.status },
        roles,
    };

    Ok(Json(crate::common::ApiResponse::ok(resp)))
}

async fn ensure_user(db: &sea_orm::DatabaseConnection, phone: &str) -> Result<users::Model, ApiError> {
    if let Some(user) = users::Entity::find()
        .filter(users::Column::Phone.eq(phone))
        .one(db)
        .await? {
        return Ok(user);
    }

    let model = users::ActiveModel {
        phone: Set(phone.to_string()),
        status: Set("active".to_string()),
        credit_score: Set(100),
        ..Default::default()
    };

    let inserted = model.insert(db).await?;
    Ok(inserted)
}

fn is_valid_phone(phone: &str) -> bool {
    if phone.len() != 11 || !phone.starts_with('1') {
        return false;
    }
    phone.chars().all(|c| c.is_ascii_digit())
}

fn is_valid_username(username: &str) -> bool {
    let name = username.trim();
    if name.len() < 2 || name.len() > 32 {
        return false;
    }
    name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '.')
}

fn generate_code() -> String {
    let nanos = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.subsec_nanos())
        .unwrap_or(0);
    format!("{:06}", (nanos % 1_000_000))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_phone() {
        assert!(is_valid_phone("13800138000"));
        assert!(!is_valid_phone("23800138000"));
        assert!(!is_valid_phone("1380013800"));
        assert!(!is_valid_phone("1380013800a"));
    }

    #[test]
    fn test_generate_code_format() {
        let code = generate_code();
        assert_eq!(code.len(), 6);
        assert!(code.chars().all(|c| c.is_ascii_digit()));
    }

    #[test]
    fn test_is_valid_username() {
        assert!(is_valid_username("admin_user"));
        assert!(is_valid_username("admin.user"));
        assert!(!is_valid_username("a"));
        assert!(!is_valid_username("admin user"));
    }
}
