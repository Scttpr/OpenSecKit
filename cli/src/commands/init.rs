//! Init command - project initialization

use crate::agents::{self, AgentConfig, AgentsConfig, DomainsInfo};
use crate::args::Agent;
use crate::config::{
    DomainsConfig, MemoryConfig, Nis2Config, OskConfig, PrinciplesConfig, ProjectConfig,
    RgpdConfig, RgsConfig, SpecsConfig, StackConfig,
};
use crate::prompts;
use crate::stack;
use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Input, Select};
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

const VERSION: &str = "3.2.0";

#[derive(Default)]
struct InstallStats {
    prompts: usize,
    schemas: usize,
    outputs: usize,
    reports: usize,
    rgpd: usize,
    rgs: usize,
    nis2: usize,
    agents_templates: usize,
    errors: Vec<String>,
}

struct AgentInstallResult {
    agent_id: String,
    agent_name: String,
    files_count: usize,
    output_path: String,
}

pub fn run(
    client: &Client,
    force: bool,
    default: bool,
    agent: Option<Agent>,
    all_agents: bool,
) -> Result<()> {
    println!("🚀 Initialisation de OpenSecKit v{}\n", VERSION);

    let config = load_or_create_config(force, default)?;
    scaffold_project(&config, force)?;
    let stats = install_resources(client, force)?;

    let osk_dir = PathBuf::from(".osk");
    let agents_config = load_agents_config_or_fallback(&osk_dir)?;
    let selected_agents = select_agents(&agents_config, default, agent, all_agents)?;
    let prompts_dir = osk_dir.join("prompts");
    let prompts_info = prompts::parse_prompts_dir(&prompts_dir)?;
    let domains = detect_domains(&stats);
    let templates_dir = osk_dir.join("templates");
    let mut agent_results = Vec::new();

    for agent_id in &selected_agents {
        if let Some(agent_config) = agents_config.agents.get(agent_id) {
            match install_agent(
                agent_id,
                agent_config,
                &prompts_info,
                &templates_dir,
                &domains,
            ) {
                Ok(result) => agent_results.push(result),
                Err(e) => eprintln!("   ⚠️  Erreur {}: {}", agent_id, e),
            }
        }
    }

    if let Some(ref universal) = agents_config.universal {
        if universal.enabled {
            if let Err(e) = agents::generate_agents_md(
                universal,
                &prompts_info,
                &templates_dir,
                VERSION,
                &domains,
            ) {
                eprintln!("   ⚠️  Erreur AGENTS.md: {}", e);
            }
        }
    }

    print_summary(&stats, &agent_results, agents_config.universal.as_ref());

    Ok(())
}

fn load_or_create_config(force: bool, default: bool) -> Result<OskConfig> {
    let config_path = Path::new(".osk/config.toml");
    let config_exists = config_path.exists();

    if config_exists && !force {
        println!("   ℹ️  Configuration existante détectée.");
        fs::read_to_string(config_path)
            .and_then(|s| {
                toml::from_str(&s)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
            })
            .or_else(|_| {
                println!("   ⚠️  Config corrompue, création d'une nouvelle...");
                if default {
                    Ok(default_configuration())
                } else {
                    prompt_configuration()
                }
            })
    } else if default {
        Ok(default_configuration())
    } else {
        prompt_configuration()
    }
}

fn load_agents_config_or_fallback(osk_dir: &Path) -> Result<AgentsConfig> {
    agents::load_agents_config(osk_dir).or_else(|_| Ok(default_agents_config()))
}

