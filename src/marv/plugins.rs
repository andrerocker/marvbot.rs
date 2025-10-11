pub trait Plugin {
    fn check(&self, message: &String) -> bool;
    fn perform(&self, message: &String) -> Vec<String>;
}

pub struct Login {}
pub struct Pong {}

impl Plugin for Login {
    fn check(&self, message: &String) -> bool {
        return message.contains("Could not resolve your hostname");
    }

    fn perform(&self, _: &String) -> Vec<String> {
        println!("--> Executando Login");

        return vec![
            "USER andrerocker * * :Andre\r\n".to_string(),
            "NICK andrerocker\r\n".to_string(),
        ];
    }
}

impl Plugin for Pong {
    fn check(&self, message: &String) -> bool {
        return message.contains("PING");
    }

    fn perform(&self, message: &String) -> Vec<String> {
        println!("--> Executando Pong");

        let code: String = message
                            .split_whitespace()
                            .collect::<Vec<&str>>()
                            .last()
                            .expect("BUMM")
                            .to_string()
                            .chars()
                            .skip(1)
                            .collect();

        return vec![format!("PONG :{}\r\n", code)];
    }
}