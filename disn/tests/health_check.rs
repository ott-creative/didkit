//! tests/health_check.rs
use disn::config;
use sqlx::PgPool;

#[tokio::test]
async fn health_check_works() {
    dotenv::dotenv().ok();
    use config::db::DbPool;
    let pg_pool = sqlx::PgPool::retrieve().await;
    // Arrange
    spawn_app(pg_pool);
    // We need to bring in `reqwest`
    // to perform HTTP requests against our application.
    let client = reqwest::Client::new();
    // Act
    let response = client
        .get("http://127.0.0.1:3000/api/v1/health_check")
        .send()
        .await
        .expect("Failed to execute request.");
    // Assert
    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

// Launch our application in the background ~somehow~
fn spawn_app(pg_pool: PgPool) {
    let server = disn::server(pg_pool);
    // Launch the server as a background task
    // tokio::spawn returns a handle to the spawned future,
    // but we have no use for it here, hence the non-binding let
    let _ = tokio::spawn(server);
}
