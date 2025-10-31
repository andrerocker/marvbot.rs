use super::service::TodoService;
use crate::marv::{models::TodoAdapter, plugins::helper};
use std::{collections::HashMap, io::Result};

pub struct TodoController {
    pub service: TodoService,
}

impl TodoController {
    pub fn create(&mut self, metadata: HashMap<String, String>) -> Result<Vec<String>> {
        let message = helper::safe_get(&metadata, "argument")?;
        let todo = TodoAdapter::to_create(message)?;

        match self.service.create(todo) {
            Ok(_) => helper::simple_channel_user_message(metadata, "created!"),
            Err(error) => {
                helper::simple_channel_user_message(metadata, &format!("Failed! {}", error))
            }
        }
    }

    pub fn update(&mut self, metadata: HashMap<String, String>) -> Result<Vec<String>> {
        let message = helper::safe_get(&metadata, "argument")?;
        let todo = TodoAdapter::to_update(message)?;

        match self.service.update(todo) {
            Ok(_) => helper::simple_channel_user_message(metadata, "updated!"),
            Err(error) => {
                helper::simple_channel_user_message(metadata, &format!("Failed! {}", error))
            }
        }
    }

    pub fn list(&mut self, metadata: HashMap<String, String>) -> Result<Vec<String>> {
        match self.service.list() {
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

    pub fn delete(&mut self, metadata: HashMap<String, String>) -> Result<Vec<String>> {
        let message = helper::safe_get(&metadata, "argument")?;

        match self.service.delete(&message) {
            Ok(_) => helper::simple_channel_user_message(metadata, "deleted!"),
            Err(error) => {
                helper::simple_channel_user_message(metadata, &format!("Failed! {}", error))
            }
        }
    }

    pub fn default(&self, metadata: HashMap<String, String>, message: &str) -> Result<Vec<String>> {
        Ok(vec![helper::channel_message(metadata, message)?])
    }

    pub fn dispatch(&mut self, message: &String) -> Result<Vec<String>> {
        let pattern = r"^:(?<nick>\w+)!(?<name>\w+)@(?<server>\w+.+) PRIVMSG #(?<channel>\w+) :todo: (?<command>\w+)(: (?<argument>.*))?";
        let metadata = helper::regex_to_map(pattern, message);
        let command = helper::safe_get(&metadata, "command")?;

        match command.as_str() {
            "create" => self.create(metadata),
            "list" => self.list(metadata),
            "update" => self.update(metadata),
            "delete" => self.delete(metadata),
            _ => self.default(metadata, "Nothing to do!"),
        }
    }
}
