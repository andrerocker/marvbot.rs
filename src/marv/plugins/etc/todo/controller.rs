use super::{adapter::TodoAdapter, service::TodoService};
use crate::marv::plugins::helper;
use std::{collections::HashMap, io::Result};

pub struct TodoController {
    pub service: TodoService,
}

impl TodoController {
    pub fn create(&mut self, metadata: HashMap<String, String>) -> Result<Vec<String>> {
        let message = helper::safe_get(&metadata, "argument")?;
        let todo = TodoAdapter::from_request_to_create(message)?;

        match self.service.create(todo) {
            Ok(_) => helper::simple_channel_user_message(metadata, "created!"),
            Err(error) => {
                helper::simple_channel_user_message(metadata, &format!("Failed! {}", error))
            }
        }
    }

    pub fn update(&mut self, metadata: HashMap<String, String>) -> Result<Vec<String>> {
        let message = helper::safe_get(&metadata, "argument")?;
        let todo = TodoAdapter::from_request_to_update(message)?;

        match self.service.update(todo) {
            Ok(_) => helper::simple_channel_user_message(metadata, "updated!"),
            Err(error) => {
                helper::simple_channel_user_message(metadata, &format!("Failed! {}", error))
            }
        }
    }

    fn current_or_default<T>(&self, current: Vec<T>, default: Vec<T>) -> Result<Vec<T>> {
        if current.len() > 0 {
            Ok(current)
        } else {
            Ok(default)
        }
    }

    pub fn list(&mut self, metadata: HashMap<String, String>) -> Result<Vec<String>> {
        match self.service.list() {
            Ok(todos) => self.current_or_default(
                TodoAdapter::from_todos_to_response(&metadata, todos)?,
                helper::simple_channel_user_message(metadata, "The're no :Todos to list")?,
            ),
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
