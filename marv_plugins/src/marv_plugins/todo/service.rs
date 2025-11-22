use super::{
    models::{NewTodo, Todo, UpdateTodo},
    repository::TodoRepository,
};
use std::io::{self, Error};

pub struct TodoService {
    pub repository: TodoRepository,
}

impl TodoService {
    pub async fn create(&self, todo: NewTodo) -> Result<Todo, Error> {
        self.repository.create(todo).await
    }

    pub async fn update(&self, todo: UpdateTodo) -> io::Result<Todo> {
        self.repository.update(todo).await
    }

    pub async fn list(&self) -> Result<Vec<Todo>, Error> {
        self.repository.list().await
    }

    pub async fn delete(&self, id: i32) -> Result<usize, Error> {
        self.repository.delete(id).await
    }
}
