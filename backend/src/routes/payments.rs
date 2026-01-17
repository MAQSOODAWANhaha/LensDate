use axum::{routing::post, Router};

use crate::handlers::payments;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(payments::create_payment))
}
