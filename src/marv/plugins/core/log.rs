use std::io::Error;

use crate::marv::{config::MarvSetup, plugins::Plugin};

pub struct Logger {}

impl Logger {
    pub fn new(_setup: &MarvSetup) -> Box<dyn Plugin> {
        Box::new(Logger {})
    }
}

impl Plugin for Logger {
    fn name(&self) -> String {
        return "Logger".to_string();
    }

    fn is_enabled(&self, _message: &String) -> bool {
        return true;
    }

    fn perform(&mut self, message: &String) -> Result<Vec<String>, Error> {
        print!("<-- {}", message);
        return Ok(vec![]);
    }
}
