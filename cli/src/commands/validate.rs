//! Validate command - validates system model

use anyhow::{bail, Result};
use serde::Serialize;
use std::collections::HashSet;
use std::fs;
use std::path::Path;

#[derive(Serialize)]
pub struct ValidationResult {
    pub valid: bool,
    pub path: String,
    pub schema_errors: usize,
    pub reference_errors: usize,
    pub warnings: usize,
    pub errors: Vec<ValidationError>,
    pub checked_files: Vec<FileStatus>,
}

#[derive(Serialize)]
pub struct FileStatus {
    pub path: String,
    pub status: String,
    pub error: Option<String>,
}

#[derive(Serialize)]
pub struct ValidationError {
    pub file: String,
    pub line: Option<usize>,
    pub message: String,
    pub severity: String,
    pub suggestion: Option<String>,
}

/// Validate system model files
/// - Schema validation for all YAML files
/// - Cross-reference validation (check all referenced IDs exist)
/// - Index.yaml line count check (<200 lines)
pub fn run(model_path: &str, json: bool) -> Result<()> {
    let model_dir = Path::new(model_path);
    if !model_dir.exists() {
        bail!(
            "System model not found at {}. Run `/osk-discover` first.",
            model_path
        );
    }

    let mut checked_files = Vec::new();
    let mut errors: Vec<ValidationError> = Vec::new();
    let mut all_ids: HashSet<String> = HashSet::new();
    let mut referenced_ids: Vec<(String, String)> = Vec::new();

    // List of expected section files
    let section_files = [
        "index.yaml",
        "business.yaml",
        "architecture.yaml",
        "data.yaml",
        "actors.yaml",
        "boundaries.yaml",
        "integrations.yaml",
        "controls.yaml",
        "tooling.yaml",
        "team.yaml",
        "gaps.yaml",
    ];

    // Validate each section file
    for filename in &section_files {
        let file_path = model_dir.join(filename);
        let path_str = file_path.to_string_lossy().to_string();

        if !file_path.exists() {
            // Optional files - just note as missing
            checked_files.push(FileStatus {
                path: path_str.clone(),
                status: "missing".to_string(),
                error: None,
            });
            continue;
        }

        // Read file
        let content = match fs::read_to_string(&file_path) {
            Ok(c) => c,
            Err(e) => {
                checked_files.push(FileStatus {
                    path: path_str.clone(),
                    status: "invalid".to_string(),
                    error: Some(format!("Cannot read file: {}", e)),
                });
                continue;
            }
        };

        // Check line count for index.yaml (<200 lines)
        if *filename == "index.yaml" {
            let line_count = content.lines().count();
            if line_count > 200 {
                errors.push(ValidationError {
                    file: filename.to_string(),
                    line: Some(200),
                    message: format!("Index file exceeds 200 lines ({} lines)", line_count),
                    severity: "error".to_string(),
                    suggestion: Some(
                        "Move detailed data to section files, keep only summaries in index"
                            .to_string(),
                    ),
                });
            }
        }

        // Parse YAML
        let yaml_value: serde_yaml::Value = match serde_yaml::from_str(&content) {
            Ok(v) => v,
            Err(e) => {
                checked_files.push(FileStatus {
                    path: path_str.clone(),
                    status: "invalid".to_string(),
                    error: Some(format!("YAML parse error: {}", e)),
                });
                errors.push(ValidationError {
                    file: filename.to_string(),
                    line: None,
                    message: format!("Invalid YAML: {}", e),
                    severity: "error".to_string(),
                    suggestion: Some("Check YAML syntax".to_string()),
                });
                continue;
            }
        };

        // Extract IDs and references
        extract_ids_and_refs(&yaml_value, filename, &mut all_ids, &mut referenced_ids);

        checked_files.push(FileStatus {
            path: path_str,
            status: "valid".to_string(),
            error: None,
        });
    }

    // Cross-reference validation
    for (file, ref_id) in &referenced_ids {
        if !all_ids.contains(ref_id) && !ref_id.starts_with("DATA-") && !ref_id.starts_with("COMP-")
        {
            // Skip well-known ID patterns that might be placeholders
            if !ref_id.contains('[') && !ref_id.is_empty() {
                errors.push(ValidationError {
                    file: file.clone(),
                    line: None,
                    message: format!("Referenced ID '{}' not found", ref_id),
                    severity: "warning".to_string(),
                    suggestion: Some(format!(
                        "Add definition for '{}' or remove reference",
                        ref_id
                    )),
                });
            }
        }
    }

    let error_count = errors.iter().filter(|e| e.severity == "error").count();
    let warning_count = errors.iter().filter(|e| e.severity == "warning").count();
    let valid = error_count == 0;

    if json {
        let result = ValidationResult {
            valid,
            path: model_path.to_string(),
            schema_errors: error_count,
            reference_errors: warning_count,
            warnings: warning_count,
            errors,
            checked_files,
        };
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("Validating system model at {}...\n", model_path);

        for file in &checked_files {
            let filename = file.path.split('/').next_back().unwrap_or(&file.path);
            match file.status.as_str() {
                "valid" => println!("  ✓ {}", filename),
                "missing" => println!("  - {} (optional)", filename),
                _ => println!(
                    "  ✗ {} ({})",
                    filename,
                    file.error.as_deref().unwrap_or("invalid")
                ),
            }
        }

        if !errors.is_empty() {
            println!("\nIssues found:");
            for err in &errors {
                let icon = if err.severity == "error" { "✗" } else { "⚠" };
                println!("  {} [{}] {}", icon, err.file, err.message);
                if let Some(ref suggestion) = err.suggestion {
                    println!("    → {}", suggestion);
                }
            }
        }

        println!("\nResult: {} errors, {} warnings", error_count, warning_count);
        if valid {
            println!("✓ System model is valid");
        } else {
            println!("✗ System model has validation errors");
        }
    }

    if !valid {
        bail!("System model validation failed with {} errors", error_count)
    }
    Ok(())
}