fn default_agents_config() -> AgentsConfig {
    let mut agents = HashMap::new();

    agents.insert(
        "claude-code".to_string(),
        AgentConfig {
            name: "Claude Code".to_string(),
            description: "Slash commands natifs".to_string(),
            format: "slash-command".to_string(),
            output_dir: Some(".claude/commands".to_string()),
            output_file: None,
            file_pattern: Some("{command}.md".to_string()),
            detect_cmd: Some("claude".to_string()),
            detect_path: None,
            template: "claude-code.tera".to_string(),
            enabled: true,
        },
    );

    agents.insert(
        "copilot".to_string(),
        AgentConfig {
            name: "GitHub Copilot".to_string(),
            description: "Instructions Copilot".to_string(),
            format: "single-file".to_string(),
            output_dir: None,
            output_file: Some(".github/copilot-instructions.md".to_string()),
            file_pattern: None,
            detect_cmd: Some("gh".to_string()),
            detect_path: Some(".github".to_string()),
            template: "copilot.tera".to_string(),
            enabled: true,
        },
    );

    agents.insert(
        "cursor".to_string(),
        AgentConfig {
            name: "Cursor".to_string(),
            description: "Règles Cursor".to_string(),
            format: "rules-dir".to_string(),
            output_dir: Some(".cursor/rules".to_string()),
            output_file: None,
            file_pattern: Some("osk-{command}.md".to_string()),
            detect_cmd: Some("cursor".to_string()),
            detect_path: Some(".cursor".to_string()),
            template: "cursor.tera".to_string(),
            enabled: true,
        },
    );

    agents.insert(
        "gemini".to_string(),
        AgentConfig {
            name: "Gemini".to_string(),
            description: "Instructions Gemini".to_string(),
            format: "single-file".to_string(),
            output_dir: None,
            output_file: Some(".gemini/instructions.md".to_string()),
            file_pattern: None,
            detect_cmd: Some("gemini".to_string()),
            detect_path: None,
            template: "gemini.tera".to_string(),
            enabled: true,
        },
    );

    AgentsConfig {
        meta: agents::MetaConfig {
            version: VERSION.to_string(),
            default_agent: "claude-code".to_string(),
        },
        agents,
        universal: Some(agents::UniversalConfig {
            enabled: true,
            output_file: "AGENTS.md".to_string(),
            template: "AGENTS.md.tera".to_string(),
        }),
    }
}

fn select_agents(
    config: &AgentsConfig,
    default: bool,
    agent: Option<Agent>,
    all_agents: bool,
) -> Result<Vec<String>> {
    if all_agents {
        Ok(config
            .agents
            .iter()
            .filter(|(_, a)| a.enabled)
            .map(|(id, _)| id.clone())
            .collect())
    } else if let Some(a) = agent {
        Ok(vec![agent_to_id(&a)])
    } else if default {
        Ok(vec![config.meta.default_agent.clone()])
    } else {
        let selected_id = prompt_agent_selection(config)?;
        Ok(vec![selected_id])
    }
}

fn agent_to_id(agent: &Agent) -> String {
    match agent {
        Agent::ClaudeCode => "claude-code".to_string(),
        Agent::Copilot => "copilot".to_string(),
        Agent::Cursor => "cursor".to_string(),
        Agent::Gemini => "gemini".to_string(),
    }
}

fn prompt_agent_selection(config: &AgentsConfig) -> Result<String> {
    let mut items: Vec<(String, String)> = config
        .agents
        .iter()
        .filter(|(_, a)| a.enabled)
        .map(|(id, a)| {
            let available = agents::is_agent_available(a);
            let label = if available {
                format!("{} ✓", a.name)
            } else {
                format!("{} (non détecté)", a.name)
            };
            (id.clone(), label)
        })
        .collect();

    items.sort_by(|a, b| a.1.cmp(&b.1));

    let labels: Vec<&str> = items.iter().map(|(_, l)| l.as_str()).collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Quel agent AI voulez-vous configurer ?")
        .items(&labels)
        .default(0)
        .interact()?;

    Ok(items[selection].0.clone())
}

