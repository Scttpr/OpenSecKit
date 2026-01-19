//! Init command - project initialization
//!
//! V4 Architecture: Minimal downloads, registry-based metadata
//! - Downloads only: config/registry.toml, config/agents.toml, templates/agents/
//! - Prompts are fetched on-demand by agents from GitHub URLs
//! - All generated output stays in .osk/

use crate::agents::{self, AgentConfig, AgentsConfig};
use crate::args::Agent;
use crate::config::{
    DomainsConfig, MemoryConfig, Nis2Config, OskConfig, PrinciplesConfig, ProjectConfig,
    RgpdConfig, RgsConfig, SpecsConfig, StackConfig,
};
use crate::registry::{self, CommandInfo, DomainsInfo};
use crate::stack;
use anyhow::{bail, Result};
use dialoguer::{theme::ColorfulTheme, Input, Select};
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

const VERSION: &str = "4.0.0";

/// Source for resources (registry, agents config, templates)
#[derive(Debug, Clone)]
pub enum ResourceSource {
    /// Use local files from the repository
    Local(PathBuf),
    /// Fetch from a specific version tag
    Version(String),
    /// Fetch from the latest release
    Latest,
}

#[derive(Default)]
struct InstallStats {
    registry: bool,
    agents_config: bool,
    agents_templates: usize,
    commands: usize,
    errors: Vec<String>,
}

struct AgentInstallResult {
    agent_id: String,
    agent_name: String,
    files_count: usize,
    output_path: String,
}

