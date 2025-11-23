use async_trait::async_trait;
use marv_api::{
    helper,
    plugins::{DynamicPlugin, Plugin},
};
use std::io::Error;

pub struct AskChatGPT {}

impl AskChatGPT {
    pub fn new() -> DynamicPlugin {
        Box::new(AskChatGPT {})
    }
}

#[async_trait]
impl Plugin for AskChatGPT {
    fn name(&self) -> String {
        "AskChatGPT".into()
    }

    fn responds_to(&self, message: &String) -> bool {
        let pattern = r"^:(?<nick>\w+)!(?<name>\w+)@(?<server>\w+.+) PRIVMSG #(?<channel>\w+) :marvy: (?<command>\w+.+)";
        let metadata = helper::regex_to_map(pattern, message);

        !metadata.is_empty()
    }

    async fn perform(&self, message: &String) -> Result<Vec<String>, Error> {
        let pattern = r"^:(?<nick>\w+)!(?<name>\w+)@(?<server>\w+.+) PRIVMSG #(?<channel>\w+) :marvy: (?<command>\w+.+)";
        let metadata = helper::regex_to_map(pattern, message);

        helper::simple_channel_user_message(&metadata, "Salve meu trutão")
    }
}
