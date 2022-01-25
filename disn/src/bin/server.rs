use disn::config;
use tracing_subscriber::EnvFilter;

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .pretty()
        .init();

    use config::db::DbPool;
    let pg_pool = sqlx::PgPool::retrieve().await;

    let server = disn::server(pg_pool);

    if let Err(err) = server.await {
        tracing::error!("server error : {:?}", err);
    }
}
