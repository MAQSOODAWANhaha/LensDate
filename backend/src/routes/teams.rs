use axum::{routing::{delete, post, put}, Router};

use crate::handlers::teams;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(teams::create_team).get(teams::list_teams))
        .route("/:id", put(teams::update_team))
        .route("/:id/members", post(teams::add_member).get(teams::list_members))
        .route("/:id/members/:user_id", delete(teams::remove_member))
}
