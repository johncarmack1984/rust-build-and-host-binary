use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn hello_world() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("hello-bin")?;

    cmd.assert()
        .success()
        .stdout(predicate::str::contains("Hello, world!"));

    Ok(())
}
