use async_trait::async_trait;

use crate::marv::plugins::{DynamicPlugin, Plugin, helper};
use std::io::Error;

pub struct Hello {}

impl Hello {
    pub fn new() -> DynamicPlugin {
        Box::new(Hello {})
    }
}

#[async_trait]
impl Plugin for Hello {
    fn name(&self) -> String {
        "Hello".into()
    }

    async fn is_enabled(&self, message: &String) -> bool {
        message.contains(" JOIN :")
    }

    async fn perform(&mut self, message: &String) -> Result<Vec<String>, Error> {
        let pattern = r"^:(?<nick>\w+)!(?<name>\w+)@(?<server>\w+.+) JOIN :#(?<channel>\w+)";
        let metadata = helper::regex_to_map(pattern, message);

        match metadata["nick"].as_str() {
            "marvy" => helper::simple_channel_message(&metadata, "Salveeeee doideeraada!"),
            _ => helper::simple_channel_user_message(&metadata, "Yo doideeraa!\r\n"),
        }
    }
}
