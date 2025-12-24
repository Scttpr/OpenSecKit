//! Gestion des compteurs auto-incrémentés
//!
//! Gère les compteurs pour :
//! - Numéros de features (NNN dans .osk/specs/NNN-name/)
//! - Numéros d'incidents (INC-DATE-NNN)

use anyhow::{Context, Result};
use regex::Regex;
use std::fs;
use std::path::Path;

/// Trouve le prochain numéro de feature disponible
///
/// Scanne .osk/specs/ pour trouver le plus grand NNN existant
/// et retourne NNN + 1
pub fn next_feature_number() -> Result<u32> {
    let specs_dir = Path::new(".osk/specs");

    if !specs_dir.exists() {
        return Ok(1);
    }

    let re = Regex::new(r"^(\d{3})-")?;
    let mut max_num: u32 = 0;

    for entry in fs::read_dir(specs_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            if let Some(caps) = re.captures(&name_str) {
                if let Some(num_str) = caps.get(1) {
                    if let Ok(num) = num_str.as_str().parse::<u32>() {
                        if num > max_num {
                            max_num = num;
                        }
                    }
                }
            }
        }
    }

    Ok(max_num + 1)
}

/// Trouve le prochain numéro d'incident pour une date donnée
///
/// Scanne docs/security/incidents/ pour trouver le plus grand
/// numéro pour la date spécifiée
pub fn next_incident_number(date: &str) -> Result<u32> {
    let incidents_dir = Path::new("docs/security/incidents");

    if !incidents_dir.exists() {
        return Ok(1);
    }

    // Pattern: INC-2025-12-24-001.md
    let pattern = format!(r"^INC-{}-(\d{{3}})\.md$", regex::escape(date));
    let re = Regex::new(&pattern)?;
    let mut max_num: u32 = 0;

    for entry in fs::read_dir(incidents_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_file() {
            let name = entry.file_name();
            let name_str = name.to_string_lossy();

            if let Some(caps) = re.captures(&name_str) {
                if let Some(num_str) = caps.get(1) {
                    if let Ok(num) = num_str.as_str().parse::<u32>() {
                        if num > max_num {
                            max_num = num;
                        }
                    }
                }
            }
        }
    }

    Ok(max_num + 1)
}

/// Trouve le répertoire d'une feature par son nom
///
/// Cherche dans .osk/specs/ un dossier matching NNN-{name}
pub fn find_feature_dir(name: &str) -> Result<Option<String>> {
    let specs_dir = Path::new(".osk/specs");

    if !specs_dir.exists() {
        return Ok(None);
    }

    let slug = slugify(name);
    let re = Regex::new(&format!(r"^\d{{3}}-{}$", regex::escape(&slug)))?;

    for entry in fs::read_dir(specs_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let dir_name = entry.file_name();
            let dir_name_str = dir_name.to_string_lossy();

            if re.is_match(&dir_name_str) {
                return Ok(Some(format!(".osk/specs/{}", dir_name_str)));
            }
        }
    }

    // Essayer aussi une recherche partielle
    for entry in fs::read_dir(specs_dir)? {
        let entry = entry?;
        if entry.file_type()?.is_dir() {
            let dir_name = entry.file_name();
            let dir_name_str = dir_name.to_string_lossy();

            if dir_name_str.to_lowercase().contains(&slug.to_lowercase()) {
                return Ok(Some(format!(".osk/specs/{}", dir_name_str)));
            }
        }
    }

    Ok(None)
}

/// Convertit un nom en slug (minuscules, tirets)
pub fn slugify(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| {
            if c.is_alphanumeric() {
                c
            } else {
                '-'
            }
        })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

/// Formate un numéro de feature sur 3 chiffres
pub fn format_feature_number(num: u32) -> String {
    format!("{:03}", num)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("User Authentication"), "user-authentication");
        assert_eq!(slugify("API v2.0"), "api-v2-0");
        assert_eq!(slugify("  spaces  "), "spaces");
    }

    #[test]
    fn test_format_feature_number() {
        assert_eq!(format_feature_number(1), "001");
        assert_eq!(format_feature_number(42), "042");
        assert_eq!(format_feature_number(100), "100");
    }

    #[test]
    fn test_next_feature_number_empty() {
        let dir = tempdir().unwrap();
        std::env::set_current_dir(&dir).unwrap();

        // Pas de dossier .osk/specs
        let num = next_feature_number().unwrap();
        assert_eq!(num, 1);
    }

    #[test]
    fn test_next_feature_number_with_existing() {
        let dir = tempdir().unwrap();
        std::env::set_current_dir(&dir).unwrap();

        fs::create_dir_all(".osk/specs/001-auth").unwrap();
        fs::create_dir_all(".osk/specs/002-payment").unwrap();
        fs::create_dir_all(".osk/specs/005-logging").unwrap();

        let num = next_feature_number().unwrap();
        assert_eq!(num, 6);
    }
}
