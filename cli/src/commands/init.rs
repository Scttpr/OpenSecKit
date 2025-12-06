use crate::config::{AgentConfig, MemoryConfig, OskConfig, ProjectConfig};
use crate::github;
use crate::stack;
use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Confirm, Input, Select};
use reqwest::blocking::Client;
use std::fs;
use std::path::{Path, PathBuf};

pub fn run(client: &Client, force: bool) -> Result<()> {
    println!("🚀 Initialisation de OpenSecKit...");

    let (config, provider_key, enable_claude_code) = prompt_configuration(client)?;

    scaffold_project(&config, &provider_key, enable_claude_code)?;

    install_resources(client, &provider_key, enable_claude_code, force)?;

    println!("\n✅ Terminé ! Configuré pour {provider_key}.");

    if let Some(proj) = &config.project {
        if let Some(stack) = &proj.stack {
            println!("   🏗️  Stack technique enregistrée : {}", stack);
        }
    }

    if enable_claude_code {
        println!("   ℹ️  Compatibilité 'Claude Code' activée (dossier .claude généré).");
    }
    Ok(())
}

fn prompt_configuration(client: &Client) -> Result<(OskConfig, String, bool)> {
    // 1. Modèles IA
    println!("   ⏳ Récupération des modèles disponibles...");
    let manifest = github::fetch_model_manifest(client);

    let items: Vec<&String> = manifest.options.iter().map(|opt| &opt.name).collect();

    let selection_index = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Quel assistant IA utiliser ?")
        .default(0)
        .items(&items)
        .interact()?;

    let selected_model = &manifest.options[selection_index];
    let provider_key = selected_model.provider_id.clone();

    let mut enable_claude_code = false;
    if provider_key == "claude" {
        enable_claude_code = Confirm::with_theme(&ColorfulTheme::default())
            .with_prompt("Voulez-vous activer l'intégration pour l'agent CLI 'Claude Code' ?")
            .default(false)
            .interact()?;
    }

    let project_name: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Nom du projet")
        .default("MonProjet".to_string())
        .interact_text()?;

    println!("   🕵️  Scan de la stack technique (Monorepo supporté)...");
    let detected_stack = stack::detect();

    let final_stack: String = Input::with_theme(&ColorfulTheme::default())
        .with_prompt("Stack technique (Validez ou modifiez)")
        .with_initial_text(detected_stack)
        .allow_empty(true)
        .interact_text()?;

    let stack_opt = if final_stack.trim().is_empty() {
        None
    } else {
        Some(final_stack)
    };

    let config = OskConfig {
        agent: AgentConfig {
            provider: provider_key.clone(),
            model: selected_model.model_id.clone(),
            temperature: 0.2,
        },
        project: Some(ProjectConfig {
            name: project_name,
            description: None,
            stack: stack_opt,
        }),
        memory: Some(MemoryConfig {
            enabled: true,
            path: ".osk/memory".to_string(),
        }),
    };

    Ok((config, provider_key, enable_claude_code))
}

fn scaffold_project(
    config: &OskConfig,
    _provider_key: &str,
    enable_claude_code: bool,
) -> Result<()> {
    fs::create_dir_all(".osk/prompts")?;
    fs::create_dir_all(".osk/templates")?;
    fs::create_dir_all(".osk/memory")?;

    if enable_claude_code {
        fs::create_dir_all(".claude/commands")?;
    }

    let toml_string = toml::to_string_pretty(config)?;
    fs::write(".osk/config.toml", toml_string)?;

    if !Path::new(".gitignore").exists() {
        let mut gitignore_content = String::from(".osk/memory/\n.osk/config.toml\n");
        if enable_claude_code {
            gitignore_content.push_str(".claude/\n");
        }
        fs::write(".gitignore", gitignore_content)?;
    }

    Ok(())
}

fn install_resources(
    client: &Client,
    _provider_key: &str,
    enable_claude_code: bool,
    force: bool,
) -> Result<()> {
    let tag = github::fetch_latest_tag(client)?;

    // 1. TÉLÉCHARGEMENT DU REGISTRE DE COMMANDES (Gestion d'erreur renforcée)
    let registry_dest = PathBuf::from(".osk/registry.toml");
    if !registry_dest.exists() || force {
        match github::download_file(client, &tag, "registry.toml", &registry_dest) {
             Ok(_) => println!("   📜 Registre des commandes mis à jour."),
             Err(e) => println!("   ⚠️  ATTENTION : Impossible de télécharger le registre des commandes ({e}).\n      Les commandes 'osk run audit...' pourraient ne pas fonctionner."),
         }
    }

    let tree_items = github::fetch_repo_tree(client, &tag)?;

    for item in tree_items {
        if item.item_type != "blob" {
            continue;
        }

        if enable_claude_code && item.path.starts_with("prompts/") {
            let filename = Path::new(&item.path).file_name().unwrap();
            let dest_claude = PathBuf::from(".claude/commands").join(filename);

            if let Some(parent) = dest_claude.parent() {
                fs::create_dir_all(parent)?;
            }
            if !dest_claude.exists() || force {
                github::download_file(client, &tag, &item.path, &dest_claude).ok();
            }
        }

        if item.path.starts_with("templates/")
            || item.path.starts_with("domaines/")
            || item.path == "constitution.md"
        {
            let dest = PathBuf::from(".osk").join(&item.path);
            if dest.exists() && !force {
                continue;
            }
            github::download_file(client, &tag, &item.path, &dest).ok();
        }
    }
    Ok(())
}
