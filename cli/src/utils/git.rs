//! Git operations helpers

use anyhow::{bail, Context, Result};
use std::process::Command;

pub fn is_git_repo() -> bool {
    Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

pub fn create_branch(name: &str) -> Result<()> {
    let check = Command::new("git")
        .args(["show-ref", "--verify", "--quiet", &format!("refs/heads/{}", name)])
        .status()
        .context("Failed to check branch")?;

    if check.success() {
        let status = Command::new("git")
            .args(["checkout", name])
            .status()
            .context("Failed to checkout branch")?;

        if !status.success() {
            bail!("Cannot checkout branch {}", name);
        }
    } else {
        let status = Command::new("git")
            .args(["checkout", "-b", name])
            .status()
            .context("Failed to create branch")?;

        if !status.success() {
            bail!("Cannot create branch {}", name);
        }
    }

    Ok(())
}

fn add(paths: &[&str]) -> Result<()> {
    let mut cmd = Command::new("git");
    cmd.arg("add");
    for path in paths {
        cmd.arg(path);
    }

    let status = cmd.status().context("Failed to git add")?;
    if !status.success() {
        bail!("Failed to stage files");
    }

    Ok(())
}

fn commit(message: &str) -> Result<String> {
    let output = Command::new("git")
        .args(["commit", "-m", message])
        .output()
        .context("Failed to git commit")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Git commit failed: {}", stderr);
    }

    let sha_output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .context("Failed to get commit SHA")?;

    Ok(String::from_utf8_lossy(&sha_output.stdout).trim().to_string())
}

pub fn add_and_commit(paths: &[&str], message: &str) -> Result<String> {
    add(paths)?;
    commit(message)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_git_repo() {
        let _ = is_git_repo();
    }
}
