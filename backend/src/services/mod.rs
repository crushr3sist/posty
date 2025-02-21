pub mod auth;

use std::sync::Arc;

use axum::Router;

use crate::db;

pub async fn build_endpoints() -> Router {
    let conn = db::establish_connection().await.unwrap();
    let shared_db = Arc::new(conn);

    Router::new()
        .with_state(shared_db)
        .nest("/auth", auth::endpoints::create_routes())
}
