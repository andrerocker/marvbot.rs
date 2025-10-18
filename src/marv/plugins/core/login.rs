use log::info;

use crate::marv::{config::MarvSetup, plugins::Plugin};

pub struct Login {
    pub nickname: String,
}

impl Login {
    pub fn new(setup: &MarvSetup) -> Box<dyn Plugin> {
        Box::new(Login {
            nickname: setup.config.nickname.to_string(),
        })
    }
}

impl Plugin for Login {
    fn name(&self) -> String {
        return "Login".to_string();
    }

    fn is_enabled(&self, message: &String) -> bool {
        return message.contains("Could not resolve your hostname");
    }

    fn perform(&mut self, _: &String) -> Vec<String> {
        info!("--> Executando Login");

        return vec![
            format!("USER {} * * :{}\r\n", self.nickname, self.nickname),
            format!("NICK {}\r\n", self.nickname),
        ];
    }
}
