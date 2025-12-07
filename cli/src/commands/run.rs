use crate::commands::ingest;
use crate::config::OskConfig;
use crate::memory::MemoryManager;
use crate::prompts::PromptBuilder;
use crate::providers;
use anyhow::{Context, Result};
use reqwest::blocking::Client;
use std::fs;
use std::path::Path;

pub fn exec_specific(
    client: &Client,
    command_key: &str,
    user_input: Option<String>,
    override_provider: Option<String>,
) -> Result<()> {
    let config_content = fs::read_to_string(".osk/config.toml").context("Config introuvable")?;
    let config: OskConfig = toml::from_str(&config_content)?;

    let provider_name = override_provider.unwrap_or(config.agent.provider.clone());
    let api_key_env = format!("{}_API_KEY", provider_name.to_uppercase());
    let api_key =
        std::env::var(&api_key_env).context(format!("Variable {api_key_env} manquante"))?;

    let provider = providers::get_provider(
        client.clone(),
        &provider_name,
        &config.agent.model,
        &api_key,
    )?;

    println!("🧠 Analyse de l'arborescence pour identifier les fichiers pertinents...");
    let file_tree = ingest::get_file_tree(".");
    let user_task_desc = user_input
        .clone()
        .unwrap_or_else(|| "Analyse globale".to_string());

    let selector_system = "Tu es un expert. Sélectionne les fichiers pertinents.";
    let selector_user = format!(
        "Arborescence :\n{file_tree}\n\nDEMANDE : Commande '{command_key}' sur '{user_task_desc}'.\n\
        TACHE : Liste les chemins des fichiers nécessaires (max 25).\n\
        FORMAT : Tableau JSON de chaînes. Exemple: [\"src/main.rs\"]"
    );

    let selection_json = provider.complete(selector_system, &selector_user)?;
    let clean_json = selection_json
        .trim()
        .trim_start_matches("```json")
        .trim_start_matches("```")
        .trim_end_matches("```")
        .trim();

    let selected_files: Vec<String> = serde_json::from_str(clean_json).unwrap_or_else(|_| {
        println!("⚠️  Sélection auto échouée. Fallback.");
        Vec::<String>::new()
    });

    let smart_context = if selected_files.is_empty() {
        if Path::new("context.txt").exists() {
            fs::read_to_string("context.txt")?
        } else {
            String::from("CONTEXTE NON DISPONIBLE")
        }
    } else {
        println!(
            "📂 Fichiers sélectionnés ({}) : {:?}",
            selected_files.len(),
            selected_files
        );
        ingest::get_files_content(&selected_files)
    };

    println!("   📂 Chargement du module : '{command_key}'");
    let prompt_template = PromptBuilder::resolve_command_key(client, command_key)?;

    let final_task = if let Some(inp) = user_input {
        prompt_template.replace("{{argument}}", &inp)
    } else {
        prompt_template.replace("{{argument}}", "le projet entier")
    };

    let system_prompt = PromptBuilder::build_system(&config);
    let user_prompt = PromptBuilder::build_user(&smart_context, "", &final_task);

    println!("🤖 Analyse approfondie via {provider_name}...");
    let response = provider.complete(&system_prompt, &user_prompt)?;

    if command_key == "context" {
        let docs_dir = Path::new("docs/context");
        fs::create_dir_all(docs_dir)?;
        let meta_path = docs_dir.join("meta.md");
        fs::write(&meta_path, &response)?;
        println!("\n✅ Mémoire du projet générée : {meta_path:?}");
        println!("📝 Vous pouvez éditer ce fichier pour affiner les règles.");
    } else {
        println!("\n{response}\n");
        if let Some(mem_config) = &config.memory {
            if mem_config.enabled {
                let mgr = MemoryManager::new(&mem_config.path);
                mgr.save(&final_task, &response)?;
            }
        }
    }

    Ok(())
}
