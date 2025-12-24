//! Auto-increment counters for features and incidents

use anyhow::Result;
use regex::Regex;
use std::fs;
use std::path::Path;

pub fn next_feature_number() -> Result<u32> {
    next_feature_number_in(Path::new("."))
}

fn next_feature_number_in(base: &Path) -> Result<u32> {
    let specs_dir = base.join(".osk/specs");

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

pub fn next_incident_number(date: &str) -> Result<u32> {
    next_incident_number_in(Path::new("."), date)
}

fn next_incident_number_in(base: &Path, date: &str) -> Result<u32> {
    let incidents_dir = base.join("docs/security/incidents");

    if !incidents_dir.exists() {
        return Ok(1);
    }

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
    use tempfile::tempdir;

    #[test]
    fn test_slugify() {
        assert_eq!(slugify("User Authentication"), "user-authentication");
        assert_eq!(slugify("API v2.0"), "api-v2-0");
        assert_eq!(slugify("  spaces  "), "spaces");
    }

    #[test]
    fn test_next_feature_number_empty() {
        let dir = tempdir().unwrap();
        let num = next_feature_number_in(dir.path()).unwrap();
        assert_eq!(num, 1);
    }

    #[test]
    fn test_next_feature_number_with_existing() {
        let dir = tempdir().unwrap();
        let base = dir.path();

        fs::create_dir_all(base.join(".osk/specs/001-auth")).unwrap();
        fs::create_dir_all(base.join(".osk/specs/002-payment")).unwrap();
        fs::create_dir_all(base.join(".osk/specs/005-logging")).unwrap();

        let num = next_feature_number_in(base).unwrap();
        assert_eq!(num, 6);
    }
}
