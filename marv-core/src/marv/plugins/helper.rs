use regex::Regex;
use std::{
    collections::HashMap,
    io::{self, Error, ErrorKind},
};

pub fn safe_get(metadata: &HashMap<String, String>, key: &str) -> Result<String, Error> {
    metadata
        .get(key)
        .ok_or(create_error(
            format!(":metadata doesn't have key :{}", key).as_str(),
        ))
        .cloned()
}

#[test]
fn test_safe_get() {
    let mut metadata: HashMap<String, String> = HashMap::new();
    metadata.insert("hack3d".into(), "1337".into());

    assert!(safe_get(&metadata, "hack3d").is_ok());
    assert_eq!(safe_get(&metadata, "hack3d").unwrap(), "1337");
    assert!(safe_get(&metadata, "not-found").is_err());
}

pub fn regex_to_map(pattern: &str, payload: &String) -> HashMap<String, String> {
    let regex = Regex::new(pattern).expect("Problems trying to initialize Regex pattern");
    let mut results: HashMap<String, String> = HashMap::new();

    for caps in regex.captures_iter(payload) {
        for name in regex.capture_names() {
            if let Some(name_str) = name {
                if let Some(matched_value) = caps.name(name_str) {
                    results.insert(name_str.into(), matched_value.as_str().into());
                }
            }
        }
    }

    return results;
}

#[test]
fn test_regex_to_map() {
    let pattern = r"^:(?<nick>\w+)!(?<name>\w+)@(?<server>\w+.+) PRIVMSG #(?<channel>\w+) :todo: (?<command>\w+)(: (?<argument>.*))?";
    let command = ":pin!pin@192.168.65.1 PRIVMSG #acme :todo: create: acme";
    let metadata = regex_to_map(pattern, &command.to_string());

    assert!(safe_get(&metadata, "nick").is_ok());
    assert!(safe_get(&metadata, "name").is_ok());
    assert!(safe_get(&metadata, "server").is_ok());
    assert!(safe_get(&metadata, "channel").is_ok());
    assert!(safe_get(&metadata, "command").is_ok());
    assert!(safe_get(&metadata, "argument").is_ok());
}

pub fn channel_message(metadata: &HashMap<String, String>, message: &str) -> Result<String, Error> {
    let channel = safe_get(&metadata, "channel")?;
    Ok(format!("PRIVMSG #{} {}\r\n", channel, message))
}

#[test]
fn test_channel_message() {
    let mut metadata: HashMap<String, String> = HashMap::new();
    metadata.insert("channel".to_string(), "super-channel".to_string());

    assert!(channel_message(&metadata, "super-message").is_ok());
    assert_eq!(
        channel_message(&metadata, "super-message").unwrap(),
        "PRIVMSG #super-channel super-message\r\n"
    );

    let empty: HashMap<String, String> = HashMap::new();
    assert!(channel_message(&empty, "super-message").is_err());
}

pub fn simple_channel_message(
    metadata: &HashMap<String, String>,
    message: &str,
) -> Result<Vec<String>, Error> {
    Ok(vec![channel_message(&metadata, message)?])
}

#[test]
fn test_simple_channel_message() {
    let mut metadata: HashMap<String, String> = HashMap::new();
    metadata.insert("channel".to_string(), "super-channel".to_string());

    assert!(simple_channel_message(&metadata, "super-message").is_ok());
    assert_eq!(
        simple_channel_message(&metadata, "super-message").unwrap(),
        vec!["PRIVMSG #super-channel super-message\r\n"]
    );

    let empty: HashMap<String, String> = HashMap::new();
    assert!(channel_message(&empty, "super-message").is_err());
}

pub fn channel_user_message(
    metadata: &HashMap<String, String>,
    message: &str,
) -> Result<String, Error> {
    let nick = safe_get(&metadata, "nick")?;
    let channel = safe_get(&metadata, "channel")?;

    Ok(format!("PRIVMSG #{} {}: {}\r\n", channel, nick, message))
}

#[test]
fn test_channel_user_message() {
    let mut metadata: HashMap<String, String> = HashMap::new();
    metadata.insert("channel".to_string(), "super-channel".to_string());
    metadata.insert("nick".to_string(), "super-user".to_string());

    assert!(channel_user_message(&metadata, "super-message").is_ok());
    assert_eq!(
        channel_user_message(&metadata, "super-message").unwrap(),
        "PRIVMSG #super-channel super-user: super-message\r\n"
    );

    let empty: HashMap<String, String> = HashMap::new();
    assert!(channel_user_message(&empty, "super-message").is_err());
}

pub fn simple_channel_user_message(
    metadata: &HashMap<String, String>,
    message: &str,
) -> Result<Vec<String>, Error> {
    Ok(vec![channel_user_message(&metadata, message)?])
}

#[test]
fn test_simple_channel_user_message() {
    let mut metadata: HashMap<String, String> = HashMap::new();
    metadata.insert("channel".to_string(), "super-channel".to_string());
    metadata.insert("nick".to_string(), "super-user".to_string());

    assert!(simple_channel_user_message(&metadata, "super-message").is_ok());
    assert_eq!(
        simple_channel_user_message(&metadata, "super-message").unwrap(),
        vec!["PRIVMSG #super-channel super-user: super-message\r\n"]
    );

    let empty: HashMap<String, String> = HashMap::new();
    assert!(simple_channel_user_message(&empty, "super-message").is_err());
}

pub fn create_error(message: &str) -> Error {
    Error::new(ErrorKind::Other, message)
}
#[test]
fn test_create_err() {
    assert_eq!(create_error("super-error").kind(), ErrorKind::Other);
    assert_eq!(create_error("super-error").to_string(), "super-error");
}

#[allow(dead_code)]
pub fn create_result_error<T>(message: &str) -> io::Result<T> {
    Err(create_error(message))
}
#[test]
fn test_create_result_err() {
    assert!(create_result_error::<String>("Hack3d").is_err());
}

pub fn create_closure_error<T, E>(message: &str) -> impl Fn(E) -> Result<T, Error> {
    |_: E| Err(create_error(message))
}

#[test]
fn test_create_closure_error() {
    let error = create_error("shouldn't be used");
    let callme = create_closure_error::<&str, Error>("rataria");

    assert_eq!(callme(error).err().unwrap().kind(), ErrorKind::Other);
}

#[allow(dead_code)]
pub fn join<T: ToString>(items: &Vec<T>, separator: &str) -> String {
    items
        .iter()
        .map(|current| current.to_string())
        .collect::<Vec<_>>()
        .join(separator)
}

#[test]
fn test_join() {
    assert_eq!(join(&vec!["we", "are", "hiring"], "-"), "we-are-hiring");
}

#[macro_export]
macro_rules! elapsed {
    ($label:expr, $expr:expr) => {{
        let __start = ::std::time::Instant::now();
        let __ret = ($expr).await;
        let __elapsed = __start.elapsed();
        eprintln!("[elapsed] {} took {:?}", $label, __elapsed);
        __ret
    }};
}
