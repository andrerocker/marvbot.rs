mod marv;

use diesel_async::AsyncPgConnection;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use diesel_async::pooled_connection::bb8::Pool;
use env_logger;
use marv::plugins::helper;
use marv::{config, engine};
use prometheus_exporter;

async fn connection() -> Pool<AsyncPgConnection> {
    let database_url = config::config().database_url.clone();
    let manager = AsyncDieselConnectionManager::<AsyncPgConnection>::new(database_url);
    let pool: Pool<AsyncPgConnection> = Pool::builder().build(manager).await.unwrap();

    pool
}

async fn initialize() -> anyhow::Result<()> {
    env_logger::init();
    prometheus_exporter::start("127.0.0.1:9184".parse().unwrap()).or_else(
        helper::create_closure_error("Can't initialize Prometheus Exporter"),
    )?;

    let _ = config::MARV.set(config::read_configuration()?);
    let _ = config::POOL.set(connection().await);

    Ok(())
}
#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    initialize().await?;
    engine::start().await
}
