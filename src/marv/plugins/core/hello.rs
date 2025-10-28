use std::io::Error;

use regex::Regex;

use crate::marv::{config::MarvSetup, plugins::Plugin};

pub struct Hello {}

impl Hello {
    pub fn new(_setup: &MarvSetup) -> Box<dyn Plugin> {
        Box::new(Hello {})
    }
}

impl Plugin for Hello {
    fn name(&self) -> String {
        return "Hello".to_string();
    }

    fn is_enabled(&self, message: &String) -> bool {
        return message.contains(" JOIN :");
    }

    fn perform(&mut self, message: &String) -> Result<Vec<String>, Error> {
        let pattern = r"^:(?<nick>\w+)!(?<name>\w+)@(?<server>\w+.+) JOIN :#(?<channel>\w+)";
        let regex = Regex::new(pattern).unwrap();
        let metadata = regex.captures(message).unwrap();

        match &metadata["nick"] {
            "marvy" => Ok(vec![format!(
                "PRIVMSG #{} : Salveeeee doideeraa!\r\n",
                &metadata["channel"]
            )]),
            _ => Ok(vec![format!(
                "PRIVMSG #{} :{}: Salveeeee doideeraa!\r\n",
                &metadata["channel"], &metadata["nick"]
            )]),
        }
    }
}
