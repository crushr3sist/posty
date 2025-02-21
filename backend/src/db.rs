use sea_orm::{Database, DatabaseConnection, DbErr};
use std::env;

pub async fn establish_connection() -> Result<DatabaseConnection, DbErr> {
    Database::connect(env::var("DATABASE_URL").expect("DATABASE_URL must be set")).await
}
