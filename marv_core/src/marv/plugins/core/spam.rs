use async_trait::async_trait;
use maplit::hashmap;
use marv_api::{
    helper,
    plugins::{DynamicPlugin, Plugin},
};
use std::io::Error;

pub struct Spam {}

impl Spam {
    pub fn new() -> DynamicPlugin {
        Box::new(Spam {})
    }
}

#[async_trait]
impl Plugin for Spam {
    fn name(&self) -> String {
        "Spam".into()
    }

    fn schedule(&self) -> Option<String> {
        Some("0 * * * * *".into())
    }

    fn responds_to(&self, _: &String) -> bool {
        false
    }

    async fn perform(&self, _: &String) -> Result<Vec<String>, Error> {
        let metadata = hashmap! {
            "nick".to_string() => "bacon".to_string(),
            "name".to_string() => "malcolm".to_string(),
            "channel".to_string() => "acme".to_string()

        };

        helper::simple_channel_message(
            &metadata,
            "Curso online no precinho! tigrinho ta pagando, bora!?",
        )
    }
}
