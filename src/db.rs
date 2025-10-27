use mongodb::{Client, Database};
use std::env;

pub async fn init_db() -> Database {
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client = Client::with_uri_str(&uri)
        .await
        .expect("Failed to connect to MongoDB");
    let db_name = env::var("DATABASE_NAME").unwrap_or_else(|_| "testdb".to_string());
    client.database(&db_name)
}
