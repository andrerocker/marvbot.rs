use super::repository::TodoRepository;
use crate::marv::plugins::helper;
use std::{collections::HashMap, io::Error};

pub struct TodoController {
    pub repository: TodoRepository,
}

impl TodoController {
    pub fn create(&mut self, metadata: HashMap<String, String>) -> Result<Vec<String>, Error> {
        let message = helper::safe_get(&metadata, "argument")?;

        match self.repository.create(&message) {
            Ok(_) => helper::simple_channel_user_message(metadata, "created!"),
            Err(error) => {
                helper::simple_channel_user_message(metadata, &format!("Failed! {}", error))
            }
        }
    }

    pub fn list(&mut self, metadata: HashMap<String, String>) -> Result<Vec<String>, Error> {
        match self.repository.list() {
            Ok(response) => {
                let formatted = response
                    .into_iter()
                    .map(|current| {
                        helper::channel_user_message(&metadata, &current.to_string()).unwrap()
                    })
                    .collect::<Vec<String>>();

                if formatted.len() > 0 {
                    Ok(formatted)
                } else {
                    helper::simple_channel_user_message(metadata, "The're no :Todos to list")
                }
            }
            Err(error) => helper::simple_channel_user_message(
                metadata,
                &format!("Failed listing Todos: {}", error),
            ),
        }
    }

    pub fn delete(&mut self, metadata: HashMap<String, String>) -> Result<Vec<String>, Error> {
        let message = helper::safe_get(&metadata, "argument")?;

        match self.repository.delete(&message) {
            Ok(_) => helper::simple_channel_user_message(metadata, "deleted!"),
            Err(error) => {
                helper::simple_channel_user_message(metadata, &format!("Failed! {}", error))
            }
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
        let pattern = r"^:(?<nick>\w+)!(?<name>\w+)@(?<server>\w+.+) PRIVMSG #(?<channel>\w+) :todo: (?<command>\w+)(: (?<argument>.*))?";
        let metadata = helper::regex_to_map(pattern, message);
        let command = helper::safe_get(&metadata, "command")?;

        match command.as_str() {
            "create" => self.create(metadata),
            "list" => self.list(metadata),
            "delete" => self.delete(metadata),
            _ => self.default(metadata, "Nothing to do!"),
        }
    }
}
