use super::errors::*;
use std::path::Path;
use std::process::Command;

pub fn bundle_create(bundle: &Path, ref_name: &str) -> Result<()> {
    let result = Command::new("git")
        .arg("bundle")
        .arg("create")
        .arg(bundle.to_str().chain_err(|| "bundle path invalid")?)
        .arg(ref_name)
        .output()
        .chain_err(|| "failed to run git")?;
    if !result.status.success() {
        bail!("git bundle failed");
    }
    Ok(())
}

pub fn bundle_unbundle(bundle: &Path, ref_name: &str) -> Result<()> {
    let result = Command::new("git")
        .arg("bundle")
        .arg("unbundle")
        .arg(bundle.to_str().chain_err(|| "bundle path invalid")?)
        .arg(ref_name)
        .output()
        .chain_err(|| "failed to run git")?;
    if !result.status.success() {
        bail!("git unbundle failed");
    }
    Ok(())
}

pub fn is_ancestor(base_ref: &str, remote_ref: &str) -> Result<bool> {
    let result = Command::new("git")
        .arg("merge-base")
        .arg("--is-ancestor")
        .arg(remote_ref)
        .arg(base_ref)
        .output()
        .chain_err(|| "failed to run git")?;
    Ok(result.status.success())
}

pub fn config2(setting: &str, default: &str) -> Result<String> {
    let result = Command::new("git")
        .arg("config")
        .arg(setting)
        .output()
        .chain_err(|| "failed to run git")?;
    if !result.status.success() {
        return Ok(default.to_string());
    }
    let s = String::from_utf8(result.stdout).chain_err(|| "not utf8")?;
    return Ok(s.trim().to_string());
}

pub fn config(setting: &str) -> Result<String> {
    let ret = config2(setting, "")?;
    if ret == "" {
        bail!("git config failed");
    }
    return Ok(ret);
}

pub fn rev_parse(rev: &str) -> Result<String> {
    let result = Command::new("git")
        .arg("rev-parse")
        .arg(rev)
        .output()
        .chain_err(|| "failed to run git")?;
    if !result.status.success() {
        bail!("git rev-parse failed");
    }
    let s = String::from_utf8(result.stdout).chain_err(|| "not utf8")?;
    Ok(s.trim().to_string())
}
