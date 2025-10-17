pub mod core;
pub mod kafka;

use super::config;
use core::{channel::Channel, hello::Hello, log::Logger, login::Login, pong::Pong};
use kafka::KafkaProducer;

pub trait Plugin {
    fn is_enabled(&self, message: &String) -> bool;
    fn perform(&mut self, message: &String) -> Vec<String>;
}

pub fn default(setup: &config::MarvSetup) -> Vec<Box<dyn Plugin>> {
    return vec![
        Logger::new(setup),
        Login::new(setup),
        Pong::new(setup),
        Channel::new(setup),
        Hello::new(setup),
        KafkaProducer::new(setup),
    ];
}
