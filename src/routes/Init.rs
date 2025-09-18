use axum::{
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};

use serde::{Deserialize, Serialize};


pub (in crate::routes) fn _routes() -> Router {
    let routes = Router::new()
        .route("/", get(root))
        .route("/users", post(create_user));

    routes
}

// basic handler that reponds with a static string
async fn root() -> &'static str {
    "Hello, world!"
}
async fn create_user(
    // this argument tells axum to parse the request body
    // as JSON into a `CreateUser` type
    Json(payload): Json<CreateUser>,
) -> (StatusCode, Json<User>) {
    // insert your application logic here
    let user = User {
        id: 1337,
        username: payload.username,
    };

    // this will be converted into a JSON respons
    // with a status code of 201 Created
    (StatusCode::CREATED, Json(user))
}

// the input our create_user handler
#[derive(Deserialize)]
struct CreateUser {
    username: String,
}

// the output to our `Create_user` handler
#[derive(Serialize)]
struct User {
    id: u64,
    username: String,
}
