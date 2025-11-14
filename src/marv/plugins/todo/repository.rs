use crate::marv::config;
use crate::marv::models::{NewTodo, Todo, UpdateTodo};
use crate::marv::plugins::helper;

use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use std::io::{self, Error};

pub struct TodoRepository {}

impl TodoRepository {
    fn get_connection(&self) -> Result<PooledConnection<ConnectionManager<PgConnection>>, Error> {
        config::POOL.get().or(helper::create_result_error(
            "Problems trying to get a connection from the pool",
        ))
    }

    pub fn create(&mut self, new_todo: NewTodo) -> Result<Todo, Error> {
        use crate::marv::schema::todos::dsl::*;

        let mut connection = self.get_connection()?;
        let result = diesel::insert_into(todos::table())
            .values(&new_todo)
            .returning(Todo::as_returning())
            .get_result(&mut connection);

        match result {
            Ok(result) => Ok(result),
            Err(error) => helper::create_result_error(
                format!("Problems trying to create Todo, {}", error).as_str(),
            ),
        }
    }

    pub fn update(&mut self, todo: UpdateTodo) -> io::Result<Todo> {
        use crate::marv::schema::todos::dsl::*;

        let mut connection = self.get_connection()?;
        let result = diesel::update(todos.filter(id.eq(todo.id)))
            .set(status.eq(todo.status))
            .get_result(&mut connection);

        match result {
            Ok(result) => Ok(result),
            Err(error) => helper::create_result_error(
                format!("Problems trying to update Todo, {}", error).as_str(),
            ),
        }
    }

    pub fn list(&mut self) -> Result<Vec<Todo>, Error> {
        use crate::marv::schema::todos::dsl::*;

        let mut connection = self.get_connection()?;
        let results = todos.select(Todo::as_select()).load(&mut connection);

        match results {
            Ok(results) => Ok(results),
            Err(error) => helper::create_result_error(
                format!("Problems trying to list Todo, {}", error).as_str(),
            ),
        }
    }

    pub fn delete(&mut self, current_id: i32) -> Result<usize, Error> {
        use crate::marv::schema::todos::dsl::*;
        let mut connection = self.get_connection()?;

        match diesel::delete(todos.filter(id.eq(current_id))).execute(&mut connection) {
            Ok(results) => Ok(results),
            Err(error) => helper::create_result_error(
                format!("Problems trying to delete Todo, {}", error).as_str(),
            ),
        }
    }
}
