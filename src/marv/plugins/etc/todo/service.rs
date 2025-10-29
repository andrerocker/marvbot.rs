use super::repository::TodoRepository;
use crate::marv::models::Todo;
use std::io::{self, Error};

pub struct TodoService {
    pub repository: TodoRepository,
}

impl TodoService {
    pub fn create(&mut self, message: &String) -> Result<Todo, Error> {
        return self.repository.create(message);
    }

    pub fn update(&mut self, message: &String) -> io::Result<Todo> {
        return self.update(message);
    }

    pub fn list(&mut self) -> Result<Vec<Todo>, Error> {
        return self.repository.list();
    }

    pub fn delete(&mut self, message: &String) -> Result<usize, Error> {
        return self.repository.delete(message);
    }
}