/// Extract IDs and references from YAML structure
fn extract_ids_and_refs(
    value: &serde_yaml::Value,
    filename: &str,
    all_ids: &mut HashSet<String>,
    referenced_ids: &mut Vec<(String, String)>,
) {
    match value {
        serde_yaml::Value::Mapping(map) => {
            for (key, val) in map {
                if let serde_yaml::Value::String(key_str) = key {
                    // Collect IDs
                    if key_str == "id" {
                        if let serde_yaml::Value::String(id) = val {
                            all_ids.insert(id.clone());
                        }
                    }
                    // Collect references (common reference field names)
                    if key_str == "trust_zone"
                        || key_str == "component"
                        || key_str == "actor"
                        || key_str == "from"
                        || key_str == "to"
                        || key_str == "data"
                        || key_str == "integration"
                        || key_str == "zone"
                    {
                        if let serde_yaml::Value::String(ref_id) = val {
                            referenced_ids.push((filename.to_string(), ref_id.clone()));
                        }
                    }
                    // Handle arrays of references
                    if key_str == "actors"
                        || key_str == "data_created"
                        || key_str == "stores"
                        || key_str == "components"
                        || key_str == "data_flows"
                        || key_str == "data_shared"
                    {
                        if let serde_yaml::Value::Sequence(arr) = val {
                            for item in arr {
                                if let serde_yaml::Value::String(ref_id) = item {
                                    referenced_ids.push((filename.to_string(), ref_id.clone()));
                                }
                            }
                        }
                    }
                }
                extract_ids_and_refs(val, filename, all_ids, referenced_ids);
            }
        }
        serde_yaml::Value::Sequence(arr) => {
            for item in arr {
                extract_ids_and_refs(item, filename, all_ids, referenced_ids);
            }
        }
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_validate_missing_model() {
        let result = run("/nonexistent/path", false);
        assert!(result.is_err());
    }

    #[test]
    fn test_validate_empty_model() {
        let dir = tempdir().unwrap();
        let model_path = dir.path().join("system-model");
        fs::create_dir_all(&model_path).unwrap();

        let result = run(model_path.to_str().unwrap(), false);
        // Should succeed with all files missing (they're optional)
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_valid_index() {
        let dir = tempdir().unwrap();
        let model_path = dir.path().join("system-model");
        fs::create_dir_all(&model_path).unwrap();

        let index = r#"
version: "1.0"
name: Test System
components: []
"#;
        fs::write(model_path.join("index.yaml"), index).unwrap();

        let result = run(model_path.to_str().unwrap(), false);
        assert!(result.is_ok());
    }

    #[test]
    fn test_validate_invalid_yaml() {
        let dir = tempdir().unwrap();
        let model_path = dir.path().join("system-model");
        fs::create_dir_all(&model_path).unwrap();

        fs::write(model_path.join("index.yaml"), "invalid: yaml: syntax:").unwrap();

        let result = run(model_path.to_str().unwrap(), false);
        assert!(result.is_err());
    }
}
