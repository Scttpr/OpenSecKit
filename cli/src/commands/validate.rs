//! Validate command - checks file consistency

use crate::args::ValidateCommands;
use crate::utils::{counter, yaml};
use anyhow::{bail, Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::{HashMap, HashSet};
use std::fs;

#[derive(Serialize)]
pub struct ValidateResult {
    pub valid: bool,
    pub command: String,
    pub checked_files: Vec<FileStatus>,
    pub error_count: usize,
    pub next_command: Option<String>,
    pub message: String,
}

#[derive(Serialize)]
pub struct FileStatus {
    pub path: String,
    pub status: String,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
struct TasksFile {
    #[serde(default)]
    tasks: Vec<Task>,
}

#[derive(Debug, Deserialize)]
struct Task {
    id: String,
    #[serde(default)]
    depends_on: Vec<String>,
}

pub fn run(command: ValidateCommands, json: bool) -> Result<()> {
    match command {
        ValidateCommands::Yaml => validate_yaml(json),
        ValidateCommands::Deps { feature } => validate_deps(&feature, json),
        ValidateCommands::Workflow { feature } => validate_workflow(&feature, json),
        ValidateCommands::SystemModel { path } => {
            validate_system_model(path.as_deref().unwrap_or(".osk/system-model"), json)
        }
    }
}

fn validate_yaml(json: bool) -> Result<()> {
    let mut checked_files = Vec::new();

    let register_path = "docs/security/risks/risk-register.yaml";
    if std::path::Path::new(register_path).exists() {
        match yaml::validate(register_path) {
            Ok(_) => {
                checked_files.push(FileStatus {
                    path: register_path.to_string(),
                    status: "valid".to_string(),
                    error: None,
                });
            }
            Err(e) => {
                checked_files.push(FileStatus {
                    path: register_path.to_string(),
                    status: "invalid".to_string(),
                    error: Some(e.to_string()),
                });
            }
        }
    }

    if std::path::Path::new(".osk/specs").is_dir() {
        for entry in fs::read_dir(".osk/specs")? {
            let entry = entry?;
            if entry.file_type()?.is_dir() {
                let tasks_path = entry.path().join("tasks.yaml");
                if tasks_path.exists() {
                    let path_str = tasks_path.to_string_lossy().to_string();
                    match yaml::validate(&path_str) {
                        Ok(_) => {
                            checked_files.push(FileStatus {
                                path: path_str,
                                status: "valid".to_string(),
                                error: None,
                            });
                        }
                        Err(e) => {
                            checked_files.push(FileStatus {
                                path: path_str,
                                status: "invalid".to_string(),
                                error: Some(e.to_string()),
                            });
                        }
                    }
                }
            }
        }
    }

    let error_count = checked_files
        .iter()
        .filter(|f| f.status == "invalid")
        .count();
    let valid_count = checked_files.iter().filter(|f| f.status == "valid").count();

    if json {
        let result = ValidateResult {
            valid: error_count == 0,
            command: "validate yaml".to_string(),
            checked_files,
            error_count,
            next_command: None,
            message: format!("{} files validated, {} errors", valid_count, error_count),
        };
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        for file in &checked_files {
            if file.status == "valid" {
                println!("✓ {}", file.path);
            } else {
                println!(
                    "✗ {}: {}",
                    file.path,
                    file.error.as_deref().unwrap_or("unknown error")
                );
            }
        }
        println!(
            "\n» {} files validated, {} errors",
            valid_count, error_count
        );
    }

    if error_count > 0 {
        bail!("YAML validation failed for {} files", error_count)
    }
    Ok(())
}

fn validate_deps(feature: &str, json: bool) -> Result<()> {
    let feature_dir = counter::find_feature_dir(feature)?
        .ok_or_else(|| anyhow::anyhow!("Feature '{}' not found", feature))?;

    let tasks_path = format!("{}/tasks.yaml", feature_dir);

    if !std::path::Path::new(&tasks_path).exists() {
        bail!("tasks.yaml not found for feature '{}'", feature);
    }

    let content = fs::read_to_string(&tasks_path)
        .with_context(|| format!("Failed to read {}", tasks_path))?;

    let tasks_file: TasksFile = serde_yaml::from_str(&content)
        .with_context(|| format!("Failed to parse {}", tasks_path))?;

    let (has_cycle, cycle_path, warnings) = detect_cycles(&tasks_file.tasks);

    let mut checked_files = vec![FileStatus {
        path: tasks_path.clone(),
        status: if has_cycle { "invalid" } else { "valid" }.to_string(),
        error: if has_cycle {
            Some(format!("Circular dependency: {}", cycle_path.join(" → ")))
        } else {
            None
        },
    }];

    for warning in &warnings {
        checked_files.push(FileStatus {
            path: tasks_path.clone(),
            status: "warning".to_string(),
            error: Some(warning.clone()),
        });
    }

    let error_count = if has_cycle { 1 } else { 0 };

    if json {
        let result = ValidateResult {
            valid: !has_cycle,
            command: "validate deps".to_string(),
            checked_files,
            error_count,
            next_command: None,
            message: if has_cycle {
                format!("Circular dependency detected: {}", cycle_path.join(" → "))
            } else {
                format!(
                    "No circular dependencies ({} tasks checked)",
                    tasks_file.tasks.len()
                )
            },
        };
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("🔍 Validating dependencies in {}...\n", tasks_path);
        println!("   {} tasks found", tasks_file.tasks.len());

        for warning in &warnings {
            println!("   ⚠  {}", warning);
        }

        if has_cycle {
            println!("\n✗ Circular dependency detected:");
            println!("   {}", cycle_path.join(" → "));
        } else {
            println!("\n✓ No circular dependencies found");
        }
    }

    if has_cycle {
        bail!("Circular dependency detected")
    }
    Ok(())
}

fn detect_cycles(tasks: &[Task]) -> (bool, Vec<String>, Vec<String>) {
    let mut warnings = Vec::new();

    let task_ids: HashSet<&str> = tasks.iter().map(|t| t.id.as_str()).collect();

    let mut graph: HashMap<&str, Vec<&str>> = HashMap::new();
    for task in tasks {
        graph.insert(&task.id, vec![]);
    }

    for task in tasks {
        for dep in &task.depends_on {
            if !task_ids.contains(dep.as_str()) {
                warnings.push(format!("Task {} depends on unknown task {}", task.id, dep));
                continue;
            }
            if let Some(edges) = graph.get_mut(task.id.as_str()) {
                edges.push(dep.as_str());
            }
        }
    }

    #[derive(Clone, Copy, PartialEq)]
    enum Color {
        White,
        Gray,
        Black,
    }

    let mut colors: HashMap<&str, Color> = task_ids.iter().map(|&id| (id, Color::White)).collect();
    let mut path: Vec<&str> = Vec::new();

    fn dfs<'a>(
        node: &'a str,
        graph: &HashMap<&'a str, Vec<&'a str>>,
        colors: &mut HashMap<&'a str, Color>,
        path: &mut Vec<&'a str>,
    ) -> Option<Vec<String>> {
        colors.insert(node, Color::Gray);
        path.push(node);

        if let Some(neighbors) = graph.get(node) {
            for &neighbor in neighbors {
                match colors.get(neighbor) {
                    Some(Color::Gray) => {
                        let cycle_start = path.iter().position(|&n| n == neighbor).unwrap();
                        let mut cycle: Vec<String> =
                            path[cycle_start..].iter().map(|s| s.to_string()).collect();
                        cycle.push(neighbor.to_string());
                        return Some(cycle);
                    }
                    Some(Color::White) => {
                        if let Some(cycle) = dfs(neighbor, graph, colors, path) {
                            return Some(cycle);
                        }
                    }
                    _ => {}
                }
            }
        }

        colors.insert(node, Color::Black);
        path.pop();
        None
    }

    for &task_id in &task_ids {
        if colors.get(task_id) == Some(&Color::White) {
            if let Some(cycle) = dfs(task_id, &graph, &mut colors, &mut path) {
                return (true, cycle, warnings);
            }
        }
    }

    (false, vec![], warnings)
}

fn validate_workflow(feature: &str, json: bool) -> Result<()> {
    let feature_dir = counter::find_feature_dir(feature)?
        .ok_or_else(|| anyhow::anyhow!("Feature '{}' not found", feature))?;

    let workflow_files = [
        ("threats.md", "/osk-analyze"),
        ("risks.md", "/osk-analyze"),
        ("requirements.md", "/osk-specify"),
        ("testing.md", "/osk-specify"),
        ("hardening.md", "/osk-harden"),
        ("plan.md", "/osk-plan"),
        ("tasks.yaml", "/osk-tasks"),
    ];

    let mut checked_files = Vec::new();
    let mut next_command: Option<String> = None;

    for (filename, command) in workflow_files {
        let path = format!("{}/{}", feature_dir, filename);
        let exists = std::path::Path::new(&path).exists();

        let has_content = if exists {
            fs::read_to_string(&path)
                .map(|c| !c.trim().is_empty() && !c.contains("TODO:"))
                .unwrap_or(false)
        } else {
            false
        };

        let (status, error) = if has_content {
            ("valid".to_string(), None)
        } else if exists {
            if next_command.is_none() {
                next_command = Some(format!("{} {}", command, feature));
            }
            (
                "incomplete".to_string(),
                Some("contains TODO or empty".to_string()),
            )
        } else {
            if next_command.is_none() {
                next_command = Some(format!("{} {}", command, feature));
            }
            ("missing".to_string(), Some("file not found".to_string()))
        };

        checked_files.push(FileStatus {
            path,
            status,
            error,
        });
    }

    let complete_count = checked_files.iter().filter(|f| f.status == "valid").count();
    let error_count = checked_files.iter().filter(|f| f.status != "valid").count();

    if json {
        let result = ValidateResult {
            valid: complete_count == 7,
            command: "validate workflow".to_string(),
            checked_files,
            error_count,
            next_command,
            message: format!("Workflow: {}/7 complete", complete_count),
        };
        println!("{}", serde_json::to_string_pretty(&result)?);
    } else {
        println!("🔍 Validating workflow for {}...\n", feature);
        for file in &checked_files {
            let filename = file.path.split('/').next_back().unwrap_or(&file.path);
            match file.status.as_str() {
                "valid" => println!("✓ {}", filename),
                "incomplete" => println!("⚠  {} (empty or TODO)", filename),
                _ => println!("✗ {} (missing)", filename),
            }
        }
        println!("\n» Workflow: {}/7 complete", complete_count);
        if complete_count == 7 {
            println!("✓ Workflow complete for '{}'", feature);
        } else if let Some(ref cmd) = next_command {
            println!("→ Next: Run {}", cmd);
        }
    }

    Ok(())
}

/// Validate system model files (FR-029)
/// - Schema validation for all YAML files
/// - Cross-reference validation (check all referenced IDs exist)
/// - Index.yaml line count check (<200 lines, FR-028)
fn validate_system_model(model_path: &str, json: bool) -> Result<()> {
    use std::path::Path;

    let model_dir = Path::new(model_path);
    if !model_dir.exists() {
        bail!("System model not found at {}. Run `osk init` first.", model_path);
    }

    let mut checked_files = Vec::new();
    let mut errors: Vec<SystemModelError> = Vec::new();
    let mut all_ids: HashSet<String> = HashSet::new();
    let mut referenced_ids: Vec<(String, String)> = Vec::new(); // (file, referenced_id)

    // List of expected section files
    let section_files = [
        "index.yaml",
        "business.yaml",
        "architecture.yaml",
        "data.yaml",
        "actors.yaml",
        "trust.yaml",
        "integrations.yaml",
        "security.yaml",
        "gaps.yaml",
    ];

    // Validate each section file
    for filename in &section_files {
        let file_path = model_dir.join(filename);
        let path_str = file_path.to_string_lossy().to_string();

        if !file_path.exists() {
            checked_files.push(FileStatus {
                path: path_str.clone(),
                status: "missing".to_string(),
                error: Some("File not found".to_string()),
            });
            errors.push(SystemModelError {
                file: filename.to_string(),
                line: None,
                message: "Section file missing".to_string(),
                severity: "error".to_string(),
                suggestion: Some(format!("Run `/osk-discover init` to create {}", filename)),
            });
            continue;
        }

        // Read and parse YAML
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

        // Check line count for index.yaml (FR-028)
        if *filename == "index.yaml" {
            let line_count = content.lines().count();
            if line_count > 200 {
                errors.push(SystemModelError {
                    file: filename.to_string(),
                    line: Some(200),
                    message: format!("Index file exceeds 200 lines ({} lines)", line_count),
                    severity: "error".to_string(),
                    suggestion: Some("Move detailed data to section files, keep only summaries in index".to_string()),
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
                errors.push(SystemModelError {
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
        if !all_ids.contains(ref_id) && !ref_id.starts_with("DATA-") && !ref_id.starts_with("COMP-") {
            // Skip well-known ID patterns that might be placeholders
            if !ref_id.contains("[") && !ref_id.is_empty() {
                errors.push(SystemModelError {
                    file: file.clone(),
                    line: None,
                    message: format!("Referenced ID '{}' not found", ref_id),
                    severity: "warning".to_string(),
                    suggestion: Some(format!("Add definition for '{}' or remove reference", ref_id)),
                });
            }
        }
    }

    let error_count = errors.iter().filter(|e| e.severity == "error").count();
    let warning_count = errors.iter().filter(|e| e.severity == "warning").count();
    let valid = error_count == 0;

    if json {
        let result = SystemModelValidation {
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
        println!("🔍 Validating system model at {}...\n", model_path);

        for file in &checked_files {
            let filename = file.path.split('/').next_back().unwrap_or(&file.path);
            match file.status.as_str() {
                "valid" => println!("   ✓ {}", filename),
                "missing" => println!("   ✗ {} (missing)", filename),
                _ => println!("   ✗ {} ({})", filename, file.error.as_deref().unwrap_or("invalid")),
            }
        }

        if !errors.is_empty() {
            println!("\n📋 Issues found:");
            for err in &errors {
                let icon = if err.severity == "error" { "✗" } else { "⚠" };
                println!("   {} [{}] {}", icon, err.file, err.message);
                if let Some(ref suggestion) = err.suggestion {
                    println!("      → {}", suggestion);
                }
            }
        }

        println!("\n» Result: {} errors, {} warnings", error_count, warning_count);
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

#[derive(Serialize)]
struct SystemModelValidation {
    valid: bool,
    path: String,
    schema_errors: usize,
    reference_errors: usize,
    warnings: usize,
    errors: Vec<SystemModelError>,
    checked_files: Vec<FileStatus>,
}

#[derive(Serialize)]
struct SystemModelError {
    file: String,
    line: Option<usize>,
    message: String,
    severity: String,
    suggestion: Option<String>,
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
                    if key_str == "trust_zone" || key_str == "component" || key_str == "actor"
                        || key_str == "from" || key_str == "to" || key_str == "data"
                        || key_str == "integration" || key_str == "zone"
                    {
                        if let serde_yaml::Value::String(ref_id) = val {
                            referenced_ids.push((filename.to_string(), ref_id.clone()));
                        }
                    }
                    // Handle arrays of references
                    if key_str == "actors" || key_str == "data_created" || key_str == "stores"
                        || key_str == "components" || key_str == "data_flows" || key_str == "data_shared"
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

    #[test]
    fn test_detect_cycles_no_cycle() {
        let tasks = vec![
            Task {
                id: "T001".to_string(),
                depends_on: vec![],
            },
            Task {
                id: "T002".to_string(),
                depends_on: vec!["T001".to_string()],
            },
            Task {
                id: "T003".to_string(),
                depends_on: vec!["T002".to_string()],
            },
        ];
        let (has_cycle, _, _) = detect_cycles(&tasks);
        assert!(!has_cycle);
    }

    #[test]
    fn test_detect_cycles_simple_cycle() {
        let tasks = vec![
            Task {
                id: "T001".to_string(),
                depends_on: vec!["T002".to_string()],
            },
            Task {
                id: "T002".to_string(),
                depends_on: vec!["T001".to_string()],
            },
        ];
        let (has_cycle, cycle_path, _) = detect_cycles(&tasks);
        assert!(has_cycle);
        assert!(cycle_path.len() >= 2);
    }

    #[test]
    fn test_detect_cycles_self_reference() {
        let tasks = vec![Task {
            id: "T001".to_string(),
            depends_on: vec!["T001".to_string()],
        }];
        let (has_cycle, cycle_path, _) = detect_cycles(&tasks);
        assert!(has_cycle);
        assert_eq!(cycle_path, vec!["T001", "T001"]);
    }

    #[test]
    fn test_detect_cycles_unknown_dep() {
        let tasks = vec![Task {
            id: "T001".to_string(),
            depends_on: vec!["T999".to_string()],
        }];
        let (has_cycle, _, warnings) = detect_cycles(&tasks);
        assert!(!has_cycle);
        assert_eq!(warnings.len(), 1);
        assert!(warnings[0].contains("unknown task T999"));
    }

    #[test]
    fn test_detect_cycles_complex() {
        let tasks = vec![
            Task {
                id: "T001".to_string(),
                depends_on: vec![],
            },
            Task {
                id: "T002".to_string(),
                depends_on: vec!["T001".to_string()],
            },
            Task {
                id: "T003".to_string(),
                depends_on: vec!["T002".to_string(), "T004".to_string()],
            },
            Task {
                id: "T004".to_string(),
                depends_on: vec!["T003".to_string()],
            },
        ];
        let (has_cycle, cycle_path, _) = detect_cycles(&tasks);
        assert!(has_cycle);
        assert!(cycle_path.contains(&"T003".to_string()));
        assert!(cycle_path.contains(&"T004".to_string()));
    }
}
