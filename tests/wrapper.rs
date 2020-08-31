use std::process::Command;
use std::process::Stdio;

use anyhow::Result;
use std::ffi::OsStr;
use tempfile::tempdir;

const APP: &str = env!("CARGO_BIN_EXE_muchutils");

#[test]
fn name_picking() -> Result<()> {
    assert_eq!(Some(2), quietly(APP).status()?.code());
    assert_eq!(Some(0), quietly(APP).arg("true").status()?.code());
    assert_eq!(Some(1), quietly(APP).arg("false").status()?.code());
    Ok(())
}

#[test]
#[cfg(unix)]
fn name_picking_arg0() -> Result<()> {
    use std::os::unix::process::CommandExt;

    assert_eq!(Some(2), quietly(APP).status()?.code());
    assert_eq!(Some(0), quietly(APP).arg0("true").status()?.code());
    assert_eq!(Some(1), quietly(APP).arg0("false").status()?.code());
    Ok(())
}

#[test]
#[cfg(unix)]
fn name_picking_symlink() -> Result<()> {
    use std::os::unix::fs::symlink;

    let dir = tempdir()?;

    let mut true_bin = dir.path().to_path_buf();
    true_bin.push("true");
    symlink(APP, &true_bin)?;

    let mut false_bin = dir.path().to_path_buf();
    false_bin.push("false");
    symlink(APP, &false_bin)?;

    let mut nonsense_bin = dir.path().to_path_buf();
    nonsense_bin.push("sausage sausage sausage");
    symlink(APP, &nonsense_bin)?;

    assert_eq!(Some(2), quietly(nonsense_bin).status()?.code());
    assert_eq!(Some(0), quietly(true_bin).status()?.code());
    assert_eq!(Some(1), quietly(false_bin).status()?.code());

    dir.close()?;
    Ok(())
}

fn quietly<S: AsRef<OsStr>>(program: S) -> Command {
    let mut command = Command::new(program);
    command.stdout(Stdio::null());
    command.stderr(Stdio::null());
    command
}
