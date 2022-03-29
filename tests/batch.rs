use assert_cmd::Command;
use predicates::prelude::*;

#[test]
fn successful_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("performance_tests")?;
    cmd.arg("-r").arg("1").arg("-m").arg("1").arg("stellar");
    cmd.assert().success().stdout(predicate::str::contains(
        "Starting performance measurements for Stellar like FBAS with upto 1 nodes.\n Performing 1 iterations per FBAS.",
    ));
    Ok(())
}

#[test]
fn no_fbas_type_in_command() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("performance_tests")?;
    cmd.arg("-m")
        .arg("10")
        .arg("-j")
        .arg("4")
        .arg("-o")
        .arg("./");
    let output = cmd.output().expect("error executing command");
    assert!(!output.status.success());
    Ok(())
}