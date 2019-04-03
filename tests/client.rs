mod server;
use self::server::setup;
use assert_cmd::prelude::*;
use assert_fs;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn calling_duma_without_args() {
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.assert().failure();
}

#[test]
fn calling_duma_with_invalid_url() {
    let mut cmd: Command = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args(&["wwww.shouldnotwork.com"]).assert().failure();
}

#[test]
fn test_request_timeout() {
    setup();
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args(&["-H", "-T", "3", "http://0.0.0.0:35550/timeout"])
        .assert()
        .failure();
}

#[test]
fn test_headers() {
    setup();
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    let output = cmd
        .args(&["-H", "http://0.0.0.0:35550/headers"])
        .output()
        .expect("failed to get command ouput");
    let stdout = String::from_utf8(output.stdout).unwrap();
    assert!(stdout.contains("Server: tiny-http (Rust)"));
    assert!(stdout.contains("Content-Type: text/plain"));
    assert!(stdout.contains("Content-Length: 0"));
}

#[test]
fn test_file() {
    setup();
    let temp = assert_fs::TempDir::new().unwrap().persist_if(true);
    println!("{}", temp.path().display());
    let input_file = temp.child("foo.txt");
    let mut cmd = Command::cargo_bin(env!("CARGO_PKG_NAME")).unwrap();
    cmd.args(&["-s", "-O", "foo.txt", "http://0.0.0.0:35550/file"])
        .current_dir(temp.path())
        .assert();
    input_file.assert(predicate::path::is_file());
}
