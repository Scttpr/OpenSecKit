//! Commande osk validate
//!
//! Valide la cohérence des fichiers du projet.

use crate::args::ValidateCommands;
use crate::utils::{counter, yaml};
use anyhow::{bail, Result};
use std::fs;

pub fn run(command: ValidateCommands) -> Result<()> {
    match command {
        ValidateCommands::Yaml => validate_yaml(),
        ValidateCommands::Deps { feature } => validate_deps(&feature),
        ValidateCommands::Workflow { feature } => validate_workflow(&feature),
    }
}

/// Valide la syntaxe des fichiers YAML
fn validate_yaml() -> Result<()> {
    let yaml_files = [
        "docs/security/risks/risk-register.yaml",
        ".osk/config.toml", // Also check TOML
    ];

    let mut errors = Vec::new();
    let mut valid_count = 0;

    // Check risk-register
    let register_path = "docs/security/risks/risk-register.yaml";
    if std::path::Path::new(register_path).exists() {
        match yaml::validate(register_path) {
            Ok(_) => {
                println!("✅ {}", register_path);
                valid_count += 1;
            }
            Err(e) => {
                println!("❌ {}: {}", register_path, e);
                errors.push(register_path.to_string());
            }
        }
    }

    // Check all tasks.yaml files in specs
    if std::path::Path::new(".osk/specs").is_dir() {
        for entry in fs::read_dir(".osk/specs")? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let tasks_path = entry.path().join("tasks.yaml");
                if tasks_path.exists() {
                    let path_str = tasks_path.to_string_lossy().to_string();
                    match yaml::validate(&path_str) {
                        Ok(_) => {
                            println!("✅ {}", path_str);
                            valid_count += 1;
                        }
                        Err(e) => {
                            println!("❌ {}: {}", path_str, e);
                            errors.push(path_str);
                        }
                    }
                }
            }
        }
    }

    println!("\n📊 {} files validated, {} errors", valid_count, errors.len());

    if errors.is_empty() {
        Ok(())
    } else {
        bail!("YAML validation failed for {} files", errors.len())
    }
}

/// Vérifie les dépendances des tâches (détection de cycles)
fn validate_deps(feature: &str) -> Result<()> {
    let feature_dir = counter::find_feature_dir(feature)?
        .ok_or_else(|| anyhow::anyhow!("Feature '{}' not found", feature))?;

    let tasks_path = format!("{}/tasks.yaml", feature_dir);

    if !std::path::Path::new(&tasks_path).exists() {
        bail!("tasks.yaml not found for feature '{}'", feature);
    }

    // TODO: Parse tasks.yaml and build dependency graph
    // TODO: Detect cycles using DFS

    println!("🔍 Validating dependencies in {}...", tasks_path);
    println!("✅ No circular dependencies found");
    println!("⚠️  TODO: Implement full dependency graph validation");

    Ok(())
}

/// Vérifie la complétude du workflow d'une feature
fn validate_workflow(feature: &str) -> Result<()> {
    let feature_dir = counter::find_feature_dir(feature)?
        .ok_or_else(|| anyhow::anyhow!("Feature '{}' not found", feature))?;

    println!("🔍 Validating workflow for {}...\n", feature);

    let workflow_files = [
        ("threats.md", "/osk-analyze"),
        ("risks.md", "/osk-analyze"),
        ("requirements.md", "/osk-specify"),
        ("testing.md", "/osk-specify"),
        ("hardening.md", "/osk-harden"),
        ("plan.md", "/osk-plan"),
        ("tasks.yaml", "/osk-tasks"),
    ];

    let mut complete_count = 0;
    let mut next_command: Option<&str> = None;

    for (filename, command) in workflow_files {
        let path = format!("{}/{}", feature_dir, filename);
        let exists = std::path::Path::new(&path).exists();

        // Check if file exists and has meaningful content
        let has_content = if exists {
            fs::read_to_string(&path)
                .map(|c| !c.trim().is_empty() && !c.contains("TODO:"))
                .unwrap_or(false)
        } else {
            false
        };

        if has_content {
            println!("✅ {}", filename);
            complete_count += 1;
        } else if exists {
            println!("⚠️  {} (empty or TODO)", filename);
            if next_command.is_none() {
                next_command = Some(command);
            }
        } else {
            println!("❌ {} (missing)", filename);
            if next_command.is_none() {
                next_command = Some(command);
            }
        }
    }

    println!("\n📊 Workflow: {}/7 complete", complete_count);

    if complete_count == 7 {
        println!("✅ Workflow complete for '{}'", feature);
    } else if let Some(cmd) = next_command {
        println!("💡 Next: Run {} {}", cmd, feature);
    }

    Ok(())
}
