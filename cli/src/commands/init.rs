use crate::config::{
    DomainsConfig, MemoryConfig, Nis2Config, OskConfig, PrinciplesConfig, ProjectConfig,
    RgpdConfig, RgsConfig, SpecsConfig, StackConfig,
};
use crate::stack;
use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, Input};
use reqwest::blocking::Client;
use std::fs;
use std::path::{Path, PathBuf};

pub fn run(client: &Client, force: bool, default: bool) -> Result<()> {
    println!("🚀 Initialisation de OpenSecKit V3...");

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
                if default {
                    default_configuration()
                } else {
                    prompt_configuration().unwrap()
                }
            })
    } else if default {
        default_configuration()
    } else {
        prompt_configuration()?
    };

    scaffold_project(&config, force)?;

    install_resources(client, force)?;
    install_slash_commands(force)?;

    println!("\n✅ OpenSecKit V3 initialisé !");
    println!("\n📂 Structure créée:");
    println!("   .osk/config.toml     - Configuration projet et stack");
    println!("   .osk/memory/         - Mémoire contextuelle");
    println!("   .osk/specs/          - Spécifications par feature");
    println!("\n📂 Slash commands disponibles dans .claude/commands/:");
    println!("\n   Workflow principal:");
    println!("   /osk-configure           - Configuration intelligente (analyse code, détection domaines)");
    println!("   /osk-baseline            - État des lieux sécurité (projets existants)");
    println!("   /osk-analyze <feature>   - Analyse menaces et risques (Principes I & II)");
    println!("   /osk-specify <feature>   - Exigences et tests (Principes III & IV)");
    println!("   /osk-harden <feature>    - Durcissement (Principes V, VI & VII)");
    println!("   /osk-plan <feature>      - Plan d'implémentation consolidé");
    println!("   /osk-tasks <feature>     - Génération des tâches ordonnées");
    println!("\n   Domaines réglementaires:");
    println!("   /osk-rgpd                - Conformité RGPD et registre des traitements");
    println!("   /osk-rgs                 - Conformité RGS et EBIOS RM");
    println!("\n   Monitoring et continuité:");
    println!("   /osk-dashboard           - Vue consolidée des métriques");
    println!("   /osk-pca-pra             - Plans de continuité et reprise");
    println!("\n💡 Prochaines étapes:");
    println!("   1. Lancez Claude Code: claude");
    println!("   2. Exécutez /osk-configure pour analyser votre code");
    println!("   3. Pour un projet existant: /osk-baseline");
    println!("   4. Pour une nouvelle feature: /osk-analyze \"nom-feature\"");

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
        .with_initial_text(detected_stack.clone())
        .allow_empty(true)
        .interact_text()?;

    let detected_vec: Vec<String> = if detected_stack.trim().is_empty() {
        vec![]
    } else {
        detected_stack
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    };

    let custom_vec: Vec<String> = if final_stack.trim() == detected_stack.trim() {
        vec![]
    } else {
        final_stack
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty() && !detected_vec.contains(&s.to_string()))
            .collect()
    };

    let config = OskConfig {
        project: Some(ProjectConfig {
            name: project_name,
            version: None,
            description: None,
        }),
        stack: Some(StackConfig {
            detected: detected_vec,
            custom: custom_vec,
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
    };

    Ok(config)
}

/// Configuration par défaut sans prompts interactifs (pour CI/tests)
fn default_configuration() -> OskConfig {
    println!("   🤖 Mode non-interactif: utilisation des valeurs par défaut");

    println!("   🕵️  Scan de la stack technique (Monorepo supporté)...");
    let detected_stack = stack::detect();

    let detected_vec: Vec<String> = if detected_stack.trim().is_empty() {
        vec![]
    } else {
        detected_stack
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect()
    };

    OskConfig {
        project: Some(ProjectConfig {
            name: "MonProjet".to_string(),
            version: None,
            description: None,
        }),
        stack: Some(StackConfig {
            detected: detected_vec,
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

fn scaffold_project(config: &OskConfig, force: bool) -> Result<()> {
    fs::create_dir_all(".osk/prompts")?;
    fs::create_dir_all(".osk/templates")?;
    fs::create_dir_all(".osk/memory")?;
    fs::create_dir_all(".osk/specs")?;
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
