use anyhow::{Context, Result};
use ignore::WalkBuilder;
use std::fs::{self, File};
use std::io::{BufWriter, Write};
use std::path::Path;

pub fn run(output_path: &str, show_stats: bool, base_path: &str) -> Result<()> {
    println!("🚜 Ingestion de la base de code depuis : '{base_path}'...");

    let file = File::create(output_path).context("Impossible de créer le fichier de sortie")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "# RESUME DE L'ARCHITECTURE (Arborescence)\n")?;
    writeln!(writer, "```text")?;

    let walker = WalkBuilder::new(base_path)
        .hidden(false)
        .git_ignore(true)
        .build();

    let mut file_paths: Vec<String> = Vec::new();

    for result in walker {
        match result {
            Ok(entry) => {
                let path = entry.path();

                // On évite de s'auto-ingérer ou de lire .git
                if path.to_string_lossy().contains(".git")
                    || path.to_string_lossy().ends_with(output_path)
                {
                    continue;
                }

                let relative_path = path.strip_prefix(base_path).unwrap_or(path);
                let depth = relative_path.components().count();

                if depth > 0 {
                    let indent = "  ".repeat(depth - 1);
                    let file_name = path.file_name().unwrap_or_default().to_string_lossy();
                    writeln!(writer, "{indent}- {file_name}")?;
                }

                if path.is_file() && !is_binary(path) {
                    file_paths.push(path.to_string_lossy().to_string());
                }
            }
            Err(err) => eprintln!("Erreur de lecture: {err}"),
        }
    }
    writeln!(writer, "```\n")?;

    writeln!(writer, "# CONTENU DES FICHIERS\n")?;

    let mut total_chars = 0;
    for path_str in file_paths {
        writeln!(writer, "## Fichier : {path_str}\n")?;
        writeln!(writer, "```")?;

        match fs::read_to_string(&path_str) {
            Ok(content) => {
                writeln!(writer, "{content}")?;
                total_chars += content.len();
            }
            Err(_) => writeln!(writer, "[Erreur de lecture ou fichier binaire]")?,
        }

        writeln!(writer, "```\n")?;
    }

    writer.flush()?;

    println!("✅ Codebase consolidée dans : {output_path}");
    if show_stats {
        println!("📊 Taille : ~{total_chars} caractères");
        let estimated_tokens = (total_chars as f64 / 3.5) as usize;
        println!("🧠 Tokens estimés : ~{estimated_tokens} tokens");
    }

    Ok(())
}

fn is_binary(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        return matches!(
            ext_str.as_str(),
            "png"
                | "jpg"
                | "jpeg"
                | "gif"
                | "ico"
                | "svg"
                | "webp"
                | "zip"
                | "tar"
                | "gz"
                | "7z"
                | "rar"
                | "exe"
                | "bin"
                | "dll"
                | "so"
                | "dylib"
                | "class"
                | "pyc"
                | "o"
                | "obj"
                | "pdf"
                | "docx"
                | "xlsx"
                | "pptx"
                | "woff"
                | "woff2"
                | "ttf"
                | "eot"
                | "db"
                | "sqlite"
                | "lock"
        );
    }
    false
}
