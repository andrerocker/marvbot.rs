use marv_api::helper;
use tokio::time::Instant;

use super::{adapter::TodoAdapter, repository::TodoRepository, service::TodoService};
use std::{collections::HashMap, io};

pub fn new() -> TodoController {
    TodoController {
        service: TodoService {
            repository: TodoRepository {},
        },
    }
}

pub struct TodoController {
    pub service: TodoService,
}

impl TodoController {
    pub async fn create(&self, metadata: &HashMap<String, String>) -> io::Result<Vec<String>> {
        let message = helper::safe_get(&metadata, "argument")?;
        let todo = TodoAdapter::from_request_to_create(message)?;

        match self.service.create(todo).await {
            Ok(_) => helper::simple_channel_user_message(&metadata, "created!"),
            Err(error) => {
                helper::simple_channel_user_message(&metadata, &format!("Failed! {}", error))
            }
        }
    }

    pub async fn update(&self, metadata: &HashMap<String, String>) -> io::Result<Vec<String>> {
        let message = helper::safe_get(&metadata, "argument")?;
        let todo = TodoAdapter::from_request_to_update(message)?;

        match self.service.update(todo).await {
            Ok(_) => helper::simple_channel_user_message(&metadata, "updated!"),
            Err(error) => {
                helper::simple_channel_user_message(&metadata, &format!("Failed! {}", error))
            }
        }
    }

    fn current_or_default<T>(&self, current: Vec<T>, default: Vec<T>) -> io::Result<Vec<T>> {
        if current.len() > 0 {
            Ok(current)
        } else {
            Ok(default)
        }
    }

    pub async fn list(&self, metadata: &HashMap<String, String>) -> io::Result<Vec<String>> {
        match self.service.list().await {
            Ok(todos) => self.current_or_default(
                TodoAdapter::from_todos_to_response(&metadata, todos)?,
                helper::simple_channel_user_message(&metadata, "The're no :Todos to list")?,
            ),
            Err(error) => helper::simple_channel_user_message(
                &metadata,
                &format!("Failed listing Todos: {}", error),
            ),
        }
    }

    pub async fn delete(&self, metadata: &HashMap<String, String>) -> io::Result<Vec<String>> {
        let message = helper::safe_get(&metadata, "argument")?;
        let id = TodoAdapter::from_request_to_delete(message)?;

        match self.service.delete(id).await {
            Ok(_) => helper::simple_channel_user_message(&metadata, "deleted!"),
            Err(error) => {
                helper::simple_channel_user_message(&metadata, &format!("Failed! {}", error))
            }
        }
    }

    pub async fn default(
        &self,
        metadata: &HashMap<String, String>,
        message: &str,
    ) -> io::Result<Vec<String>> {
        Ok(vec![helper::channel_message(&metadata, message)?])
    }

    pub async fn dispatch(&self, message: &String) -> io::Result<Vec<String>> {
        let started = Instant::now();
        let pattern = r"^:(?<nick>\w+)!(?<name>\w+)@(?<server>\w+.+) PRIVMSG #(?<channel>\w+) :todo: (?<command>\w+)(: (?<argument>.*))?";
        let metadata = helper::regex_to_map(pattern, message);
        let command = helper::safe_get(&metadata, "command")?;

        let mut result = match command.as_str() {
            "create" => self.create(&metadata).await?,
            "list" => self.list(&metadata).await?,
            "update" => self.update(&metadata).await?,
            "delete" => self.delete(&metadata).await?,
            _ => self.default(&metadata, "Nothing to do!").await?,
        };

        let elapsed = started.elapsed();
        let elapsed_message = helper::channel_message(
            &metadata,
            format!("--- success: time elapsed: {:?}", elapsed).as_str(),
        )?;

        result.push(elapsed_message);
        Ok(result)
    }
}
