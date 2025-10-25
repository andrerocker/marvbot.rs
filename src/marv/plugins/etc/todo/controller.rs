use crate::marv::plugins::etc::todo::helper::{self, safe_get};
use std::{collections::HashMap, io::Error};

use super::repository::TodoRepository;

pub struct TodoController {
    pub repository: TodoRepository,
}

impl TodoController {
    pub fn create(&mut self, metadata: HashMap<String, String>) -> Result<Vec<String>, Error> {
        let message = safe_get(&metadata, "argument")?;

        match self.repository.create(&message) {
            Ok(_) => Ok(vec![helper::channel_user_message(metadata, "created!")?]),
            Err(error) => Ok(vec![helper::channel_user_message(
                metadata,
                &format!("Failed! {}", error),
            )?]),
        }
    }

    pub fn default(
        &self,
        metadata: HashMap<String, String>,
        message: &str,
    ) -> Result<Vec<String>, Error> {
        Ok(vec![helper::channel_message(metadata, message)?])
    }

    pub fn dispatch(&mut self, message: &String) -> Result<Vec<String>, Error> {
        let pattern = r"^:(?<nick>\w+)!(?<name>\w+)@(?<server>\w+.+) PRIVMSG #(?<channel>\w+) :todo: (?<command>\w+): (?<argument>.*)";
        let metadata = helper::regex_to_map(pattern, message);
        let command = metadata.get("command").unwrap().as_str();

        return match command {
            "create" => self.create(metadata),
            _ => self.default(metadata, "Nothing to do!"),
        };
    }
}
