use crate::marv::models::*;
use crate::marv::schema::messages;
use crate::marv::{config::MarvSetup, plugins::Plugin};
use diesel::Connection;
use diesel::PgConnection;
use diesel::prelude::*;

pub struct Database {
    pub connection: PgConnection,
}

impl Database {
    pub fn new(setup: &MarvSetup) -> Box<dyn Plugin> {
        let database_url = setup.config.database_url.clone();

        let connection = PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        Box::new(Database {
            connection: connection,
        })
    }

    pub fn messages(&mut self) {
        use crate::marv::schema::messages::dsl::*;
        let connection = &mut self.connection;

        let results = messages
            .filter(published.eq(true))
            .select(Message::as_select())
            .load(connection)
            .expect("Error loading messages");

        println!("Displaying {} messages", results.len());
        for post in results {
            println!("{}", post.title);
            println!("-----------\n");
            println!("{}", post.body);
        }
    }

    pub fn create(&mut self, message: &String) -> Message {
        let title = "Acme";
        let body = message;

        let new_post = NewMessage { title, body };
        let connection = &mut self.connection;

        diesel::insert_into(messages::table)
            .values(&new_post)
            .returning(Message::as_returning())
            .get_result(connection)
            .expect("Error saving new post")
    }
}

impl Plugin for Database {
    fn name(&self) -> String {
        return "Database".to_string();
    }

    fn is_enabled(&self, _message: &String) -> bool {
        return true;
    }

    fn perform(&mut self, message: &String) -> Vec<String> {
        self.create(message);

        return vec![];
    }
}
