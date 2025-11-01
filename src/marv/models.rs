use diesel::prelude::*;

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

impl Todo {
    pub fn to_string(&self) -> String {
        format!("{} - {} - {}", self.id, self.body, self.status)
    }
}
