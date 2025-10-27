use mongodb::{bson::oid::ObjectId, Client, Database};
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Item {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub description: String,
}

pub async fn connect_db() -> Database {
    dotenvy::dotenv().ok();
    let uri = env::var("MONGODB_URI").expect("MONGODB_URI must be set");
    let client = Client::with_uri_str(&uri)
        .await
        .expect("Failed to connect to MongoDB");
    client.database("rust_crud_app")
}
