use log::info;

use crate::marv::{config::MarvSetup, plugins::Plugin};

pub struct Pong {}

impl Pong {
    pub fn new(_setup: &MarvSetup) -> Box<dyn Plugin> {
        Box::new(Pong {})
    }
}

impl Plugin for Pong {
    fn name(&self) -> String {
        return "Pong".to_string();
    }

    fn is_enabled(&self, message: &String) -> bool {
        return message.contains("PING");
    }

    fn perform(&mut self, message: &String) -> Vec<String> {
        info!("--> Executando Pong");

        let code: String = message
            .split(":")
            .collect::<Vec<&str>>()
            .last()
            .expect("BUMM")
            .to_string();

        return vec![format!("PONG :{}\r\n", code)];
    }
}
