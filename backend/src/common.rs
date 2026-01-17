use serde::Serialize;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("invalid_param")]
    InvalidParam,
    #[error("unauthorized")]
    Unauthorized,
    #[error("forbidden")]
    Forbidden,
    #[error("not_found")]
    NotFound,
    #[error("conflict")]
    Conflict,
    #[error("internal")]
    Internal,
}

#[derive(Serialize)]
pub struct ApiResponse<T> {
    pub code: i32,
    pub message: String,
    pub data: Option<T>,
}

impl<T> ApiResponse<T> {
    pub fn ok(data: T) -> Self {
        Self { code: 0, message: "ok".to_string(), data: Some(data) }
    }

    pub fn err(code: i32, message: &str) -> Self {
        Self { code, message: message.to_string(), data: None }
    }
}
