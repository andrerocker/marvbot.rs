use crate::marv::plugins::etc::todo::helper;
use std::{collections::HashMap, io::Error};

fn create(metadata: HashMap<String, String>) -> Result<Vec<String>, Error> {
    Ok(vec![helper::channel_user_message(metadata, "created!")?])
}

fn default(metadata: HashMap<String, String>, message: &str) -> Result<Vec<String>, Error> {
    Ok(vec![helper::channel_message(metadata, message)?])
}

pub fn dispatch(message: &String) -> Result<Vec<String>, Error> {
    let pattern = r"^:(?<nick>\w+)!(?<name>\w+)@(?<server>\w+.+) PRIVMSG #(?<channel>\w+) :todo: (?<command>\w+): (?<argument>.*)";
    let metadata = helper::regex_to_map(pattern, message);
    let command = metadata.get("command").unwrap().as_str();

    return match command {
        "create" => create(metadata),
        _ => default(metadata, "Nothing to do!"),
    };
}
