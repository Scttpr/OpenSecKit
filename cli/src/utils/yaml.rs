//! YAML validation helpers

use anyhow::{Context, Result};
use std::fs;

pub fn validate(path: &str) -> Result<bool> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Cannot read {}", path))?;

    let _: serde_yaml::Value = serde_yaml::from_str(&content)
        .with_context(|| format!("Invalid YAML in {}", path))?;

    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

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
        fs::write(&path, "key: [unclosed").unwrap();
        assert!(validate(path.to_str().unwrap()).is_err());
    }
}
