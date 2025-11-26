use async_trait::async_trait;
use log::info;
use marv_api::{
    config,
    plugins::{DynamicPlugin, Plugin},
};
use std::io::Error;

pub struct Login {
    pub nickname: String,
}

impl Login {
    pub fn new() -> DynamicPlugin {
        let config = config::config();

        Box::new(Login {
            nickname: config.nickname.clone(),
        })
    }
}

#[async_trait]
impl Plugin for Login {
    fn name(&self) -> String {
        "Login".into()
    }

    fn responds_to(&self, message: &String) -> bool {
        message.contains("Could not resolve your hostname")
    }

    fn schedule(&self) -> Option<String> {
        None
    }

    async fn perform(&self, _: &String) -> Result<Vec<String>, Error> {
        info!("--> Executando Login");

        return Ok(vec![
            format!("USER {} * * :{}\r\n", self.nickname, self.nickname),
            format!("NICK {}\r\n", self.nickname),
        ]);
    }
}
