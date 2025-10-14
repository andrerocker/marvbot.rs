use log::info;

use crate::marv::plugins::Plugin;

pub struct Channel {
    pub channel: String,
}

impl Plugin for Channel {
    fn initialize(&mut self, _setup: &crate::marv::config::MarvSetup) {}

    fn is_enabled(&self, message: &String) -> bool {
        return message.contains("End of message of the da");
    }

    fn perform(&self, _message: &String) -> Vec<String> {
        info!("--> Executando Channel");
        return vec![format!("JOIN {}\r\n", self.channel)];
    }
}