fn install_agent(
    agent_id: &str,
    agent_config: &AgentConfig,
    prompts_info: &[prompts::PromptInfo],
    templates_dir: &Path,
    domains: &DomainsInfo,
) -> Result<AgentInstallResult> {
    let files_count = agents::generate_agent_files(
        agent_id,
        agent_config,
        prompts_info,
        templates_dir,
        VERSION,
        domains,
    )?;

    let output_path = agent_config
        .output_dir
        .clone()
        .or_else(|| agent_config.output_file.clone())
        .unwrap_or_default();

    Ok(AgentInstallResult {
        agent_id: agent_id.to_string(),
        agent_name: agent_config.name.clone(),
        files_count,
        output_path,
    })
}

fn detect_domains(stats: &InstallStats) -> DomainsInfo {
    DomainsInfo {
        rgpd: stats.rgpd > 0,
        rgs: stats.rgs > 0,
        nis2: stats.nis2 > 0,
    }
}

fn print_summary(
    stats: &InstallStats,
    agent_results: &[AgentInstallResult],
    universal: Option<&agents::UniversalConfig>,
) {
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
    if stats.agents_templates > 0 {
        println!("      ✓ Templates     {} agents", stats.agents_templates);
    }

    println!("\n   🤖 Agent(s) configuré(s):");
    for result in agent_results {
        println!(
            "      ✓ {} → {} ({} fichiers)",
            result.agent_name, result.output_path, result.files_count
        );
    }

    if let Some(u) = universal {
        if u.enabled {
            println!("      ✓ AGENTS.md (universel)");
        }
    }

    if !stats.errors.is_empty() {
        println!(
            "\n   ⚠️  {} erreur(s) lors du téléchargement",
            stats.errors.len()
        );
    }

    println!("\n✅ OpenSecKit prêt !");

    if let Some(first) = agent_results.first() {
        match first.agent_id.as_str() {
            "claude-code" => {
                println!("\n💡 Prochaines étapes:");
                println!("   1. Lancez Claude Code: claude");
                println!("   2. Exécutez /osk-configure pour analyser votre code");
            }
            "copilot" => {
                println!("\n💡 Prochaines étapes:");
                println!("   1. Ouvrez VS Code avec GitHub Copilot");
                println!("   2. Demandez à Copilot d'analyser la sécurité du projet");
            }
            "cursor" => {
                println!("\n💡 Prochaines étapes:");
                println!("   1. Ouvrez Cursor dans ce projet");
                println!("   2. Les règles OSK sont automatiquement chargées");
            }
            "gemini" => {
                println!("\n💡 Prochaines étapes:");
                println!("   1. Utilisez Gemini avec le contexte du projet");
            }
            _ => {}
        }
    }
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

    let detected_vec = parse_stack(&detected_stack);
    let custom_vec = parse_custom_stack(&final_stack, &detected_vec);

    Ok(build_config(project_name, detected_vec, custom_vec))
}

fn default_configuration() -> OskConfig {
    println!("   🤖 Mode non-interactif: valeurs par défaut");
    println!("   🔍 Scan de la stack technique...");

    let detected_stack = stack::detect();
    let detected_vec = parse_stack(&detected_stack);

    build_config("MonProjet".to_string(), detected_vec, vec![])
}

fn parse_stack(stack: &str) -> Vec<String> {
    if stack.trim().is_empty() {
        vec![]
    } else {
        stack
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    }
}

fn parse_custom_stack(final_stack: &str, detected: &[String]) -> Vec<String> {
    final_stack
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && !detected.contains(s))
        .collect()
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

    let registry_dest = PathBuf::from(".osk/registry.toml");
    if !registry_dest.exists() || force {
        if let Err(e) = github::download_file(client, &tag, "registry.toml", &registry_dest) {
            stats.errors.push(format!("registry.toml: {}", e));
        }
    }

    let agents_dest = PathBuf::from(".osk/agents.toml");
    if !agents_dest.exists() || force {
        if let Err(e) = github::download_file(client, &tag, "agents.toml", &agents_dest) {
            stats.errors.push(format!("agents.toml: {}", e));
        }
    }

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
            count_resource(&item.path, &mut stats);
            continue;
        }

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

