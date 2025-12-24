//! Update command - mechanical updates to files

use crate::args::UpdateCommands;
use crate::utils::git;
use anyhow::{bail, Context, Result};
use chrono::Local;
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize)]
pub struct UpdateResult {
    pub success: bool,
    pub command: String,
    pub updated_files: Vec<String>,
    pub changes: Vec<Change>,
    pub message: String,
}

#[derive(Serialize)]
pub struct Change {
    pub field: String,
    pub old_value: Option<String>,
    pub new_value: String,
}

#[derive(Debug, Deserialize, Serialize)]
struct RiskRegister {
    metadata: serde_yaml::Value,
    stats: RiskStats,
    #[serde(default)]
    metriques: serde_yaml::Value,
    #[serde(default)]
    conformite: serde_yaml::Value,
    #[serde(default)]
    risques: Vec<Risk>,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct RiskStats {
    total: u32,
    par_statut: StatutCounts,
    par_severite: SeveriteCounts,
    #[serde(default)]
    score_total: u32,
    #[serde(default)]
    score_residuel: u32,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct StatutCounts {
    #[serde(default)]
    ouverts: u32,
    #[serde(default)]
    en_cours: u32,
    #[serde(default)]
    resolus: u32,
    #[serde(default)]
    verifies: u32,
    #[serde(default)]
    acceptes: u32,
}

#[derive(Debug, Deserialize, Serialize, Default)]
struct SeveriteCounts {
    #[serde(default)]
    critiques: u32,
    #[serde(default)]
    importants: u32,
    #[serde(default)]
    mineurs: u32,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Risk {
    id: String,
    #[serde(default)]
    statut: String,
    #[serde(default)]
    severite: String,
    #[serde(default)]
    priorite: String,
    #[serde(default)]
    score_initial: Option<u32>,
    #[serde(default)]
    score_residuel: Option<u32>,
    #[serde(flatten)]
    other: serde_yaml::Value,
}

#[derive(Debug, Deserialize, Serialize)]
struct TasksFile {
    metadata: serde_yaml::Value,
    tasks: Vec<Task>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
struct Task {
    id: String,
    #[serde(default)]
    status: String,
    #[serde(flatten)]
    other: serde_yaml::Value,
}

pub fn run(command: UpdateCommands, json: bool) -> Result<()> {
    match command {
        UpdateCommands::Stats => update_stats(json),
        UpdateCommands::Task { id, done } => update_task(&id, done, json),
        UpdateCommands::Risk { id, status } => update_risk(&id, &status, json),
        UpdateCommands::Dashboard => update_dashboard(json),
    }
}

fn update_stats(json: bool) -> Result<()> {
    let register_path = "docs/security/risks/risk-register.yaml";

    if !std::path::Path::new(register_path).exists() {
        bail!("risk-register.yaml not found. Run /osk-analyze first.");
    }

    // Lire le fichier
    let content = fs::read_to_string(register_path).context("Failed to read risk-register.yaml")?;

    let mut register: RiskRegister =
        serde_yaml::from_str(&content).context("Failed to parse risk-register.yaml")?;

    // Calculer les stats
    let old_total = register.stats.total;
    let mut stats = RiskStats {
        total: register.risques.len() as u32,
        ..Default::default()
    };

    for risk in &register.risques {
        // Par statut
        match risk.statut.to_uppercase().as_str() {
            "OUVERT" => stats.par_statut.ouverts += 1,
            "EN_COURS" => stats.par_statut.en_cours += 1,
            "RESOLU" => stats.par_statut.resolus += 1,
            "VERIFIE" => stats.par_statut.verifies += 1,
            "ACCEPTE" => stats.par_statut.acceptes += 1,
            _ => stats.par_statut.ouverts += 1, // Default to open
        }

        // Par sévérité
        match risk.severite.to_uppercase().as_str() {
            "CRITIQUE" => stats.par_severite.critiques += 1,
            "IMPORTANT" => stats.par_severite.importants += 1,
            _ => stats.par_severite.mineurs += 1,
        }

        // Scores
        if let Some(score) = risk.score_initial {
            stats.score_total += score;
        }
        if let Some(score) = risk.score_residuel {
            stats.score_residuel += score;
        }
    }

    register.stats = stats;

    // Mettre à jour last_updated dans metadata
    if let serde_yaml::Value::Mapping(ref mut map) = register.metadata {
        map.insert(
            serde_yaml::Value::String("last_updated".to_string()),
            serde_yaml::Value::String(Local::now().format("%Y-%m-%d").to_string()),
        );
    }

    // Sauvegarder
    let yaml = serde_yaml::to_string(&register)?;
    fs::write(register_path, yaml)?;

    let changes = vec![
        Change {
            field: "stats.total".to_string(),
            old_value: Some(old_total.to_string()),
            new_value: register.stats.total.to_string(),
        },
        Change {
            field: "stats.par_statut.ouverts".to_string(),
            old_value: None,
            new_value: register.stats.par_statut.ouverts.to_string(),
        },
        Change {
            field: "stats.par_statut.resolus".to_string(),
            old_value: None,
            new_value: register.stats.par_statut.resolus.to_string(),
        },
    ];

    if json {
        let result = UpdateResult {
            success: true,
            command: "update stats".to_string(),
            updated_files: vec![register_path.to_string()],
            changes,
            message: format!(
                "Stats updated: {} risks ({} open, {} resolved)",
                register.stats.total,
                register.stats.par_statut.ouverts,
                register.stats.par_statut.resolus
            ),
        };
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("📊 Updated stats in {}", register_path);
        println!("   Total: {} risks", register.stats.total);
        println!(
            "   Open: {}, In progress: {}, Resolved: {}",
            register.stats.par_statut.ouverts,
            register.stats.par_statut.en_cours,
            register.stats.par_statut.resolus
        );
        println!(
            "   Critical: {}, Important: {}, Minor: {}",
            register.stats.par_severite.critiques,
            register.stats.par_severite.importants,
            register.stats.par_severite.mineurs
        );
    }

    Ok(())
}

fn update_task(id: &str, done: bool, json: bool) -> Result<()> {
    if !done {
        bail!("Use --done to mark task as completed");
    }

    // Chercher le fichier tasks.yaml contenant cette tâche
    let specs_dir = std::path::Path::new(".osk/specs");
    if !specs_dir.exists() {
        bail!("No .osk/specs directory found");
    }

    let mut found_file: Option<String> = None;
    let mut old_status: Option<String> = None;

    for entry in fs::read_dir(specs_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let tasks_path = entry.path().join("tasks.yaml");
            if tasks_path.exists() {
                let content = fs::read_to_string(&tasks_path)?;
                let mut tasks_file: TasksFile = serde_yaml::from_str(&content)
                    .with_context(|| format!("Failed to parse {}", tasks_path.display()))?;

                // Chercher la tâche
                let mut task_found = false;
                for task in &mut tasks_file.tasks {
                    if task.id == id {
                        old_status = Some(task.status.clone());
                        task.status = "done".to_string();
                        task_found = true;
                        break;
                    }
                }

                if task_found {
                    // Sauvegarder le fichier
                    let yaml = serde_yaml::to_string(&tasks_file)?;
                    fs::write(&tasks_path, &yaml)?;
                    found_file = Some(tasks_path.to_string_lossy().to_string());
                    break;
                }
            }
        }
    }

    let Some(file_path) = found_file else {
        bail!("Task {} not found in any tasks.yaml", id);
    };

    // Git commit si dans un repo
    let commit_sha = if git::is_git_repo() {
        git::add_and_commit(
            &[&file_path],
            &format!("chore(security): mark task {} as done", id),
        )
        .ok()
    } else {
        None
    };

    let changes = vec![Change {
        field: format!("{}.status", id),
        old_value: old_status,
        new_value: "done".to_string(),
    }];

    if json {
        let result = UpdateResult {
            success: true,
            command: "update task".to_string(),
            updated_files: vec![file_path.clone()],
            changes,
            message: format!("Task {} marked as done", id),
        };
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("✅ Task {} marked as done in {}", id, file_path);
        if let Some(sha) = commit_sha {
            println!("📝 Committed: {}", sha);
        }
    }

    Ok(())
}

fn update_risk(id: &str, status: &str, json: bool) -> Result<()> {
    let valid_statuses = ["OUVERT", "EN_COURS", "RESOLU", "VERIFIE", "ACCEPTE"];
    let status_upper = status.to_uppercase();

    if !valid_statuses.contains(&status_upper.as_str()) {
        bail!(
            "Invalid status '{}'. Must be one of: {}",
            status,
            valid_statuses.join(", ")
        );
    }

    let register_path = "docs/security/risks/risk-register.yaml";

    if !std::path::Path::new(register_path).exists() {
        bail!("risk-register.yaml not found");
    }

    let content = fs::read_to_string(register_path)?;
    let mut register: RiskRegister = serde_yaml::from_str(&content)?;

    // Trouver et mettre à jour le risque
    let mut old_status: Option<String> = None;
    let mut risk_found = false;

    for risk in &mut register.risques {
        if risk.id == id {
            old_status = Some(risk.statut.clone());
            risk.statut = status_upper.clone();
            risk_found = true;

            // Si résolu, mettre à jour score_residuel à 0
            if status_upper == "RESOLU" || status_upper == "VERIFIE" {
                risk.score_residuel = Some(0);
            }
            break;
        }
    }

    if !risk_found {
        bail!("Risk {} not found in risk-register.yaml", id);
    }

    // Mettre à jour last_updated
    if let serde_yaml::Value::Mapping(ref mut map) = register.metadata {
        map.insert(
            serde_yaml::Value::String("last_updated".to_string()),
            serde_yaml::Value::String(Local::now().format("%Y-%m-%d").to_string()),
        );
    }

    // Sauvegarder
    let yaml = serde_yaml::to_string(&register)?;
    fs::write(register_path, yaml)?;

    let changes = vec![Change {
        field: format!("{}.statut", id),
        old_value: old_status.clone(),
        new_value: status_upper.clone(),
    }];

    if json {
        let result = UpdateResult {
            success: true,
            command: "update risk".to_string(),
            updated_files: vec![register_path.to_string()],
            changes,
            message: format!(
                "Risk {} updated: {} → {}",
                id,
                old_status.unwrap_or_default(),
                status_upper
            ),
        };
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("📝 Risk {} updated to {}", id, status_upper);
        if let Some(old) = old_status {
            println!("   {} → {}", old, status_upper);
        }
    }

    Ok(())
}

fn update_dashboard(json: bool) -> Result<()> {
    update_dashboard_in(std::path::Path::new("."), json)
}

fn update_dashboard_in(base: &std::path::Path, json: bool) -> Result<()> {
    let register_path = base.join("docs/security/risks/risk-register.yaml");
    let dashboard_path = base.join("docs/security/dashboard.md");

    let (total, open, resolved, critical) = if register_path.exists() {
        let content = fs::read_to_string(register_path)?;
        let register: RiskRegister = serde_yaml::from_str(&content)?;
        (
            register.stats.total,
            register.stats.par_statut.ouverts,
            register.stats.par_statut.resolus,
            register.stats.par_severite.critiques,
        )
    } else {
        (0, 0, 0, 0)
    };

    let specs_dir = base.join(".osk/specs");
    let feature_count = if specs_dir.exists() {
        fs::read_dir(&specs_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().map(|t| t.is_dir()).unwrap_or(false))
            .count()
    } else {
        0
    };

    let incidents_dir = base.join("docs/security/incidents");
    let incident_count = if incidents_dir.exists() {
        fs::read_dir(&incidents_dir)?
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().starts_with("INC-"))
            .count()
    } else {
        0
    };

    let now = Local::now().format("%Y-%m-%d %H:%M").to_string();
    let resolution_rate = if total > 0 {
        (resolved as f64 / total as f64 * 100.0) as u32
    } else {
        0
    };

    let dashboard = format!(
        r#"# Security Dashboard

> Generated: {now}

## Overview

| Metric | Value |
|--------|-------|
| Total Risks | {total} |
| Open | {open} |
| Resolved | {resolved} |
| Critical | {critical} |
| Resolution Rate | {resolution_rate}% |

## Features Analyzed

{feature_count} feature(s) in `.osk/specs/`

## Incidents

{incident_count} incident(s) tracked

## Risk Distribution

```
Critical:  {critical_bar}  {critical}
Open:      {open_bar}  {open}
Resolved:  {resolved_bar}  {resolved}
```

## Quick Actions

- Run `/osk-analyze <feature>` to analyze a new feature
- Run `osk update stats` to refresh statistics
- Run `osk validate yaml` to check file integrity

---

*Auto-generated by `osk update dashboard`*
"#,
        now = now,
        total = total,
        open = open,
        resolved = resolved,
        critical = critical,
        resolution_rate = resolution_rate,
        feature_count = feature_count,
        incident_count = incident_count,
        critical_bar = "█".repeat(critical.min(20) as usize),
        open_bar = "█".repeat(open.min(20) as usize),
        resolved_bar = "█".repeat(resolved.min(20) as usize),
    );

    if let Some(parent) = dashboard_path.parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(&dashboard_path, &dashboard)?;

    let dashboard_path_str = dashboard_path.to_string_lossy().to_string();

    if json {
        let result = UpdateResult {
            success: true,
            command: "update dashboard".to_string(),
            updated_files: vec![dashboard_path_str.clone()],
            changes: vec![
                Change {
                    field: "total_risks".to_string(),
                    old_value: None,
                    new_value: total.to_string(),
                },
                Change {
                    field: "resolution_rate".to_string(),
                    old_value: None,
                    new_value: format!("{}%", resolution_rate),
                },
            ],
            message: format!("Dashboard regenerated with {} risks", total),
        };
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("📊 Dashboard regenerated: {}", dashboard_path_str);
        println!("   {} risks ({} open, {} resolved)", total, open, resolved);
        println!(
            "   {} features, {} incidents",
            feature_count, incident_count
        );
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_update_dashboard_empty() {
        let dir = tempdir().unwrap();
        update_dashboard_in(dir.path(), false).unwrap();

        let dashboard = dir.path().join("docs/security/dashboard.md");
        assert!(dashboard.exists());

        let content = fs::read_to_string(dashboard).unwrap();
        assert!(content.contains("# Security Dashboard"));
        assert!(content.contains("Total Risks | 0"));
        assert!(content.contains("0 feature(s)"));
    }

    #[test]
    fn test_update_dashboard_with_features() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::create_dir_all(base.join(".osk/specs/001-auth")).unwrap();
        fs::create_dir_all(base.join(".osk/specs/002-api")).unwrap();

        update_dashboard_in(base, false).unwrap();

        let content = fs::read_to_string(base.join("docs/security/dashboard.md")).unwrap();
        assert!(content.contains("2 feature(s)"));
    }

    #[test]
    fn test_update_dashboard_with_incidents() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::create_dir_all(base.join("docs/security/incidents")).unwrap();
        fs::write(
            base.join("docs/security/incidents/INC-2025-01-01-001.md"),
            "",
        )
        .unwrap();
        fs::write(
            base.join("docs/security/incidents/INC-2025-01-01-002.md"),
            "",
        )
        .unwrap();

        update_dashboard_in(base, false).unwrap();

        let content = fs::read_to_string(base.join("docs/security/dashboard.md")).unwrap();
        assert!(content.contains("2 incident(s)"));
    }

    #[test]
    fn test_update_dashboard_with_risks() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::create_dir_all(base.join("docs/security/risks")).unwrap();
        let risk_register = r#"
metadata:
  version: "1.0"
stats:
  total: 5
  par_statut:
    ouverts: 3
    resolus: 2
  par_severite:
    critiques: 1
    importants: 2
    mineurs: 2
risques: []
"#;
        fs::write(
            base.join("docs/security/risks/risk-register.yaml"),
            risk_register,
        )
        .unwrap();

        update_dashboard_in(base, false).unwrap();

        let content = fs::read_to_string(base.join("docs/security/dashboard.md")).unwrap();
        assert!(content.contains("Total Risks | 5"));
        assert!(content.contains("Open | 3"));
        assert!(content.contains("Resolved | 2"));
    }
}
