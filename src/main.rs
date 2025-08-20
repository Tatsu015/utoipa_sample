use axum::{Json, Router, routing::get};
use serde::Serialize;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route("/", get(root))
        .route("/user", get(get_user));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root() -> &'static str {
    "Hello"
}

#[derive(Serialize)]
struct User {
    id: u8,
    name: &'static str,
}

async fn get_user() -> Json<User> {
    let user = User {
        id: 1,
        name: "sample_name",
    };
    Json(user)
}
