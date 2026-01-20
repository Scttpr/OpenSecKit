//! Scan command - file listing utility for agents
//!
//! Provides gitignore-aware file listing that agents use for consistent results.
//! See FR-041 (respect gitignore) and FR-042 (analyze gitignore patterns).

use anyhow::{Context, Result};
use ignore::WalkBuilder;
use serde::Serialize;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Serialize)]
pub struct ScanResult {
    pub files: Vec<String>,
    pub file_count: usize,
    pub gitignore_patterns: Vec<String>,
    pub gitignore_hints: Vec<GitignoreHint>,
}

#[derive(Serialize)]
pub struct GitignoreHint {
    pub pattern: String,
    pub implies: String,
}

/// Run the scan command
pub fn run(path: Option<&str>, json: bool) -> Result<()> {
    let root = path
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."));
    let result = scan_directory(&root)?;

    if json {
        let output = serde_json::to_string_pretty(&result)?;
        println!("{}", output);
    } else {
        println!("  Scanned {} files", result.file_count);
        println!();
        for file in &result.files {
            println!("  {}", file);
        }
        if !result.gitignore_hints.is_empty() {
            println!();
            println!("  Gitignore hints:");
            for hint in &result.gitignore_hints {
                println!("  {} → {}", hint.pattern, hint.implies);
            }
        }
    }

    Ok(())
}

/// Scan directory respecting .gitignore (FR-041)
pub fn scan_directory(root: &Path) -> Result<ScanResult> {
    let mut files = Vec::new();

    let walker = WalkBuilder::new(root)
        .git_ignore(true)
        .git_global(true)
        .git_exclude(true)
        .hidden(false)
        .build();

    for entry in walker {
        let entry = entry.context("Failed to read directory entry")?;
        if entry.file_type().map(|ft| ft.is_file()).unwrap_or(false) {
            if let Ok(rel_path) = entry.path().strip_prefix(root) {
                files.push(rel_path.to_string_lossy().to_string());
            }
        }
    }

    files.sort();
    let file_count = files.len();

    // Parse gitignore patterns (FR-042)
    let gitignore_patterns = parse_gitignore(root)?;
    let gitignore_hints = analyze_gitignore_patterns(&gitignore_patterns);

    Ok(ScanResult {
        files,
        file_count,
        gitignore_patterns,
        gitignore_hints,
    })
}

/// Parse .gitignore file and extract patterns
fn parse_gitignore(root: &Path) -> Result<Vec<String>> {
    let gitignore_path = root.join(".gitignore");
    if !gitignore_path.exists() {
        return Ok(Vec::new());
    }

    let content = fs::read_to_string(&gitignore_path).context("Failed to read .gitignore")?;

    let patterns: Vec<String> = content
        .lines()
        .filter(|line| !line.trim().is_empty() && !line.trim().starts_with('#'))
        .map(|line| line.trim().to_string())
        .collect();

    Ok(patterns)
}

