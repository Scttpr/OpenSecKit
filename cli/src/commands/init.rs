use crate::args::Agent;
use crate::config::{
    DomainsConfig, MemoryConfig, Nis2Config, OskConfig, PrinciplesConfig, ProjectConfig,
    RgpdConfig, RgsConfig, SpecsConfig, StackConfig,
};
use crate::stack;
use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use reqwest::blocking::Client;
use std::fs;
use std::path::{Path, PathBuf};

/// Statistiques d'installation pour l'affichage condensé
#[derive(Default)]
struct InstallStats {
    prompts: usize,
    schemas: usize,
    outputs: usize,
    reports: usize,
    rgpd: usize,
    rgs: usize,
    nis2: usize,
    errors: Vec<String>,
}

/// Information sur un agent détecté
struct AgentInfo {
    agent: Agent,
    name: &'static str,
    available: bool,
}

pub fn run(
    client: &Client,
    force: bool,
    default: bool,
    agent: Option<Agent>,
    all_agents: bool,
) -> Result<()> {
    println!("🚀 Initialisation de OpenSecKit v3.1.0\n");

    // Configuration projet
    let config_exists = Path::new(".osk/config.toml").exists();
    let config = if config_exists && !force {
        println!("   ℹ️  Configuration existante détectée.");
        fs::read_to_string(".osk/config.toml")
            .and_then(|s| {
                toml::from_str(&s)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
            })
            .unwrap_or_else(|_| {
                println!("   ⚠️  Config corrompue, création d'une nouvelle...");
                if default {
                    default_configuration()
                } else {
                    prompt_configuration().unwrap()
                }
            })
    } else if default {
        default_configuration()
    } else {
        prompt_configuration()?
    };

    // Sélection des agents
    let selected_agents = select_agents(default, agent, all_agents)?;

    // Scaffold et installation
    scaffold_project(&config, force)?;
    let stats = install_resources(client, force)?;

    // Installation des configurations agents
    for agent in &selected_agents {
        install_agent_config(agent, &stats, force)?;
    }

    // Affichage du résumé
    print_summary(&stats, &selected_agents);

    Ok(())
}

/// Sélectionne les agents à configurer
fn select_agents(default: bool, agent: Option<Agent>, all_agents: bool) -> Result<Vec<Agent>> {
    if all_agents {
        Ok(Agent::all())
    } else if let Some(a) = agent {
        Ok(vec![a])
    } else if default {
        // Mode CI: Claude Code par défaut
        Ok(vec![Agent::ClaudeCode])
    } else {
        // Mode interactif: sélection
        let selected = prompt_agent_selection()?;
        Ok(vec![selected])
    }
}

/// Détecte les agents disponibles sur le système
fn detect_available_agents() -> Vec<AgentInfo> {
    vec![
        AgentInfo {
            agent: Agent::ClaudeCode,
            name: "Claude Code",
            available: which::which("claude").is_ok(),
        },
        AgentInfo {
            agent: Agent::Copilot,
            name: "GitHub Copilot",
            available: Path::new(".github").exists() || which::which("gh").is_ok(),
        },
        AgentInfo {
            agent: Agent::Cursor,
            name: "Cursor",
            available: which::which("cursor").is_ok() || Path::new(".cursor").exists(),
        },
        AgentInfo {
            agent: Agent::Gemini,
            name: "Gemini",
            available: which::which("gemini").is_ok(),
        },
    ]
}

/// Affiche un menu de sélection d'agent
fn prompt_agent_selection() -> Result<Agent> {
    let agents = detect_available_agents();

    let items: Vec<String> = agents
        .iter()
        .map(|info| {
            if info.available {
                format!("{} ✓", info.name)
            } else {
                format!("{} (non détecté)", info.name)
            }
        })
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Quel agent AI voulez-vous configurer ?")
        .items(&items)
        .default(0)
        .interact()?;

    Ok(agents[selection].agent)
}

