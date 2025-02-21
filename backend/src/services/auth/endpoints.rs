use super::auth_handler;
use axum::{routing::get, Router};

pub fn create_routes() -> Router {
    Router::new()
        .route("/login", get(|| async { "login" }))
        .route("/register", get(|| async { "register" }))
        .route("/user", get(auth_handler::get_user))
}
