use axum::{routing::put, Router};

use crate::handlers::configs;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/:key", put(configs::upsert_config).get(configs::get_config))
}
