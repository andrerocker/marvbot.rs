use crate::marv::plugins::Plugin;
use std::io::Error;

pub struct Logger {}

impl Logger {
    pub fn new() -> Box<dyn Plugin> {
        Box::new(Logger {})
    }
}

impl Plugin for Logger {
    fn name(&self) -> String {
        "Logger".to_string()
    }

    fn is_enabled(&self, _message: &String) -> bool {
        true
    }

    fn perform(&mut self, message: &String) -> Result<Vec<String>, Error> {
        print!("<-- {}", message);
        Ok(vec![])
    }
}
