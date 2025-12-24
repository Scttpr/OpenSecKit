//! Commande osk scaffold
//!
//! Crée des structures de fichiers pour les différents workflows.

use crate::args::ScaffoldCommands;
use crate::utils::{counter, git, template};
use anyhow::{Context, Result};
use std::fs;

pub fn run(command: ScaffoldCommands) -> Result<()> {
    match command {
        ScaffoldCommands::Feature { name, no_branch } => scaffold_feature(&name, no_branch),
        ScaffoldCommands::Incident { description, severity } => {
            scaffold_incident(&description, &severity)
        }
        ScaffoldCommands::Rgpd => scaffold_rgpd(),
        ScaffoldCommands::Rgs { system } => scaffold_rgs(&system),
    }
}

/// Crée la structure d'une nouvelle feature
fn scaffold_feature(name: &str, no_branch: bool) -> Result<()> {
    // 1. Obtenir le prochain numéro
    let num = counter::next_feature_number()?;
    let slug = counter::slugify(name);
    let dir_name = format!("{:03}-{}", num, slug);
    let spec_dir = format!(".osk/specs/{}", dir_name);

    // 2. Créer le répertoire
    fs::create_dir_all(&spec_dir)
        .with_context(|| format!("Impossible de créer {}", spec_dir))?;

    println!("📁 Created {}/", spec_dir);

    // 3. Créer les fichiers templates
    let templates = [
        ("threats.md", include_str!("../../templates/feature/threats.md.tmpl")),
        ("risks.md", include_str!("../../templates/feature/risks.md.tmpl")),
        ("requirements.md", include_str!("../../templates/feature/requirements.md.tmpl")),
        ("testing.md", include_str!("../../templates/feature/testing.md.tmpl")),
        ("hardening.md", include_str!("../../templates/feature/hardening.md.tmpl")),
        ("plan.md", include_str!("../../templates/feature/plan.md.tmpl")),
        ("tasks.yaml", include_str!("../../templates/feature/tasks.yaml.tmpl")),
    ];

    let variables = [
        ("feature_name", name),
        ("feature_id", &dir_name),
        ("feature_slug", &slug),
    ];

    for (filename, content) in templates {
        let path = format!("{}/{}", spec_dir, filename);
        template::create_from_template(&path, content, &variables)?;
        println!("   ├── {}", filename);
    }

    // 4. Créer la branche git si demandé
    if !no_branch && git::is_git_repo() {
        let branch_name = format!("security/{}", dir_name);
        git::create_branch(&branch_name)?;
        println!("🔀 Created branch: {}", branch_name);
    }

    println!("\n✅ Feature '{}' scaffolded as {}", name, dir_name);
    println!("💡 Next: Run /osk-analyze {}", name);

    Ok(())
}

/// Crée un rapport d'incident
fn scaffold_incident(description: &str, severity: &str) -> Result<()> {
    use chrono::Local;

    // 1. Générer l'ID
    let date = Local::now().format("%Y-%m-%d").to_string();
    let seq = counter::next_incident_number(&date)?;
    let incident_id = format!("INC-{}-{:03}", date, seq);

    // 2. Créer le répertoire
    fs::create_dir_all("docs/security/incidents")?;

    // 3. Créer le fichier incident
    let file_path = format!("docs/security/incidents/{}.md", incident_id);
    let content = format!(
        r#"# {incident_id}

## Statut: 🚨 EN COURS

## Informations

| Champ | Valeur |
|-------|--------|
| ID | {incident_id} |
| Date détection | {date} |
| Sévérité | {severity} |
| Description | {description} |

## Chronologie

- `{date}` - Incident détecté

## Actions Immédiates

- [ ] Contenir l'incident
- [ ] Évaluer l'impact
- [ ] Notifier les parties concernées

## Investigation

TODO: Documenter l'investigation

## Résolution

TODO: Documenter la résolution

## Post-Mortem

TODO: Analyse post-mortem après résolution
"#,
        incident_id = incident_id,
        date = date,
        severity = severity,
        description = description,
    );

    fs::write(&file_path, content)?;
    println!("📁 Created {}", file_path);

    // 4. Ajouter au risk-register si existe
    let risk_register = "docs/security/risks/risk-register.yaml";
    if std::path::Path::new(risk_register).exists() {
        let risk_id = format!("RISK-{}", incident_id);
        println!("⚠️  TODO: Add {} to risk-register.yaml", risk_id);
    }

    println!("\n✅ Incident {} created", incident_id);

    Ok(())
}

/// Crée la structure RGPD
fn scaffold_rgpd() -> Result<()> {
    let rgpd_dir = "docs/security/rgpd";
    fs::create_dir_all(rgpd_dir)?;

    println!("📁 Created {}/", rgpd_dir);

    let files = [
        ("registre-traitements.md", "# Registre des Traitements (Art. 30)\n\nTODO: Documenter les traitements"),
        ("dpia-global.md", "# DPIA Global (Art. 35)\n\nTODO: Analyse d'impact"),
        ("procedure-violation.md", "# Procédure de Violation de Données\n\nTODO: Procédure notification"),
    ];

    for (filename, content) in files {
        let path = format!("{}/{}", rgpd_dir, filename);
        if !std::path::Path::new(&path).exists() {
            fs::write(&path, content)?;
            println!("   ├── {}", filename);
        }
    }

    println!("\n✅ RGPD structure created");
    println!("💡 Next: Run /osk-rgpd to configure");

    Ok(())
}

/// Crée la structure RGS
fn scaffold_rgs(system: &str) -> Result<()> {
    let rgs_dir = "docs/security/rgs";
    fs::create_dir_all(rgs_dir)?;

    println!("📁 Created {}/", rgs_dir);

    let slug = counter::slugify(system);
    let files = [
        (format!("EBIOS-RM-{}.md", slug), format!("# EBIOS RM - {}\n\nTODO: Analyse de risques", system)),
        (format!("DOSSIER-HOMOLOGATION-{}.md", slug), format!("# Dossier d'Homologation - {}\n\nTODO: Dossier SSI", system)),
        (format!("MCS-{}.md", slug), format!("# Maintien en Conditions de Sécurité - {}\n\nTODO: Plan MCS", system)),
    ];

    for (filename, content) in files {
        let path = format!("{}/{}", rgs_dir, filename);
        if !std::path::Path::new(&path).exists() {
            fs::write(&path, content)?;
            println!("   ├── {}", filename);
        }
    }

    println!("\n✅ RGS structure created for '{}'", system);
    println!("💡 Next: Run /osk-rgs to configure");

    Ok(())
}
