//! Registry parsing module - extracts command metadata from registry.toml

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct Registry {
    pub metadata: RegistryMetadata,
    pub parts: HashMap<String, PartConfig>,
    pub roles: HashMap<String, RoleConfig>,
    pub commands: HashMap<String, CommandConfig>,
    #[serde(default)]
    pub schemas: HashMap<String, SchemaConfig>,
    #[serde(default)]
    pub templates: HashMap<String, TemplateConfig>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RegistryMetadata {
    pub version: String,
    pub description: String,
    pub repository: String,
    #[serde(default)]
    pub supported_agents: Vec<String>,
    #[serde(default)]
    pub architecture: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct PartConfig {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub hub: bool,
    #[serde(default)]
    pub consumes: Vec<String>,
    #[serde(default)]
    pub output: String,
    #[serde(default)]
    pub commands: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct RoleConfig {
    pub name: String,
    pub description: String,
    #[serde(default)]
    pub part: String,
    #[serde(default)]
    pub commands: Vec<String>,
    #[serde(default)]
    pub domains: Vec<String>,
    #[serde(default)]
    pub principles: Vec<String>,
}

#[derive(Debug, Deserialize, Clone)]
#[allow(dead_code)]
pub struct CommandConfig {
    pub url: String,
    #[serde(default)]
    pub version: String,
    #[serde(default)]
    pub part: String,
    #[serde(default)]
    pub phase: String,
    pub description: String,
    #[serde(default)]
    pub argument: Option<String>,
    #[serde(default)]
    pub requires: Vec<String>,
    #[serde(default)]
    pub generates: Vec<String>,
    #[serde(default)]
    pub model_sections: Vec<String>,
    #[serde(default)]
    pub domain: Option<String>,
    #[serde(default)]
    pub agents: Vec<String>,
    #[serde(default)]
    pub roles: Vec<String>,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct SchemaConfig {
    pub url: String,
    #[serde(default)]
    pub description: String,
}

#[derive(Debug, Deserialize)]
#[allow(dead_code)]
pub struct TemplateConfig {
    pub url: String,
    #[serde(default)]
    pub description: String,
}

/// CommandInfo - serializable struct for agent templates
/// Compatible with the fields used in Tera templates
#[derive(Debug, Clone, Serialize)]
pub struct CommandInfo {
    pub name: String,
    pub url: String,
    pub description: String,
    pub argument: Option<String>,
    pub part: String,
    pub phase: String,
    pub requires: Vec<String>,
    pub outputs: Vec<String>,
    pub domain: Option<String>,
}

impl From<(&str, &CommandConfig)> for CommandInfo {
    fn from((key, config): (&str, &CommandConfig)) -> Self {
        // Convert registry key (e.g., "discover-init") to command name (e.g., "osk-discover-init")
        let name = format!("osk-{}", key);

        CommandInfo {
            name,
            url: config.url.clone(),
            description: config.description.clone(),
            argument: config.argument.clone(),
            part: config.part.clone(),
            phase: config.phase.clone(),
            requires: config.requires.clone(),
            outputs: config.generates.clone(),
            domain: config.domain.clone(),
        }
    }
}

/// Load and parse the registry.toml file
///
/// The registry defines all available commands, their dependencies,
/// and metadata used for agent file generation.
pub fn load_registry(path: &Path) -> Result<Registry> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Cannot read registry: {}", path.display()))?;

    let registry: Registry =
        toml::from_str(&content).with_context(|| "Failed to parse registry.toml")?;

    Ok(registry)
}

/// Extract all commands from the registry as CommandInfo structs
///
/// Returns a sorted list of commands ready for template rendering.
pub fn get_commands(registry: &Registry) -> Vec<CommandInfo> {
    let mut commands: Vec<CommandInfo> = registry
        .commands
        .iter()
        .map(|(key, config)| CommandInfo::from((key.as_str(), config)))
        .collect();

    commands.sort_by(|a, b| a.name.cmp(&b.name));
    commands
}

/// Detect which domains are available based on command configs
pub fn detect_domains(registry: &Registry) -> DomainsInfo {
    let mut domains = DomainsInfo::default();

    for config in registry.commands.values() {
        if let Some(ref domain) = config.domain {
            match domain.as_str() {
                "rgpd" => domains.rgpd = true,
                "rgs" => domains.rgs = true,
                "nis2" => domains.nis2 = true,
                "iso27001" => domains.iso27001 = true,
                "soc2" => domains.soc2 = true,
                _ => {}
            }
        }
    }

    domains
}

/// Available compliance domains detected from registry commands
#[derive(Debug, Clone, Default, Serialize)]
pub struct DomainsInfo {
    pub rgpd: bool,
    pub rgs: bool,
    pub nis2: bool,
    pub iso27001: bool,
    pub soc2: bool,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_command_info_from() {
        let config = CommandConfig {
            url: "https://example.com/prompt.md".to_string(),
            version: "4.0.0".to_string(),
            part: "discover".to_string(),
            phase: "discovery".to_string(),
            description: "Test command".to_string(),
            argument: Some("--test".to_string()),
            requires: vec![".osk/config.toml".to_string()],
            generates: vec![".osk/output.yaml".to_string()],
            model_sections: vec![],
            domain: None,
            agents: vec![],
            roles: vec![],
        };

        let info = CommandInfo::from(("discover-init", &config));
        assert_eq!(info.name, "osk-discover-init");
        assert_eq!(info.part, "discover");
        assert_eq!(info.outputs.len(), 1);
    }

    #[test]
    fn test_detect_domains() {
        let mut commands = HashMap::new();
        commands.insert(
            "comply-rgpd".to_string(),
            CommandConfig {
                url: "https://example.com".to_string(),
                version: "4.0.0".to_string(),
                part: "comply".to_string(),
                phase: "compliance".to_string(),
                description: "RGPD".to_string(),
                argument: None,
                requires: vec![],
                generates: vec![],
                model_sections: vec![],
                domain: Some("rgpd".to_string()),
                agents: vec![],
                roles: vec![],
            },
        );

        let registry = Registry {
            metadata: RegistryMetadata {
                version: "4.0.0".to_string(),
                description: "Test".to_string(),
                repository: "https://github.com/test".to_string(),
                supported_agents: vec![],
                architecture: "hub-and-spokes".to_string(),
            },
            parts: HashMap::new(),
            roles: HashMap::new(),
            commands,
            schemas: HashMap::new(),
            templates: HashMap::new(),
        };

        let domains = detect_domains(&registry);
        assert!(domains.rgpd);
        assert!(!domains.rgs);
    }
}
