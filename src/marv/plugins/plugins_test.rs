use super::*;

#[test]
fn plugin_should_be_selected() {
    let login = Login {
        nickname: "malcolm".to_string(),
    };
    let protocol = "Blach Blah! Could not resolve your hostname Bleh bleh".to_string();

    assert_eq!(login.is_enabled(&protocol), true);
}

#[test]
fn plugin_shouldnt_be_selected() {
    let login = Login {
        nickname: "malcolm".to_string(),
    };
    let protocol = "Blach Blah! Bleh bleh".to_string();

    assert_eq!(login.is_enabled(&protocol), false);
}

#[test]
fn plugin_should_return_login_payload() {
    let login = Login {
        nickname: "malcolm".to_string(),
    };
    let protocol = "".to_string();
    let response = vec![
        "USER malcolm * * :malcolm\r\n".to_string(),
        "NICK malcolm\r\n".to_string(),
    ];

    assert_eq!(login.perform(&protocol), response);
}
