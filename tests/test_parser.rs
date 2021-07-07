use eriscord::parser::{Command, CommandParser};

#[test]
fn test_simple() {
    let parser = CommandParser::new(".".to_owned());
    let command = parser
        .parse_command(".echo hello world")
        .expect("Expected command to be valid");
    assert_eq!(command.action, "echo");
    let args = command.unparsed_args.parse(true);
    assert_eq!(args.text, Some("hello world".into()));
    assert_eq!(args.args.len(), 0);
    assert_eq!(args.set_args.len(), 0);
    assert_eq!(args.words, [] as [&str; 0]);
}

#[test]
fn test_empty() {
    let parser = CommandParser::new(".".to_owned());
    let command = parser
        .parse_command(".echo")
        .expect("Expected command to be valid");
    assert_eq!(command.action, "echo");
    let args = command.unparsed_args.parse(true);
    assert_eq!(args.text, None);
    assert_eq!(args.args.len(), 0);
    assert_eq!(args.set_args.len(), 0);
    assert_eq!(args.words, [] as [&str; 0]);
}

#[test]
fn test_args() {
    let parser = CommandParser::new(".".to_owned());
    let command = parser
        .parse_command(".echo -mention hello world")
        .expect("Expected command to be valid");
    assert_eq!(command.action, "echo");
    let args = command.unparsed_args.parse(true);
    assert_eq!(args.text, Some("hello world".into()));
    assert_eq!(args.args.len(), 1);
    assert!(args.args.contains("mention"));
    assert_eq!(args.set_args.len(), 0);
    assert_eq!(args.words, [] as [&str; 0]);
}

#[test]
fn test_set_args() {
    let parser = CommandParser::new(".".to_owned());
    let command = parser
        .parse_command(".echo -format=clap hello world")
        .expect("Expected command to be valid");
    assert_eq!(command.action, "echo");
    let args = command.unparsed_args.parse(true);
    assert_eq!(args.text, Some("hello world".into()));
    assert_eq!(args.set_args.len(), 1);
    assert_eq!(args.set_args.get("format").copied(), Some("clap"));
    assert_eq!(args.args.len(), 0);
    assert_eq!(args.words, [] as [&str; 0]);
}

#[test]
fn test_set_args_and_args() {
    let parser = CommandParser::new(".".to_owned());
    let command = parser
        .parse_command(".echo -format=clap -mention hello world")
        .expect("Expected command to be valid");
    assert_eq!(command.action, "echo");
    let args = command.unparsed_args.parse(true);
    assert_eq!(args.text, Some("hello world".into()));
    assert_eq!(args.set_args.len(), 1);
    assert_eq!(args.set_args.get("format").copied(), Some("clap"));
    assert_eq!(args.args.len(), 1);
    assert!(args.args.contains("mention"));
    assert_eq!(args.words, [] as [&str; 0]);
}

#[test]
fn test_words() {
    let parser = CommandParser::new(".".to_owned());
    let command = parser
        .parse_command(".echo hello world")
        .expect("Expected command to be valid");
    assert_eq!(command.action, "echo");
    let args = command.unparsed_args.parse(false);
    assert_eq!(args.text, None);
    assert_eq!(args.set_args.len(), 0);
    assert_eq!(args.args.len(), 0);
    assert_eq!(args.words, ["hello", "world"]);
}

#[test]
fn test_words_with_args() {
    let parser = CommandParser::new(".".to_owned());
    let commands: Vec<Command> = [
        ".echo -test hello world",
        ".echo hello -test world",
        ".echo hello world -test",
    ]
    .iter()
    .map(|&message| {
        parser
            .parse_command(message)
            .expect("Expected command to be valid")
    })
    .collect();
    for command in commands {
        assert_eq!(command.action, "echo");
        let args = command.unparsed_args.parse(false);
        assert_eq!(args.text, None);
        assert_eq!(args.set_args.len(), 0);
        assert_eq!(args.args.len(), 1);
        assert!(args.args.contains("test"));
        assert_eq!(args.words, ["hello", "world"]);
    }
}

#[test]
fn test_words_with_set_args() {
    let parser = CommandParser::new(".".to_owned());
    let commands: Vec<Command> = [
        ".echo -test=hello hello world",
        ".echo hello -test=hello world",
        ".echo hello world -test=hello",
    ]
    .iter()
    .map(|&message| {
        parser
            .parse_command(message)
            .expect("Expected command to be valid")
    })
    .collect();
    for command in commands {
        assert_eq!(command.action, "echo");
        let args = command.unparsed_args.parse(false);
        assert_eq!(args.text, None);
        assert_eq!(args.set_args.len(), 1);
        assert_eq!(args.set_args.get("test").copied(), Some("hello"));
        assert_eq!(args.args.len(), 0);
        assert_eq!(args.words, ["hello", "world"]);
    }
}

#[test]
fn test_words_with_args_and_set_args() {
    let parser = CommandParser::new(".".to_owned());
    let commands: Vec<Command> = [
        ".echo -foo -test=hello hello world",
        ".echo hello -foo -test=hello world",
        ".echo hello world -foo -test=hello",
        ".echo hello -foo world -test=hello",
        ".echo -foo hello world -test=hello",
        ".echo -foo hello -test=hello world",
        ".echo -test=hello hello -foo world",
        ".echo -test=hello hello world -foo",
    ]
    .iter()
    .map(|&message| {
        parser
            .parse_command(message)
            .expect("Expected command to be valid")
    })
    .collect();
    for command in commands {
        assert_eq!(command.action, "echo");
        let args = command.unparsed_args.parse(false);
        assert_eq!(args.text, None);
        assert_eq!(args.set_args.len(), 1);
        assert_eq!(args.set_args.get("test").copied(), Some("hello"));
        assert_eq!(args.args.len(), 1);
        assert!(args.args.contains("foo"));
        assert_eq!(args.words, ["hello", "world"]);
    }
}
