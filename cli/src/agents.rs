//! Multi-agent configuration and file generation

use anyhow::{Context, Result};
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;
use tera::{Context as TeraContext, Tera};

use crate::prompts::PromptInfo;

#[derive(Debug, Deserialize)]
pub struct AgentsConfig {
    pub meta: MetaConfig,
    pub agents: HashMap<String, AgentConfig>,
    pub universal: Option<UniversalConfig>,
}

#[derive(Debug, Deserialize)]
pub struct MetaConfig {
    #[serde(default)]
    #[allow(dead_code)]
    pub version: String,
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

#[derive(Debug, Clone, Default, serde::Serialize)]
pub struct DomainsInfo {
    pub rgpd: bool,
    pub rgs: bool,
    pub nis2: bool,
}

pub fn load_agents_config(osk_dir: &Path) -> Result<AgentsConfig> {
    let config_path = osk_dir.join("agents.toml");

    if !config_path.exists() {
        anyhow::bail!("agents.toml non trouvé dans {}", osk_dir.display());
    }

    let content = fs::read_to_string(&config_path)
        .context("Impossible de lire agents.toml")?;

    let config: AgentsConfig = toml::from_str(&content)
        .context("Erreur de parsing agents.toml")?;

    Ok(config)
}

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

pub fn generate_agent_files(
    agent_id: &str,
    agent: &AgentConfig,
    prompts: &[PromptInfo],
    templates_dir: &Path,
    version: &str,
    domains: &DomainsInfo,
) -> Result<usize> {
    if !agent.enabled {
        return Ok(0);
    }

    let template_path = templates_dir.join("agents").join(&agent.template);
    let template_content = fs::read_to_string(&template_path)
        .with_context(|| format!("Template non trouvé: {}", template_path.display()))?;

    let mut tera = Tera::default();
    tera.add_raw_template(&agent.template, &template_content)?;

    match agent.format.as_str() {
        "slash-command" => generate_slash_commands(agent, prompts, &tera, version),
        "single-file" => generate_single_file(agent, prompts, &tera, version, domains),
        "rules-dir" => generate_rules_dir(agent, prompts, &tera, version, domains),
        _ => {
            eprintln!("   ⚠️  Format inconnu pour {}: {}", agent_id, agent.format);
            Ok(0)
        }
    }
}

fn generate_slash_commands(
    agent: &AgentConfig,
    prompts: &[PromptInfo],
    tera: &Tera,
    _version: &str,
) -> Result<usize> {
    let output_dir = agent.output_dir.as_ref()
        .ok_or_else(|| anyhow::anyhow!("output_dir requis pour format slash-command"))?;
    let pattern = agent.file_pattern.as_ref()
        .ok_or_else(|| anyhow::anyhow!("file_pattern requis pour format slash-command"))?;

    fs::create_dir_all(output_dir)?;

    let mut count = 0;
    for prompt in prompts {
        let mut context = TeraContext::new();
        context.insert("raw_content", &prompt.raw_content);
        context.insert("prompt", prompt);

        let content = tera.render(&agent.template, &context)?;
        let filename = pattern.replace("{command}", &prompt.name);
        let path = Path::new(output_dir).join(&filename);

        fs::write(&path, content)?;
        count += 1;
    }

    Ok(count)
}

fn generate_single_file(
    agent: &AgentConfig,
    prompts: &[PromptInfo],
    tera: &Tera,
    version: &str,
    domains: &DomainsInfo,
) -> Result<usize> {
    let output_file = agent.output_file.as_ref()
        .ok_or_else(|| anyhow::anyhow!("output_file requis pour format single-file"))?;

    if let Some(parent) = Path::new(output_file).parent() {
        fs::create_dir_all(parent)?;
    }

    let mut context = TeraContext::new();
    context.insert("prompts", prompts);
    context.insert("version", version);
    context.insert("domains", domains);

    let content = tera.render(&agent.template, &context)?;
    fs::write(output_file, content)?;

    Ok(1)
}

fn generate_rules_dir(
    agent: &AgentConfig,
    prompts: &[PromptInfo],
    tera: &Tera,
    version: &str,
    domains: &DomainsInfo,
) -> Result<usize> {
    let output_dir = agent.output_dir.as_ref()
        .ok_or_else(|| anyhow::anyhow!("output_dir requis pour format rules-dir"))?;
    let pattern = agent.file_pattern.as_ref()
        .ok_or_else(|| anyhow::anyhow!("file_pattern requis pour format rules-dir"))?;

    fs::create_dir_all(output_dir)?;

    let mut count = 0;
    for prompt in prompts {
        let mut context = TeraContext::new();
        context.insert("prompt", prompt);
        context.insert("version", version);
        context.insert("domains", domains);

        let content = tera.render(&agent.template, &context)?;
        let filename = pattern.replace("{command}", &prompt.name);
        let path = Path::new(output_dir).join(&filename);

        fs::write(&path, content)?;
        count += 1;
    }

    Ok(count)
}

pub fn generate_agents_md(
    config: &UniversalConfig,
    prompts: &[PromptInfo],
    templates_dir: &Path,
    version: &str,
    domains: &DomainsInfo,
) -> Result<()> {
    if !config.enabled {
        return Ok(());
    }

    let template_path = templates_dir.join("agents").join(&config.template);
    let template_content = fs::read_to_string(&template_path)
        .with_context(|| format!("Template AGENTS.md non trouvé: {}", template_path.display()))?;

    let mut tera = Tera::default();
    tera.add_raw_template(&config.template, &template_content)?;

    let mut context = TeraContext::new();
    context.insert("prompts", prompts);
    context.insert("version", version);
    context.insert("domains", domains);

    let content = tera.render(&config.template, &context)?;
    fs::write(&config.output_file, content)?;

    Ok(())
}
