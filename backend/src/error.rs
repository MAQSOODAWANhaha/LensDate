use axum::{http::StatusCode, response::IntoResponse, Json};
use crate::common::ApiResponse;
use crate::errors::{DomainError, ServiceError};
use tracing::error;

pub type ApiResult<T> = Result<Json<ApiResponse<T>>, ApiError>;

#[derive(Debug)]
pub struct ApiError {
    pub code: i32,
    pub message: String,
    pub status: StatusCode,
}

impl ApiError {
    pub fn bad_request(msg: &str) -> Self {
        Self { code: 1001, message: msg.to_string(), status: StatusCode::BAD_REQUEST }
    }

    pub fn unauthorized() -> Self {
        Self { code: 1002, message: "unauthorized".to_string(), status: StatusCode::UNAUTHORIZED }
    }

    pub fn forbidden() -> Self {
        Self { code: 1003, message: "forbidden".to_string(), status: StatusCode::FORBIDDEN }
    }

    pub fn not_found() -> Self {
        Self { code: 1004, message: "not_found".to_string(), status: StatusCode::NOT_FOUND }
    }

    pub fn conflict(msg: &str) -> Self {
        Self { code: 1005, message: msg.to_string(), status: StatusCode::CONFLICT }
    }

    pub fn internal() -> Self {
        Self { code: 1500, message: "internal".to_string(), status: StatusCode::INTERNAL_SERVER_ERROR }
    }
}

impl From<sea_orm::DbErr> for ApiError {
    fn from(err: sea_orm::DbErr) -> Self {
        error!(error = ?err, "db_error");
        ApiError::internal()
    }
}

impl From<anyhow::Error> for ApiError {
    fn from(err: anyhow::Error) -> Self {
        error!(error = ?err, "internal_error");
        ApiError::internal()
    }
}

impl From<DomainError> for ApiError {
    fn from(err: DomainError) -> Self {
        match err {
            DomainError::BadRequest(message) => ApiError::bad_request(&message),
            DomainError::Conflict(message) => ApiError::conflict(&message),
            DomainError::InvalidName => ApiError::bad_request("invalid_name"),
            DomainError::InvalidStatus => ApiError::bad_request("invalid_status"),
            DomainError::InvalidRole => ApiError::bad_request("invalid_role"),
            DomainError::InvalidAddress => ApiError::bad_request("invalid_address"),
            DomainError::InvalidAmount => ApiError::bad_request("invalid_amount"),
            DomainError::ItemsRequired => ApiError::bad_request("items_required"),
            DomainError::Forbidden => ApiError::forbidden(),
            DomainError::NotFound => ApiError::not_found(),
            DomainError::MemberExists => ApiError::conflict("member_exists"),
            DomainError::ContactUser => ApiError::conflict("contact_user"),
        }
    }
}

impl From<ServiceError> for ApiError {
    fn from(err: ServiceError) -> Self {
        match err {
            ServiceError::Domain(domain) => ApiError::from(domain),
            ServiceError::Infra(err) => ApiError::from(err),
        }
    }
}

impl IntoResponse for ApiError {
    fn into_response(self) -> axum::response::Response {
        let body = Json(ApiResponse::<()>::err(self.code, &self.message));
        (self.status, body).into_response()
    }
}
