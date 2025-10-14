use crate::marv::plugins::Plugin;

pub struct Logger {}

impl Plugin for Logger {
    fn is_enabled(&self, _message: &String) -> bool {
        return true;
    }

    fn perform(&self, message: &String) -> Vec<String> {
        print!("<-- {}", message);
        return vec![];
    }
}
