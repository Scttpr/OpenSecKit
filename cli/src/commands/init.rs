use crate::github;
use anyhow::Result;
use reqwest::blocking::Client;
use std::path::{Path, PathBuf};
use std::fs;
use std::io::Write;

pub fn run(client: &Client, force: bool) -> Result<()> {
    println!("🚀 Initialisation de OpenSecKit (Structure .osk)...");

    let tag = github::fetch_latest_tag(client)?;
    println!("📦 Version cible : {}", tag);

    println!("🔍 Analyse du dépôt distant...");
    let tree_items = github::fetch_repo_tree(client, &tag)?;

    let mut download_count = 0;

    fs::create_dir_all(".claude/commands")?;
    fs::create_dir_all(".osk")?;

    for item in tree_items {
        if item.item_type != "blob" { continue; }

        let remote_path = &item.path;
        let mut local_dest: Option<PathBuf> = None;

        if remote_path.starts_with("prompts/") {
            let filename = Path::new(remote_path).file_name().unwrap();
            local_dest = Some(PathBuf::from(".claude/commands").join(filename));
        }
        else if remote_path == "constitution.md" {
            local_dest = Some(PathBuf::from(".osk/constitution.md"));
        }
        else if remote_path.starts_with("templates/") || remote_path.starts_with("domaines/") {
            local_dest = Some(PathBuf::from(".osk").join(remote_path));
        }
        
        if let Some(local_path) = local_dest {
            if local_path.exists() && !force { continue; }
            github::download_file(client, &tag, remote_path, &local_path)?;
            download_count += 1;
        }
    }

    setup_gitignore()?;

    println!("\n✅ Terminé ! {} fichiers installés.", download_count);
    println!("   📂 Vos standards sont dans : .osk/");
    println!("   🤖 Vos agents sont prêts : essayez 'claude run /osk-analyze'");

    Ok(())
}

fn setup_gitignore() -> Result<()> {
    let gitignore_path = Path::new(".gitignore");
    
    if !gitignore_path.exists() {
        let mut file = fs::File::create(gitignore_path)?;
        writeln!(file, ".claude/")?;
        writeln!(file, "osk")?;
        println!("   📝 .gitignore créé (ignore .claude et osk)");
    }

    Ok(())
}