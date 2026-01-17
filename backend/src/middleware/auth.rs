use axum::extract::{FromRef, FromRequestParts};
use axum::http::request::Parts;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};

use crate::error::ApiError;
use crate::state::AppState;
use crate::entity::sessions;

#[derive(Debug, Clone)]
pub struct AuthUser {
    pub user_id: i64,
}

impl<S> FromRequestParts<S> for AuthUser
where
    AppState: FromRef<S>,
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let auth_header = parts
            .headers
            .get(axum::http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .ok_or_else(ApiError::unauthorized)?;

        let token = auth_header
            .strip_prefix("Bearer ")
            .ok_or_else(ApiError::unauthorized)?;

        let AppState { orm, .. } = AppState::from_ref(state);
        let session = sessions::Entity::find()
            .filter(sessions::Column::Token.eq(token))
            .filter(sessions::Column::ExpiredAt.gt(chrono::Utc::now()))
            .one(&orm)
            .await
            .map_err(|_| ApiError::unauthorized())?;

        let user_id = session.ok_or_else(ApiError::unauthorized)?.user_id;
        Ok(AuthUser { user_id })
    }
}
