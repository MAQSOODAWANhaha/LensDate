use axum::{routing::post, Router};

use crate::handlers::conversations;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(conversations::create_conversation).get(conversations::list_conversations))
}
