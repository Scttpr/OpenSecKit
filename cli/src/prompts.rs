use crate::config::OskConfig;
use anyhow::{bail, Context, Result};
use reqwest::blocking::Client;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::time::Duration;

#[derive(Deserialize)]
struct CommandRegistry {
    commands: HashMap<String, String>,
}

pub struct PromptBuilder;

impl PromptBuilder {
    pub fn build_system(config: &OskConfig) -> String {
        let mut prompt =
            String::from("Tu es OpenSecKit, un expert en sécurité des applications.\n");

        if let Some(proj) = &config.project {
            prompt.push_str(&format!("PROJET : {}\n", proj.name));
            if let Some(desc) = &proj.description {
                prompt.push_str(&format!("DESCRIPTION : {desc}\n"));
            }
            if let Some(stack) = &proj.stack {
                prompt.push_str(&format!("STACK TECHNIQUE : {stack}\n"));
            }
        }

        prompt.push_str(
            "\nDIRECTIVE : Tes réponses doivent être concises, techniques et actionnables.",
        );
        prompt
    }

    pub fn build_user(codebase: &str, memory: &str, task: &str) -> String {
        format!(
            "=== CONTEXTE TECHNIQUE (Codebase) ===\n{codebase}\n\n\
             === MÉMOIRE (Historique) ===\n{memory}\n\n\
             === TÂCHE / INSTRUCTION ===\n{task}"
        )
    }

    pub fn find_command_match(keywords: &[String]) -> Result<Option<(String, Vec<String>)>> {
        let registry = Self::load_registry()?;

        // Recherche gloutonne (Longest Prefix Match)
        let len = keywords.len();
        for i in (0..len).rev() {
            let (potential_cmd, remainder) = keywords.split_at(i + 1);
            let key_attempt = potential_cmd.join(".");

            if registry.commands.contains_key(&key_attempt) {
                return Ok(Some((key_attempt, remainder.to_vec())));
            }
        }

        Ok(None)
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

    fn load_registry() -> Result<CommandRegistry> {
        let path = Path::new(".osk/registry.toml");
        if !path.exists() {
            return Ok(CommandRegistry {
                commands: HashMap::new(),
            });
        }
        let content = fs::read_to_string(path)?;
        let registry: CommandRegistry = toml::from_str(&content)?;
        Ok(registry)
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

        let cache_filename = format!("{}.md", key.replace('.', "_"));
        let cache_path = cache_dir.join(cache_filename);

        println!("   🌍 Vérification de la commande '{key}'...");

        let network_result = client.get(url).timeout(Duration::from_secs(3)).send();

        match network_result {
            Ok(resp) => {
                if resp.status().is_success() {
                    let content = resp.text()?;
                    let clean_content = Self::strip_frontmatter(&content);

                    // Mise à jour du cache
                    if let Err(e) = fs::write(&cache_path, &clean_content) {
                        eprintln!("   ⚠️  Impossible de mettre à jour le cache : {e}");
                    }

                    return Ok(clean_content);
                }
                println!(
                    "   ⚠️  Erreur serveur ({}). Tentative cache...",
                    resp.status()
                );
            }
            Err(_) => {
                println!("   ⚠️  Réseau indisponible. Tentative cache...");
            }
        }

        if cache_path.exists() {
            let content = fs::read_to_string(&cache_path)?;
            println!("   📦 Utilisation de la version en cache.");
            return Ok(content);
        }

        bail!("Impossible de récupérer la commande '{key}' (Ni réseau, ni cache)");
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
