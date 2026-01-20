//! Multi-agent configuration and file generation

use anyhow::{Context, Result};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tera::{Context as TeraContext, Tera};

use crate::registry::{CommandInfo, DomainsInfo};

#[derive(Debug, Deserialize)]
pub struct AgentsConfig {
    #[allow(dead_code)]
    pub meta: MetaConfig,
    pub agents: HashMap<String, AgentConfig>,
    pub universal: Option<UniversalConfig>,
}

#[derive(Debug, Deserialize)]
pub struct MetaConfig {
    #[serde(default)]
    #[allow(dead_code)]
    pub version: String,
    #[allow(dead_code)]
    pub default_agent: String,
}

#[derive(Debug, Deserialize)]
pub struct AgentConfig {
    pub name: String,
    #[serde(default)]
    #[allow(dead_code)]
    pub description: String,
    pub format: String,
    pub output_dir: Option<String>,
    pub output_file: Option<String>,
    pub file_pattern: Option<String>,
    pub detect_cmd: Option<String>,
    pub detect_path: Option<String>,
    pub template: String,
    pub enabled: bool,
}

#[derive(Debug, Deserialize)]
pub struct UniversalConfig {
    pub enabled: bool,
    pub output_file: String,
    pub template: String,
}

pub fn load_agents_config(osk_dir: &Path) -> Result<AgentsConfig> {
    let config_path = osk_dir.join("agents.toml");

    if !config_path.exists() {
        anyhow::bail!("agents.toml not found in {}", osk_dir.display());
    }

    let content = fs::read_to_string(&config_path).context("Failed to read agents.toml")?;

    let config: AgentsConfig = toml::from_str(&content).context("Failed to parse agents.toml")?;

    Ok(config)
}

#[allow(dead_code)]
pub fn is_agent_available(agent: &AgentConfig) -> bool {
    if let Some(cmd) = &agent.detect_cmd {
        if which::which(cmd).is_ok() {
            return true;
        }
    }
    if let Some(path) = &agent.detect_path {
        if Path::new(path).exists() {
            return true;
        }
    }
    false
}

#[allow(clippy::too_many_arguments)]
pub fn generate_agent_files(
    client: &Client,
    agent_id: &str,
    agent: &AgentConfig,
    commands: &[CommandInfo],
    templates_dir: &Path,
    version: &str,
    domains: &DomainsInfo,
    local_repo: Option<&Path>,
) -> Result<usize> {
    if !agent.enabled {
        return Ok(0);
    }

    let template_path = templates_dir.join("agents").join(&agent.template);
    let template_content = fs::read_to_string(&template_path)
        .with_context(|| format!("Template not found: {}", template_path.display()))?;

    let mut tera = Tera::default();
    tera.add_raw_template(&agent.template, &template_content)?;

    match agent.format.as_str() {
        "slash-command" => {
            generate_slash_commands(client, agent, commands, &tera, version, local_repo)
        }
        "single-file" => generate_single_file(agent, commands, &tera, version, domains),
        "rules-dir" => generate_rules_dir(agent, commands, &tera, version, domains),
        _ => {
            eprintln!("  ⚠ Unknown format for {}: {}", agent_id, agent.format);
            Ok(0)
        }
    }
}

fn generate_slash_commands(
    client: &Client,
    agent: &AgentConfig,
    commands: &[CommandInfo],
    tera: &Tera,
    _version: &str,
    local_repo: Option<&Path>,
) -> Result<usize> {
    let output_dir = agent
        .output_dir
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("output_dir required for slash-command format"))?;
    let pattern = agent
        .file_pattern
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("file_pattern required for slash-command format"))?;

    fs::create_dir_all(output_dir)?;

    let mut count = 0;
    for cmd in commands {
        // Get prompt content: from local file if available, otherwise fetch from URL
        let raw_content = if let Some(repo_root) = local_repo {
            // Local mode: derive path from URL
            // URL format: https://raw.githubusercontent.com/.../main/{path}
            // Extract the path after "main/" to get the local file path
            let local_path = if let Some(pos) = cmd.url.find("/main/") {
                let relative_path = &cmd.url[pos + 6..]; // Skip "/main/"
                repo_root.join(relative_path)
            } else {
                // Fallback to legacy path for backwards compatibility
                repo_root.join("prompts").join(format!("{}.md", cmd.name))
            };
            match fs::read_to_string(&local_path) {
                Ok(content) => content,
                Err(e) => {
                    eprintln!("   ⚠  Cannot read {}: {}", local_path.display(), e);
                    format!("# {}\n\nFailed to read local prompt: {}", cmd.name, e)
                }
            }
        } else {
            // Remote mode: fetch from GitHub URL
            match client.get(&cmd.url).send() {
                Ok(response) => {
                    if response.status().is_success() {
                        response.text().unwrap_or_else(|_| {
                            format!(
                                "# {}\n\nFailed to read prompt content from {}",
                                cmd.name, cmd.url
                            )
                        })
                    } else {
                        eprintln!("   ⚠  HTTP {} fetching {}", response.status(), cmd.url);
                        format!(
                            "# {}\n\nFailed to fetch prompt (HTTP {})\nURL: {}",
                            cmd.name,
                            response.status(),
                            cmd.url
                        )
                    }
                }
                Err(e) => {
                    eprintln!("   ⚠  Error fetching {}: {}", cmd.url, e);
                    format!(
                        "# {}\n\nFailed to fetch prompt: {}\nURL: {}",
                        cmd.name, e, cmd.url
                    )
                }
            }
        };

        let mut context = TeraContext::new();
        context.insert("command", cmd);
        context.insert("raw_content", &raw_content);

        let content = tera.render(&agent.template, &context)?;
        let filename = pattern.replace("{command}", &cmd.name);
        let path = Path::new(output_dir).join(&filename);

        fs::write(&path, content)?;
        count += 1;
    }

    Ok(count)
}

