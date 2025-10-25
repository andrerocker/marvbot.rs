use std::{collections::HashMap, io::Error};

use crate::marv::plugins::etc::todo::helper;

fn create(metadata: HashMap<String, String>) -> Result<Vec<String>, Error> {
    let response = format!(
        "PRIVMSG #{} - {} created!\r\n",
        metadata.get("channel").unwrap(),
        metadata.get("name").unwrap(),
    );

    return Ok(vec![response]);
}

fn channel_message(metadata: HashMap<String, String>, message: &str) -> String {
    return format!(
        "PRIVMSG #{} - {}\r\n",
        metadata.get("channel").unwrap(),
        message,
    );
}

pub fn dispatch(message: &String) -> Result<Vec<String>, Error> {
    let pattern = r"^:(?<nick>\w+)!(?<name>\w+)@(?<server>\w+.+) PRIVMSG #(?<channel>\w+) :todo: (?<command>\w+): (?<argument>.*)";
    let metadata = helper::regex_to_map(pattern, message);

    return match metadata.get("command").unwrap().as_str() {
        "create" => create(metadata),
        _ => Ok(vec![channel_message(metadata, "Nothing to do!")]),
    };
}
