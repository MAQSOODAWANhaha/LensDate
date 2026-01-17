use axum::{routing::post, Router};

use crate::handlers::messages;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new().route("/", post(messages::send_message).get(messages::list_messages))
}
