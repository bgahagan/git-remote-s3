use super::errors::*;
use std::path::Path;
use std::process::Command;
use std::io::{Write};

pub fn encrypt(recipients: &[String], i: &Path, o: &Path, gpgcmd: &String) -> Result<()> {
    let mut cmd = Command::new(gpgcmd);
    cmd.arg("-q").arg("--batch");
    for recipient in recipients {
        cmd.arg("-r").arg(recipient);
    }
    let result = cmd
        .arg("-o")
        .arg(o.to_str().chain_err(|| "out path invalid")?)
        .arg("-e")
        .arg(i.to_str().chain_err(|| "in path invalid")?)
        .output()
        .chain_err(|| "failed to run gpg")?;

    if !result.status.success() {
        std::io::stdout().write_all(&result.stdout).unwrap();
        std::io::stderr().write_all(&result.stderr).unwrap();
        bail!("gpg encrypt failed");
    }
    Ok(())
}

pub fn decrypt(i: &Path, o: &Path, gpgcmd: &String) -> Result<()> {
    let result = Command::new(gpgcmd)
        .arg("-q")
        .arg("--batch")
        .arg("-o")
        .arg(o.to_str().chain_err(|| "out path invalid")?)
        .arg("-d")
        .arg(i.to_str().chain_err(|| "in path invalid")?)
        .output()
        .chain_err(|| "failed to run gpg")?;
    if !result.status.success() {
        std::io::stdout().write_all(&result.stdout).unwrap();
        std::io::stderr().write_all(&result.stderr).unwrap();
        bail!("gpg decrypt failed");
    }
    Ok(())
}
