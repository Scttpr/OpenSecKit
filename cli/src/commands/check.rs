//! Commande osk check
//!
//! Vérifie les prérequis pour les différentes commandes du workflow.
//! Retourne un code de sortie 0 si tout est OK, 1 si des fichiers manquent.

use crate::args::CheckCommands;
use crate::utils::counter;
use anyhow::Result;
use serde::Serialize;
use std::path::Path;

/// Résultat d'une vérification
#[derive(Debug, Serialize)]
pub struct CheckResult {
    pub ready: bool,
    pub command: String,
    pub found: Vec<String>,
    pub missing: Vec<String>,
    pub suggestion: Option<String>,
}

impl CheckResult {
    fn new(command: &str) -> Self {
        Self {
            ready: true,
            command: command.to_string(),
            found: Vec::new(),
            missing: Vec::new(),
            suggestion: None,
        }
    }

    fn check_file(&mut self, path: &str) {
        if Path::new(path).exists() {
            self.found.push(path.to_string());
        } else {
            self.missing.push(path.to_string());
            self.ready = false;
        }
    }

    fn check_dir(&mut self, path: &str) {
        if Path::new(path).is_dir() {
            self.found.push(path.to_string());
        } else {
            self.missing.push(path.to_string());
            self.ready = false;
        }
    }

    fn with_suggestion(mut self, suggestion: &str) -> Self {
        if !self.ready {
            self.suggestion = Some(suggestion.to_string());
        }
        self
    }

    fn print(&self) {
        for path in &self.found {
            println!("✅ {}", path);
        }
        for path in &self.missing {
            println!("❌ {} (missing)", path);
        }

        if self.ready {
            println!("\n🚀 Ready for /{}", self.command);
        } else if let Some(ref suggestion) = self.suggestion {
            println!("\n💡 {}", suggestion);
        }
    }

    fn print_json(&self) {
        println!("{}", serde_json::to_string_pretty(self).unwrap_or_default());
    }
}

pub fn run(command: CheckCommands, json: bool) -> Result<()> {
    let result = match command {
        CheckCommands::Configure => check_configure(),
        CheckCommands::Analyze => check_analyze(),
        CheckCommands::Specify { feature } => check_specify(&feature)?,
        CheckCommands::Harden { feature } => check_harden(&feature)?,
        CheckCommands::Plan { feature } => check_plan(&feature)?,
        CheckCommands::Tasks { feature } => check_tasks(&feature)?,
        CheckCommands::Implement { feature } => check_implement(&feature)?,
        CheckCommands::Dashboard => check_dashboard(),
    };

    if json {
        result.print_json();
    } else {
        result.print();
    }

    // Code de sortie basé sur le résultat
    if result.ready {
        Ok(())
    } else {
        std::process::exit(1);
    }
}

/// Vérifier les prérequis pour /osk-configure
fn check_configure() -> CheckResult {
    let mut result = CheckResult::new("osk-configure");

    result.check_dir(".osk");
    result.check_file(".osk/config.toml");

    result.with_suggestion("Run 'osk init' first")
}

/// Vérifier les prérequis pour /osk-analyze
fn check_analyze() -> CheckResult {
    let mut result = CheckResult::new("osk-analyze");

    result.check_file(".osk/memory/context.md");
    result.check_file(".osk/memory/constitution.md");

    result.with_suggestion("Run /osk-configure first")
}

/// Vérifier les prérequis pour /osk-specify
fn check_specify(feature: &str) -> Result<CheckResult> {
    let mut result = CheckResult::new("osk-specify");

    let feature_dir = get_feature_dir(feature, &mut result)?;

    if let Some(dir) = feature_dir {
        result.check_file(&format!("{}/threats.md", dir));
        result.check_file(&format!("{}/risks.md", dir));
    }

    Ok(result.with_suggestion(&format!("Run /osk-analyze {} first", feature)))
}

/// Vérifier les prérequis pour /osk-harden
fn check_harden(feature: &str) -> Result<CheckResult> {
    let mut result = CheckResult::new("osk-harden");

    let feature_dir = get_feature_dir(feature, &mut result)?;

    if let Some(dir) = feature_dir {
        result.check_file(&format!("{}/requirements.md", dir));
        result.check_file(&format!("{}/testing.md", dir));
    }

    Ok(result.with_suggestion(&format!("Run /osk-specify {} first", feature)))
}

/// Vérifier les prérequis pour /osk-plan
fn check_plan(feature: &str) -> Result<CheckResult> {
    let mut result = CheckResult::new("osk-plan");

    let feature_dir = get_feature_dir(feature, &mut result)?;

    if let Some(dir) = feature_dir {
        result.check_file(&format!("{}/threats.md", dir));
        result.check_file(&format!("{}/risks.md", dir));
        result.check_file(&format!("{}/requirements.md", dir));
        result.check_file(&format!("{}/testing.md", dir));
        result.check_file(&format!("{}/hardening.md", dir));
    }

    Ok(result.with_suggestion(&format!("Run /osk-harden {} first", feature)))
}

/// Vérifier les prérequis pour /osk-tasks
fn check_tasks(feature: &str) -> Result<CheckResult> {
    let mut result = CheckResult::new("osk-tasks");

    let feature_dir = get_feature_dir(feature, &mut result)?;

    if let Some(dir) = feature_dir {
        result.check_file(&format!("{}/plan.md", dir));
    }

    Ok(result.with_suggestion(&format!("Run /osk-plan {} first", feature)))
}

/// Vérifier les prérequis pour /osk-implement
fn check_implement(feature: &str) -> Result<CheckResult> {
    let mut result = CheckResult::new("osk-implement");

    let feature_dir = get_feature_dir(feature, &mut result)?;

    if let Some(dir) = feature_dir {
        result.check_file(&format!("{}/tasks.yaml", dir));
    }

    Ok(result.with_suggestion(&format!("Run /osk-tasks {} first", feature)))
}

/// Vérifier les prérequis pour /osk-dashboard
fn check_dashboard() -> CheckResult {
    let mut result = CheckResult::new("osk-dashboard");

    result.check_file("docs/security/risks/risk-register.yaml");
    result.check_dir(".osk/specs");

    result.with_suggestion("Run /osk-analyze on at least one feature first")
}

/// Helper pour trouver le répertoire d'une feature
fn get_feature_dir(feature: &str, result: &mut CheckResult) -> Result<Option<String>> {
    match counter::find_feature_dir(feature)? {
        Some(dir) => {
            result.found.push(format!("{} (feature directory)", dir));
            Ok(Some(dir))
        }
        None => {
            result.missing.push(format!(".osk/specs/*-{} (feature not found)", feature));
            result.ready = false;
            Ok(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    use std::fs;

    #[test]
    fn test_check_configure_missing() {
        let dir = tempdir().unwrap();
        std::env::set_current_dir(&dir).unwrap();

        let result = check_configure();
        assert!(!result.ready);
        assert!(!result.missing.is_empty());
    }

    #[test]
    fn test_check_configure_ready() {
        let dir = tempdir().unwrap();
        std::env::set_current_dir(&dir).unwrap();

        fs::create_dir_all(".osk").unwrap();
        fs::write(".osk/config.toml", "").unwrap();

        let result = check_configure();
        assert!(result.ready);
        assert!(result.missing.is_empty());
    }

    #[test]
    fn test_check_analyze_missing() {
        let dir = tempdir().unwrap();
        std::env::set_current_dir(&dir).unwrap();

        let result = check_analyze();
        assert!(!result.ready);
        assert_eq!(result.missing.len(), 2);
    }
}
