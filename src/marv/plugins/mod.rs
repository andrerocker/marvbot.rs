pub mod core;
pub mod kafka;

use super::config::{self};
use core::{channel, hello, log, login, pong};

pub trait Plugin {
    fn initialize(&self, setup: &config::MarvSetup);
    fn is_enabled(&self, message: &String) -> bool;
    fn perform(&self, message: &String) -> Vec<String>;
}

pub fn default(setup: &config::MarvSetup) -> Vec<Box<dyn Plugin>> {
    let config = setup.config.clone();

    return vec![
        Box::new(log::Logger {}),
        Box::new(login::Login {
            nickname: config.nickname,
        }),
        Box::new(pong::Pong {}),
        Box::new(channel::Channel {
            channel: config.channel,
        }),
        Box::new(hello::Hello {}),
    ];
}