/// Affiche le résumé d'installation condensé
fn print_summary(stats: &InstallStats, agents: &[Agent]) {
    println!("\n   📦 Modules chargés:");
    println!(
        "      ✓ Core          {} prompts, {} schemas, {} outputs",
        stats.prompts, stats.schemas, stats.outputs
    );

    if stats.rgpd > 0 {
        println!("      ✓ RGPD          {} fichiers", stats.rgpd);
    }
    if stats.rgs > 0 {
        println!("      ✓ RGS           {} fichiers", stats.rgs);
    }
    if stats.nis2 > 0 {
        println!("      ✓ NIS2          {} fichiers", stats.nis2);
    }

    println!("\n   🤖 Agent(s) configuré(s):");
    for agent in agents {
        match agent {
            Agent::ClaudeCode => {
                println!(
                    "      ✓ {} → .claude/commands/ ({} slash commands)",
                    agent.display_name(),
                    stats.prompts
                )
            }
            Agent::Copilot => {
                println!(
                    "      ✓ {} → .github/copilot-instructions.md",
                    agent.display_name()
                )
            }
            Agent::Cursor => {
                println!(
                    "      ✓ {} → .cursor/rules/ ({} règles)",
                    agent.display_name(),
                    stats.prompts
                )
            }
            Agent::Gemini => {
                println!("      ✓ {} → .gemini/instructions.md", agent.display_name())
            }
        }
    }

    if !stats.errors.is_empty() {
        println!(
            "\n   ⚠️  {} erreur(s) lors du téléchargement",
            stats.errors.len()
        );
    }

    println!("\n✅ OpenSecKit prêt !");

    // Instructions selon l'agent principal
    if let Some(agent) = agents.first() {
        match agent {
            Agent::ClaudeCode => {
                println!("\n💡 Prochaines étapes:");
                println!("   1. Lancez Claude Code: claude");
                println!("   2. Exécutez /osk-configure pour analyser votre code");
            }
            Agent::Copilot => {
                println!("\n💡 Prochaines étapes:");
                println!("   1. Ouvrez VS Code avec GitHub Copilot");
                println!("   2. Demandez à Copilot d'analyser la sécurité de votre projet");
            }
            Agent::Cursor => {
                println!("\n💡 Prochaines étapes:");
                println!("   1. Ouvrez Cursor dans ce projet");
                println!("   2. Les règles OSK seront automatiquement chargées");
            }
            Agent::Gemini => {
                println!("\n💡 Prochaines étapes:");
                println!("   1. Utilisez Gemini avec le contexte .gemini/instructions.md");
            }
        }
    }
}

/// Installe la configuration pour un agent spécifique
fn install_agent_config(agent: &Agent, stats: &InstallStats, force: bool) -> Result<()> {
    match agent {
        Agent::ClaudeCode => install_claude_code(force),
        Agent::Copilot => install_copilot(stats),
        Agent::Cursor => install_cursor(stats),
        Agent::Gemini => install_gemini(stats),
    }
}

/// Installation Claude Code (slash commands)
fn install_claude_code(force: bool) -> Result<()> {
    let prompts_dir = PathBuf::from(".osk/prompts");
    let commands_dir = PathBuf::from(".claude/commands");

    if !prompts_dir.exists() {
        return Ok(());
    }

    fs::create_dir_all(&commands_dir)?;

    for entry in fs::read_dir(prompts_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            if let Some(filename) = path.file_name() {
                let dest = commands_dir.join(filename);
                if !dest.exists() || force {
                    fs::copy(&path, &dest)?;
                }
            }
        }
    }

    Ok(())
}

/// Installation GitHub Copilot
fn install_copilot(stats: &InstallStats) -> Result<()> {
    fs::create_dir_all(".github")?;

    let content = generate_copilot_instructions(stats);
    fs::write(".github/copilot-instructions.md", content)?;

    Ok(())
}

/// Installation Cursor
fn install_cursor(stats: &InstallStats) -> Result<()> {
    fs::create_dir_all(".cursor/rules")?;

    // Règle principale OSK
    let content = generate_cursor_rules(stats);
    fs::write(".cursor/rules/openseckit.md", content)?;

    Ok(())
}

/// Installation Gemini
fn install_gemini(stats: &InstallStats) -> Result<()> {
    fs::create_dir_all(".gemini")?;

    let content = generate_gemini_instructions(stats);
    fs::write(".gemini/instructions.md", content)?;

    Ok(())
}

/// Génère les instructions pour GitHub Copilot
fn generate_copilot_instructions(stats: &InstallStats) -> String {
    format!(
        r#"# OpenSecKit Security Instructions

This project uses OpenSecKit for security-as-code workflows.

## Project Structure

- `.osk/config.toml` - Project configuration and detected stack
- `.osk/memory/` - Persistent context between sessions
- `.osk/specs/` - Security specifications per feature
- `docs/security/risks/risk-register.yaml` - Central risk registry

## Security Workflow

When asked about security, follow this workflow:

### 1. Configuration (`/osk-configure` equivalent)
- Analyze the codebase to detect the technology stack
- Identify applicable compliance domains (RGPD, RGS, NIS2)
- Update `.osk/config.toml` with findings

### 2. Threat Analysis (`/osk-analyze` equivalent)
- Apply STRIDE methodology for threat modeling
- Score risks using Impact × Probability × Exposure (1-125 scale)
- Document in `.osk/specs/NNN-feature/threats.md` and `risks.md`

### 3. Security Requirements (`/osk-specify` equivalent)
- Define security requirements based on identified risks
- Create test strategies for validation
- Document in `.osk/specs/NNN-feature/requirements.md`

### 4. Hardening (`/osk-harden` equivalent)
- Propose hardening measures for Principles V, VI, VII
- Secrets management, audit logging, patch management
- Document in `.osk/specs/NNN-feature/hardening.md`

### 5. Implementation (`/osk-implement` equivalent)
- Execute tasks from `.osk/specs/NNN-feature/tasks.yaml`
- Update risk-register.yaml with resolutions
- Mark risks as RESOLU when mitigated

## Available Resources

- {} prompts defining security workflows
- {} schemas for structured output
- {} output templates

## Compliance Domains

{}{}{}

## Important Rules

1. Never skip threat modeling before implementation
2. Always update the risk register after implementing mitigations
3. Follow the 7 security principles defined in constitution.md
4. Use YAML format for structured data (risks, tasks, requirements)
"#,
        stats.prompts,
        stats.schemas,
        stats.outputs,
        if stats.rgpd > 0 {
            "- RGPD: Data protection and privacy\n"
        } else {
            ""
        },
        if stats.rgs > 0 {
            "- RGS: French government security (EBIOS RM)\n"
        } else {
            ""
        },
        if stats.nis2 > 0 {
            "- NIS2: EU cybersecurity directive\n"
        } else {
            ""
        },
    )
}

