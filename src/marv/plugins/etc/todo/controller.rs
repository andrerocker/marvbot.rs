use crate::marv::plugins::etc::todo::helper;
use std::{collections::HashMap, io::Error};

fn create(metadata: HashMap<String, String>) -> Result<Vec<String>, Error> {
    return Ok(vec![helper::channel_user_message(metadata, "created!")]);
}

pub fn dispatch(message: &String) -> Result<Vec<String>, Error> {
    let pattern = r"^:(?<nick>\w+)!(?<name>\w+)@(?<server>\w+.+) PRIVMSG #(?<channel>\w+) :todo: (?<command>\w+): (?<argument>.*)";
    let metadata = helper::regex_to_map(pattern, message);
    let command = metadata.get("command").unwrap().as_str();

    return match command {
        "create" => create(metadata),
        _ => Ok(vec![helper::channel_message(metadata, "Nothing to do!")]),
    };
}
