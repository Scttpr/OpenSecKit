//! Init command - project initialization
//!
//! V4 Architecture: Minimal setup, auto-detection
//! - Auto-detects project name from folder/git
//! - Auto-detects available agents on system
//! - Downloads: config/registry.toml, config/agents.toml, cli/templates/agents/
//! - Generates agent command files for detected agents only

use crate::agents::{self, AgentConfig, AgentsConfig};
use crate::args::Agent;
use crate::config::OskConfig;
use crate::registry::{self, CommandInfo, DomainsInfo};
use anyhow::{bail, Result};
use dialoguer::{theme::ColorfulTheme, Confirm, Input};
use reqwest::blocking::Client;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
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
    #[allow(dead_code)]
    agents_config: bool,
    #[allow(dead_code)]
    agents_templates: usize,
    errors: Vec<String>,
}

struct AgentDetection {
    id: String,
    name: String,
    detected: bool,
    #[allow(dead_code)]
    reason: String,
}

struct AgentInstallResult {
    #[allow(dead_code)]
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
    let source = determine_source(local, local_path, version)?;

    // Print header
    print_header(&source);

    // Step 1: Get project name
    let project_name = get_project_name(default)?;
    println!("  Project: {}\n", project_name);

    // Step 2: Create .osk directory
    fs::create_dir_all(".osk")?;

    // Step 3: Install resources (registry, agents config, templates)
    let stats = install_resources(client, force, &source)?;

    // Step 4: Load agents config and detect available agents
    let osk_dir = PathBuf::from(".osk");
    let agents_config = load_agents_config_or_fallback(&osk_dir)?;
    let detections = detect_agents(&agents_config);

    // Print agent detection results
    print_agent_detection(&detections);

    // Step 5: Determine which agents to install
    let selected_agents = select_agents_to_install(
        &agents_config,
        &detections,
        default,
        agent,
        all_agents,
    )?;

    if selected_agents.is_empty() {
        println!("\n  No agents selected for installation.\n");
        return Ok(());
    }

    // Step 6: Save config
    let config = OskConfig::new(project_name.clone(), selected_agents.clone());
    save_config(&config)?;

    // Step 7: Scaffold project directories
    scaffold_project(force)?;

    // Step 8: Load registry and generate agent files
    let registry_path = osk_dir.join("registry.toml");
    let (commands, domains) = load_registry(&registry_path)?;

    let (templates_dir, local_repo) = match &source {
        ResourceSource::Local(path) => (path.join("cli/templates"), Some(path.as_path())),
        _ => (osk_dir.join("cli/templates"), None),
    };

    let agent_results = install_selected_agents(
        client,
        &selected_agents,
        &agents_config,
        &commands,
        &templates_dir,
        &domains,
        local_repo,
    );

    // Step 9: Generate AGENTS.md if enabled
    if let Some(ref universal) = agents_config.universal {
        if universal.enabled {
            let _ = agents::generate_agents_md(
                universal,
                &commands,
                &templates_dir,
                VERSION,
                &domains,
            );
        }
    }

    // Print summary
    print_summary(&project_name, &stats, &agent_results, commands.len());

    Ok(())
}

fn determine_source(
    local: bool,
    local_path: Option<String>,
    version: Option<String>,
) -> Result<ResourceSource> {
    if let Some(path) = local_path {
        let repo_root = PathBuf::from(&path);
        validate_repo_root(&repo_root)?;
        Ok(ResourceSource::Local(repo_root.canonicalize().unwrap_or(repo_root)))
    } else if local {
        let repo_root = find_repo_root()?;
        Ok(ResourceSource::Local(repo_root))
    } else if let Some(v) = version {
        Ok(ResourceSource::Version(v))
    } else {
        Ok(ResourceSource::Latest)
    }
}

