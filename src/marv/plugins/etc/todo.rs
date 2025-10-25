use std::{collections::HashMap, hash::Hash};

use kafka::client::metadata;
use log::info;
use regex::{Captures, Regex};

use crate::marv::{config::MarvSetup, plugins::Plugin};

pub struct Todo {}

impl Todo {
    pub fn new(_setup: &MarvSetup) -> Box<dyn Plugin> {
        Box::new(Todo {})
    }

    pub fn extract_metadata(&self, message: &String) -> HashMap<String, String> {
        let pattern = r"^:(?<nick>\w+)!(?<name>\w+)@(?<server>\w+.+) PRIVMSG #(?<channel>\w+) :todo: (?<command>\w+): (?<argument>.*)";
        let regex = Regex::new(pattern).expect("Problems trying to initialize Regex pattern");
        let mut results: HashMap<String, String> = HashMap::new();

        for caps in regex.captures_iter(message) {
            for name in regex.capture_names() {
                if let Some(name_str) = name {
                    if let Some(matched_value) = caps.name(name_str) {
                        results.insert(name_str.to_string(), matched_value.as_str().to_string());
                    }
                }
            }
        }

        return results;
    }
}

impl Plugin for Todo {
    fn name(&self) -> String {
        return "Todo".to_string();
    }

    fn is_enabled(&self, message: &String) -> bool {
        return message.contains(" PRIVMSG ") && message.contains(" :todo: ");
    }

    fn perform(&mut self, message: &String) -> Vec<String> {
        info!("--> Executando Todo");
        let metadata = self.extract_metadata(message);

        let response = format!(
            "PRIVMSG #{} - {} \r\n",
            metadata.get("channel").unwrap(),
            metadata.get("command").unwrap()
        );

        return vec![response];
    }
}
