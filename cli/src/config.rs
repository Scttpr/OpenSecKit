use serde::{Deserialize, Serialize};

pub const REPO_OWNER: &str = "Scttpr";
pub const REPO_NAME: &str = "OpenSecKit";
pub const USER_AGENT: &str = "osk-cli";

#[derive(Debug, Serialize, Deserialize)]
pub struct OskConfig {
    pub project: Option<ProjectConfig>,
    pub stack: Option<StackConfig>,
    pub domains: Option<DomainsConfig>,
    pub principles: Option<PrinciplesConfig>,
    pub specs: Option<SpecsConfig>,
    pub memory: Option<MemoryConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StackConfig {
    pub detected: Vec<String>,
    pub custom: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DomainsConfig {
    pub active: Vec<String>,
    pub rgpd: Option<RgpdConfig>,
    pub nis2: Option<Nis2Config>,
    pub rgs: Option<RgsConfig>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RgpdConfig {
    pub enabled: bool,
    pub niveau: String,
    pub dpia_required: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Nis2Config {
    pub enabled: bool,
    pub type_entite: String,
    pub secteur: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RgsConfig {
    pub enabled: bool,
    pub niveau: String,
    pub perimetre: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrinciplesConfig {
    pub threat_modeling: String,
    pub risk_analysis: String,
    pub security_requirements: String,
    pub security_testing: String,
    pub secrets_management: String,
    pub audit_logging: String,
    pub patch_management: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpecsConfig {
    pub counter: u32,
    pub features: Vec<FeatureSpec>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FeatureSpec {
    pub id: String,
    pub name: String,
    pub status: String,
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
            stack: Some(StackConfig {
                detected: vec![],
                custom: vec![],
            }),
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
}
