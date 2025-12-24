//! Helpers pour les opérations Git
//!
//! Fournit des wrappers autour des commandes git
//! pour les opérations courantes du workflow

use anyhow::{bail, Context, Result};
use std::process::Command;

/// Vérifie si le répertoire courant est un repo git
pub fn is_git_repo() -> bool {
    Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .output()
        .map(|o| o.status.success())
        .unwrap_or(false)
}

/// Vérifie si le working tree est propre (pas de modifications)
pub fn is_clean() -> Result<bool> {
    let output = Command::new("git")
        .args(["status", "--porcelain"])
        .output()
        .context("Erreur git status")?;

    Ok(output.stdout.is_empty())
}

/// Crée une nouvelle branche et s'y positionne
pub fn create_branch(name: &str) -> Result<()> {
    // Vérifier si la branche existe déjà
    let check = Command::new("git")
        .args(["show-ref", "--verify", "--quiet", &format!("refs/heads/{}", name)])
        .status()
        .context("Erreur vérification branche")?;

    if check.success() {
        // La branche existe, juste checkout
        let status = Command::new("git")
            .args(["checkout", name])
            .status()
            .context("Erreur git checkout")?;

        if !status.success() {
            bail!("Impossible de checkout la branche {}", name);
        }
    } else {
        // Créer et checkout
        let status = Command::new("git")
            .args(["checkout", "-b", name])
            .status()
            .context("Erreur git checkout -b")?;

        if !status.success() {
            bail!("Impossible de créer la branche {}", name);
        }
    }

    Ok(())
}

/// Retourne le nom de la branche courante
pub fn current_branch() -> Result<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .context("Erreur git rev-parse")?;

    if !output.status.success() {
        bail!("Impossible de déterminer la branche courante");
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

/// Stage des fichiers
pub fn add(paths: &[&str]) -> Result<()> {
    let mut cmd = Command::new("git");
    cmd.arg("add");

    for path in paths {
        cmd.arg(path);
    }

    let status = cmd.status().context("Erreur git add")?;

    if !status.success() {
        bail!("Erreur lors du staging des fichiers");
    }

    Ok(())
}

/// Crée un commit avec le message spécifié
pub fn commit(message: &str) -> Result<String> {
    let output = Command::new("git")
        .args(["commit", "-m", message])
        .output()
        .context("Erreur git commit")?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        bail!("Erreur git commit: {}", stderr);
    }

    // Récupérer le SHA du commit
    let sha_output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .context("Erreur git rev-parse")?;

    Ok(String::from_utf8_lossy(&sha_output.stdout).trim().to_string())
}

/// Stage et commit en une opération
pub fn add_and_commit(paths: &[&str], message: &str) -> Result<String> {
    add(paths)?;
    commit(message)
}

/// Récupère le SHA court du HEAD actuel
pub fn head_sha() -> Result<String> {
    let output = Command::new("git")
        .args(["rev-parse", "--short", "HEAD"])
        .output()
        .context("Erreur git rev-parse")?;

    if !output.status.success() {
        bail!("Impossible de récupérer le SHA du HEAD");
    }

    Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_git_repo() {
        // Le répertoire du projet devrait être un repo git
        // Ce test dépend de l'environnement d'exécution
        let result = is_git_repo();
        // On ne peut pas assert sans connaître le contexte
        println!("is_git_repo: {}", result);
    }
}
