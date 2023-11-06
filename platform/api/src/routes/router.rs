use axum::Router;
use super::account;

pub fn api() -> Router {
    return Router::new()
        .nest("/account", account::router());
}