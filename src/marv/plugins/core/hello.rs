use crate::marv::{
    config::MarvSetup,
    plugins::{Plugin, helper},
};
use std::io::Error;

pub struct Hello {}

impl Hello {
    pub fn new(_setup: &MarvSetup) -> Box<dyn Plugin> {
        Box::new(Hello {})
    }
}

impl Plugin for Hello {
    fn name(&self) -> String {
        "Hello".to_string()
    }

    fn is_enabled(&self, message: &String) -> bool {
        message.contains(" JOIN :")
    }

    fn perform(&mut self, message: &String) -> Result<Vec<String>, Error> {
        let pattern = r"^:(?<nick>\w+)!(?<name>\w+)@(?<server>\w+.+) JOIN :#(?<channel>\w+)";
        let metadata = helper::regex_to_map(pattern, message);

        match metadata["nick"].as_str() {
            "marvy" => helper::simple_channel_message(metadata, "Salveeeee doideeraada!"),
            _ => helper::simple_channel_user_message(metadata, "Yo doideeraa!\r\n"),
        }
    }
}
