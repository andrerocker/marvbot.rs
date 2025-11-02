use crate::marv::{
    models::{NewTodo, Todo, UpdateTodo},
    plugins::helper,
};
use std::{collections::HashMap, io};

pub struct TodoAdapter {}

impl TodoAdapter {
    pub fn from_request_to_create(message: String) -> io::Result<NewTodo> {
        Ok(NewTodo {
            body: message,
            status: "created".to_string(),
        })
    }

    pub fn from_request_to_update(message: String) -> io::Result<UpdateTodo> {
        let parts = message.split(" ").collect::<Vec<&str>>();
        let id = parts
            .first()
            .ok_or(helper::create_error(":id is a required field"))?
            .trim()
            .parse::<i32>()
            .or_else(helper::create_closure_error(":id needs to be a number"))?;

        let status0 = parts
            .last()
            .ok_or(helper::create_error(":status is a required field"))?
            .trim()
            .to_string();

        Ok(UpdateTodo {
            id: id,
            status: status0,
        })
    }

    pub fn from_request_to_delete(message: String) -> io::Result<i32> {
        let parts = message.split(" ").collect::<Vec<&str>>();

        parts
            .first()
            .ok_or(helper::create_error(":id is a required field"))?
            .trim()
            .parse::<i32>()
            .or_else(helper::create_closure_error(":id needs to be a number"))
    }

    pub fn from_todos_to_response(
        metadata: &HashMap<String, String>,
        todos: Vec<Todo>,
    ) -> io::Result<Vec<String>> {
        Ok(todos
            .into_iter()
            .map(|current| helper::channel_user_message(&metadata, &current.to_string()).unwrap())
            .collect::<Vec<String>>())
    }
}
