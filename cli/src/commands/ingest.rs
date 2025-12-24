use anyhow::{Context, Result};
use ignore::WalkBuilder;
use std::fs::{self, File};
use std::io::{BufWriter, Read, Write};
use std::path::Path;

pub fn get_file_tree(base_path: &str) -> String {
    let mut tree = String::new();
    let walker = WalkBuilder::new(base_path)
        .hidden(false)
        .git_ignore(true)
        .build();

    for entry in walker.flatten() {
        let path = entry.path();
        if path.to_string_lossy().contains(".git")
            || path.to_string_lossy().contains(".osk")
            || path.is_dir()
        {
            continue;
        }

        let relative = path.to_string_lossy();
        tree.push_str(&format!("- {relative}\n"));
    }
    tree
}

pub fn get_files_content(files: &[String]) -> String {
    let mut content = String::new();

    for file_path in files {
        let path = Path::new(file_path);
        content.push_str(&format!("\n## Fichier : {file_path}\n```\n"));

        if is_binary(path) {
            content.push_str("[Fichier binaire ou trop volumineux - Contenu masqué]");
        } else {
            match fs::read_to_string(file_path) {
                Ok(c) => content.push_str(&c),
                Err(_) => content.push_str("[Erreur de lecture]"),
            }
        }

        content.push_str("\n```\n");
    }
    content
}

pub fn run(output_path: &str, _stats: bool, base_path: &str) -> Result<()> {
    println!("📦 Génération du contexte statique...");

    let tree = get_file_tree(base_path);

    let walker = WalkBuilder::new(base_path)
        .hidden(false)
        .git_ignore(true)
        .build();

    let mut all_files = Vec::new();

    for entry in walker.flatten() {
        let path = entry.path();

        if path.is_file()
            && !path.to_string_lossy().contains(".git")
            && !path.to_string_lossy().contains(".osk")
            && !path.to_string_lossy().ends_with(output_path)
            && !is_binary(path)
        {
            all_files.push(path.to_string_lossy().to_string());
        }
    }

    let content = get_files_content(&all_files);

    let file = File::create(output_path).context("Impossible de créer le fichier de sortie")?;
    let mut writer = BufWriter::new(file);

    writeln!(writer, "# ARBORESCENCE\n\n{tree}\n\n# CONTENU\n\n{content}")?;

    println!("✅ Contexte exporté avec succès vers {output_path}");
    Ok(())
}

fn is_binary(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext_str = ext.to_string_lossy().to_lowercase();
        let binary_extensions = [
            "png", "jpg", "jpeg", "gif", "ico", "svg", "webp", "avif", "zip", "tar", "gz", "7z",
            "rar", "jar", "exe", "bin", "dll", "so", "dylib", "class", "pyc", "o", "obj", "pdf",
            "docx", "xlsx", "pptx", "db", "sqlite", "lock", "wasm",
        ];

        if binary_extensions.contains(&ext_str.as_str()) {
            return true;
        }
    }

    let mut file = match File::open(path) {
        Ok(f) => f,
        Err(_) => return false,
    };

    let mut buffer = [0; 8192];
    let n = match file.read(&mut buffer) {
        Ok(n) => n,
        Err(_) => return false,
    };

    buffer[..n].contains(&0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_is_binary_by_extension() {
        let dir = tempdir().unwrap();

        let png_path = dir.path().join("test.png");
        fs::write(&png_path, "fake content").unwrap();
        assert!(is_binary(&png_path));

        let txt_path = dir.path().join("test.txt");
        fs::write(&txt_path, "hello world").unwrap();
        assert!(!is_binary(&txt_path));
    }

    #[test]
    fn test_is_binary_by_content() {
        let dir = tempdir().unwrap();

        let binary_path = dir.path().join("test.dat");
        fs::write(&binary_path, &[0x00, 0x01, 0x02, 0x03]).unwrap();
        assert!(is_binary(&binary_path));

        let text_path = dir.path().join("test.log");
        fs::write(&text_path, "just text").unwrap();
        assert!(!is_binary(&text_path));
    }

    #[test]
    fn test_get_file_tree() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::create_dir_all(base.join("src")).unwrap();
        fs::write(base.join("src/main.rs"), "fn main() {}").unwrap();
        fs::write(base.join("README.md"), "# Test").unwrap();

        let tree = get_file_tree(&base.to_string_lossy());

        assert!(tree.contains("src/main.rs") || tree.contains("main.rs"));
        assert!(tree.contains("README.md"));
    }

    #[test]
    fn test_get_file_tree_excludes_git() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::create_dir_all(base.join(".git/objects")).unwrap();
        fs::write(base.join(".git/config"), "git config").unwrap();
        fs::write(base.join("code.rs"), "fn test() {}").unwrap();

        let tree = get_file_tree(&base.to_string_lossy());

        assert!(!tree.contains(".git"));
        assert!(tree.contains("code.rs"));
    }

    #[test]
    fn test_get_files_content() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        let file1 = base.join("file1.txt");
        let file2 = base.join("file2.txt");
        fs::write(&file1, "content one").unwrap();
        fs::write(&file2, "content two").unwrap();

        let content = get_files_content(&[
            file1.to_string_lossy().to_string(),
            file2.to_string_lossy().to_string(),
        ]);

        assert!(content.contains("content one"));
        assert!(content.contains("content two"));
        assert!(content.contains("## Fichier :"));
    }

    #[test]
    fn test_get_files_content_binary() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        let binary_file = base.join("image.png");
        fs::write(&binary_file, &[0x89, 0x50, 0x4E, 0x47]).unwrap();

        let content = get_files_content(&[binary_file.to_string_lossy().to_string()]);

        assert!(content.contains("[Fichier binaire ou trop volumineux"));
    }
}
