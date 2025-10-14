use regex::Regex;

use crate::marv::plugins::Plugin;

pub struct Hello {}

fn extract_metadata(message: &String) -> Option<regex::Captures<'_>> {
    let pattern = r"^:(?<nick>\w+)!(?<name>\w+)@(?<server>\w+.+) JOIN :#(?<channel>\w+)";
    let regex = Regex::new(pattern).unwrap();
    return regex.captures(message);
}

impl Plugin for Hello {
    fn initialize(&mut self, _setup: &crate::marv::config::MarvSetup) {}

    fn is_enabled(&self, message: &String) -> bool {
        return message.contains(" JOIN :");
    }

    fn perform(&self, message: &String) -> Vec<String> {
        let metadata = extract_metadata(message).unwrap();
        let response = format!(
            "PRIVMSG #{} :{}: Iaaeee tru!\r\n",
            &metadata["channel"], &metadata["nick"]
        );

        return vec![response];
    }
}
