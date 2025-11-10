use super::repository::TodoRepository;
use crate::marv::{
    models::{NewTodo, Todo, UpdateTodo},
    plugins::helper,
};
use std::io::{self, Error};

pub struct TodoService {
    pub repository: TodoRepository,
}

impl TodoService {
    pub fn create(&mut self, todo: NewTodo) -> Result<Todo, Error> {
        // self.repository.create(todo)
        Err(helper::create_error("hack3d"))
    }

    pub fn update(&mut self, todo: UpdateTodo) -> io::Result<Todo> {
        // self.repository.update(todo)
        Err(helper::create_error("hack3d"))
    }

    pub fn list(&mut self) -> Result<Vec<Todo>, Error> {
        // self.repository.list()
        Err(helper::create_error("hack3d"))
    }

    pub fn delete(&mut self, id: i32) -> Result<usize, Error> {
        // self.repository.delete(id)
        Err(helper::create_error("hack3d"))
    }
}
