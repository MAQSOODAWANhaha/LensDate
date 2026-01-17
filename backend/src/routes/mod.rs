use axum::routing::get;
use axum::Router;

use crate::state::AppState;

mod admin;
mod auth;
mod configs;
mod demands;
mod deliveries;
mod disputes;
mod merchants;
mod conversations;
mod messages;
mod notifications;
mod orders;
mod payments;
mod photographers;
mod quotes;
mod refunds;
mod reviews;
mod teams;
mod users;
mod uploads;

pub fn create_router() -> Router<AppState> {
    Router::new()
        .route("/health", get(health))
        .nest("/auth", auth::router())
        .nest("/users", users::router())
        .nest("/photographers", photographers::router())
        .nest("/portfolios", photographers::portfolio_router())
        .nest("/demands", demands::router())
        .nest("/quotes", quotes::router())
        .nest("/orders", orders::router())
        .nest("/payments", payments::router())
        .nest("/refunds", refunds::router())
        .nest("/deliveries", deliveries::router())
        .nest("/reviews", reviews::router())
        .nest("/disputes", disputes::router())
        .nest("/teams", teams::router())
        .nest("/merchants", merchants::router())
        .nest("/merchant-templates", merchants::template_router())
        .nest("/merchant-approvals", merchants::approval_router())
        .nest("/merchant-contracts", merchants::contract_router())
        .nest("/merchant-invoices", merchants::invoice_router())
        .nest("/conversations", conversations::router())
        .nest("/messages", messages::router())
        .nest("/notifications", notifications::router())
        .nest("/uploads", uploads::router())
        .nest("/admin", admin::router())
        .nest("/admin/configs", configs::router())
}

async fn health() -> &'static str {
    "ok"
}
