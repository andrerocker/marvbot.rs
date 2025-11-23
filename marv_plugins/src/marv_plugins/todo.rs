pub mod adapter;
pub mod controller;
pub mod models;
pub mod repository;
pub mod schema;
pub mod service;

use async_trait::async_trait;
use controller::TodoController;
use marv_api::plugins::{DynamicPlugin, Plugin};
use std::io::Error;

pub struct Todo {
    pub controller: TodoController,
}

impl Todo {
    pub fn new() -> DynamicPlugin {
        Box::new(Todo {
            controller: controller::new(),
        })
    }
}

#[async_trait]
impl Plugin for Todo {
    fn name(&self) -> String {
        "Todo".into()
    }

    fn responds_to(&self, message: &String) -> bool {
        message.contains(" PRIVMSG ") && message.contains(" :todo: ")
    }

    async fn perform(&self, message: &String) -> Result<Vec<String>, Error> {
        self.controller.dispatch(message).await
    }
}
