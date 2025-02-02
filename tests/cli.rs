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
        .stderr(predicate::str::contains("does not exist"));

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
        .stderr(predicate::str::contains("is not a directory"));

    Ok(())
}

// --------------------------------------------------

#[test]
fn one_file() -> Result<()> {
    let tmp_dir = TempDir::new()?;
    let file = tmp_dir.child("tmp file.txt");
    file.touch()?;

    Command::cargo_bin(PRG)?
        .arg(tmp_dir.path())
        .assert()
        .success();

    let renamed_file = tmp_dir.child("tmp_file.txt");
    assert!(renamed_file.exists());

    Ok(())
}

// --------------------------------------------------

#[test]
fn multiple_files() -> Result<()> {
    let tmp_dir = TempDir::new()?;
    let files = ["tmp file.txt", "another file.txt"];
    for file in &files {
        let file_path = tmp_dir.child(file);
        file_path.touch()?;
    }

    Command::cargo_bin(PRG)?
        .arg(tmp_dir.path())
        .assert()
        .success();

    for file in &files {
        let renamed_file = tmp_dir.child(file.replace(" ", "_"));
        assert!(renamed_file.exists());
    }

    Ok(())
}

// --------------------------------------------------

#[test]
fn nested_directories() -> Result<()> {
    let tmp_dir = TempDir::new()?;
    let file = tmp_dir.child("test/tmp file.txt");
    file.touch()?;

    Command::cargo_bin(PRG)?
        .arg(tmp_dir.path())
        .assert()
        .success();

    let renamed_file = tmp_dir.child("test/tmp_file.txt");
    assert!(renamed_file.exists());

    Ok(())
}

// --------------------------------------------------

#[test]
fn hidden_file() -> Result<()> {
    let tmp_dir = TempDir::new()?;
    let file = tmp_dir.child(".tmp file.txt");
    file.touch()?;

    Command::cargo_bin(PRG)?
        .args([tmp_dir.path().to_str().unwrap(), "--include-hidden"])
        .assert()
        .success();

    let renamed_file = tmp_dir.child(".tmp_file.txt");
    assert!(renamed_file.exists());

    Ok(())
}

// --------------------------------------------------

#[test]
fn hidden_file_skipped() -> Result<()> {
    let tmp_dir = TempDir::new()?;
    let file = tmp_dir.child(".tmp file.txt");
    file.touch()?;

    Command::cargo_bin(PRG)?
        .arg(tmp_dir.path())
        .assert()
        .success();

    let not_renamed_file = tmp_dir.child(".tmp file.txt");
    assert!(not_renamed_file.exists());

    Ok(())
}

// --------------------------------------------------

#[test]
fn verbose() -> Result<()> {
    let tmp_dir = TempDir::new()?;
    let file = tmp_dir.child("tmp file.txt");
    file.touch()?;

    Command::cargo_bin(PRG)?
        .args([tmp_dir.path().to_str().unwrap(), "--verbose"])
        .assert()
        .success()
        .stdout(predicate::str::contains("Renamed"));

    Ok(())
}

// --------------------------------------------------

#[test]
fn collision() -> Result<()> {
    let tmp_dir = TempDir::new()?;
    let file = tmp_dir.child("tmp file.txt");
    file.touch()?;

    let file2 = tmp_dir.child("tmp_file.txt");
    file2.touch()?;

    Command::cargo_bin(PRG)?
        .arg(tmp_dir.path())
        .assert()
        .success();

    let renamed_file = tmp_dir.child("tmp_file_1.txt");
    assert!(renamed_file.exists());

    Ok(())
}
