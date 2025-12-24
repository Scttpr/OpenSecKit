//! Template substitution helpers

use anyhow::{Context, Result};
use chrono::Local;
use std::fs;
use std::path::Path;

pub fn render(content: &str, variables: &[(&str, &str)]) -> String {
    let mut result = content.to_string();

    let now = Local::now();
    result = result.replace("{{DATE}}", &now.format("%Y-%m-%d").to_string());
    result = result.replace("{{DATETIME}}", &now.format("%Y-%m-%d %H:%M:%S").to_string());
    result = result.replace("{{YEAR}}", &now.format("%Y").to_string());

    for (key, value) in variables {
        let pattern = format!("{{{{{}}}}}", key);
        result = result.replace(&pattern, value);
    }

    result
}

pub fn create_from_template(dst: &str, template_content: &str, variables: &[(&str, &str)]) -> Result<()> {
    let rendered = render(template_content, variables);

    if let Some(parent) = Path::new(dst).parent() {
        fs::create_dir_all(parent)?;
    }

    fs::write(dst, rendered).with_context(|| format!("Cannot write {}", dst))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_render_simple() {
        let template = "Hello {{name}}!";
        let result = render(template, &[("name", "World")]);
        assert_eq!(result, "Hello World!");
    }

    #[test]
    fn test_render_multiple_vars() {
        let template = "{{greeting}} {{name}}!";
        let result = render(template, &[("greeting", "Hello"), ("name", "Alice")]);
        assert_eq!(result, "Hello Alice!");
    }

    #[test]
    fn test_render_date() {
        let template = "Today is {{DATE}}";
        let result = render(template, &[]);
        assert!(!result.contains("{{DATE}}"));
    }

    #[test]
    fn test_render_no_match() {
        let template = "No variables here";
        let result = render(template, &[("unused", "value")]);
        assert_eq!(result, "No variables here");
    }
}
