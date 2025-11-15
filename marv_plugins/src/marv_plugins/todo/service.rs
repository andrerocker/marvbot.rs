use super::{
    models::{NewTodo, Todo, UpdateTodo},
    repository::TodoRepository,
};
use std::io::{self, Error};

pub struct TodoService {
    pub repository: TodoRepository,
}

impl TodoService {
    pub async fn create(&mut self, todo: NewTodo) -> Result<Todo, Error> {
        self.repository.create(todo).await
    }

    pub async fn update(&mut self, todo: UpdateTodo) -> io::Result<Todo> {
        self.repository.update(todo).await
    }

    pub async fn list(&mut self) -> Result<Vec<Todo>, Error> {
        self.repository.list().await
    }

    pub async fn delete(&mut self, id: i32) -> Result<usize, Error> {
        self.repository.delete(id).await
    }
}
