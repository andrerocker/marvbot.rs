mod marv;
use marv::engine;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    engine::initialize().await;
    engine::start().await
}
