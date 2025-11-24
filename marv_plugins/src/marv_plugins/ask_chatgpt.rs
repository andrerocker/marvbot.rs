use async_trait::async_trait;
use marv_api::{
    helper,
    plugins::{DynamicPlugin, Plugin},
};
use serde_json::{Value, json};
use std::{collections::HashMap, io::Error};

pub struct AskChatGPT {}

impl AskChatGPT {
    pub fn new() -> DynamicPlugin {
        Box::new(AskChatGPT {})
    }
}

fn build_metadata(message: &String) -> HashMap<String, String> {
    let pattern = r"^:(?<nick>\w+)!(?<name>\w+)@(?<server>\w+.+) PRIVMSG #(?<channel>\w+) :marvy: (?<command>\w+.+)";
    helper::regex_to_map(pattern, message)
}

async fn ask_chat(question: &str) -> String {
    let url = "https://api.openai.com/v1/chat/completions";

    let payload = serde_json::to_string(&json!({
        "model": "gpt-4.1-mini",
        "messages": [{
            "role": "user",
            "content": question
        }]
    }))
    .unwrap();

    log::info!("chatgpt request: {:?}", payload);

    let client = reqwest::Client::new()
        .post(url)
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer hack3d")
        .body(payload)
        .send()
        .await
        .unwrap();

    let response = client.text().await.unwrap();

    log::info!("chatgpt response: {:?}", response);

    response
}

#[async_trait]
impl Plugin for AskChatGPT {
    fn name(&self) -> String {
        "AskChatGPT".into()
    }

    fn responds_to(&self, message: &String) -> bool {
        !build_metadata(message).is_empty()
    }

    async fn perform(&self, message: &String) -> Result<Vec<String>, Error> {
        let metadata = build_metadata(message);
        let question = helper::safe_get(&metadata, "command")?;
        let raw_response = ask_chat(&question).await;
        let parsed: HashMap<String, Value> = serde_json::from_str(&raw_response).unwrap();

        let response = parsed
            .get("choices")
            .unwrap()
            .as_array()
            .unwrap()
            .get(0)
            .unwrap()
            .get("message")
            .unwrap()
            .get("content")
            .unwrap()
            .as_str()
            .unwrap();

        helper::simple_channel_user_message(&metadata, response)
    }
}
