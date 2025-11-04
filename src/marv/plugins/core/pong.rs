use crate::marv::plugins::Plugin;
use std::io::{Error, ErrorKind};

pub struct Pong {}

impl Pong {
    pub fn new() -> Box<dyn Plugin> {
        Box::new(Pong {})
    }
}

impl Plugin for Pong {
    fn name(&self) -> String {
        "Pong".to_string()
    }

    fn is_enabled(&self, message: &String) -> bool {
        message.contains("PING")
    }

    fn perform(&mut self, message: &String) -> Result<Vec<String>, Error> {
        let code = message
            .split(":")
            .last()
            .ok_or(Error::new(
                ErrorKind::Other,
                "Problems trying to extract :host from PONG message",
            ))?
            .trim();

        Ok(vec![format!("PONG :{}\r\n", code)])
    }
}