/// Génère les règles pour Cursor
fn generate_cursor_rules(stats: &InstallStats) -> String {
    format!(
        r#"# OpenSecKit Security Rules for Cursor

## Context

This project uses OpenSecKit v3.1.0 for security-as-code.
- Configuration: `.osk/config.toml`
- Specifications: `.osk/specs/`
- Risk Registry: `docs/security/risks/risk-register.yaml`

## Security Analysis Rules

When analyzing code for security:

1. **Threat Modeling**: Apply STRIDE (Spoofing, Tampering, Repudiation, Information Disclosure, Denial of Service, Elevation of Privilege)

2. **Risk Scoring**: Use the formula `Impact × Probability × Exposure` (scale 1-125)
   - CRITIQUE: score ≥ 64
   - IMPORTANT: score 27-63
   - MINEUR: score < 27

3. **Output Format**: Always use the schemas in `.osk/templates/schemas/`

## Available Commands (conceptual)

- `osk-configure`: Analyze codebase and detect stack
- `osk-analyze <feature>`: Threat model a feature
- `osk-specify <feature>`: Define security requirements
- `osk-harden <feature>`: Propose hardening measures
- `osk-implement <feature>`: Execute security tasks

## Resources Loaded

- {} security prompts
- {} data schemas
- {} output templates

## Compliance

{}{}{}
"#,
        stats.prompts,
        stats.schemas,
        stats.outputs,
        if stats.rgpd > 0 { "- RGPD enabled\n" } else { "" },
        if stats.rgs > 0 { "- RGS enabled\n" } else { "" },
        if stats.nis2 > 0 { "- NIS2 enabled\n" } else { "" },
    )
}

/// Génère les instructions pour Gemini
fn generate_gemini_instructions(stats: &InstallStats) -> String {
    format!(
        r#"# OpenSecKit Security Context for Gemini

## Overview

This project uses OpenSecKit for structured security analysis.

## Key Files

- `.osk/config.toml`: Project configuration
- `.osk/specs/`: Feature-specific security specs
- `docs/security/risks/risk-register.yaml`: Risk tracking

## Methodology

1. STRIDE threat modeling
2. Risk scoring (Impact × Probability × Exposure, 1-125)
3. Security requirements definition
4. Hardening recommendations
5. Implementation with risk resolution

## Resources

{} prompts | {} schemas | {} templates

## Active Domains

{}{}{}
"#,
        stats.prompts,
        stats.schemas,
        stats.outputs,
        if stats.rgpd > 0 { "RGPD " } else { "" },
        if stats.rgs > 0 { "RGS " } else { "" },
        if stats.nis2 > 0 { "NIS2" } else { "" },
    )
}

fn prompt_configuration() -> Result<OskConfig> {
    let project_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Nom du projet")
        .default("MonProjet".to_string())
        .interact_text()?;

    println!("   🔍 Scan de la stack technique...");
    let detected_stack = stack::detect();

    let final_stack: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Stack technique (Validez ou modifiez)")
        .with_initial_text(detected_stack.clone())
        .allow_empty(true)
        .interact_text()?;

    let detected_vec: Vec<String> = if detected_stack.trim().is_empty() {
        vec![]
    } else {
        detected_stack
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    };

    let custom_vec: Vec<String> = if final_stack.trim() == detected_stack.trim() {
        vec![]
    } else {
        final_stack
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty() && !detected_vec.contains(&s.to_string()))
            .collect()
    };

    Ok(build_config(project_name, detected_vec, custom_vec))
}

