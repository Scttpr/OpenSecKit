//! Scaffold command - creates file structures

use crate::args::ScaffoldCommands;
use crate::utils::{counter, git, template};
use anyhow::{Context, Result};
use serde::Serialize;
use std::fs;

#[derive(Serialize)]
pub struct ScaffoldResult {
    pub success: bool,
    pub command: String,
    pub created_dir: Option<String>,
    pub created_files: Vec<String>,
    pub branch: Option<String>,
    pub next_command: Option<String>,
    pub message: String,
}

pub fn run(command: ScaffoldCommands, json: bool) -> Result<()> {
    match command {
        ScaffoldCommands::Feature { name, no_branch } => scaffold_feature(&name, no_branch, json),
        ScaffoldCommands::Incident {
            description,
            severity,
        } => scaffold_incident(&description, &severity, json),
        ScaffoldCommands::Rgpd => scaffold_rgpd(json),
        ScaffoldCommands::Rgs { system } => scaffold_rgs(&system, json),
    }
}

fn scaffold_feature(name: &str, no_branch: bool, json: bool) -> Result<()> {
    let num = counter::next_feature_number()?;
    let slug = counter::slugify(name);
    let dir_name = format!("{:03}-{}", num, slug);
    let spec_dir = format!(".osk/specs/{}", dir_name);

    fs::create_dir_all(&spec_dir).with_context(|| format!("Impossible de créer {}", spec_dir))?;

    let template_files = [
        (
            "threats.md",
            include_str!("../../templates/feature/threats.md.tmpl"),
        ),
        (
            "risks.md",
            include_str!("../../templates/feature/risks.md.tmpl"),
        ),
        (
            "requirements.md",
            include_str!("../../templates/feature/requirements.md.tmpl"),
        ),
        (
            "testing.md",
            include_str!("../../templates/feature/testing.md.tmpl"),
        ),
        (
            "hardening.md",
            include_str!("../../templates/feature/hardening.md.tmpl"),
        ),
        (
            "plan.md",
            include_str!("../../templates/feature/plan.md.tmpl"),
        ),
        (
            "tasks.yaml",
            include_str!("../../templates/feature/tasks.yaml.tmpl"),
        ),
    ];

    let variables = [
        ("feature_name", name),
        ("feature_id", &dir_name),
        ("feature_slug", &slug),
    ];

    let mut created_files = Vec::new();
    for (filename, content) in template_files {
        let path = format!("{}/{}", spec_dir, filename);
        template::create_from_template(&path, content, &variables)?;
        created_files.push(path);
    }

    let branch = if !no_branch && git::is_git_repo() {
        let branch_name = format!("security/{}", dir_name);
        git::create_branch(&branch_name)?;
        Some(branch_name)
    } else {
        None
    };

    let next_cmd = format!("/osk-analyze {}", name);

    if json {
        let result = ScaffoldResult {
            success: true,
            command: "scaffold feature".to_string(),
            created_dir: Some(spec_dir.clone()),
            created_files,
            branch,
            next_command: Some(next_cmd),
            message: format!("Feature '{}' scaffolded as {}", name, dir_name),
        };
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("📁 Created {}/", spec_dir);
        for file in &created_files {
            let filename = file.split('/').next_back().unwrap_or(file);
            println!("   ├── {}", filename);
        }
        if let Some(ref b) = branch {
            println!("🔀 Created branch: {}", b);
        }
        println!("\n✅ Feature '{}' scaffolded as {}", name, dir_name);
        println!("💡 Next: Run {}", next_cmd);
    }

    Ok(())
}

