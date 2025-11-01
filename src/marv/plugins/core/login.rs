use std::io::Error;

use log::info;

use crate::marv::{config::MarvSetup, plugins::Plugin};

pub struct Login {
    pub nickname: String,
}

impl Login {
    pub fn new(setup: &MarvSetup) -> Box<dyn Plugin> {
        Box::new(Login {
            nickname: setup.config.nickname.clone(),
        })
    }
}

impl Plugin for Login {
    fn name(&self) -> String {
        "Login".to_string()
    }

    fn is_enabled(&self, message: &String) -> bool {
        message.contains("Could not resolve your hostname")
    }

    fn perform(&mut self, _: &String) -> Result<Vec<String>, Error> {
        info!("--> Executando Login");

        return Ok(vec![
            format!("USER {} * * :{}\r\n", self.nickname, self.nickname),
            format!("NICK {}\r\n", self.nickname),
        ]);
    }
}