fn print_header(source: &ResourceSource) {
    let source_label = match source {
        ResourceSource::Local(path) => format!("local: {}", path.display()),
        ResourceSource::Version(v) => format!("version: {}", v),
        ResourceSource::Latest => "latest".to_string(),
    };

    println!();
    println!("  ╭─────────────────────────────────────────╮");
    println!("  │  OpenSecKit v{}                       │", VERSION);
    println!("  │  Source: {:<30}│", source_label);
    println!("  ╰─────────────────────────────────────────╯");
    println!();
}

fn get_project_name(default: bool) -> Result<String> {
    // Try to detect from git remote or folder name
    let detected = detect_project_name();

    if default {
        return Ok(detected);
    }

    let name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("  Project name")
        .default(detected)
        .interact_text()?;

    Ok(name)
}

fn detect_project_name() -> String {
    // Try git remote first
    if let Ok(output) = Command::new("git")
        .args(["remote", "get-url", "origin"])
        .output()
    {
        if output.status.success() {
            let url = String::from_utf8_lossy(&output.stdout);
            if let Some(name) = extract_repo_name(&url) {
                return name;
            }
        }
    }

    // Fall back to current directory name
    std::env::current_dir()
        .ok()
        .and_then(|p| p.file_name().map(|n| n.to_string_lossy().to_string()))
        .unwrap_or_else(|| "my-project".to_string())
}

fn extract_repo_name(url: &str) -> Option<String> {
    let url = url.trim();
    // Handle both HTTPS and SSH URLs
    // https://github.com/user/repo.git
    // git@github.com:user/repo.git
    let name = url
        .rsplit('/')
        .next()
        .or_else(|| url.rsplit(':').next())?
        .trim_end_matches(".git")
        .to_string();

    if name.is_empty() {
        None
    } else {
        Some(name)
    }
}

fn detect_agents(config: &AgentsConfig) -> Vec<AgentDetection> {
    config
        .agents
        .iter()
        .filter(|(_, agent)| agent.enabled)
        .map(|(id, agent)| {
            let (detected, reason) = check_agent_available(agent);
            AgentDetection {
                id: id.clone(),
                name: agent.name.clone(),
                detected,
                reason,
            }
        })
        .collect()
}

fn check_agent_available(agent: &AgentConfig) -> (bool, String) {
    // Check command availability
    if let Some(ref cmd) = agent.detect_cmd {
        if Command::new("which").arg(cmd).output().map(|o| o.status.success()).unwrap_or(false) {
            return (true, format!("`{}` found", cmd));
        }
    }

    // Check path existence
    if let Some(ref path) = agent.detect_path {
        if Path::new(path).exists() {
            return (true, format!("`{}` found", path));
        }
    }

    // Not detected
    let reason = match (&agent.detect_cmd, &agent.detect_path) {
        (Some(cmd), Some(path)) => format!("`{}` or `{}` not found", cmd, path),
        (Some(cmd), None) => format!("`{}` not found", cmd),
        (None, Some(path)) => format!("`{}` not found", path),
        (None, None) => "no detection method".to_string(),
    };

    (false, reason)
}

fn print_agent_detection(detections: &[AgentDetection]) {
    println!("  Detected agents:");
    for d in detections {
        let icon = if d.detected { "✓" } else { "✗" };
        let style = if d.detected { "" } else { " (not detected)" };
        println!("    {} {}{}", icon, d.name, style);
    }
    println!();
}

