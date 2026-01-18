use axum::{routing::get, Router};
use api_gateway::routes::create_router;
use api_gateway::state::AppState;
use sea_orm::Database;
use std::env;
use tower_http::services::{ServeDir, ServeFile};
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is required");
    let orm = Database::connect(&database_url)
        .await
        .expect("failed to connect orm");

    let state = AppState { orm };

    let admin_dir = env::var("ADMIN_WEB_DIR").unwrap_or_else(|_| "admin_web".to_string());
    let static_service =
        ServeDir::new(&admin_dir).fallback(ServeFile::new(format!("{}/index.html", admin_dir)));

    let app = Router::new()
        .route("/health", get(health))
        .nest("/api/v1", create_router())
        .fallback_service(static_service)
        .with_state(state);

    let addr = "0.0.0.0:8080";
    tracing::info!("listening on {}", addr);
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn health() -> &'static str {
    "ok"
}
