use crate::marv::models::{NewTodo, Todo, UpdateTodo};

use diesel::prelude::*;
use diesel::r2d2::ConnectionManager;
use diesel::{associations::HasTable, r2d2::Pool};
use once_cell::sync::Lazy;
use std::io::{self, Error, ErrorKind};

pub static POOL: Lazy<Pool<ConnectionManager<PgConnection>>> = Lazy::new(|| {
    let manager = ConnectionManager::<PgConnection>::new(
        "postgres://deploy42:deploy42@localhost:5432/deploy42",
    );
    Pool::builder()
        .max_size(5)
        .build(manager)
        .expect("Problems trying to process Marv.toml configuration file")
});

pub struct TodoRepository {}

impl TodoRepository {
    pub fn create(&mut self, new_todo: NewTodo) -> Result<Todo, Error> {
        use crate::marv::schema::todos::dsl::*;

        let mut connection = POOL.get().unwrap();
        let result = diesel::insert_into(todos::table())
            .values(&new_todo)
            .returning(Todo::as_returning())
            .get_result(&mut connection);

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

        let mut connection = POOL.get().unwrap();
        let result = diesel::update(todos.filter(id.eq(todo.id)))
            .set(status.eq(todo.status))
            .get_result(&mut connection);

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

        let mut connection = POOL.get().unwrap();
        let results = todos.select(Todo::as_select()).load(&mut connection);

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
        let mut connection = POOL.get().unwrap();

        match diesel::delete(todos.filter(id.eq(current_id))).execute(&mut connection) {
            Ok(results) => Ok(results),
            Err(error) => Err(Error::new(
                ErrorKind::Other,
                format!("Problems trying to delete Todo, {}", error),
            )),
        }
    }
}
