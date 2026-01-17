use axum::{routing::{get, post}, Router};

use crate::handlers::admin;
use crate::state::AppState;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/audits", post(admin::create_audit).get(admin::list_audits))
        .route("/metrics", get(admin::get_metrics))
        .route("/metrics/trends", get(admin::get_metrics_trends))
        .route("/reports/orders", get(admin::export_orders_report))
        .route("/users", get(admin::list_admin_users))
        .route("/orders", get(admin::list_admin_orders))
        .route("/orders/:id", get(admin::get_admin_order_detail))
        .route("/disputes", get(admin::list_admin_disputes))
        .route("/disputes/:id", get(admin::get_admin_dispute_detail))
        .route("/portfolios", get(admin::list_admin_portfolios))
        .route("/portfolios/:id/review", post(admin::review_portfolio))
        .route("/merchant-approvals", get(admin::list_merchant_approvals))
        .route(
            "/merchant-approvals/:id/review",
            post(admin::review_merchant_approval),
        )
        .route("/merchant-templates", get(admin::list_merchant_templates))
        .route("/photographers/:id/review", post(admin::review_photographer))
        .route("/orders/:id/freeze", post(admin::freeze_order))
        .route("/disputes/:id/resolve", post(admin::resolve_dispute))
}
