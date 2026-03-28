use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn test_word_count() {
    let mut cmd = Command::cargo_bin("nwc").unwrap();

    cmd.arg("-w")
        .arg("test.txt")
        .assert()
        .success()
        .stdout(contains("58164 test.txt"));
}

#[test]
fn test_char_count() {
    let mut cmd = Command::cargo_bin("nwc").unwrap();

    cmd.arg("-m")
        .arg("test.txt")
        .assert()
        .success()
        .stdout(contains("339292 test.txt"));
}

#[test]
fn test_line_count() {
    let mut cmd = Command::cargo_bin("nwc").unwrap();

    cmd.arg("-l")
        .arg("test.txt")
        .assert()
        .success()
        .stdout(contains("7145 test.txt"));
}

#[test]
fn test_byte_count() {
    let mut cmd = Command::cargo_bin("nwc").unwrap();

    cmd.arg("-c")
        .arg("test.txt")
        .assert()
        .success()
        .stdout(contains("342190 test.txt"));
}

#[test]
fn test_default_behaviour() {
    let mut cmd = Command::cargo_bin("nwc").unwrap();

    cmd.arg("test.txt")
        .assert()
        .success()
        .stdout(contains("7145 58164 342190 test.txt"));
}

#[test]
fn test_line_count_stdin() {
    let mut cmd = Command::cargo_bin("nwc").unwrap();

    cmd.write_stdin("Hello world\nHello back at you\n")
        .arg("-l")
        .assert()
        .success()
        .stdout(contains("2"));
}

#[test]
fn test_word_count_stdin() {
    let mut cmd = Command::cargo_bin("nwc").unwrap();

    cmd.write_stdin("Hello world\nHello back at you\n")
        .arg("-w")
        .assert()
        .success()
        .stdout(contains("6"));
}

#[test]
fn test_char_count_stdin() {
    let mut cmd = Command::cargo_bin("nwc").unwrap();

    cmd.write_stdin("Hello world\nHello back at you\n")
        .arg("-m")
        .assert()
        .success()
        .stdout(contains("30"));
}

#[test]
fn test_byte_count_stdin() {
    let mut cmd = Command::cargo_bin("nwc").unwrap();

    cmd.write_stdin("hello world\n")
        .arg("-c")
        .assert()
        .success()
        .stdout(contains("12"));
}

#[test]
fn test_default_behaviour_stdin() {
    let mut cmd = Command::cargo_bin("nwc").unwrap();

    cmd.write_stdin("hello world\n")
        .assert()
        .success()
        .stdout(contains("1 2 12"));
}
