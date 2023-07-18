use assert_cmd::Command;

#[test]
fn no_input() {
    let mut cmd = Command::cargo_bin("list_players").unwrap();
    cmd.assert().success().stdout("Player 1: N/A\nPlayer 2: N/A");
}

#[test]
fn one_input() {    
    let mut cmd = Command::cargo_bin("list_players").unwrap();
    cmd.arg("Mike").assert().success().stdout("Player 1: Mike\nPlayer 2: N/A");
}

#[test]
fn two_input() {    
    let mut cmd = Command::cargo_bin("list_players").unwrap();
    cmd.arg("Mike");
    cmd.arg("Leo");
    cmd.assert().success().stdout("Player 1: Mike\nPlayer 2: Leo");
}

#[test]
fn three_input() {    
    let mut cmd = Command::cargo_bin("list_players").unwrap();
    cmd.arg("Mike");
    cmd.arg("Leo");
    cmd.arg("Ralph");
    cmd.assert().success().stdout("Player 1: Mike\nPlayer 2: Leo");
}