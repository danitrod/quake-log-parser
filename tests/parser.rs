use assert_cmd::Command;

#[test]
fn simple_game() {
    let simple_game_logs = std::fs::read_to_string(format!(
        "{}/tests/simple-game.log",
        env!("CARGO_MANIFEST_DIR")
    ))
    .expect("could not find simple-game.log file in tests folder");

    let output = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .write_stdin(simple_game_logs)
        .unwrap();

    let stdout = std::str::from_utf8(&output.stdout).unwrap();

    assert!(stdout.contains(r#""total_kills": 29"#));
    assert!(stdout.contains(r#""Oootsimo": 8"#));
}

#[test]
fn multiple_games() {
    let multiple_games_logs =
        std::fs::read_to_string(format!("{}/tests/qgames.log", env!("CARGO_MANIFEST_DIR")))
            .expect("could not find simple-game.log file in tests folder");

    let output = Command::cargo_bin(env!("CARGO_PKG_NAME"))
        .unwrap()
        .write_stdin(multiple_games_logs)
        .unwrap();

    let stdout = std::str::from_utf8(&output.stdout).unwrap();

    assert!(stdout.contains("game_1"));
    assert!(stdout.contains("game_21"));
}
