use std::io::Error;

use log::info;

use crate::marv::{config, plugins::Plugin};

pub struct Channel {
    pub channel: String,
}

impl Channel {
    pub fn new() -> Box<dyn Plugin> {
        let config = &config::CONFIG.lock().unwrap().config;

        Box::new(Channel {
            channel: config.channel.clone(),
        })
    }
}

impl Plugin for Channel {
    fn name(&self) -> String {
        "Channel".to_string()
    }

    fn is_enabled(&self, message: &String) -> bool {
        message.contains("End of message of the da")
    }

    fn perform(&mut self, _message: &String) -> Result<Vec<String>, Error> {
        info!("--> Executando Channel");
        Ok(vec![format!("JOIN {}\r\n", self.channel)])
    }
}
