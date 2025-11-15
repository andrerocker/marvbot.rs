mod marv;
use marv::engine;

#[tokio::main]
pub async fn main() {
    engine::initialize().await;

    match engine::execute().await {
        Ok(_) => log::info!("Bye :D"),
        Err(error) => log::error!("Service stopped: {}", error),
    }
}
