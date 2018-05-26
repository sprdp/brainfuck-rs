extern crate bfrs;
use std::process::Command;

#[test]
fn hello_world() {
    let output = Command::new("./target/debug/bfrs.exe").arg("tests/hello.bf").output().unwrap();
    assert!(output.stderr.is_empty());
    assert_eq!(output.stdout, b"Hello World!\n");
}