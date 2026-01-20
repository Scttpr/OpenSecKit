//! Changes command - git change detection utility
//!
//! Detects files changed since last scan for incremental updates.
//! See FR-011: Analyze git diff since last scan.

use anyhow::{bail, Context, Result};
use serde::Serialize;
use std::path::Path;
use std::process::Command;

#[derive(Serialize)]
pub struct ChangesResult {
    pub since_commit: String,
    pub current_commit: String,
    pub changes: Vec<FileChange>,
    pub summary: ChangesSummary,
}

#[derive(Serialize)]
pub struct FileChange {
    pub path: String,
    pub change_type: ChangeType,
}

#[derive(Serialize, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ChangeType {
    Added,
    Modified,
    Deleted,
    Renamed,
}

#[derive(Serialize, Default)]
pub struct ChangesSummary {
    pub added: usize,
    pub modified: usize,
    pub deleted: usize,
    pub renamed: usize,
    pub total: usize,
}

/// Run the changes command
pub fn run(since: Option<&str>, json: bool) -> Result<()> {
    // Try to read last commit from index.yaml if --since not provided
    let since_commit = match since {
        Some(commit) => commit.to_string(),
        None => read_last_commit_from_index()?,
    };

    let result = get_changes(&since_commit)?;

    if json {
        let output = serde_json::to_string_pretty(&result)?;
        println!("{}", output);
    } else if result.changes.is_empty() {
        println!("No changes detected since {}", &since_commit[..7.min(since_commit.len())]);
    } else {
        println!(
            "» {} changes since {}",
            result.summary.total,
            &since_commit[..7.min(since_commit.len())]
        );
        println!();

        for change in &result.changes {
            let symbol = match change.change_type {
                ChangeType::Added => "+",
                ChangeType::Modified => "~",
                ChangeType::Deleted => "-",
                ChangeType::Renamed => "→",
            };
            println!("  {} {}", symbol, change.path);
        }

        println!();
        println!(
            "Summary: {} added, {} modified, {} deleted, {} renamed",
            result.summary.added,
            result.summary.modified,
            result.summary.deleted,
            result.summary.renamed
        );
    }

    Ok(())
}

/// Read last_commit from .osk/system-model/index.yaml
fn read_last_commit_from_index() -> Result<String> {
    let index_path = Path::new(".osk/system-model/index.yaml");

    if !index_path.exists() {
        bail!("No system model found. Run `osk init` and `/osk-discover init` first.");
    }

    let content = std::fs::read_to_string(index_path)
        .context("Failed to read index.yaml")?;

    // Parse YAML to find last_commit
    let yaml: serde_yaml::Value = serde_yaml::from_str(&content)
        .context("Failed to parse index.yaml")?;

    let last_commit = yaml
        .get("metadata")
        .and_then(|m| m.get("last_commit"))
        .and_then(|c| c.as_str())
        .ok_or_else(|| anyhow::anyhow!("No last_commit found in index.yaml. Use --since <commit>."))?;

    Ok(last_commit.to_string())
}

/// Get current HEAD commit
pub fn get_current_commit() -> Result<String> {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .context("Failed to run git rev-parse")?;

    if !output.status.success() {
        bail!("Not a git repository or git command failed");
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Get changes since a specific commit
pub fn get_changes(since_commit: &str) -> Result<ChangesResult> {
    let current_commit = get_current_commit()?;

    // Get diff with status
    let output = Command::new("git")
        .args([
            "diff",
            "--name-status",
            &format!("{}..HEAD", since_commit),
        ])
        .output()
        .context("Failed to run git diff")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Git diff failed: {}", stderr);
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut changes = Vec::new();
    let mut summary = ChangesSummary::default();

    for line in stdout.lines() {
        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() < 2 {
            continue;
        }

        let status = parts[0];
        let path = parts[1].to_string();

        let change_type = match status.chars().next() {
            Some('A') => {
                summary.added += 1;
                ChangeType::Added
            }
            Some('M') => {
                summary.modified += 1;
                ChangeType::Modified
            }
            Some('D') => {
                summary.deleted += 1;
                ChangeType::Deleted
            }
            Some('R') => {
                summary.renamed += 1;
                ChangeType::Renamed
            }
            _ => continue, // Skip unknown statuses
        };

        changes.push(FileChange { path, change_type });
    }

    summary.total = summary.added + summary.modified + summary.deleted + summary.renamed;

    Ok(ChangesResult {
        since_commit: since_commit.to_string(),
        current_commit,
        changes,
        summary,
    })
}
