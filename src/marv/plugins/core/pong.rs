use log::info;

use crate::marv::plugins::Plugin;

pub struct Pong {}

impl Plugin for Pong {
    fn initialize(&self, _setup: &crate::marv::config::MarvSetup) {}

    fn is_enabled(&self, message: &String) -> bool {
        return message.contains("PING");
    }

    fn perform(&self, message: &String) -> Vec<String> {
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