fn generate_single_file(
    agent: &AgentConfig,
    commands: &[CommandInfo],
    tera: &Tera,
    version: &str,
    domains: &DomainsInfo,
) -> Result<usize> {
    let output_file = agent
        .output_file
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("output_file required for single-file format"))?;

    if let Some(parent) = Path::new(output_file).parent() {
        fs::create_dir_all(parent)?;
    }

    let mut context = TeraContext::new();
    context.insert("commands", commands);
    context.insert("version", version);
    context.insert("domains", domains);

    let content = tera.render(&agent.template, &context)?;
    fs::write(output_file, content)?;

    Ok(1)
}

fn generate_rules_dir(
    agent: &AgentConfig,
    commands: &[CommandInfo],
    tera: &Tera,
    version: &str,
    domains: &DomainsInfo,
) -> Result<usize> {
    let output_dir = agent
        .output_dir
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("output_dir required for rules-dir format"))?;
    let pattern = agent
        .file_pattern
        .as_ref()
        .ok_or_else(|| anyhow::anyhow!("file_pattern required for rules-dir format"))?;

    fs::create_dir_all(output_dir)?;

    let mut count = 0;
    for cmd in commands {
        let mut context = TeraContext::new();
        context.insert("command", cmd);
        context.insert("version", version);
        context.insert("domains", domains);

        let content = tera.render(&agent.template, &context)?;
        let filename = pattern.replace("{command}", &cmd.name);
        let path = Path::new(output_dir).join(&filename);

        fs::write(&path, content)?;
        count += 1;
    }

    Ok(count)
}

pub fn generate_agents_md(
    config: &UniversalConfig,
    commands: &[CommandInfo],
    templates_dir: &Path,
    version: &str,
    domains: &DomainsInfo,
) -> Result<()> {
    if !config.enabled {
        return Ok(());
    }

    let template_path = templates_dir.join("agents").join(&config.template);
    let template_content = fs::read_to_string(&template_path)
        .with_context(|| format!("AGENTS.md template not found: {}", template_path.display()))?;

    let mut tera = Tera::default();
    tera.add_raw_template(&config.template, &template_content)?;

    let mut context = TeraContext::new();
    context.insert("commands", commands);
    context.insert("version", version);
    context.insert("domains", domains);

    let content = tera.render(&config.template, &context)?;
    fs::write(&config.output_file, content)?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_domains_info_default() {
        let domains = DomainsInfo::default();
        assert!(!domains.rgpd);
        assert!(!domains.rgs);
        assert!(!domains.nis2);
    }

    #[test]
    fn test_load_agents_config_missing() {
        let dir = tempdir().unwrap();
        let result = load_agents_config(dir.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_load_agents_config_valid() {
        let dir = tempdir().unwrap();
        let config_content = r#"
[meta]
version = "1.0"
default_agent = "test-agent"

[agents.test-agent]
name = "Test Agent"
description = "A test agent"
format = "single-file"
output_file = "test.md"
template = "test.tera"
enabled = true
"#;
        fs::write(dir.path().join("agents.toml"), config_content).unwrap();

        let config = load_agents_config(dir.path()).unwrap();
        assert_eq!(config.meta.default_agent, "test-agent");
        assert!(config.agents.contains_key("test-agent"));
    }

    #[test]
    fn test_load_agents_config_invalid_toml() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("agents.toml"), "invalid { toml }").unwrap();

        let result = load_agents_config(dir.path());
        assert!(result.is_err());
    }

    #[test]
    fn test_is_agent_available_with_path() {
        let dir = tempdir().unwrap();
        let detect_path = dir.path().join("marker");
        fs::write(&detect_path, "").unwrap();

        let agent = AgentConfig {
            name: "Test".to_string(),
            description: String::new(),
            format: "single-file".to_string(),
            output_dir: None,
            output_file: Some("out.md".to_string()),
            file_pattern: None,
            detect_cmd: None,
            detect_path: Some(detect_path.to_string_lossy().to_string()),
            template: "t.tera".to_string(),
            enabled: true,
        };

        assert!(is_agent_available(&agent));
    }

    #[test]
    fn test_is_agent_available_missing() {
        let agent = AgentConfig {
            name: "Test".to_string(),
            description: String::new(),
            format: "single-file".to_string(),
            output_dir: None,
            output_file: Some("out.md".to_string()),
            file_pattern: None,
            detect_cmd: Some("nonexistent-command-xyz".to_string()),
            detect_path: Some("/nonexistent/path/xyz".to_string()),
            template: "t.tera".to_string(),
            enabled: true,
        };

        assert!(!is_agent_available(&agent));
    }
}
