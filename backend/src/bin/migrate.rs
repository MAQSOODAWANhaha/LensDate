use sea_orm_migration::prelude::*;
use sea_orm::Database;

#[tokio::main]
async fn main() {
    let database_url = std::env::var("DATABASE_URL").expect("DATABASE_URL is required");
    let db = Database::connect(&database_url)
        .await
        .expect("failed to connect database");

    api_gateway::migration::Migrator::up(&db, None)
        .await
        .expect("migration failed");
}
