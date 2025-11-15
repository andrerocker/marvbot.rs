use async_trait::async_trait;
use marv_api::plugins::{DynamicPlugin, Plugin};
use std::io::Error;

pub struct Logger {}

impl Logger {
    pub fn new() -> DynamicPlugin {
        Box::new(Logger {})
    }
}

#[async_trait]
impl Plugin for Logger {
    fn name(&self) -> String {
        "Logger".into()
    }

    async fn is_enabled(&self, _message: &String) -> bool {
        true
    }

    async fn perform(&mut self, message: &String) -> Result<Vec<String>, Error> {
        print!("<-- {}", message);
        Ok(vec![])
    }
}