fn scaffold_incident(description: &str, severity: &str, json: bool) -> Result<()> {
    use chrono::Local;

    let date = Local::now().format("%Y-%m-%d").to_string();
    let seq = counter::next_incident_number(&date)?;
    let incident_id = format!("INC-{}-{:03}", date, seq);

    fs::create_dir_all("docs/security/incidents")?;

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

    if json {
        let result = ScaffoldResult {
            success: true,
            command: "scaffold incident".to_string(),
            created_dir: Some("docs/security/incidents".to_string()),
            created_files: vec![file_path],
            branch: None,
            next_command: None,
            message: format!("Incident {} created", incident_id),
        };
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("📁 Created {}", file_path);
        let risk_register = "docs/security/risks/risk-register.yaml";
        if std::path::Path::new(risk_register).exists() {
            let risk_id = format!("RISK-{}", incident_id);
            println!("⚠️  TODO: Add {} to risk-register.yaml", risk_id);
        }
        println!("\n✅ Incident {} created", incident_id);
    }

    Ok(())
}

fn scaffold_rgpd(json: bool) -> Result<()> {
    let rgpd_dir = "docs/security/rgpd";
    fs::create_dir_all(rgpd_dir)?;

    let files = [
        (
            "registre-traitements.md",
            "# Registre des Traitements (Art. 30)\n\nTODO: Documenter les traitements",
        ),
        (
            "dpia-global.md",
            "# DPIA Global (Art. 35)\n\nTODO: Analyse d'impact",
        ),
        (
            "procedure-violation.md",
            "# Procédure de Violation de Données\n\nTODO: Procédure notification",
        ),
    ];

    let mut created_files = Vec::new();
    for (filename, content) in files {
        let path = format!("{}/{}", rgpd_dir, filename);
        if !std::path::Path::new(&path).exists() {
            fs::write(&path, content)?;
            created_files.push(path);
        }
    }

    if json {
        let result = ScaffoldResult {
            success: true,
            command: "scaffold rgpd".to_string(),
            created_dir: Some(rgpd_dir.to_string()),
            created_files,
            branch: None,
            next_command: Some("/osk-rgpd".to_string()),
            message: "RGPD structure created".to_string(),
        };
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("📁 Created {}/", rgpd_dir);
        for file in &created_files {
            let filename = file.split('/').next_back().unwrap_or(file);
            println!("   ├── {}", filename);
        }
        println!("\n✅ RGPD structure created");
        println!("💡 Next: Run /osk-rgpd to configure");
    }

    Ok(())
}

fn scaffold_rgs(system: &str, json: bool) -> Result<()> {
    let rgs_dir = "docs/security/rgs";
    fs::create_dir_all(rgs_dir)?;

    let slug = counter::slugify(system);
    let files = [
        (
            format!("EBIOS-RM-{}.md", slug),
            format!("# EBIOS RM - {}\n\nTODO: Analyse de risques", system),
        ),
        (
            format!("DOSSIER-HOMOLOGATION-{}.md", slug),
            format!("# Dossier d'Homologation - {}\n\nTODO: Dossier SSI", system),
        ),
        (
            format!("MCS-{}.md", slug),
            format!(
                "# Maintien en Conditions de Sécurité - {}\n\nTODO: Plan MCS",
                system
            ),
        ),
    ];

    let mut created_files = Vec::new();
    for (filename, content) in files {
        let path = format!("{}/{}", rgs_dir, filename);
        if !std::path::Path::new(&path).exists() {
            fs::write(&path, content)?;
            created_files.push(path);
        }
    }

    if json {
        let result = ScaffoldResult {
            success: true,
            command: "scaffold rgs".to_string(),
            created_dir: Some(rgs_dir.to_string()),
            created_files,
            branch: None,
            next_command: Some("/osk-rgs".to_string()),
            message: format!("RGS structure created for '{}'", system),
        };
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("📁 Created {}/", rgs_dir);
        for file in &created_files {
            let filename = file.split('/').next_back().unwrap_or(file);
            println!("   ├── {}", filename);
        }
        println!("\n✅ RGS structure created for '{}'", system);
        println!("💡 Next: Run /osk-rgs to configure");
    }

    Ok(())
}
