use super::repository::TodoRepository;
use crate::marv::models::{NewTodo, Todo, UpdateTodo};
use std::io::{self, Error};

pub struct TodoService {
    pub repository: TodoRepository,
}

impl TodoService {
    pub fn create(&mut self, todo: NewTodo) -> Result<Todo, Error> {
        self.repository.create(todo)
    }

    pub fn update(&mut self, todo: UpdateTodo) -> io::Result<Todo> {
        self.repository.update(todo)
    }

    pub fn list(&mut self) -> Result<Vec<Todo>, Error> {
        self.repository.list()
    }

    pub fn delete(&mut self, message: &String) -> Result<usize, Error> {
        self.repository.delete(message)
    }
}
