// Note this useful idiom: importing names from outer (for mod tests) scope.
use super::*;

#[test]
fn plugin_should_be_selected() {
    let login = Login{};
    let protocol = "Blach Blah! Could not resolve your hostname Bleh bleh".to_string();

    assert_eq!(login.check(&protocol), true);
}

#[test]
fn plugin_shouldnt_be_selected() {
    let login = Login{};
    let protocol = "Blach Blah! Bleh bleh".to_string();

    assert_eq!(login.check(&protocol), false);
}

#[test]
fn plugin_should_return_login_payload() {
    let login = Login{};
    let protocol = "".to_string();
    let response = vec!["NICK andrerocker\r\n".to_string(), "USER andrerocker * * :Andre\r\n".to_string()];

    assert_eq!(login.perform(&protocol), response);
}

// #[test]
// fn test_bad_add() {
//     // This assert would fire and test will fail.
//     // Please note, that private functions can be tested too!
//     assert_eq!(bad_add(1, 2), 3);
// }