#[allow(clippy::too_many_arguments)]
pub fn run(
    client: &Client,
    force: bool,
    default: bool,
    agent: Option<Agent>,
    all_agents: bool,
    local: bool,
    local_path: Option<String>,
    version: Option<String>,
) -> Result<()> {
    // Determine resource source
    let source = if let Some(path) = local_path {
        // Explicit path provided
        let repo_root = PathBuf::from(&path);
        validate_repo_root(&repo_root)?;
        ResourceSource::Local(repo_root.canonicalize().unwrap_or(repo_root))
    } else if local {
        // Auto-detect repo root
        let repo_root = find_repo_root()?;
        ResourceSource::Local(repo_root)
    } else if let Some(v) = version {
        ResourceSource::Version(v)
    } else {
        ResourceSource::Latest
    };

    let source_label = match &source {
        ResourceSource::Local(path) => format!("local ({})", path.display()),
        ResourceSource::Version(v) => format!("version {}", v),
        ResourceSource::Latest => "latest".to_string(),
    };

    println!("🚀 Initialisation de OpenSecKit v{} [{}]\n", VERSION, source_label);

    let config = load_or_create_config(force, default)?;
    scaffold_project(&config, force)?;
    let mut stats = install_resources(client, force, &source)?;

    let osk_dir = PathBuf::from(".osk");
    let agents_config = load_agents_config_or_fallback(&osk_dir)?;
    let selected_agents = select_agents(&agents_config, default, agent, all_agents)?;

    // Load registry and get commands + domains
    let registry_path = osk_dir.join("registry.toml");
    let (commands, domains) = if registry_path.exists() {
        let reg = registry::load_registry(&registry_path)?;
        let cmds = registry::get_commands(&reg);
        let doms = registry::detect_domains(&reg);
        stats.commands = cmds.len();
        (cmds, doms)
    } else {
        eprintln!("   ⚠️  registry.toml non trouvé, utilisation des valeurs par défaut");
        (Vec::new(), DomainsInfo::default())
    };

    // In local mode, use templates directly from repo; in remote mode, use downloaded ones
    let (templates_dir, local_repo) = match &source {
        ResourceSource::Local(path) => (path.join("templates"), Some(path.as_path())),
        _ => (osk_dir.join("templates"), None),
    };
    let mut agent_results = Vec::new();

    for agent_id in &selected_agents {
        if let Some(agent_config) = agents_config.agents.get(agent_id) {
            match install_agent(
                client,
                agent_id,
                agent_config,
                &commands,
                &templates_dir,
                &domains,
                local_repo,
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
                &commands,
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
    client: &Client,
    agent_id: &str,
    agent_config: &AgentConfig,
    commands: &[CommandInfo],
    templates_dir: &Path,
    domains: &DomainsInfo,
    local_repo: Option<&Path>,
) -> Result<AgentInstallResult> {
    let files_count = agents::generate_agent_files(
        client,
        agent_id,
        agent_config,
        commands,
        templates_dir,
        VERSION,
        domains,
        local_repo,
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

fn print_summary(
    stats: &InstallStats,
    agent_results: &[AgentInstallResult],
    universal: Option<&agents::UniversalConfig>,
) {
    println!("\n   📦 Resources:");
    if stats.registry {
        println!("      ✓ Registry      {} commands", stats.commands);
    }
    if stats.agents_config {
        println!("      ✓ Agents config loaded");
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
            "\n   ⚠️  {} erreur(s) lors de l'installation",
            stats.errors.len()
        );
        for err in &stats.errors {
            println!("      - {}", err);
        }
    }

    println!("\n✅ OpenSecKit prêt !");
    println!("   📁 System model: .osk/system-model/");
    println!("   📄 Prompts: fetched on-demand from GitHub");

    if let Some(first) = agent_results.first() {
        match first.agent_id.as_str() {
            "claude-code" => {
                println!("\n💡 Prochaines étapes:");
                println!("   1. Lancez Claude Code: claude");
                println!("   2. Exécutez /osk-discover-init pour analyser votre code");
            }
            "copilot" => {
                println!("\n💡 Prochaines étapes:");
                println!("   1. Ouvrez VS Code avec GitHub Copilot");
                println!("   2. Exécutez /osk-discover-init pour analyser votre code");
            }
            "cursor" => {
                println!("\n💡 Prochaines étapes:");
                println!("   1. Ouvrez Cursor dans ce projet");
                println!("   2. Exécutez /osk-discover-init pour analyser votre code");
            }
            "gemini" => {
                println!("\n💡 Prochaines étapes:");
                println!("   1. Utilisez Gemini avec le contexte du projet");
                println!("   2. Exécutez /osk-discover-init pour analyser votre code");
            }
            _ => {}
        }
    } else {
        println!("\n💡 Prochaine étape: Exécutez /osk-discover-init pour analyser votre code");
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
    fs::create_dir_all(".osk/system-model")?;

    let config_path = Path::new(".osk/config.toml");
    if !config_path.exists() || force {
        let toml_string = toml::to_string_pretty(config)?;
        fs::write(config_path, toml_string)?;
    }

    // Scaffold all system-model files
    scaffold_system_model(force)?;

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

/// Scaffold all system-model YAML files with placeholder content
fn scaffold_system_model(force: bool) -> Result<()> {
    let system_model_dir = Path::new(".osk/system-model");

    // Define all skeleton files
    let skeletons: Vec<(&str, &str)> = vec![
        ("index.yaml", r#"# OpenSecKit System Model Index
# Run /osk-discover init to populate this file
metadata:
  schema_version: "4.0.0"
  created_at: null
  last_scan: null
  confidence: 0.0

stats:
  components: 0
  data_categories: 0
  actors: 0
  trust_zones: 0
  integrations: 0
  tools: 0
  discovery_gaps: 0

summaries: {}

sections:
  business: "business.yaml"
  architecture: "architecture.yaml"
  data: "data.yaml"
  actors: "actors.yaml"
  trust: "trust.yaml"
  integrations: "integrations.yaml"
  security: "security.yaml"
  tooling: "tooling.yaml"
  team: "team.yaml"
  gaps: "gaps.yaml"

component_data_map: {}
sensitive_flows: []
pii_integrations: []
compliance_hints: {}
"#),
        ("business.yaml", r#"# OpenSecKit System Model - Business Context
# Run /osk-discover init to populate this file
metadata:
  section: "business"
  parent: "index.yaml"

business_context:
  domain: null
  description: null
  stakeholders: []

business_processes: []

business_rules: []
"#),
        ("architecture.yaml", r#"# OpenSecKit System Model - Architecture
# Run /osk-discover init to populate this file
metadata:
  section: "architecture"
  parent: "index.yaml"

architecture:
  style: null
  description: null

components: []

data_flows: []

infrastructure:
  hosting: null
  regions: []
  environments: []
"#),
        ("data.yaml", r#"# OpenSecKit System Model - Data Categories
# Run /osk-discover init to populate this file
metadata:
  section: "data"
  parent: "index.yaml"

data_categories: []

storage_locations: []

retention_policies: []
"#),
        ("actors.yaml", r#"# OpenSecKit System Model - Actors
# Run /osk-discover init to populate this file
metadata:
  section: "actors"
  parent: "index.yaml"

users: []

roles: []

privileged_accounts: []
"#),
        ("trust.yaml", r#"# OpenSecKit System Model - Trust Zones
# Run /osk-discover init to populate this file
metadata:
  section: "trust"
  parent: "index.yaml"

trust_zones: []

boundaries: []
"#),
        ("integrations.yaml", r#"# OpenSecKit System Model - Integrations
# Run /osk-discover init to populate this file
metadata:
  section: "integrations"
  parent: "index.yaml"

external_services: []

internal_dependencies: []
"#),
        ("security.yaml", r#"# OpenSecKit System Model - Security Controls
# Run /osk-discover init to populate this file
metadata:
  section: "security"
  parent: "index.yaml"

controls:
  authentication: []
  authorization: []
  encryption: []
  input_validation: []
  logging: []
  secrets: []

vulnerabilities: []
"#),
        ("tooling.yaml", r#"# OpenSecKit System Model - Tooling & DevOps
# Run /osk-discover init to populate this file
metadata:
  section: "tooling"
  parent: "index.yaml"

source_control:
  provider: null
  branch_protection: null
  code_review_required: null

ci_cd:
  platform: null
  pipelines: []

security_tools:
  sast: []
  dast: []
  sca: []
  secrets_scanning: []

observability:
  logging: null
  monitoring: null
  alerting: null

secrets_management:
  provider: null
  rotation_policy: null
"#),
        ("team.yaml", r#"# OpenSecKit System Model - Team & Processes
# Run /osk-discover init to populate this file
metadata:
  section: "team"
  parent: "index.yaml"

team:
  name: null
  size: null
  security_champion: null
  incident_responder: null

processes:
  onboarding_documented: false
  offboarding_documented: false
  code_review_required: false
  security_training_required: false

incident_response:
  plan_documented: false
  runbooks: []
  last_drill: null

security_practices:
  threat_modeling_conducted: false
  last_pen_test: null
  bug_bounty: false
"#),
        ("gaps.yaml", r#"# OpenSecKit System Model - Discovery Gaps
# Run /osk-discover init to populate this file
metadata:
  section: "gaps"
  parent: "index.yaml"

gaps: []

# Gap severities: critical, high, medium, low
# Gap categories: business, architecture, data, actors, trust, integrations, security, tooling, team
"#),
    ];

    for (filename, content) in skeletons {
        let file_path = system_model_dir.join(filename);
        if !file_path.exists() || force {
            fs::write(&file_path, content)?;
        }
    }

    Ok(())
}

fn install_resources(client: &Client, force: bool, source: &ResourceSource) -> Result<InstallStats> {
    match source {
        ResourceSource::Local(repo_root) => install_resources_local(repo_root, force),
        ResourceSource::Version(tag) => install_resources_remote(client, force, tag),
        ResourceSource::Latest => {
            use crate::github;
            let tag = github::fetch_latest_tag(client)?;
            install_resources_remote(client, force, &tag)
        }
    }
}

/// Validate that a path contains OpenSecKit resources
fn validate_repo_root(path: &Path) -> Result<()> {
    let registry = path.join("config/registry.toml");
    let templates_dir = path.join("templates/agents");

    if !registry.exists() || !templates_dir.exists() {
        bail!(
            "Invalid OpenSecKit repository path: {}\n\
            Missing config/registry.toml or templates/agents/ directory.",
            path.display()
        );
    }
    Ok(())
}

/// Find the OpenSecKit repository root
fn find_repo_root() -> Result<PathBuf> {
    let current_exe = std::env::current_exe().ok();

    let candidates: Vec<PathBuf> = vec![
        PathBuf::from("."),
        PathBuf::from(".."),
        PathBuf::from("../.."),
        current_exe
            .as_ref()
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .and_then(|p| p.parent())
            .map(|p| p.to_path_buf())
            .unwrap_or_default(),
    ];

    for candidate in candidates {
        let registry = candidate.join("config/registry.toml");
        let templates_dir = candidate.join("templates/agents");

        if registry.exists() && templates_dir.exists() {
            return Ok(candidate.canonicalize().unwrap_or(candidate));
        }
    }

    bail!(
        "Cannot find OpenSecKit repository root. \
        Make sure you run 'osk init --local' from within the OpenSecKit repository \
        or use remote mode (default)."
    )
}

/// Install resources from local repository
/// V4: Only copies registry.toml and agents.toml (templates used directly from repo)
fn install_resources_local(repo_root: &Path, force: bool) -> Result<InstallStats> {
    let mut stats = InstallStats::default();

    // Copy config/registry.toml
    let registry_src = repo_root.join("config/registry.toml");
    let registry_dest = PathBuf::from(".osk/registry.toml");

    if registry_src.exists() && (!registry_dest.exists() || force) {
        match fs::copy(&registry_src, &registry_dest) {
            Ok(_) => stats.registry = true,
            Err(e) => stats.errors.push(format!("registry.toml: {}", e)),
        }
    } else if registry_dest.exists() {
        stats.registry = true;
    }

    // Copy config/agents.toml
    let agents_src = repo_root.join("config/agents.toml");
    let agents_dest = PathBuf::from(".osk/agents.toml");
    if agents_src.exists() && (!agents_dest.exists() || force) {
        match fs::copy(&agents_src, &agents_dest) {
            Ok(_) => stats.agents_config = true,
            Err(e) => stats.errors.push(format!("agents.toml: {}", e)),
        }
    } else if agents_dest.exists() {
        stats.agents_config = true;
    }

    // In local mode, templates are used directly from repo (not copied)
    // Just count them for stats
    let agents_templates_src = repo_root.join("templates/agents");
    if agents_templates_src.exists() {
        for entry in WalkDir::new(&agents_templates_src).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                stats.agents_templates += 1;
            }
        }
    }

    Ok(stats)
}

/// Install resources from remote GitHub repository
/// V4: Only downloads registry.toml, agents.toml, and templates/agents/
fn install_resources_remote(client: &Client, force: bool, tag: &str) -> Result<InstallStats> {
    use crate::github;

    let mut stats = InstallStats::default();

    // Download config/registry.toml
    let registry_dest = PathBuf::from(".osk/registry.toml");
    if !registry_dest.exists() || force {
        if let Err(e) = github::download_file(client, tag, "config/registry.toml", &registry_dest) {
            stats.errors.push(format!("config/registry.toml: {}", e));
        } else {
            stats.registry = true;
        }
    } else {
        stats.registry = true;
    }

    // Download config/agents.toml
    let agents_dest = PathBuf::from(".osk/agents.toml");
    if !agents_dest.exists() || force {
        if let Err(e) = github::download_file(client, tag, "config/agents.toml", &agents_dest) {
            stats.errors.push(format!("config/agents.toml: {}", e));
        } else {
            stats.agents_config = true;
        }
    } else {
        stats.agents_config = true;
    }

    // Download only templates/agents/ (for agent file rendering)
    let tree_items = match github::fetch_repo_tree(client, tag) {
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

        // Only download templates/agents/ files
        if !item.path.starts_with("templates/agents/") {
            continue;
        }

        let dest = PathBuf::from(".osk").join(&item.path);
        if dest.exists() && !force {
            stats.agents_templates += 1;
            continue;
        }

        if let Some(parent) = dest.parent() {
            fs::create_dir_all(parent).ok();
        }

        if let Err(e) = github::download_file(client, tag, &item.path, &dest) {
            stats.errors.push(format!("{}: {}", item.path, e));
        } else {
            stats.agents_templates += 1;
        }
    }

    Ok(stats)
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
    fn test_install_stats_default() {
        let stats = InstallStats::default();
        assert!(!stats.registry);
        assert!(!stats.agents_config);
        assert_eq!(stats.agents_templates, 0);
        assert_eq!(stats.commands, 0);
        assert!(stats.errors.is_empty());
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
