use axum::{extract::Query, routing::get, Json, Router};
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct HelloParams {
    name: Option<String>,
}

#[derive(Serialize)]
struct HelloResponse {
    message: String,
}

async fn hello(Query(params): Query<HelloParams>) -> Json<HelloResponse> {
    let name = params.name.unwrap_or_else(|| "world".into());
    Json(HelloResponse {
        message: format!("Hello, {}!", name),
    })
}

pub fn routes() -> Router {
    Router::new().route("/hello", get(hello))
}
