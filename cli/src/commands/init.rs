use crate::config::{MemoryConfig, OskConfig, ProjectConfig};
use crate::stack;
use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Input};
use reqwest::blocking::Client;
use std::fs;
use std::path::{Path, PathBuf};

pub fn run(client: &Client, force: bool) -> Result<()> {
    println!("🚀 Initialisation de OpenSecKit...");

    let config_exists = Path::new(".osk/config.toml").exists();

    let config = if config_exists && !force {
        println!("   ℹ️  Configuration existante détectée.");
        fs::read_to_string(".osk/config.toml")
            .and_then(|s| {
                toml::from_str(&s)
                    .map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))
            })
            .unwrap_or_else(|_| {
                println!("   ⚠️  Config corrompue, création d'une nouvelle...");
                prompt_configuration().unwrap()
            })
    } else {
        prompt_configuration()?
    };

    scaffold_project(&config, force)?;

    install_resources(client, force)?;
    install_slash_commands(force)?;

    println!("\n✅ OpenSecKit initialisé !");
    println!("\n📂 Slash commands générés dans .claude/commands/");
    println!("   Utilisez directement : /audit, /spec, /assess, /domain, /context, /incident");
    println!("\n💡 Lancez Claude Code et exécutez les slash commands :");
    println!("   claude");
    println!("   >>> /audit");

    Ok(())
}

fn prompt_configuration() -> Result<OskConfig> {
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

    Ok(config)
}

fn scaffold_project(config: &OskConfig, force: bool) -> Result<()> {
    fs::create_dir_all(".osk/prompts")?;
    fs::create_dir_all(".osk/templates")?;
    fs::create_dir_all(".osk/memory")?;
    fs::create_dir_all(".claude/commands")?;

    let config_path = Path::new(".osk/config.toml");
    if !config_path.exists() || force {
        let toml_string = toml::to_string_pretty(config)?;
        fs::write(config_path, toml_string)?;
        if force && config_path.exists() {
            println!("   ✅ Configuration mise à jour");
        }
    }

    if Path::new(".gitignore").exists() {
        println!("   ℹ️  Pensez à ajouter .osk/memory/ et .osk/config.toml à votre .gitignore");
    } else {
        let gitignore_content = ".osk/memory/\n.osk/config.toml\n.claude/\n";
        fs::write(".gitignore", gitignore_content)?;
    }

    Ok(())
}

fn install_resources(client: &Client, force: bool) -> Result<()> {
    use crate::github;

    let tag = github::fetch_latest_tag(client)?;

    let registry_dest = PathBuf::from(".osk/registry.toml");
    if !registry_dest.exists() || force {
        match github::download_file(client, &tag, "registry.toml", &registry_dest) {
            Ok(_) => println!("   📜 Registre des commandes mis à jour."),
            Err(e) => eprintln!("   ⚠️  ATTENTION : Echec téléchargement registre ({e}). Les commandes pourraient échouer."),
        }
    }

    let tree_items = github::fetch_repo_tree(client, &tag)?;

    for item in tree_items {
        if item.item_type != "blob" {
            continue;
        }

        if item.path.starts_with("prompts/") {
            let dest = PathBuf::from(".osk").join(&item.path);

            if dest.exists() && !force {
                continue;
            }

            if let Err(e) = github::download_file(client, &tag, &item.path, &dest) {
                eprintln!("   ⚠️  Echec prompt '{}': {}", item.path, e);
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

            if let Err(e) = github::download_file(client, &tag, &item.path, &dest) {
                eprintln!("   ⚠️  Echec ressource '{}': {}", item.path, e);
            }
        }
    }
    Ok(())
}

fn install_slash_commands(force: bool) -> Result<()> {
    println!("   📝 Installation des slash commands...");

    let prompts_dir = PathBuf::from(".osk/prompts");
    let commands_dir = PathBuf::from(".claude/commands");

    if !prompts_dir.exists() {
        eprintln!("   ⚠️  Dossier .osk/prompts/ introuvable. Slash commands non installés.");
        return Ok(());
    }

    fs::create_dir_all(&commands_dir)?;

    let mut count_installed = 0;
    let mut count_updated = 0;
    let mut count_skipped = 0;

    for entry in fs::read_dir(prompts_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            if let Some(filename) = path.file_name() {
                let dest = commands_dir.join(filename);

                if dest.exists() && !force {
                    count_skipped += 1;
                } else {
                    fs::copy(&path, &dest)?;
                    if dest.metadata().is_ok() && !force {
                        count_updated += 1;
                    } else {
                        count_installed += 1;
                    }
                }
            }
        }
    }

    if force && count_updated > 0 {
        println!("   ✅ {} slash commands mis à jour", count_installed);
    } else if count_installed > 0 {
        println!("   ✅ {} slash commands installés", count_installed);
    }

    if count_skipped > 0 && !force {
        println!(
            "   ℹ️  {} slash commands déjà présents (utilisez --force pour mettre à jour)",
            count_skipped
        );
    }

    Ok(())
}
