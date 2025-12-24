//! Commande osk update
//!
//! Met à jour les fichiers de manière mécanique (stats, statuts, etc.)

use crate::args::UpdateCommands;
use anyhow::{bail, Result};

pub fn run(command: UpdateCommands) -> Result<()> {
    match command {
        UpdateCommands::Stats => update_stats(),
        UpdateCommands::Task { id, done } => update_task(&id, done),
        UpdateCommands::Risk { id, status } => update_risk(&id, &status),
        UpdateCommands::Dashboard => update_dashboard(),
    }
}

/// Recalcule les statistiques du risk-register
fn update_stats() -> Result<()> {
    let register_path = "docs/security/risks/risk-register.yaml";

    if !std::path::Path::new(register_path).exists() {
        bail!("risk-register.yaml not found. Run /osk-analyze first.");
    }

    // TODO: Implémenter le parsing et recalcul des stats
    println!("📊 Updating stats in {}...", register_path);
    println!("⚠️  TODO: Implement stats calculation");

    Ok(())
}

/// Met à jour le statut d'une tâche
fn update_task(id: &str, done: bool) -> Result<()> {
    if !done {
        bail!("Use --done to mark task as completed");
    }

    // TODO: Trouver le fichier tasks.yaml contenant la tâche
    // TODO: Mettre à jour le statut
    // TODO: Git commit

    println!("✅ Task {} marked as done", id);
    println!("⚠️  TODO: Implement task update and git commit");

    Ok(())
}

/// Met à jour le statut d'un risque
fn update_risk(id: &str, status: &str) -> Result<()> {
    let valid_statuses = ["OUVERT", "EN_COURS", "RESOLU", "ACCEPTE", "TRANSFERE"];

    if !valid_statuses.contains(&status) {
        bail!(
            "Invalid status '{}'. Must be one of: {}",
            status,
            valid_statuses.join(", ")
        );
    }

    // TODO: Mettre à jour le risk-register.yaml
    println!("📝 Updating risk {} to status {}", id, status);
    println!("⚠️  TODO: Implement risk status update");

    Ok(())
}

/// Régénère le dashboard
fn update_dashboard() -> Result<()> {
    // TODO: Collecter les métriques
    // TODO: Générer docs/security/dashboard.md

    println!("📊 Regenerating dashboard...");
    println!("⚠️  TODO: Implement dashboard generation");

    Ok(())
}