/// Analyze gitignore patterns to infer context clues (FR-042)
fn analyze_gitignore_patterns(patterns: &[String]) -> Vec<GitignoreHint> {
    let mut hints = Vec::new();

    for pattern in patterns {
        let hint = match pattern.as_str() {
            ".env" | ".env*" | ".env.local" | "*.env" => Some(GitignoreHint {
                pattern: pattern.clone(),
                implies: "Environment variables configuration used".to_string(),
            }),
            "*.key" | "*.pem" | "*.crt" | "*.p12" => Some(GitignoreHint {
                pattern: pattern.clone(),
                implies: "SSL/TLS certificates or private keys".to_string(),
            }),
            "secrets/" | "secrets/*" | ".secrets" => Some(GitignoreHint {
                pattern: pattern.clone(),
                implies: "Secrets directory exists".to_string(),
            }),
            "credentials.json" | "*credentials*" => Some(GitignoreHint {
                pattern: pattern.clone(),
                implies: "Cloud provider credentials used".to_string(),
            }),
            ".aws/" | ".gcloud/" | ".azure/" => Some(GitignoreHint {
                pattern: pattern.clone(),
                implies: "Cloud provider configuration present".to_string(),
            }),
            "node_modules/" | "node_modules" => Some(GitignoreHint {
                pattern: pattern.clone(),
                implies: "Node.js/JavaScript project".to_string(),
            }),
            "target/" | "target" => Some(GitignoreHint {
                pattern: pattern.clone(),
                implies: "Rust project (Cargo build directory)".to_string(),
            }),
            "__pycache__/" | "*.pyc" | ".venv/" | "venv/" => Some(GitignoreHint {
                pattern: pattern.clone(),
                implies: "Python project".to_string(),
            }),
            "vendor/" => Some(GitignoreHint {
                pattern: pattern.clone(),
                implies: "Vendored dependencies (Go, PHP, Ruby)".to_string(),
            }),
            ".terraform/" | "*.tfstate*" | "*.tfvars" => Some(GitignoreHint {
                pattern: pattern.clone(),
                implies: "Terraform infrastructure-as-code".to_string(),
            }),
            "docker-compose.override.yml" | ".docker/" => Some(GitignoreHint {
                pattern: pattern.clone(),
                implies: "Docker configuration present".to_string(),
            }),
            _ => None,
        };

        if let Some(h) = hint {
            // Avoid duplicates
            if !hints
                .iter()
                .any(|existing: &GitignoreHint| existing.implies == h.implies)
            {
                hints.push(h);
            }
        }
    }

    hints
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_scan_empty_directory() {
        let temp = TempDir::new().unwrap();
        let result = scan_directory(temp.path()).unwrap();
        assert_eq!(result.file_count, 0);
        assert!(result.files.is_empty());
    }

    #[test]
    fn test_scan_with_files() {
        let temp = TempDir::new().unwrap();
        fs::write(temp.path().join("file1.rs"), "fn main() {}").unwrap();
        fs::write(temp.path().join("file2.rs"), "fn test() {}").unwrap();

        let result = scan_directory(temp.path()).unwrap();
        assert_eq!(result.file_count, 2);
        assert!(result.files.contains(&"file1.rs".to_string()));
        assert!(result.files.contains(&"file2.rs".to_string()));
    }

    #[test]
    fn test_scan_respects_gitignore() {
        let temp = TempDir::new().unwrap();

        // Initialize a git repo for .gitignore to be respected
        std::process::Command::new("git")
            .args(["init"])
            .current_dir(temp.path())
            .output()
            .expect("Failed to init git repo");

        fs::write(temp.path().join(".gitignore"), "ignored.txt\n").unwrap();
        fs::write(temp.path().join("included.rs"), "").unwrap();
        fs::write(temp.path().join("ignored.txt"), "").unwrap();

        let result = scan_directory(temp.path()).unwrap();
        assert!(result.files.contains(&"included.rs".to_string()));
        assert!(!result.files.contains(&"ignored.txt".to_string()));
    }

    #[test]
    fn test_gitignore_hints() {
        let patterns = vec![
            ".env".to_string(),
            "node_modules/".to_string(),
            "*.key".to_string(),
        ];
        let hints = analyze_gitignore_patterns(&patterns);

        assert!(hints.iter().any(|h| h.implies.contains("Environment")));
        assert!(hints.iter().any(|h| h.implies.contains("Node.js")));
        assert!(hints.iter().any(|h| h.implies.contains("SSL/TLS")));
    }

    #[test]
    fn test_parse_gitignore() {
        let temp = TempDir::new().unwrap();
        let gitignore_content = "# Comment\n.env\nnode_modules/\n\n*.log\n";
        fs::write(temp.path().join(".gitignore"), gitignore_content).unwrap();

        let patterns = parse_gitignore(temp.path()).unwrap();
        assert_eq!(patterns.len(), 3);
        assert!(patterns.contains(&".env".to_string()));
        assert!(patterns.contains(&"node_modules/".to_string()));
        assert!(patterns.contains(&"*.log".to_string()));
    }
}
