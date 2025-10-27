mod auth;
mod routes;
mod utils;

use axum::{Router, routing::get};
use mongodb::Database;
use utils::connect_db;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let db: Database = connect_db().await;

    let app = Router::new()
        .nest("/", auth::router())
        .nest("/", routes::router())
        .with_state(db.clone());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("🚀 Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}
