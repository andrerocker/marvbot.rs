use diesel::prelude::*;
use std::io::Result;

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::marv::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub body: String,
    pub status: String,
}

#[derive(Insertable)]
#[diesel(table_name = crate::marv::schema::todos)]
pub struct NewTodo {
    pub body: String,
    pub status: String,
}

pub struct UpdateTodo {
    pub id: i32,
    pub status: String,
}

pub struct TodoAdapter {}

impl TodoAdapter {
    pub fn to_create(message: String) -> Result<NewTodo> {
        Ok(NewTodo {
            body: message,
            status: "created".to_string(),
        })
    }

    pub fn to_update(message: String) -> Result<UpdateTodo> {
        let parts = message.split(" ").collect::<Vec<&str>>();
        let todo_id = parts.first().unwrap().trim().parse::<i32>().unwrap();
        let status0 = parts.last().unwrap();

        Ok(UpdateTodo {
            id: todo_id,
            status: status0.to_string(),
        })
    }
}

impl Todo {
    pub fn to_string(&self) -> String {
        format!("{} - {} - {}", self.id, self.body, self.status)
    }
}
