use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("{0}")]
    BadRequest(String),
    #[error("{0}")]
    Conflict(String),
    #[error("invalid_name")]
    InvalidName,
    #[error("invalid_status")]
    InvalidStatus,
    #[error("invalid_role")]
    InvalidRole,
    #[error("invalid_address")]
    InvalidAddress,
    #[error("invalid_amount")]
    InvalidAmount,
    #[error("items_required")]
    ItemsRequired,
    #[error("forbidden")]
    Forbidden,
    #[error("not_found")]
    NotFound,
    #[error("member_exists")]
    MemberExists,
    #[error("contact_user")]
    ContactUser,
}
