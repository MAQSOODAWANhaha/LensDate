use axum::{routing::{get, post}, Router};

use crate::handlers::quotes;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(quotes::create_quote).get(quotes::list_quotes))
        .route("/mine", get(quotes::list_my_quotes))
        .route("/:id", get(quotes::get_quote).put(quotes::update_quote))
        .route("/:id/versions", get(quotes::list_quote_versions))
        .route("/:id/withdraw", post(quotes::withdraw_quote))
        .route("/:id/accept", post(quotes::accept_quote))
}
