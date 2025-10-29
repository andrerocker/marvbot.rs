use crate::marv::models::{NewTodo, Todo};
use diesel::PgConnection;
use diesel::associations::HasTable;
use diesel::prelude::*;
use std::io::{self, Error, ErrorKind};

pub struct TodoRepository {
    pub connection: PgConnection,
}

impl TodoRepository {
    pub fn create(&mut self, message: &String) -> Result<Todo, Error> {
        use crate::marv::schema::todos::dsl::*;
        let new_todo = NewTodo {
            body: message,
            status: "created",
        };

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

    pub fn update(&mut self, message: &String) -> io::Result<Todo> {
        use crate::marv::schema::todos::dsl::*;
        let parts = message.split(" ").collect::<Vec<&str>>();
        let todo_id = parts.first().unwrap().trim().parse::<i32>().unwrap();
        let status0 = parts.last().unwrap();

        let result = diesel::update(todos.filter(id.eq(todo_id)))
            .set(status.eq(status0))
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

    pub fn delete(&mut self, message: &String) -> Result<usize, Error> {
        use crate::marv::schema::todos::dsl::*;

        match message.trim().parse::<i32>() {
            Ok(current_id) => {
                match diesel::delete(todos.filter(id.eq(current_id))).execute(&mut self.connection)
                {
                    Ok(results) => Ok(results),
                    Err(error) => Err(Error::new(
                        ErrorKind::Other,
                        format!("Problems trying to delete Todo, {}", error),
                    )),
                }
            }
            Err(error) => Err(Error::new(
                ErrorKind::Other,
                format!(
                    "Problems trying to parse Todo ID: '{}' - {}",
                    message, error
                ),
            )),
        }
    }
}
