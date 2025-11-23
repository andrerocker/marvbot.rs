use std::io::Error;

use async_trait::async_trait;
use log::info;

use marv_api::{
    config,
    plugins::{DynamicPlugin, Plugin},
};

pub struct Channel {
    pub channel: String,
}

impl Channel {
    pub fn new() -> DynamicPlugin {
        let config = config::config();

        Box::new(Channel {
            channel: config.channel.clone(),
        })
    }
}

#[async_trait]
impl Plugin for Channel {
    fn name(&self) -> String {
        "Channel".into()
    }

    fn responds_to(&self, message: &String) -> bool {
        message.contains("End of message of the da")
    }

    async fn perform(&self, _message: &String) -> Result<Vec<String>, Error> {
        info!("--> Executando Channel");
        Ok(vec![format!("JOIN {}\r\n", self.channel)])
    }
}
