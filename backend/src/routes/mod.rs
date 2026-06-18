use axum::Router;

mod health;
mod hello;

pub fn router() -> Router {
    Router::new().merge(health::routes()).merge(hello::routes())
}
