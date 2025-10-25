pub mod controller;
pub mod helper;

use std::io::Error;

use crate::marv::{config::MarvSetup, plugins::Plugin};
use log::info;

pub struct Todo {}

impl Todo {
    pub fn new(_setup: &MarvSetup) -> Box<dyn Plugin> {
        Box::new(Todo {})
    }
}

impl Plugin for Todo {
    fn name(&self) -> String {
        return "Todo".to_string();
    }

    fn is_enabled(&self, message: &String) -> bool {
        return message.contains(" PRIVMSG ") && message.contains(" :todo: ");
    }

    fn perform(&mut self, message: &String) -> Result<Vec<String>, Error> {
        info!("--> Executando Todo");
        let pattern = r"^:(?<nick>\w+)!(?<name>\w+)@(?<server>\w+.+) PRIVMSG #(?<channel>\w+) :todo: (?<command>\w+): (?<argument>.*)";
        let metadata = helper::regex_to_map(pattern, message);

        let response = format!(
            "PRIVMSG #{} - {} - {} - {} - {} - {}\r\n",
            metadata.get("channel").unwrap(),
            metadata.get("name").unwrap(),
            metadata.get("server").unwrap(),
            metadata.get("channel").unwrap(),
            metadata.get("command").unwrap(),
            metadata.get("argument").unwrap(),
        );

        info!("--> Response: {response}");

        return Ok(vec![response]);
    }
}
