use async_trait::async_trait;
use marv_api::{
    helper,
    plugins::{DynamicPlugin, Plugin},
};
use std::{io::Error, time::Duration};
use tokio::time::sleep;

pub struct HelloSlow {}

impl HelloSlow {
    pub fn new() -> DynamicPlugin {
        Box::new(HelloSlow {})
    }
}

#[async_trait]
impl Plugin for HelloSlow {
    fn name(&self) -> String {
        "HelloSlow".into()
    }

    fn schedule(&self) -> Option<String> {
        None
    }

    fn responds_to(&self, message: &String) -> bool {
        message.to_lowercase().contains("hello")
    }

    async fn perform(&self, message: &String) -> Result<Vec<String>, Error> {
        log::info!("SLOW - Executando");
        sleep(Duration::from_secs(20)).await;
        log::info!("SLOW - DONE");

        let pattern = r"^:(?<nick>\w+)!(?<name>\w+)@(?<server>\w+.+) PRIVMSG #(?<channel>\w+) :(?<extras>\w+.+)";
        let metadata = helper::regex_to_map(pattern, message);

        match metadata["nick"].as_str() {
            "marvy" => helper::simple_channel_message(&metadata, "[slow] Salveeeee doideeraada!"),
            _ => helper::simple_channel_user_message(&metadata, "[slow] Yo doideeraa!\r\n"),
        }
    }
}
