use std::io::{Error, ErrorKind};

use diesel::PgConnection;
use diesel::associations::HasTable;
use diesel::prelude::*;

use crate::marv::models::{NewTodo, Todo};

pub struct TodoRepository {
    pub connection: PgConnection,
}

impl TodoRepository {
    pub fn create(&mut self, message: &String) -> Result<Todo, Error> {
        use crate::marv::schema::todos::dsl::*;
        let new_todo = NewTodo { body: message };

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
}
