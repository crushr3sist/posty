use axum::Json;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct UserQuery {
    pub user_id: String,
}

#[derive(Serialize)]
pub struct UserResponse {
    pub username: String,
    pub email: String,
}

pub async fn get_user(query: axum::extract::Query<UserQuery>) -> Json<UserResponse> {
    let user = UserResponse {
        username: format!("user_{}", query.user_id),
        email: format!("user_{}@example.com", query.user_id),
    };
    Json(user)
}
