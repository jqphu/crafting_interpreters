mod util;

use anyhow::Result;
use assert_cmd::prelude::*;
use predicates::prelude::*;
use std::fs::{self, File};
use std::io::{prelude::*, ErrorKind};
use std::path::PathBuf;
use std::process::Command;
use util::BIN_NAME;

/// Temporary directory created to place files which will be deleted when dropped.
///
/// During test panics this directory will remain but will be deleted on the next run.
struct TestingTempDir {
    path: PathBuf,
}

impl TestingTempDir {
    /// Create a new temp dir. This will override the previous dir. Do not use concurrently.
    ///
    /// ```
    /// TestingTempDir::new()?
    /// TestingTempDir::new()?
    /// # Ok::<(), anyhow::Result>(())
    /// ```
    pub fn new() -> Result<TestingTempDir> {
        let path = PathBuf::from("/tmp/jlox_testing_dir");

        fs::create_dir(&path).unwrap_or_else(|error| {
            if error.kind() != ErrorKind::AlreadyExists {
                panic!("unexpected error creating dir: {}", error)
            }
        });

        Ok(TestingTempDir { path })
    }

    /// Create a directory with this file.
    pub fn new_with_file(name: &str, data: &str) -> Result<(TestingTempDir, PathBuf)> {
        let dir = TestingTempDir::new()?;

        let path_buf = dir.create_file(name, data)?;

        Ok((dir, path_buf))
    }

    // Create a file in directory.
    pub fn create_file(&self, name: &str, data: &str) -> Result<PathBuf> {
        let mut new_path = self.path.clone();
        new_path.push(name);

        // Remove everything in this path!
        fs::remove_file(&new_path).unwrap_or_else(|error| {
            if error.kind() != ErrorKind::NotFound {
                panic!("Error removing file: {}", error);
            }
        });

        let mut file = File::create(&new_path)?;
        file.write_all(data.as_bytes())?;
        file.flush()?;

        Ok(new_path)
    }
}

#[test]
fn open_empty_file() -> Result<()> {
    // Local file.
    let (_, path) = TestingTempDir::new_with_file("empty_file.lox", "")?;

    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("-f").arg(path);

    cmd.assert().success();

    Ok(())
}

#[test]
fn non_existent_file() -> Result<()> {
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("-f").arg("non_existent_file.lox");

    // I don't think this error is portable (it's an OS error not one outputted by the
    // application).
    // We probably shouldn't test this but let's just say this binary is not cross platform and
    // also hope the error message doesn't change :).
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

    Ok(())
}

#[test]
fn basic_read() -> Result<()> {
    let (_, path) = TestingTempDir::new_with_file("basic_file.lox", "fn hello() {}")?;
    let mut cmd = Command::cargo_bin(BIN_NAME)?;
    cmd.arg("-f").arg(path);
    cmd.assert().success();
    Ok(())
}
