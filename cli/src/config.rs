use serde::{Deserialize, Serialize};

pub const REPO_OWNER: &str = "Scttpr";
pub const REPO_NAME: &str = "OpenSecKit";
pub const USER_AGENT: &str = "osk-cli";

/// Minimal project configuration for init
/// Compliance and other settings are handled by their respective commands
#[derive(Debug, Serialize, Deserialize)]
pub struct OskConfig {
    pub version: String,
    pub project: ProjectConfig,
    pub agents: AgentsSetup,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentsSetup {
    pub installed: Vec<String>,
}

impl OskConfig {
    pub fn new(name: String, agents: Vec<String>) -> Self {
        OskConfig {
            version: "4.0.0".to_string(),
            project: ProjectConfig { name },
            agents: AgentsSetup { installed: agents },
        }
    }
}
