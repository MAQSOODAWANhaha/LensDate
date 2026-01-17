use axum::{routing::{get, post}, Router};

use crate::handlers::notifications;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(notifications::list_notifications))
        .route("/summary", get(notifications::get_summary))
        .route("/read-all", post(notifications::mark_all_read))
        .route("/:id", get(notifications::get_notification))
        .route("/:id/read", post(notifications::mark_notification_read))
}
