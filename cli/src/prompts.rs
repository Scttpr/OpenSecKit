use crate::config::OskConfig;
use anyhow::{bail, Context, Result};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;
use walkdir::WalkDir;

#[derive(Deserialize)]
struct CommandRegistry {
    commands: HashMap<String, String>,
}

pub struct PromptBuilder;

impl PromptBuilder {
    pub fn build_system(config: &OskConfig) -> String {
        let mut prompt =
            String::from("Tu es OpenSecKit, un expert en sécurité des applications.\n");

        // 1. Identité Projet
        if let Some(proj) = &config.project {
            prompt.push_str(&format!("PROJET : {}\n", proj.name));
            if let Some(stack) = &proj.stack {
                prompt.push_str(&format!("STACK : {stack}\n"));
            }
        }

        // 2. MÉMOIRE PROJET (Règles d'Or)
        let meta_context_path = Path::new("docs/context/meta.md");
        if meta_context_path.exists() {
            if let Ok(content) = fs::read_to_string(meta_context_path) {
                prompt.push_str("\n=== RÈGLES ARCHITECTURALES (Réf) ===\n");
                prompt.push_str(&content);
                prompt.push_str("\n====================================\n");
            }
        }

        // 3. INDEX SÉCURITÉ (Artefacts existants)
        let security_docs = Self::scan_security_artifacts();
        if !security_docs.is_empty() {
            prompt.push_str("\n=== HISTORIQUE SÉCURITÉ (Artefacts disponibles) ===\n");
            prompt.push_str(
                "L'IA a déjà généré les documents suivants. Si pertinent, demande à les lire :\n",
            );
            prompt.push_str(&security_docs);
            prompt.push_str("===================================================\n");
        }

        prompt.push_str(
            "\nDIRECTIVE : Tes réponses doivent être concises, techniques et actionnables.",
        );
        prompt
    }

    pub fn build_user(codebase: &str, memory: &str, task: &str) -> String {
        format!(
            "=== CONTEXTE TECHNIQUE (Fichiers sélectionnés) ===\n{codebase}\n\n\
             === HISTORIQUE CONVERSATION ===\n{memory}\n\n\
             === TÂCHE / INSTRUCTION ===\n{task}"
        )
    }

    pub fn resolve_command_key(client: &Client, key: &str) -> Result<String> {
        let registry = Self::load_registry()?;
        if let Some(target) = registry.commands.get(key) {
            if target.starts_with("http") {
                return Self::fetch_remote_prompt(client, key, target);
            }
            return Self::load_local_file(target);
        }
        bail!("Clé de commande introuvable : {key}")
    }

    // --- UTILITAIRES ---

    fn scan_security_artifacts() -> String {
        let docs_path = Path::new("docs/security");
        let mut summary = String::new();
        if !docs_path.exists() {
            return summary;
        }

        for e in WalkDir::new(docs_path).max_depth(2).into_iter().flatten() {
            if e.file_type().is_file() && e.path().extension().is_some_and(|ext| ext == "md") {
                let path = e.path().to_string_lossy();
                let title = fs::read_to_string(e.path())
                    .ok()
                    .and_then(|c| c.lines().next().map(|l| l.to_string()))
                    .unwrap_or_else(|| "Sans titre".to_string());
                let clean_title = title.trim_start_matches('#').trim();
                summary.push_str(&format!("- Fichier: '{}' (Sujet: {})\n", path, clean_title));
            }
        }
        summary
    }

    fn load_registry() -> Result<CommandRegistry> {
        let path = Path::new(".osk/registry.toml");
        if !path.exists() {
            return Ok(CommandRegistry {
                commands: HashMap::new(),
            });
        }
        let content = fs::read_to_string(path)?;
        Ok(toml::from_str(&content)?)
    }

    fn load_local_file(path_str: &str) -> Result<String> {
        let path = PathBuf::from(".osk/prompts").join(path_str);
        let content = fs::read_to_string(&path)
            .context(format!("Impossible de lire le prompt local : {path:?}"))?;
        Ok(Self::strip_frontmatter(&content))
    }

    fn fetch_remote_prompt(client: &Client, key: &str, url: &str) -> Result<String> {
        let cache_dir = Path::new(".osk/prompts/cache");
        fs::create_dir_all(cache_dir)?;
        let cache_path = cache_dir.join(format!("{}.md", key.replace('.', "_")));

        println!("   🌍 Vérification de la commande '{key}'...");
        if let Ok(resp) = client.get(url).timeout(Duration::from_secs(3)).send() {
            if resp.status().is_success() {
                let content = Self::strip_frontmatter(&resp.text()?);
                let _ = fs::write(&cache_path, &content);
                return Ok(content);
            }
        }

        if cache_path.exists() {
            println!("   📦 Utilisation de la version en cache.");
            return Ok(fs::read_to_string(cache_path)?);
        }
        bail!("Impossible de récupérer la commande '{key}'")
    }

    fn strip_frontmatter(content: &str) -> String {
        if content.trim_start().starts_with("---") {
            let parts: Vec<&str> = content.splitn(3, "---").collect();
            if parts.len() >= 3 {
                return parts[2].trim().to_string();
            }
        }
        content.to_string()
    }
}
