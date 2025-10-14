mod core;
mod kafka;

use super::config;

pub trait Plugin {
    fn is_enabled(&self, message: &String) -> bool;
    fn perform(&self, message: &String) -> Vec<String>;
}

pub fn default(setup: &config::MarvSetup) -> Vec<Box<dyn Plugin>> {
    let config = setup.config.clone();

    return vec![
        Box::new(core::Logger {}),
        Box::new(core::Login {
            nickname: config.nickname,
        }),
        Box::new(core::Pong {}),
        Box::new(core::Channel {
            channel: config.channel,
        }),
        Box::new(core::Hello {}),
    ];
}
