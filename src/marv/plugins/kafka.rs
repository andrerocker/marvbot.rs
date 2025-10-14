use crate::marv::config::MarvSetup;

use super::Plugin;

pub struct KafkaForwarder {
    pub broker: String,
    pub topic: String,
}

impl Plugin for KafkaForwarder {
    fn initialize(&mut self, setup: &MarvSetup) {
        self.broker = setup.config.broker.clone().to_string();
    }

    fn is_enabled(&self, message: &String) -> bool {
        return message.contains(" JOIN :");
    }

    fn perform(&self, _message: &String) -> Vec<String> {
        return vec![];
    }
}
