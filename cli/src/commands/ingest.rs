use anyhow::{Context, Result};
use ignore::WalkBuilder;
use std::fs::{self, File};
use std::io::{Write, BufWriter};
use std::path::Path;

pub fn run(output_path: &str, show_stats: bool) -> Result<()> {
    println!("🚜 Ingestion de la base de code locale...");
    
    let file = File::create(output_path).context("Impossible de créer le fichier de sortie")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "# RESUME DE L'ARCHITECTURE (Arborescence)\n")?;
    writeln!(writer, "```text")?;
    
    let walker = WalkBuilder::new("./").hidden(false).git_ignore(true).build();
    let mut file_paths: Vec<String> = Vec::new();

    for result in walker {
        match result {
            Ok(entry) => {
                let path = entry.path();
                
                if path.to_string_lossy().contains(".git") || path.ends_with(output_path) {
                    continue;
                }
                
                let depth = path.components().count();
                if depth > 0 {
                    let indent = "  ".repeat(depth - 1);
                    let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                    writeln!(writer, "{}- {}", indent, file_name)?;
                }

                if path.is_file() && !is_binary(path) {
                    file_paths.push(path.to_string_lossy().to_string());
                }
            }
            Err(err) => eprintln!("Erreur de lecture: {}", err),
        }
    }
    writeln!(writer, "```\n")?;

    writeln!(writer, "# CONTENU DES FICHIERS\n")?;
    
    let mut total_chars = 0;

    for path_str in file_paths {
        writeln!(writer, "## Fichier : {}\n", path_str)?;
        writeln!(writer, "```")?;
        
        match fs::read_to_string(&path_str) {
            Ok(content) => {
                writeln!(writer, "{}", content)?;
                total_chars += content.len();
            }
            Err(_) => writeln!(writer, "[Erreur de lecture ou fichier binaire]")?,
        }
        
        writeln!(writer, "```\n")?;
    }

    writer.flush()?;
    
    println!("✅ Codebase consolidée dans : {}", output_path);
    if show_stats {
        println!("📊 Taille : ~{} caractères", total_chars);
        println!("🧠 Tokens estimés : ~{}k tokens", total_chars / 4000);
    }

    Ok(())
}

fn is_binary(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        return matches!(ext_str.as_str(), 
            "png" | "jpg" | "jpeg" | "gif" | "ico" | "pdf" | "zip" | "exe" | "bin" | "lock"
        );
    }
    false
}