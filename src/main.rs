use axum::{Json, Router, routing::get};
use serde::Serialize;
use utoipa::openapi::OpenApi;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;

#[tokio::main]
async fn main() {
    let (router, api) = OpenApiRouter::new().routes(routes!(root)).split_for_parts();

    let app = router.into_make_service();
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[utoipa::path(get, path = "/")]
async fn root() -> &'static str {
    "Hello"
}

// #[derive(Serialize)]
// struct User {
//     id: u8,
//     name: &'static str,
// }

// async fn get_user() -> Json<User> {
//     let user = User {
//         id: 1,
//         name: "sample_name",
//     };
//     Json(user)
// }
