use crate::marv::models::{NewTodo, Todo, UpdateTodo};
use diesel::PgConnection;
use diesel::associations::HasTable;
use diesel::prelude::*;
use std::io::{self, Error, ErrorKind};

pub struct TodoRepository {
    pub connection: PgConnection,
}

impl TodoRepository {
    pub fn create(&mut self, new_todo: NewTodo) -> Result<Todo, Error> {
        use crate::marv::schema::todos::dsl::*;

        let result = diesel::insert_into(todos::table())
            .values(&new_todo)
            .returning(Todo::as_returning())
            .get_result(&mut self.connection);

        match result {
            Ok(result) => Ok(result),
            Err(error) => Err(Error::new(
                ErrorKind::Other,
                format!("Problems trying to save Todo, {}", error),
            )),
        }
    }

    pub fn update(&mut self, todo: UpdateTodo) -> io::Result<Todo> {
        use crate::marv::schema::todos::dsl::*;

        let result = diesel::update(todos.filter(id.eq(todo.id)))
            .set(status.eq(todo.status))
            .get_result(&mut self.connection);

        match result {
            Ok(result) => Ok(result),
            Err(error) => Err(Error::new(
                ErrorKind::Other,
                format!("Problems trying to update Todo, {}", error),
            )),
        }
    }

    pub fn list(&mut self) -> Result<Vec<Todo>, Error> {
        use crate::marv::schema::todos::dsl::*;
        let results = todos.select(Todo::as_select()).load(&mut self.connection);

        match results {
            Ok(results) => Ok(results),
            Err(error) => Err(Error::new(
                ErrorKind::Other,
                format!("Problems trying to save Todo, {}", error),
            )),
        }
    }

    pub fn delete(&mut self, current_id: i32) -> Result<usize, Error> {
        use crate::marv::schema::todos::dsl::*;

        match diesel::delete(todos.filter(id.eq(current_id))).execute(&mut self.connection) {
            Ok(results) => Ok(results),
            Err(error) => Err(Error::new(
                ErrorKind::Other,
                format!("Problems trying to delete Todo, {}", error),
            )),
        }
    }
}
