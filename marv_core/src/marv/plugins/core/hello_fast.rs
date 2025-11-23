use async_trait::async_trait;
use marv_api::{
    helper,
    plugins::{DynamicPlugin, Plugin},
};
use std::{io::Error, time::Duration};
use tokio::time::sleep;

pub struct HelloFast {}

impl HelloFast {
    pub fn new() -> DynamicPlugin {
        Box::new(HelloFast {})
    }
}

#[async_trait]
impl Plugin for HelloFast {
    fn name(&self) -> String {
        "HelloFast".into()
    }

    fn responds_to(&self, message: &String) -> bool {
        message.to_lowercase().contains("hello")
    }

    async fn perform(&self, message: &String) -> Result<Vec<String>, Error> {
        log::info!("FAST - Executando");
        sleep(Duration::from_secs(10)).await;
        log::info!("FAST - DONE");

        let pattern = r"^:(?<nick>\w+)!(?<name>\w+)@(?<server>\w+.+) PRIVMSG #(?<channel>\w+) :(?<extras>\w+.+)";
        let metadata = helper::regex_to_map(pattern, message);

        match metadata["nick"].as_str() {
            "marvy" => helper::simple_channel_message(&metadata, "[fast] Salveeeee doideeraada!"),
            _ => helper::simple_channel_user_message(&metadata, "[fast] Yo doideeraa!\r\n"),
        }
    }
}
