use crate::config::OskConfig;
use crate::memory::MemoryManager;
use crate::prompts::PromptBuilder;
use crate::providers;
use anyhow::{bail, Context, Result};
use reqwest::blocking::Client;
use std::fs;
use std::path::Path;

pub fn exec(
    client: &Client,
    keywords: Vec<String>,
    input_flag: Option<String>,
    override_provider: Option<String>,
) -> Result<()> {
    // 1. Chargement de la Configuration
    let config_content =
        fs::read_to_string(".osk/config.toml").context("Config introuvable. Lancez 'osk init'.")?;
    let config: OskConfig = toml::from_str(&config_content)?;

    // 2. Initialisation du Provider
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

    // 3. Gestion du Contexte & Mémoire
    let context_path = Path::new("context.txt");
    if !context_path.exists() {
        println!("⚠️  Contexte introuvable. Lancement automatique de 'osk ingest'...");
        crate::commands::ingest::run("context.txt", false, ".")?;
    }

    let codebase = fs::read_to_string(context_path)
        .unwrap_or_else(|_| "Aucun contexte technique.".to_string());

    let mut memory_content = String::new();
    let mut memory_mgr = None;

    if let Some(mem_config) = &config.memory {
        if mem_config.enabled {
            let mgr = MemoryManager::new(&mem_config.path);
            memory_content = mgr.load_history()?;
            memory_mgr = Some(mgr);
        }
    }

    // 4. RÉSOLUTION DE LA COMMANDE ET INPUT
    // On cherche si les mots-clés correspondent à une commande enregistrée
    let (command_name, final_task) = match PromptBuilder::find_command_match(&keywords)? {
        // CAS A : Commande reconnue (ex: "audit.security")
        Some((key, leftovers)) => {
            // RÈGLE STRICTE : Si commande reconnue, aucun argument positionnel n'est toléré après
            if !leftovers.is_empty() {
                bail!(
                    "❌ Argument inattendu : '{}'.\n💡 Pour passer des données à la commande '{}', utilisez l'option --input (ou -i).", 
                    leftovers.join(" "),
                    key
                );
            }

            println!("   📂 Commande chargée : '{}'", key);
            let prompt_content = PromptBuilder::resolve_command_key(client, &key)?;

            // Injection de l'input structuré
            let task = if let Some(inp) = input_flag {
                // Si le template a un placeholder {{argument}}, on le remplace
                if prompt_content.contains("{{argument}}") {
                    prompt_content.replace("{{argument}}", &inp)
                } else {
                    // Sinon on l'ajoute à la fin proprement
                    format!(
                        "{}\n\nCONTEXTE SPÉCIFIQUE FOURNI PAR L'UTILISATEUR :\n{}",
                        prompt_content, inp
                    )
                }
            } else {
                // Pas d'input fourni ? On remplace le placeholder par une valeur par défaut
                prompt_content.replace("{{argument}}", "le projet entier")
            };

            (key, task)
        }

        // CAS B : Aucune commande connue -> Mode "Prompt Libre"
        None => {
            let prompt = keywords.join(" ");
            // En mode libre, on peut aussi avoir un --input, on concatène tout
            let full_prompt = if let Some(inp) = input_flag {
                format!("{} \n\nDETAILS : {}", prompt, inp)
            } else {
                prompt
            };
            ("Prompt Libre".to_string(), full_prompt)
        }
    };

    // 5. Exécution
    let system_prompt = PromptBuilder::build_system(&config);
    let user_prompt = PromptBuilder::build_user(&codebase, &memory_content, &final_task);

    println!("🤖 Appel à {provider_name} ({command_name})...");
    let response = provider.complete(&system_prompt, &user_prompt)?;

    println!("\n{response}\n");

    if let Some(mgr) = memory_mgr {
        mgr.save(&final_task, &response)?;
    }

    Ok(())
}
