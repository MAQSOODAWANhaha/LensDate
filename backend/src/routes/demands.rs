use axum::{routing::{get, post}, Router};

use crate::handlers::demands;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(demands::create_demand).get(demands::list_demands))
        .route("/:id", get(demands::get_demand))
        .route(
            "/:id/merchant-assets",
            get(demands::list_demand_merchant_assets),
        )
        .route("/:id/close", post(demands::close_demand))
}
