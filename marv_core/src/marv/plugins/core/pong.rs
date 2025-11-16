use async_trait::async_trait;
use marv_api::plugins::{DynamicPlugin, Plugin};
use std::io::{Error, ErrorKind};

pub struct Pong {}

impl Pong {
    pub fn new() -> DynamicPlugin {
        Box::new(Pong {})
    }
}

#[async_trait]
impl Plugin for Pong {
    fn name(&self) -> String {
        "Pong".into()
    }

    fn is_enabled(&self, message: &String) -> bool {
        message.contains("PING")
    }

    async fn perform(&mut self, message: &String) -> Result<Vec<String>, Error> {
        let code = message
            .split(":")
            .last()
            .ok_or(Error::new(
                ErrorKind::Other,
                "Problems trying to extract :host from PONG message",
            ))?
            .trim();

        Ok(vec![format!("PONG :{}\r\n", code)])
    }
}
