use crate::marv::{config::MarvSetup, plugins::Plugin};
use std::io::{Error, ErrorKind};

pub struct Pong {}

impl Pong {
    pub fn new(_setup: &MarvSetup) -> Box<dyn Plugin> {
        Box::new(Pong {})
    }
}

impl Plugin for Pong {
    fn name(&self) -> String {
        "Pong".to_string()
    }

    fn is_enabled(&self, message: &String) -> bool {
        return message.contains("PING");
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

        return Ok(vec![format!("PONG :{}\r\n", code)]);
    }
}
