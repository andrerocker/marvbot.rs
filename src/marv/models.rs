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
