use axum::{routing::{delete, get, post, put}, Router};

use crate::handlers::merchants;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(merchants::create_merchant))
        .route("/mine", get(merchants::list_my_merchants))
        .route("/orders", get(merchants::list_merchant_orders))
        .route("/reports/orders", get(merchants::export_merchant_orders_report))
        .route("/orders/:id", get(merchants::get_merchant_order))
        .route("/:id/locations", post(merchants::create_location).get(merchants::list_locations))
        .route(
            "/:id/locations/:location_id",
            put(merchants::update_location).delete(merchants::delete_location),
        )
        .route("/:id/members", post(merchants::add_member).get(merchants::list_members))
        .route("/:id/members/:user_id", delete(merchants::remove_member))
        .route(
            "/:id/assets",
            post(merchants::create_merchant_asset).get(merchants::list_merchant_assets),
        )
        .route(
            "/assets/:asset_id/versions",
            get(merchants::list_merchant_asset_versions)
                .post(merchants::create_merchant_asset_version),
        )
}

pub fn template_router() -> Router<AppState> {
    Router::new().route(
        "/",
        post(merchants::create_template).get(merchants::list_templates),
    )
}

pub fn approval_router() -> Router<AppState> {
    Router::new().route(
        "/",
        post(merchants::create_approval).get(merchants::list_approvals),
    )
}

pub fn contract_router() -> Router<AppState> {
    Router::new().route(
        "/",
        post(merchants::create_contract).get(merchants::list_contracts),
    )
}

pub fn invoice_router() -> Router<AppState> {
    Router::new().route(
        "/",
        post(merchants::create_invoice).get(merchants::list_invoices),
    )
}
