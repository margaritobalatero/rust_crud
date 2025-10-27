use axum::{
    extract::{Form, State},
    response::{Html, Redirect},
    routing::{get, post},
    Router,
};
use mongodb::{bson::doc, Database};
use serde::Deserialize;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use std::env;

#[derive(Deserialize)]
pub struct SignupForm {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(serde::Serialize)]
struct Claims {
    sub: String,
    exp: usize,
}

async fn signup_page() -> Html<&'static str> {
    Html("<form action='/signup' method='post'>
        Username: <input name='username'>
        Password: <input name='password' type='password'>
        <button type='submit'>Sign Up</button>
    </form>")
}

async fn signup(State(db): State<Database>, Form(form): Form<SignupForm>) -> Redirect {
    let users = db.collection::<mongodb::bson::Document>("users");

    let hashed = hash(&form.password, DEFAULT_COST).unwrap();
    let new_user = doc! { "username": &form.username, "password": hashed };

    users.insert_one(new_user).await.unwrap();
    Redirect::to("/login")
}

async fn login_page() -> Html<&'static str> {
    Html("<form action='/login' method='post'>
        Username: <input name='username'>
        Password: <input name='password' type='password'>
        <button type='submit'>Login</button>
    </form>")
}

async fn login(State(db): State<Database>, Form(form): Form<LoginForm>) -> Redirect {
    let users = db.collection::<mongodb::bson::Document>("users");

    let filter = doc! { "username": &form.username };
    if let Some(user_doc) = users.find_one(filter).await.unwrap() {
        let stored_password = user_doc.get_str("password").unwrap();
        if verify(&form.password, stored_password).unwrap() {
            let secret = env::var("JWT_SECRET").unwrap_or("dev_secret".to_string());
            let claims = Claims {
                sub: form.username.clone(),
                exp: 2000000000,
            };

            let token = encode(
                &Header::default(),
                &claims,
                &EncodingKey::from_secret(secret.as_bytes()),
            )
            .unwrap();

            println!("✅ Logged in! Token: {}", token);
            return Redirect::to("/dashboard");
        }
    }

    Redirect::to("/login")
}

pub fn router() -> Router<Database> {
    Router::new()
        .route("/login", get(login_page).post(login))
        .route("/signup", get(signup_page).post(signup))
}
