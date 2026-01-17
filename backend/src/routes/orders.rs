use axum::{routing::{get, post}, Router};

use crate::handlers::orders;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(orders::list_orders))
        .route("/:id", get(orders::get_order))
        .route("/:id/refund-preview", get(orders::refund_preview))
        .route("/:id/cancel", post(orders::cancel_order))
}
