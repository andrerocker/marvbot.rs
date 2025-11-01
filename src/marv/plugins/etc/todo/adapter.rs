use std::{collections::HashMap, io::Result};

use crate::marv::{
    models::{NewTodo, Todo, UpdateTodo},
    plugins::helper,
};

pub struct TodoAdapter {}

impl TodoAdapter {
    pub fn from_request_to_create(message: String) -> Result<NewTodo> {
        Ok(NewTodo {
            body: message,
            status: "created".to_string(),
        })
    }

    pub fn from_request_to_update(message: String) -> Result<UpdateTodo> {
        let parts = message.split(" ").collect::<Vec<&str>>();
        let todo_id = parts.first().unwrap().trim().parse::<i32>().unwrap();
        let status0 = parts.last().unwrap();

        Ok(UpdateTodo {
            id: todo_id,
            status: status0.to_string(),
        })
    }

    pub fn from_todos_to_response(
        metadata: &HashMap<String, String>,
        todos: Vec<Todo>,
    ) -> Result<Vec<String>> {
        Ok(todos
            .into_iter()
            .map(|current| helper::channel_user_message(&metadata, &current.to_string()).unwrap())
            .collect::<Vec<String>>())
    }
}
