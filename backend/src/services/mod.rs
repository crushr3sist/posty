pub mod auth;

use axum::Router;

pub fn build_endpoints() -> Router {
    Router::new().nest("/auth", auth::endpoints::create_routes())
}
