//! Feature directory helpers

use anyhow::Result;
use regex::Regex;
use std::fs;
use std::path::Path;

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

pub fn slugify(name: &str) -> String {
    name.to_lowercase()
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '-' })
        .collect::<String>()
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("User Authentication"), "user-authentication");
        assert_eq!(slugify("API v2.0"), "api-v2-0");
        assert_eq!(slugify("  spaces  "), "spaces");
    }

    #[test]
    fn test_slugify_special_chars() {
        assert_eq!(slugify("Test@#$%Feature"), "test-feature");
        assert_eq!(slugify("___underscores___"), "underscores");
        assert_eq!(slugify(""), "");
    }
}
