use async_trait::async_trait;
use marv_api::{
    helper,
    plugins::{DynamicPlugin, Plugin},
};
use std::{collections::HashMap, io::Error};

pub struct AskChatGPT {}

impl AskChatGPT {
    pub fn new() -> DynamicPlugin {
        Box::new(AskChatGPT {})
    }
}

impl AskChatGPT {
    fn metadata(&self, message: &String) -> HashMap<String, String> {
        let pattern = r"^:(?<nick>\w+)!(?<name>\w+)@(?<server>\w+.+) PRIVMSG #(?<channel>\w+) :marvy: (?<command>\w+.+)";
        helper::regex_to_map(pattern, message)
    }
}

#[async_trait]
impl Plugin for AskChatGPT {
    fn name(&self) -> String {
        "AskChatGPT".into()
    }

    fn responds_to(&self, message: &String) -> bool {
        !self.metadata(message).is_empty()
    }

    async fn perform(&self, message: &String) -> Result<Vec<String>, Error> {
        let metadata = self.metadata(message);
        helper::simple_channel_user_message(&metadata, "Salve meu trutão")
    }
}
