use askama::Template;
use askama_axum::IntoResponse;
use axum::{
    extract::{Form, State},
    response::Html,
    routing::{get, post},
    Router,
};
use futures_util::TryStreamExt;
use mongodb::{bson::doc, Database};
use crate::utils::Item;

#[derive(Template)]
#[template(path = "dashboard.html")]
pub struct DashboardTemplate<'a> {
    pub items: &'a [Item],
}

#[derive(Template)]
#[template(path = "new_item.html")]
pub struct NewItemTemplate;

async fn dashboard(State(db): State<Database>) -> impl IntoResponse {
    let col = db.collection::<Item>("items");
    let mut cursor = col.find(doc! {}).await.unwrap();
    let mut items = Vec::new();

    while let Some(item) = cursor.try_next().await.unwrap() {
        items.push(item);
    }

    Html(DashboardTemplate { items: &items }.render().unwrap())
}

async fn new_item_page() -> impl IntoResponse {
    Html(NewItemTemplate.render().unwrap())
}

#[derive(serde::Deserialize)]
struct NewItemForm {
    name: String,
    description: String,
}

async fn create_item(State(db): State<Database>, Form(form): Form<NewItemForm>) -> Html<String> {
    let col = db.collection::<Item>("items");
    let new_item = Item {
        id: None,
        name: form.name,
        description: form.description,
    };

    col.insert_one(new_item).await.unwrap();
    Html("<p>Item added! <a href='/dashboard'>Go back</a></p>".to_string())
}

pub fn router() -> Router<Database> {
    Router::new()
        .route("/dashboard", get(dashboard))
        .route("/items/new", get(new_item_page))
        .route("/items", post(create_item))
}
