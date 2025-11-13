pub mod adapter;
pub mod controller;
pub mod repository;
pub mod service;

use super::DynamicPlugin;
use crate::marv::plugins::Plugin;
use async_trait::async_trait;
use controller::TodoController;
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
        "Todo".to_string()
    }

    async fn is_enabled(&self, message: &String) -> bool {
        message.contains(" PRIVMSG ") && message.contains(" :todo: ")
    }

    async fn perform(&mut self, message: &String) -> Result<Vec<String>, Error> {
        self.controller.dispatch(message)
    }
}
