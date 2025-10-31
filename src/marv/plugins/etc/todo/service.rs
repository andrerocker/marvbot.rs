use super::repository::TodoRepository;
use crate::marv::models::{NewTodo, Todo, UpdateTodo};
use std::io::{self, Error};

pub struct TodoService {
    pub repository: TodoRepository,
}

impl TodoService {
    pub fn create(&mut self, todo: NewTodo) -> Result<Todo, Error> {
        return self.repository.create(todo);
    }

    pub fn update(&mut self, todo: UpdateTodo) -> io::Result<Todo> {
        return self.repository.update(todo);
    }

    pub fn list(&mut self) -> Result<Vec<Todo>, Error> {
        return self.repository.list();
    }

    pub fn delete(&mut self, message: &String) -> Result<usize, Error> {
        return self.repository.delete(message);
    }
}
