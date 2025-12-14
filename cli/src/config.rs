use serde::{Deserialize, Serialize};

pub const REPO_OWNER: &str = "Scttpr";
pub const REPO_NAME: &str = "OpenSecKit";
pub const USER_AGENT: &str = "osk-cli";

#[derive(Debug, Serialize, Deserialize)]
pub struct OskConfig {
    pub project: Option<ProjectConfig>,
    pub memory: Option<MemoryConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub description: Option<String>,
    pub stack: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemoryConfig {
    pub enabled: bool,
    pub path: String,
}

impl Default for OskConfig {
    fn default() -> Self {
        OskConfig {
            project: None,
            memory: Some(MemoryConfig {
                enabled: true,
                path: ".osk/memory".to_string(),
            }),
        }
    }
}
