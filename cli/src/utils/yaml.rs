//! Helpers pour la manipulation de fichiers YAML
//!
//! Fournit des fonctions pour lire, écrire et manipuler
//! les fichiers YAML du projet (risk-register, tasks, etc.)

use anyhow::{Context, Result};
use serde::{de::DeserializeOwned, Serialize};
use std::fs;
use std::path::Path;

/// Charge un fichier YAML et le désérialise
pub fn load<T: DeserializeOwned>(path: &str) -> Result<T> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Impossible de lire {}", path))?;

    serde_yaml::from_str(&content)
        .with_context(|| format!("Erreur de parsing YAML dans {}", path))
}

/// Sauvegarde une structure en YAML
pub fn save<T: Serialize>(path: &str, data: &T) -> Result<()> {
    // Créer le dossier parent si nécessaire
    if let Some(parent) = Path::new(path).parent() {
        fs::create_dir_all(parent)?;
    }

    let content = serde_yaml::to_string(data)
        .context("Erreur de sérialisation YAML")?;

    fs::write(path, content)
        .with_context(|| format!("Impossible d'écrire {}", path))
}

/// Vérifie si un fichier YAML est valide
pub fn validate(path: &str) -> Result<bool> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Impossible de lire {}", path))?;

    // Essayer de parser comme valeur YAML générique
    let _: serde_yaml::Value = serde_yaml::from_str(&content)
        .with_context(|| format!("YAML invalide dans {}", path))?;

    Ok(true)
}

/// Lit une valeur spécifique depuis un fichier YAML
pub fn get_value(path: &str, key_path: &str) -> Result<Option<serde_yaml::Value>> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Impossible de lire {}", path))?;

    let root: serde_yaml::Value = serde_yaml::from_str(&content)
        .with_context(|| format!("Erreur de parsing YAML dans {}", path))?;

    // Naviguer dans le chemin de clés (ex: "stats.total")
    let keys: Vec<&str> = key_path.split('.').collect();
    let mut current = &root;

    for key in keys {
        match current {
            serde_yaml::Value::Mapping(map) => {
                if let Some(value) = map.get(serde_yaml::Value::String(key.to_string())) {
                    current = value;
                } else {
                    return Ok(None);
                }
            }
            _ => return Ok(None),
        }
    }

    Ok(Some(current.clone()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tempfile::tempdir;

    #[test]
    fn test_save_and_load() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("test.yaml");
        let path_str = path.to_str().unwrap();

        let mut data: HashMap<String, String> = HashMap::new();
        data.insert("key".to_string(), "value".to_string());

        save(path_str, &data).unwrap();
        let loaded: HashMap<String, String> = load(path_str).unwrap();

        assert_eq!(loaded.get("key"), Some(&"value".to_string()));
    }

    #[test]
    fn test_validate_valid_yaml() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("valid.yaml");
        fs::write(&path, "key: value\nlist:\n  - item1\n  - item2").unwrap();

        assert!(validate(path.to_str().unwrap()).unwrap());
    }

    #[test]
    fn test_validate_invalid_yaml() {
        let dir = tempdir().unwrap();
        let path = dir.path().join("invalid.yaml");
        fs::write(&path, "key: value\n  bad indent").unwrap();

        assert!(validate(path.to_str().unwrap()).is_err());
    }
}
