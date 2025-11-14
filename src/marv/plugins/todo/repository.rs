use crate::marv::config;
use crate::marv::models::{NewTodo, Todo, UpdateTodo};
use crate::marv::plugins::helper;

use bb8::PooledConnection;
use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel_async::AsyncPgConnection;
use diesel_async::RunQueryDsl;
use diesel_async::pooled_connection::AsyncDieselConnectionManager;
use std::io::{self, Error};

pub struct TodoRepository {}

impl TodoRepository {
    async fn connection(
        &self,
    ) -> io::Result<PooledConnection<'_, AsyncDieselConnectionManager<AsyncPgConnection>>> {
        config::POOL
            .get()
            .unwrap()
            .get()
            .await
            .or(helper::create_result_error(
                "Problems trying to fetch a connection",
            ))
    }

    pub async fn create(&mut self, new_todo: NewTodo) -> Result<Todo, Error> {
        use crate::marv::schema::todos::dsl::*;

        let mut connection = self.connection().await?;
        let result = diesel::insert_into(todos::table())
            .values(&new_todo)
            .returning(Todo::as_returning())
            .get_result(&mut connection)
            .await;

        match result {
            Ok(todo) => Ok(todo),
            Err(error) => helper::create_result_error(
                format!("Problems trying to create Todo, {}", error).as_str(),
            ),
        }
    }

    pub async fn update(&mut self, todo: UpdateTodo) -> io::Result<Todo> {
        use crate::marv::schema::todos::dsl::*;

        let mut connection = self.connection().await?;
        let result = diesel::update(todos.filter(id.eq(todo.id)))
            .set(status.eq(todo.status))
            .get_result(&mut connection)
            .await;

        match result {
            Ok(result) => Ok(result),
            Err(error) => helper::create_result_error(
                format!("Problems trying to update Todo, {}", error).as_str(),
            ),
        }
    }

    pub async fn list(&mut self) -> Result<Vec<Todo>, Error> {
        use crate::marv::schema::todos::dsl::*;

        let mut connection = self.connection().await?;
        let results = todos.select(Todo::as_select()).load(&mut connection).await;

        match results {
            Ok(todo_list) => Ok(todo_list),
            Err(error) => helper::create_result_error(
                format!("Problems trying to list Todo, {}", error).as_str(),
            ),
        }
    }

    pub async fn delete(&mut self, current_id: i32) -> Result<usize, Error> {
        use crate::marv::schema::todos::dsl::*;

        let mut connection = self.connection().await?;
        let result = diesel::delete(todos.filter(id.eq(current_id)))
            .execute(&mut connection)
            .await;

        match result {
            Ok(affected) => Ok(affected),
            Err(error) => helper::create_result_error(
                format!("Problems trying to delete Todo, {}", error).as_str(),
            ),
        }
    }
}
