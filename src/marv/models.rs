use crate::marv::schema::{messages, todos};
use diesel::prelude::*;

// use crate::marv::{config::MarvSetup, plugins::Plugin};

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::marv::schema::messages)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Message {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[diesel(table_name = messages)]
pub struct NewMessage<'a> {
    pub title: &'a str,
    pub body: &'a str,
}

#[derive(Queryable, Selectable)]
#[diesel(table_name = crate::marv::schema::todos)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Todo {
    pub id: i32,
    pub body: String,
}

#[derive(Insertable)]
#[diesel(table_name = todos)]
pub struct NewTodo<'a> {
    pub body: &'a str,
}

impl Todo {
    pub fn to_string(&self) -> String {
        format!("{} - {}", self.id, self.body)
    }
}
