use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Serialize, Deserialize)]
struct MemoryEntry {
    timestamp: u64,
    task: String,
    response: String,
}

pub struct MemoryManager {
    file_path: PathBuf,
}

impl MemoryManager {
    pub fn new(base_path: &str) -> Self {
        let dir = PathBuf::from(base_path);
        // On s'assure que le dossier existe
        fs::create_dir_all(&dir).unwrap_or_default();

        Self {
            file_path: dir.join("history.jsonl"),
        }
    }

    pub fn save(&self, task: &str, response: &str) -> Result<()> {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let entry = MemoryEntry {
            timestamp,
            task: task.to_string(),
            response: response.to_string(),
        };

        let json_line = serde_json::to_string(&entry).context("Erreur de sérialisation JSON")?;

        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .context("Impossible d'ouvrir le fichier d'historique")?;

        writeln!(file, "{}", json_line)?;

        println!("   💾 Mémoire mise à jour (1 entrée ajoutée).");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_memory_persistence() -> Result<()> {
        let dir = tempdir()?;
        let path_str = dir.path().to_string_lossy().to_string();

        let mgr = MemoryManager::new(&path_str);

        // 1. Sauvegarde
        mgr.save("Tâche 1", "Réponse 1")?;
        mgr.save("Tâche 2", "Réponse 2")?;

        // 2. Vérification du fichier physique
        let file_path = dir.path().join("history.jsonl");
        assert!(file_path.exists());

        let content = fs::read_to_string(&file_path)?;
        assert_eq!(content.lines().count(), 2);

        // 3. Rechargement
        let history = mgr.load_history()?;
        assert!(history.contains("Tâche 1"));
        assert!(history.contains("Réponse 2"));

        Ok(())
    }

    #[test]
    fn test_memory_limit() -> Result<()> {
        let dir = tempdir()?;
        let mgr = MemoryManager::new(&dir.path().to_string_lossy());

        for i in 0..10 {
            mgr.save(&format!("Task {i}"), "Resp")?;
        }

        let history = mgr.load_history()?;

        assert!(history.contains("Task 9"));
        assert!(!history.contains("Task 0"));

        Ok(())
    }
}