fn select_agents_to_install(
    config: &AgentsConfig,
    detections: &[AgentDetection],
    default: bool,
    agent: Option<Agent>,
    all_agents: bool,
) -> Result<Vec<String>> {
    // If specific agent requested
    if let Some(a) = agent {
        return Ok(vec![agent_to_id(&a)]);
    }

    // If all agents requested
    if all_agents {
        return Ok(config
            .agents
            .iter()
            .filter(|(_, a)| a.enabled)
            .map(|(id, _)| id.clone())
            .collect());
    }

    // Get detected agents
    let detected: Vec<String> = detections
        .iter()
        .filter(|d| d.detected)
        .map(|d| d.id.clone())
        .collect();

    if detected.is_empty() {
        if default {
            // In default mode, use claude-code as fallback
            return Ok(vec!["claude-code".to_string()]);
        }

        println!("  No agents auto-detected on your system.");
        let install_default = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("  Install Claude Code anyway?")
            .default(true)
            .interact()?;

        if install_default {
            return Ok(vec!["claude-code".to_string()]);
        } else {
            return Ok(vec![]);
        }
    }

    if default {
        return Ok(detected);
    }

    // Confirm installation of detected agents
    let confirm = Confirm::with_theme(&ColorfulTheme::default())
        .with_prompt(format!("  Install for {} detected agent(s)?", detected.len()))
        .default(true)
        .interact()?;

    if confirm {
        Ok(detected)
    } else {
        Ok(vec![])
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

fn save_config(config: &OskConfig) -> Result<()> {
    let config_path = Path::new(".osk/config.toml");
    let toml_string = toml::to_string_pretty(config)?;
    fs::write(config_path, toml_string)?;
    Ok(())
}

fn scaffold_project(force: bool) -> Result<()> {
    // Create minimal directory structure
    fs::create_dir_all(".osk/system-model")?;

    // Scaffold system model files
    scaffold_system_model(force)?;

    // Update .gitignore if needed
    update_gitignore()?;

    Ok(())
}

fn update_gitignore() -> Result<()> {
    let gitignore_path = Path::new(".gitignore");
    let osk_ignore = ".osk/\n";

    if gitignore_path.exists() {
        let content = fs::read_to_string(gitignore_path)?;
        if !content.contains(".osk/") && !content.contains(".osk\n") {
            fs::write(gitignore_path, format!("{}{}", content, osk_ignore))?;
        }
    } else {
        fs::write(gitignore_path, osk_ignore)?;
    }

    Ok(())
}

fn load_registry(registry_path: &Path) -> Result<(Vec<CommandInfo>, DomainsInfo)> {
    if registry_path.exists() {
        let reg = registry::load_registry(registry_path)?;
        let cmds = registry::get_commands(&reg);
        let doms = registry::detect_domains(&reg);
        Ok((cmds, doms))
    } else {
        Ok((Vec::new(), DomainsInfo::default()))
    }
}

fn install_selected_agents(
    client: &Client,
    selected: &[String],
    agents_config: &AgentsConfig,
    commands: &[CommandInfo],
    templates_dir: &Path,
    domains: &DomainsInfo,
    local_repo: Option<&Path>,
) -> Vec<AgentInstallResult> {
    let mut results = Vec::new();

    for agent_id in selected {
        if let Some(agent_config) = agents_config.agents.get(agent_id) {
            match install_agent(
                client,
                agent_id,
                agent_config,
                commands,
                templates_dir,
                domains,
                local_repo,
            ) {
                Ok(result) => results.push(result),
                Err(e) => eprintln!("  ⚠ Error installing {}: {}", agent_id, e),
            }
        }
    }

    results
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
    project_name: &str,
    stats: &InstallStats,
    agent_results: &[AgentInstallResult],
    commands_count: usize,
) {
    println!();
    println!("  ╭─────────────────────────────────────────╮");
    println!("  │  Setup Complete                         │");
    println!("  ╰─────────────────────────────────────────╯");
    println!();
    println!("  Project: {}", project_name);
    println!("  Config:  .osk/config.toml");
    println!();

    if !agent_results.is_empty() {
        println!("  Installed agents:");
        for result in agent_results {
            println!(
                "    ✓ {} → {} ({} files)",
                result.agent_name, result.output_path, result.files_count
            );
        }
        println!();
    }

    if stats.errors.is_empty() {
        println!("  Commands: {} available", commands_count);
    } else {
        println!("  ⚠ {} error(s):", stats.errors.len());
        for err in &stats.errors {
            println!("    - {}", err);
        }
    }

    println!();
    println!("  Next step: Run /osk-discover to analyze your codebase");
    println!();
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
            description: "Native slash commands".to_string(),
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
            description: "Copilot instructions".to_string(),
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
            description: "Cursor rules".to_string(),
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
            description: "Gemini instructions".to_string(),
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

/// Scaffold minimal system-model files
fn scaffold_system_model(force: bool) -> Result<()> {
    let system_model_dir = Path::new(".osk/system-model");

    let index_content = r#"# OpenSecKit System Model
# Run /osk-discover to populate

metadata:
  version: "4.0.0"
  created_at: null
  last_scan: null

project:
  name: null
  description: null

sections:
  - business.yaml
  - architecture.yaml
  - data.yaml
  - actors.yaml
  - integrations.yaml
  - controls.yaml
"#;

    let index_path = system_model_dir.join("index.yaml");
    if !index_path.exists() || force {
        fs::write(&index_path, index_content)?;
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

fn validate_repo_root(path: &Path) -> Result<()> {
    let registry = path.join("config/registry.toml");
    let templates_dir = path.join("cli/templates/agents");

    if !registry.exists() || !templates_dir.exists() {
        bail!(
            "Invalid OpenSecKit repository path: {}\n\
            Missing config/registry.toml or cli/templates/agents/ directory.",
            path.display()
        );
    }
    Ok(())
}

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
        let templates_dir = candidate.join("cli/templates/agents");

        if registry.exists() && templates_dir.exists() {
            return Ok(candidate.canonicalize().unwrap_or(candidate));
        }
    }

    bail!(
        "Cannot find OpenSecKit repository root.\n\
        Run 'osk init --local' from within the repository or use remote mode (default)."
    )
}

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

    // Count templates (used directly from repo in local mode)
    let agents_templates_src = repo_root.join("cli/templates/agents");
    if agents_templates_src.exists() {
        for entry in WalkDir::new(&agents_templates_src).into_iter().filter_map(|e| e.ok()) {
            if entry.file_type().is_file() {
                stats.agents_templates += 1;
            }
        }
    }

    Ok(stats)
}

fn install_resources_remote(client: &Client, force: bool, tag: &str) -> Result<InstallStats> {
    use crate::github;

    let mut stats = InstallStats::default();

    // Download config/registry.toml
    let registry_dest = PathBuf::from(".osk/registry.toml");
    if !registry_dest.exists() || force {
        if let Err(e) = github::download_file(client, tag, "config/registry.toml", &registry_dest) {
            stats.errors.push(format!("registry.toml: {}", e));
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
            stats.errors.push(format!("agents.toml: {}", e));
        } else {
            stats.agents_config = true;
        }
    } else {
        stats.agents_config = true;
    }

    // Download cli/templates/agents/
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

        if !item.path.starts_with("cli/templates/agents/") {
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
    fn test_extract_repo_name_https() {
        assert_eq!(
            extract_repo_name("https://github.com/user/my-repo.git"),
            Some("my-repo".to_string())
        );
    }

    #[test]
    fn test_extract_repo_name_ssh() {
        assert_eq!(
            extract_repo_name("git@github.com:user/my-repo.git"),
            Some("my-repo".to_string())
        );
    }

    #[test]
    fn test_extract_repo_name_no_git_suffix() {
        assert_eq!(
            extract_repo_name("https://github.com/user/my-repo"),
            Some("my-repo".to_string())
        );
    }

    #[test]
    fn test_agent_to_id() {
        assert_eq!(agent_to_id(&Agent::ClaudeCode), "claude-code");
        assert_eq!(agent_to_id(&Agent::Copilot), "copilot");
        assert_eq!(agent_to_id(&Agent::Cursor), "cursor");
        assert_eq!(agent_to_id(&Agent::Gemini), "gemini");
    }
}
