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

    fn schedule(&self) -> Option<String> {
        None
    }

    fn responds_to(&self, _message: &String) -> bool {
        true
    }

    async fn perform(&self, message: &String) -> Result<Vec<String>, Error> {
        print!("<-- {}", message);
        Ok(vec![])
    }
}
