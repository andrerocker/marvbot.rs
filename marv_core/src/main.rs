mod marv;
use marv::{
    engine,
    plugins::{
        self,
        core::{hello_fast::HelloFast, hello_slow::HelloSlow, spam::Spam},
    },
};
use marv_plugins::{
    ask_chatgpt::AskChatGPT,
    kafka::{consumer::KafkaConsumer, producer::KafkaProducer},
    todo::Todo,
};

#[tokio::main]
pub async fn main() {
    engine::initialize().await;
    plugins::initialize(vec![
        KafkaProducer::new(),
        KafkaConsumer::new(),
        Todo::new(),
        HelloFast::new(),
        HelloSlow::new(),
        AskChatGPT::new(),
        Spam::new(),
    ]);

    match engine::execute().await {
        Ok(_) => log::info!("Bye :D"),
        Err(error) => log::error!("Service stopped: {}", error),
    }
}
