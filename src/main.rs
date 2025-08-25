use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use std::sync::{Arc, RwLock};
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;
use utoipa_axum::routes;
use utoipa_swagger_ui::SwaggerUi;

use axum::{Json, extract::Path, http::StatusCode, response::IntoResponse};

use once_cell::sync::Lazy;

static USERS: Lazy<Arc<RwLock<HashMap<u64, User>>>> =
    Lazy::new(|| Arc::new(RwLock::new(HashMap::new())));

#[tokio::main]
async fn main() {
    let (router, api) = OpenApiRouter::new()
        .routes(routes!(get_user, create_user))
        .split_for_parts();

    let app = axum::Router::new()
        .merge(router)
        .merge(SwaggerUi::new("/docs").url("/apidoc/openapi.json", api));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[utoipa::path(get, path = "/", tag = "root_tag", description = "root description",
responses ((
    status = 200,
    body = &'static str
)))]
async fn root() -> &'static str {
    "Hello"
}

#[derive(ToSchema, Deserialize, Serialize, Clone)]
struct User {
    id: u64,
    name: String,
}

#[utoipa::path(get, path = "/users/{id}", tag = "user_tag", description = "user description", responses((status = 200, body = User)), params(("id"=u64, Path, description = "find user id")))]
async fn get_user(Path(id): Path<u64>) -> Result<Json<User>, StatusCode> {
    let users = USERS.read().unwrap();
    users
        .get(&id)
        .map(|user| Ok(Json(user.clone())))
        .unwrap_or(Err(StatusCode::NOT_FOUND))
}

#[utoipa::path(
    post,
    path = "/users",
    request_body = User,
    responses(
        (status = 201, description = "User created", body = User)
    )
)]
async fn create_user(Json(user): Json<User>) -> impl IntoResponse {
    let mut db = USERS.write().unwrap();
    db.insert(user.id, user.clone());
    (StatusCode::CREATED, Json(user))
}
// async fn create_user(Json(user): Json<User>) -> impl IntoResponse {
//     let mut db = users.write().unwrap();
//     db.insert(user.id as u64, user.clone());
//     (StatusCode::CREATED, Json(user))
// }

// #[utoipa::path(post, path = "/user", tag = "user_tag", description = "create user description", request_body = User, responses((status = 201, body = User)))]
// async fn create_user(Json(user): Json<User>) -> (axum::http::StatusCode, Json<User>) {
//     let mut db = users.write().unwrap();
//     db.insert(user.id as u64, user.clone());
//     (axum::http::StatusCode::CREATED, Json(user))
// }

// async fn create_user(Json(user): Json<User>) -> Json<User> {
//     let mut db = users.write().unwrap();
//     db.insert(user.id, user);
// }

// async fn create_user(Json(user): Json<User>) -> Json<User> {
//     let mut db = users.write().unwrap();
//     db.insert(user.id as u64, user.clone());
//     Json(user)
// }

// use std::io;
// use std::net::Ipv4Addr;

// use tokio::net::TcpListener;
// use utoipa::OpenApi;
// use utoipa_axum::router::OpenApiRouter;
// use utoipa_axum::routes;
// use utoipa_swagger_ui::SwaggerUi;

// const CUSTOMER_TAG: &str = "customer";
// const ORDER_TAG: &str = "order";

// #[derive(OpenApi)]
// #[openapi(
//     tags(
//         (name = CUSTOMER_TAG, description = "Customer API endpoints"),
//         (name = ORDER_TAG, description = "Order API endpoints")
//     )
// )]
// struct ApiDoc;

// /// Get health of the API.
// #[utoipa::path(
//     method(get, head),
//     path = "/api/health",
//     responses(
//         (status = OK, description = "Success", body = str, content_type = "text/plain")
//     )
// )]
// async fn health() -> &'static str {
//     "ok"
// }

// #[tokio::main]
// async fn main() -> Result<(), io::Error> {
//     let (router, api) = OpenApiRouter::with_openapi(ApiDoc::openapi())
//         .routes(routes!(health))
//         .nest("/api/customer", customer::router())
//         .nest("/api/order", order::router())
//         .routes(routes!(
//             inner::secret_handlers::get_secret,
//             inner::secret_handlers::post_secret
//         ))
//         .split_for_parts();

//     let router = router.merge(SwaggerUi::new("/swagger-ui").url("/apidoc/openapi.json", api));

//     let listener = TcpListener::bind((Ipv4Addr::LOCALHOST, 8080)).await?;
//     axum::serve(listener, router).await
// }

// mod customer {
//     use axum::Json;
//     use serde::Serialize;
//     use utoipa::ToSchema;
//     use utoipa_axum::router::OpenApiRouter;
//     use utoipa_axum::routes;

//     /// This is the customer
//     #[derive(ToSchema, Serialize)]
//     struct Customer {
//         name: String,
//     }

//     /// expose the Customer OpenAPI to parent module
//     pub fn router() -> OpenApiRouter {
//         OpenApiRouter::new().routes(routes!(get_customer))
//     }

//     /// Get customer
//     ///
//     /// Just return a static Customer object
//     #[utoipa::path(get, path = "", responses((status = OK, body = Customer)), tag = super::CUSTOMER_TAG)]
//     async fn get_customer() -> Json<Customer> {
//         Json(Customer {
//             name: String::from("Bill Book"),
//         })
//     }
// }

// mod order {
//     use axum::Json;
//     use serde::{Deserialize, Serialize};
//     use utoipa::ToSchema;
//     use utoipa_axum::router::OpenApiRouter;
//     use utoipa_axum::routes;

//     /// This is the order
//     #[derive(ToSchema, Serialize)]
//     struct Order {
//         id: i32,
//         name: String,
//     }

//     #[derive(ToSchema, Deserialize, Serialize)]
//     struct OrderRequest {
//         name: String,
//     }

//     /// expose the Order OpenAPI to parent module
//     pub fn router() -> OpenApiRouter {
//         OpenApiRouter::new().routes(routes!(get_order, create_order))
//     }

//     /// Get static order object
//     #[utoipa::path(get, path = "", responses((status = OK, body = Order)), tag = super::ORDER_TAG)]
//     async fn get_order() -> Json<Order> {
//         Json(Order {
//             id: 100,
//             name: String::from("Bill Book"),
//         })
//     }

//     /// Create an order.
//     ///
//     /// Create an order by basically passing through the name of the request with static id.
//     #[utoipa::path(post, path = "", responses((status = OK, body = Order)), tag = super::ORDER_TAG)]
//     async fn create_order(Json(order): Json<OrderRequest>) -> Json<Order> {
//         Json(Order {
//             id: 120,
//             name: order.name,
//         })
//     }
// }

// mod inner {
//     pub mod secret_handlers {

//         /// This is some secret inner handler
//         #[utoipa::path(get, path = "/api/inner/secret", responses((status = OK, body = str)))]
//         pub async fn get_secret() -> &'static str {
//             "secret"
//         }

//         /// Post some secret inner handler
//         #[utoipa::path(post, path = "/api/inner/secret", responses((status = OK)))]
//         pub async fn post_secret() {
//             println!("You posted a secret")
//         }
//     }
// }
