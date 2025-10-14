use regex::Regex;

use crate::marv::{config::MarvSetup, plugins::Plugin};

pub struct Hello {}

impl Hello {
    pub fn new(_setup: &MarvSetup) -> Box<dyn Plugin> {
        Box::new(Hello {})
    }
}

impl Plugin for Hello {
    fn is_enabled(&self, message: &String) -> bool {
        return message.contains(" JOIN :");
    }

    fn perform(&mut self, message: &String) -> Vec<String> {
        let pattern = r"^:(?<nick>\w+)!(?<name>\w+)@(?<server>\w+.+) JOIN :#(?<channel>\w+)";
        let regex = Regex::new(pattern).unwrap();
        let metadata = regex.captures(message).unwrap();

        let response = format!(
            "PRIVMSG #{} :{}: Iaaeee tru!\r\n",
            &metadata["channel"], &metadata["nick"]
        );

        return vec![response];
    }
}
