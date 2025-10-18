use log::info;

use crate::marv::{config::MarvSetup, plugins::Plugin};

pub struct Channel {
    pub channel: String,
}

impl Channel {
    pub fn new(setup: &MarvSetup) -> Box<dyn Plugin> {
        Box::new(Channel {
            channel: setup.config.channel.to_string(),
        })
    }
}

impl Plugin for Channel {
    fn name(&self) -> String {
        return "Channel".to_string();
    }

    fn is_enabled(&self, message: &String) -> bool {
        return message.contains("End of message of the da");
    }

    fn perform(&mut self, _message: &String) -> Vec<String> {
        info!("--> Executando Channel");
        return vec![format!("JOIN {}\r\n", self.channel)];
    }
}