fn count_resource(path: &str, stats: &mut InstallStats) {
    if path.starts_with("prompts/") && path.ends_with(".md") {
        stats.prompts += 1;
    } else if path.starts_with("templates/schemas/") {
        stats.schemas += 1;
    } else if path.starts_with("templates/outputs/") {
        stats.outputs += 1;
    } else if path.starts_with("templates/reports/") {
        stats.reports += 1;
    } else if path.starts_with("templates/agents/") {
        stats.agents_templates += 1;
    } else if path.starts_with("domaines/rgpd/") {
        stats.rgpd += 1;
    } else if path.starts_with("domaines/gouvernement-rgs/") {
        stats.rgs += 1;
    } else if path.starts_with("domaines/nis2/") {
        stats.nis2 += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_stack_empty() {
        assert!(parse_stack("").is_empty());
        assert!(parse_stack("   ").is_empty());
    }

    #[test]
    fn test_parse_stack_single() {
        let result = parse_stack("rust");
        assert_eq!(result, vec!["rust"]);
    }

    #[test]
    fn test_parse_stack_multiple() {
        let result = parse_stack("rust, python, typescript");
        assert_eq!(result, vec!["rust", "python", "typescript"]);
    }

    #[test]
    fn test_parse_custom_stack() {
        let detected = vec!["rust".to_string(), "python".to_string()];
        let result = parse_custom_stack("rust, go, python, java", &detected);
        assert_eq!(result, vec!["go", "java"]);
    }

    #[test]
    fn test_agent_to_id() {
        assert_eq!(agent_to_id(&Agent::ClaudeCode), "claude-code");
        assert_eq!(agent_to_id(&Agent::Copilot), "copilot");
        assert_eq!(agent_to_id(&Agent::Cursor), "cursor");
        assert_eq!(agent_to_id(&Agent::Gemini), "gemini");
    }

    #[test]
    fn test_count_resource_prompts() {
        let mut stats = InstallStats::default();
        count_resource("prompts/osk-analyze.md", &mut stats);
        count_resource("prompts/osk-configure.md", &mut stats);
        assert_eq!(stats.prompts, 2);
    }

    #[test]
    fn test_count_resource_templates() {
        let mut stats = InstallStats::default();
        count_resource("templates/schemas/context.yaml", &mut stats);
        count_resource("templates/outputs/report.md", &mut stats);
        count_resource("templates/agents/cursor.tera", &mut stats);
        assert_eq!(stats.schemas, 1);
        assert_eq!(stats.outputs, 1);
        assert_eq!(stats.agents_templates, 1);
    }

    #[test]
    fn test_count_resource_domains() {
        let mut stats = InstallStats::default();
        count_resource("domaines/rgpd/checklist.md", &mut stats);
        count_resource("domaines/gouvernement-rgs/guide.md", &mut stats);
        count_resource("domaines/nis2/requirements.md", &mut stats);
        assert_eq!(stats.rgpd, 1);
        assert_eq!(stats.rgs, 1);
        assert_eq!(stats.nis2, 1);
    }

    #[test]
    fn test_detect_domains() {
        let stats = InstallStats {
            rgpd: 3,
            rgs: 0,
            nis2: 1,
            ..Default::default()
        };
        let domains = detect_domains(&stats);
        assert!(domains.rgpd);
        assert!(!domains.rgs);
        assert!(domains.nis2);
    }

    #[test]
    fn test_default_agents_config() {
        let config = default_agents_config();
        assert_eq!(config.meta.default_agent, "claude-code");
        assert!(config.agents.contains_key("claude-code"));
        assert!(config.agents.contains_key("copilot"));
        assert!(config.agents.contains_key("cursor"));
        assert!(config.agents.contains_key("gemini"));
        assert!(config.universal.is_some());
    }
}
