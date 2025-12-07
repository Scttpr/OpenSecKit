use serde::{Deserialize, Serialize};

pub const REPO_OWNER: &str = "Scttpr";
pub const REPO_NAME: &str = "OpenSecKit";
pub const USER_AGENT: &str = "osk-cli";

#[derive(Debug, Deserialize, Clone)]
pub struct ModelManifest {
    pub options: Vec<ModelOption>,
}

#[derive(Debug, Deserialize, Clone)]
pub struct ModelOption {
    pub name: String,
    pub provider_id: String,
    pub model_id: String,
}

impl ModelManifest {
    pub fn fallback() -> Self {
        Self {
            options: vec![
                ModelOption {
                    name: "Claude 3.5 Sonnet (Recommandé)".to_string(),
                    provider_id: "claude".to_string(),
                    model_id: "claude-3-5-sonnet-20241022".to_string(),
                },
                ModelOption {
                    name: "Gemini 3 Pro".to_string(),
                    provider_id: "gemini".to_string(),
                    model_id: "gemini-3-pro".to_string(),
                },
            ],
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct OskConfig {
    pub agent: AgentConfig,
    pub project: Option<ProjectConfig>,
    pub memory: Option<MemoryConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AgentConfig {
    pub provider: String,
    pub model: String,
    pub temperature: f32,
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
        let defaults = ModelManifest::fallback();
        let default_model = &defaults.options[0];

        OskConfig {
            agent: AgentConfig {
                provider: default_model.provider_id.clone(),
                model: default_model.model_id.clone(),
                temperature: 0.2,
            },
            project: None,
            memory: Some(MemoryConfig {
                enabled: true,
                path: ".osk/memory".to_string(),
            }),
        }
    }
}
