//! Prompt parsing module

use anyhow::{Context, Result};
use lazy_static::lazy_static;
use regex::Regex;
use serde::Serialize;
use std::fs;
use std::path::Path;

lazy_static! {
    static ref FRONTMATTER_RE: Regex = Regex::new(r"(?s)^---\n(.*?)\n---").unwrap();
    static ref DESCRIPTION_RE: Regex = Regex::new(r#"description:\s*"?([^"\n]+)"?"#).unwrap();
    static ref ARGUMENT_RE: Regex = Regex::new(r#"argument:\s*"?([^"\n]+)"?"#).unwrap();
}

#[derive(Debug, Clone, Serialize)]
pub struct PromptInfo {
    pub name: String,
    pub filename: String,
    pub description: String,
    pub argument: Option<String>,
    pub phase: String,
    pub principles: Vec<String>,
    pub requires: Vec<String>,
    pub outputs: Vec<String>,
    pub steps: Vec<String>,
    pub instructions: String,
    pub raw_content: String,
}

pub fn parse_prompts_dir(prompts_dir: &Path) -> Result<Vec<PromptInfo>> {
    let mut prompts = Vec::new();

    if !prompts_dir.exists() {
        return Ok(prompts);
    }

    for entry in fs::read_dir(prompts_dir)? {
        let entry = entry?;
        let path = entry.path();

        if path.extension().and_then(|s| s.to_str()) == Some("md") {
            match parse_prompt_file(&path) {
                Ok(info) => prompts.push(info),
                Err(e) => eprintln!("   ⚠️  Erreur parsing {}: {}", path.display(), e),
            }
        }
    }

    prompts.sort_by(|a, b| a.name.cmp(&b.name));

    Ok(prompts)
}

pub fn parse_prompt_file(path: &Path) -> Result<PromptInfo> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Impossible de lire {}", path.display()))?;

    let filename = path
        .file_name()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown.md")
        .to_string();

    let name = filename.trim_end_matches(".md").to_string();

    let (description, argument) = parse_frontmatter(&content);
    let phase = extract_phase(&name);
    let principles = extract_principles(&content);
    let requires = extract_requires(&content);
    let outputs = extract_outputs(&content);
    let steps = extract_steps(&content);
    let instructions = extract_instructions(&content);

    Ok(PromptInfo {
        name,
        filename,
        description,
        argument,
        phase,
        principles,
        requires,
        outputs,
        steps,
        instructions,
        raw_content: content,
    })
}

fn parse_frontmatter(content: &str) -> (String, Option<String>) {
    let description = DESCRIPTION_RE
        .captures(content)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string())
        .unwrap_or_else(|| "No description".to_string());

    let argument = ARGUMENT_RE
        .captures(content)
        .and_then(|c| c.get(1))
        .map(|m| m.as_str().to_string());

    (description, argument)
}

fn extract_phase(name: &str) -> String {
    match name {
        "osk-configure" => "setup",
        "osk-baseline" => "baseline",
        "osk-analyze" => "analyze",
        "osk-specify" => "specify",
        "osk-harden" => "harden",
        "osk-plan" => "plan",
        "osk-tasks" => "tasks",
        "osk-implement" => "implement",
        "osk-rgpd" | "osk-rgs" => "domain",
        "osk-dashboard" => "monitor",
        "osk-pca-pra" => "continuity",
        "osk-incident" => "incident",
        _ => "unknown",
    }
    .to_string()
}

fn extract_principles(content: &str) -> Vec<String> {
    let mut principles = Vec::new();
    let patterns = [
        ("I", r"(?i)principe\s+I\b"),
        ("II", r"(?i)principe\s+II\b"),
        ("III", r"(?i)principe\s+III\b"),
        ("IV", r"(?i)principe\s+IV\b"),
        ("V", r"(?i)principe\s+V\b"),
        ("VI", r"(?i)principe\s+VI\b"),
        ("VII", r"(?i)principe\s+VII\b"),
    ];

    for (num, pattern) in patterns {
        if let Ok(re) = Regex::new(pattern) {
            if re.is_match(content) {
                principles.push(num.to_string());
            }
        }
    }

    if content.contains("principles = [") || content.contains("Principes:") {
        for num in ["I", "II", "III", "IV", "V", "VI", "VII"] {
            let pattern = format!(r#""{}"#, num);
            if content.contains(&pattern) && !principles.contains(&num.to_string()) {
                principles.push(num.to_string());
            }
        }
    }

    principles
}

fn extract_requires(content: &str) -> Vec<String> {
    let mut requires = Vec::new();
    let re = Regex::new(r"`(\.osk/[^`]+|docs/[^`]+)`").unwrap();

    for cap in re.captures_iter(content) {
        if let Some(m) = cap.get(1) {
            let path = m.as_str().to_string();
            if !requires.contains(&path) && (path.contains("context") || path.contains("config")) {
                requires.push(path);
            }
        }
    }

    requires.truncate(5);
    requires
}

fn extract_outputs(content: &str) -> Vec<String> {
    let mut outputs = Vec::new();
    let patterns = [r"`(\.osk/specs/[^`]+)`", r"`(docs/security/[^`]+)`"];

    for pattern in patterns {
        if let Ok(re) = Regex::new(pattern) {
            for cap in re.captures_iter(content) {
                if let Some(m) = cap.get(1) {
                    let path = m.as_str().to_string();
                    if !outputs.contains(&path) {
                        outputs.push(path);
                    }
                }
            }
        }
    }

    outputs.truncate(5);
    outputs
}

fn extract_steps(content: &str) -> Vec<String> {
    let mut steps = Vec::new();
    let re = Regex::new(r"^\d+\.\s+\*\*([^*]+)\*\*").unwrap();

    for line in content.lines() {
        if let Some(cap) = re.captures(line.trim()) {
            if let Some(m) = cap.get(1) {
                steps.push(m.as_str().trim().to_string());
            }
        }
    }

    if steps.is_empty() {
        let re = Regex::new(r"^##\s+(?:Phase\s+\d+\s*[:.]?\s*)?(.+)$").unwrap();
        for line in content.lines() {
            if let Some(cap) = re.captures(line.trim()) {
                if let Some(m) = cap.get(1) {
                    let step = m.as_str().trim().to_string();
                    if !step.starts_with("Context") && !step.starts_with("Output") {
                        steps.push(step);
                    }
                }
            }
        }
    }

    steps.truncate(6);
    steps
}

fn extract_instructions(content: &str) -> String {
    let without_frontmatter = FRONTMATTER_RE.replace(content, "");
    let mut instructions = String::new();
    let mut in_paragraph = false;
    let mut paragraph_count = 0;

    for line in without_frontmatter.lines() {
        let trimmed = line.trim();

        if trimmed.is_empty() {
            if in_paragraph {
                instructions.push('\n');
                paragraph_count += 1;
                in_paragraph = false;
            }
            continue;
        }

        if trimmed.starts_with('#') || trimmed.starts_with("```") {
            continue;
        }

        if paragraph_count < 3 {
            if !instructions.is_empty() && !instructions.ends_with('\n') {
                instructions.push(' ');
            }
            instructions.push_str(trimmed);
            in_paragraph = true;
        }
    }

    if instructions.len() > 500 {
        instructions.truncate(500);
        instructions.push_str("...");
    }

    instructions.trim().to_string()
}
