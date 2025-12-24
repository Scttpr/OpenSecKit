//! Helpers pour la substitution de templates
//!
//! Fournit des fonctions simples pour remplacer
//! des variables dans les templates

use anyhow::{Context, Result};
use chrono::Local;
use std::fs;
use std::path::Path;

/// Substitue les variables dans un contenu de template
///
/// Variables supportées :
/// - {{variable}} : remplacée par la valeur fournie
/// - {{DATE}} : date du jour (YYYY-MM-DD)
/// - {{DATETIME}} : date et heure (YYYY-MM-DD HH:MM:SS)
/// - {{YEAR}} : année courante
pub fn render(content: &str, variables: &[(&str, &str)]) -> String {
    let mut result = content.to_string();

    // Variables automatiques
    let now = Local::now();
    result = result.replace("{{DATE}}", &now.format("%Y-%m-%d").to_string());
    result = result.replace("{{DATETIME}}", &now.format("%Y-%m-%d %H:%M:%S").to_string());
    result = result.replace("{{YEAR}}", &now.format("%Y").to_string());

    // Variables fournies
    for (key, value) in variables {
        let pattern = format!("{{{{{}}}}}", key);
        result = result.replace(&pattern, value);
    }

    result
}

/// Copie un template vers une destination avec substitution de variables
pub fn copy_template(
    src: &str,
    dst: &str,
    variables: &[(&str, &str)],
) -> Result<()> {
    let content = fs::read_to_string(src)
        .with_context(|| format!("Impossible de lire le template {}", src))?;

    let rendered = render(&content, variables);

    // Créer le dossier parent si nécessaire
    if let Some(parent) = Path::new(dst).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(dst, rendered)
        .with_context(|| format!("Impossible d'écrire {}", dst))?;

    Ok(())
}

/// Crée un fichier à partir d'un contenu template embarqué
pub fn create_from_template(
    dst: &str,
    template_content: &str,
    variables: &[(&str, &str)],
) -> Result<()> {
    let rendered = render(template_content, variables);

    // Créer le dossier parent si nécessaire
    if let Some(parent) = Path::new(dst).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(dst, rendered)
        .with_context(|| format!("Impossible d'écrire {}", dst))?;

    Ok(())
}

/// Vérifie si un fichier template existe
pub fn template_exists(name: &str) -> bool {
    let path = format!(".osk/templates/outputs/{}.tmpl", name);
    Path::new(&path).exists()
}

/// Chemin vers un template de sortie
pub fn output_template_path(name: &str) -> String {
    format!(".osk/templates/outputs/{}.tmpl", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_simple() {
        let template = "Hello {{name}}!";
        let result = render(template, &[("name", "World")]);
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn test_render_multiple_vars() {
        let template = "{{greeting}} {{name}}, welcome to {{place}}!";
        let result = render(template, &[
            ("greeting", "Hello"),
            ("name", "Alice"),
            ("place", "Wonderland"),
        ]);
        assert_eq!(result, "Hello Alice, welcome to Wonderland!");
    }

    #[test]
    fn test_render_date() {
        let template = "Today is {{DATE}}";
        let result = render(template, &[]);
        // On ne peut pas tester la date exacte, mais on vérifie le format
        assert!(result.contains("-"));
        assert!(!result.contains("{{DATE}}"));
    }

    #[test]
    fn test_render_no_match() {
        let template = "No variables here";
        let result = render(template, &[("unused", "value")]);
        assert_eq!(result, "No variables here");
    }
}
