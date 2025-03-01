use dotenvy::dotenv;
use posty::*;
use sea_orm_migration::MigratorTrait;
use std::{env, net::SocketAddr};
use tokio::signal;

#[tokio::main]
async fn main() {
    dotenv().ok();

    env::set_var(
        "DATABASE_URL",
        "postgresql://postgres:ronny@localhost:5432/postgres",
    );

    // Run migrations
    let db_conn = db::establish_connection()
        .await
        .expect("Failed to connect to the database");

    migration::Migrator::up(&db_conn, None)
        .await
        .expect("Failed to run migrations");

    let app = services::build_endpoints().await;

    let addr = SocketAddr::from(([127, 0, 0, 1], 8000));
    println!("listening on http://{}", addr);
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.unwrap();

    async fn shutdown_signal() {
        let ctrl_c = async {
            signal::ctrl_c()
                .await
                .expect("failed to install Ctrl+C handler");
        };

        #[cfg(unix)]
        let terminate = async {
            signal::unix::signal(signal::unix::SignalKind::terminate())
                .expect("failed to install signal handler")
                .recv()
                .await;
        };

        #[cfg(not(unix))]
        let terminate = std::future::pending::<()>();

        tokio::select! {
            _ = ctrl_c => {},
            _ = terminate => {},
        }
        println!("signal received, starting graceful shutdown");
    }
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .unwrap();
}
