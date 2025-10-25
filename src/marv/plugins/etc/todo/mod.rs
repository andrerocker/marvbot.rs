pub mod controller;
pub mod helper;

use crate::marv::{config::MarvSetup, plugins::Plugin};
use std::io::Error;

pub struct Todo {}

impl Todo {
    pub fn new(_setup: &MarvSetup) -> Box<dyn Plugin> {
        Box::new(Todo {})
    }
}

impl Plugin for Todo {
    fn name(&self) -> String {
        return "Todo".to_string();
    }

    fn is_enabled(&self, message: &String) -> bool {
        return message.contains(" PRIVMSG ") && message.contains(" :todo: ");
    }

    fn perform(&mut self, message: &String) -> Result<Vec<String>, Error> {
        return controller::dispatch(message);
    }
}
