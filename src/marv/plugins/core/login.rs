use log::info;

use crate::marv::plugins::Plugin;

pub struct Login {
    pub nickname: String,
}

impl Plugin for Login {
    fn initialize(&self, _setup: &crate::marv::config::MarvSetup) {}

    fn is_enabled(&self, message: &String) -> bool {
        return message.contains("Could not resolve your hostname");
    }

    fn perform(&self, _: &String) -> Vec<String> {
        info!("--> Executando Login");

        return vec![
            format!("USER {} * * :{}\r\n", self.nickname, self.nickname),
            format!("NICK {}\r\n", self.nickname),
        ];
    }
}
