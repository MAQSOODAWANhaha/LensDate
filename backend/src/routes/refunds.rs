use axum::{routing::post, Router};

use crate::handlers::refunds;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(refunds::create_refund))
}
