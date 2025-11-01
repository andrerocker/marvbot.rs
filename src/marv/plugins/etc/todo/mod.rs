pub mod controller;
pub mod repository;
pub mod service;

use crate::marv::{config::MarvSetup, plugins::Plugin};
use controller::TodoController;
use diesel::PgConnection;
use diesel::prelude::*;
use repository::TodoRepository;
use service::TodoService;
use std::io::Error;

pub struct Todo {
    pub controller: TodoController,
}

impl Todo {
    pub fn new(setup: &MarvSetup) -> Box<dyn Plugin> {
        let database_url = setup.config.database_url.clone();
        let connection = PgConnection::establish(&database_url)
            .unwrap_or_else(|_| panic!("Error connecting to {}", database_url));

        Box::new(Todo {
            controller: controller::TodoController {
                service: TodoService {
                    repository: TodoRepository { connection },
                },
            },
        })
    }
}

impl Plugin for Todo {
    fn name(&self) -> String {
        "Todo".to_string()
    }

    fn is_enabled(&self, message: &String) -> bool {
        message.contains(" PRIVMSG ") && message.contains(" :todo: ")
    }

    fn perform(&mut self, message: &String) -> Result<Vec<String>, Error> {
        self.controller.dispatch(message)
    }
}