fn default_configuration() -> OskConfig {
    println!("   🤖 Mode non-interactif: valeurs par défaut");
    println!("   🔍 Scan de la stack technique...");

    let detected_stack = stack::detect();
    let detected_vec: Vec<String> = if detected_stack.trim().is_empty() {
        vec![]
    } else {
        detected_stack
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    };

    build_config("MonProjet".to_string(), detected_vec, vec![])
}

fn build_config(name: String, detected: Vec<String>, custom: Vec<String>) -> OskConfig {
    OskConfig {
        project: Some(ProjectConfig {
            name,
            version: None,
            description: None,
        }),
        stack: Some(StackConfig { detected, custom }),
        domains: Some(DomainsConfig {
            active: vec![],
            rgpd: Some(RgpdConfig {
                enabled: false,
                niveau: "standard".to_string(),
                dpia_required: false,
            }),
            nis2: Some(Nis2Config {
                enabled: false,
                type_entite: String::new(),
                secteur: String::new(),
            }),
            rgs: Some(RgsConfig {
                enabled: false,
                niveau: String::new(),
                perimetre: String::new(),
            }),
        }),
        principles: Some(PrinciplesConfig {
            threat_modeling: "high".to_string(),
            risk_analysis: "high".to_string(),
            security_requirements: "high".to_string(),
            security_testing: "medium".to_string(),
            secrets_management: "high".to_string(),
            audit_logging: "medium".to_string(),
            patch_management: "medium".to_string(),
        }),
        specs: Some(SpecsConfig {
            counter: 1,
            features: vec![],
        }),
        memory: Some(MemoryConfig {
            enabled: true,
            path: ".osk/memory".to_string(),
        }),
    }
}

fn scaffold_project(config: &OskConfig, force: bool) -> Result<()> {
    fs::create_dir_all(".osk/prompts")?;
    fs::create_dir_all(".osk/templates")?;
    fs::create_dir_all(".osk/memory")?;
    fs::create_dir_all(".osk/specs")?;

    let config_path = Path::new(".osk/config.toml");
    if !config_path.exists() || force {
        let toml_string = toml::to_string_pretty(config)?;
        fs::write(config_path, toml_string)?;
    }

    if Path::new(".gitignore").exists() {
        let content = fs::read_to_string(".gitignore")?;
        if !content.contains(".osk/memory/") {
            println!("   ℹ️  Pensez à ajouter .osk/memory/ à votre .gitignore");
        }
    } else {
        let gitignore_content = ".osk/memory/\n.osk/config.toml\n";
        fs::write(".gitignore", gitignore_content)?;
    }

    Ok(())
}

fn install_resources(client: &Client, force: bool) -> Result<InstallStats> {
    use crate::github;

    let mut stats = InstallStats::default();

    let tag = match github::fetch_latest_tag(client) {
        Ok(t) => t,
        Err(e) => {
            stats.errors.push(format!("Fetch tag: {}", e));
            return Ok(stats);
        }
    };

    // Registry
    let registry_dest = PathBuf::from(".osk/registry.toml");
    if !registry_dest.exists() || force {
        if let Err(e) = github::download_file(client, &tag, "registry.toml", &registry_dest) {
            stats.errors.push(format!("registry.toml: {}", e));
        }
    }

    // Fetch tree and download resources
    let tree_items = match github::fetch_repo_tree(client, &tag) {
        Ok(items) => items,
        Err(e) => {
            stats.errors.push(format!("Fetch tree: {}", e));
            return Ok(stats);
        }
    };

    for item in tree_items {
        if item.item_type != "blob" {
            continue;
        }

        let dest = PathBuf::from(".osk").join(&item.path);
        if dest.exists() && !force {
            // Count existing files
            count_resource(&item.path, &mut stats);
            continue;
        }

        // Download and count
        if item.path.starts_with("prompts/")
            || item.path.starts_with("templates/")
            || item.path.starts_with("domaines/")
            || item.path == "constitution.md"
        {
            if let Err(e) = github::download_file(client, &tag, &item.path, &dest) {
                stats.errors.push(format!("{}: {}", item.path, e));
            } else {
                count_resource(&item.path, &mut stats);
            }
        }
    }

    Ok(stats)
}

/// Compte les ressources par catégorie
fn count_resource(path: &str, stats: &mut InstallStats) {
    if path.starts_with("prompts/") && path.ends_with(".md") {
        stats.prompts += 1;
    } else if path.starts_with("templates/schemas/") {
        stats.schemas += 1;
    } else if path.starts_with("templates/outputs/") {
        stats.outputs += 1;
    } else if path.starts_with("templates/reports/") {
        stats.reports += 1;
    } else if path.starts_with("domaines/rgpd/") {
        stats.rgpd += 1;
    } else if path.starts_with("domaines/gouvernement-rgs/") {
        stats.rgs += 1;
    } else if path.starts_with("domaines/nis2/") {
        stats.nis2 += 1;
    }
}
