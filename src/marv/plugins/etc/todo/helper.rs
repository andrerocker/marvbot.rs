use std::{
    collections::HashMap,
    io::{Error, ErrorKind},
};

use regex::Regex;

fn safe_get(metadata: &HashMap<String, String>, key: &str) -> Result<String, Error> {
    metadata
        .get(key)
        .ok_or(Error::new(
            ErrorKind::Other,
            ":metadata doesn't have key :{key}",
        ))
        .cloned()
}

pub fn regex_to_map(pattern: &str, payload: &String) -> HashMap<String, String> {
    let regex = Regex::new(pattern).expect("Problems trying to initialize Regex pattern");
    let mut results: HashMap<String, String> = HashMap::new();

    for caps in regex.captures_iter(payload) {
        for name in regex.capture_names() {
            if let Some(name_str) = name {
                if let Some(matched_value) = caps.name(name_str) {
                    results.insert(name_str.to_string(), matched_value.as_str().to_string());
                }
            }
        }
    }

    return results;
}

pub fn channel_message(metadata: HashMap<String, String>, message: &str) -> Result<String, Error> {
    let channel = safe_get(&metadata, "channel")?;
    Ok(format!("PRIVMSG #{} - {}\r\n", channel, message))
}

pub fn channel_user_message(
    metadata: HashMap<String, String>,
    message: &str,
) -> Result<String, Error> {
    let nick = safe_get(&metadata, "nick")?;
    let channel = safe_get(&metadata, "channel")?;

    Ok(format!("PRIVMSG #{} {}: {}\r\n", channel, nick, message))
}
