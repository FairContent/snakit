use anyhow::Result;
use assert_cmd::Command;
use assert_fs::prelude::{FileTouch, PathChild};
use assert_fs::TempDir;
use predicates::prelude::*;

// --------------------------------------------------

const PRG: &str = "snakit";

// --------------------------------------------------

#[test]
fn usage() -> Result<()> {
    for flag in &["-h", "--help"] {
        Command::cargo_bin(PRG)?
            .arg(flag)
            .assert()
            .stdout(predicate::str::contains("Usage"));
    }

    Ok(())
}

// --------------------------------------------------

#[test]
fn dies_no_args() -> Result<()> {
    Command::cargo_bin(PRG)?
        .assert()
        .failure()
        .stderr(predicate::str::contains("Usage"));

    Ok(())
}

// --------------------------------------------------

#[test]
fn invalid_path() -> Result<()> {
    Command::cargo_bin(PRG)?
        .arg("invalid_path")
        .assert()
        .failure()
        .stderr(predicate::str::contains("is invalid or not a directory"));

    Ok(())
}
// --------------------------------------------------

#[test]
fn path_is_file() -> Result<()> {
    let tmp_dir = TempDir::new()?;
    let file = tmp_dir.child("tmp.txt");
    file.touch()?;

    Command::cargo_bin(PRG)?
        .arg(file.path())
        .assert()
        .failure()
        .stderr(predicate::str::contains("is invalid or not a directory"));

    Ok(())
}
