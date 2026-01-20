//! ID command - component ID generation utility
//!
//! Generates consistent component IDs from file paths.
//! See FR-043: Component IDs derived from code artifacts (path + name).

use anyhow::Result;
use std::path::Path;

/// Run the id command
pub fn run(path: &str, json: bool) -> Result<()> {
    let id = generate_component_id(Path::new(path));

    if json {
        println!(r#"{{"path": "{}", "id": "{}"}}"#, path, id);
    } else {
        println!("{}", id);
    }

    Ok(())
}

/// Generate a component ID from a file path (FR-043)
///
/// Formula: `{directory}-{name}` with:
/// - Path separators replaced by hyphens
/// - File extension removed
/// - Lowercase
/// - Leading src/ prefix removed
///
/// # Examples
///
/// ```
/// use osk::commands::id::generate_component_id;
/// use std::path::Path;
///
/// assert_eq!(generate_component_id(Path::new("src/api/users.rs")), "api-users");
/// assert_eq!(generate_component_id(Path::new("services/payment.py")), "services-payment");
/// assert_eq!(generate_component_id(Path::new("lib/auth/jwt.ts")), "lib-auth-jwt");
/// ```
pub fn generate_component_id(path: &Path) -> String {
    // Get file stem (name without extension)
    let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("");

    // Get parent directory
    let parent = path.parent().unwrap_or(Path::new(""));

    // Remove common prefixes (src/, lib/)
    let clean_parent = parent
        .strip_prefix("src")
        .or_else(|_| parent.strip_prefix("lib"))
        .unwrap_or(parent);

    // Convert path separators to hyphens
    let parent_str = clean_parent.to_string_lossy().replace(['/', '\\'], "-");

    // Combine parent and stem
    let id = if parent_str.is_empty() {
        stem.to_string()
    } else {
        format!("{}-{}", parent_str, stem)
    };

    // Clean up: lowercase, remove leading/trailing hyphens, collapse multiple hyphens
    id.to_lowercase()
        .trim_matches('-')
        .split('-')
        .filter(|s| !s.is_empty())
        .collect::<Vec<_>>()
        .join("-")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_path() {
        assert_eq!(
            generate_component_id(Path::new("src/api/users.rs")),
            "api-users"
        );
    }

    #[test]
    fn test_nested_path() {
        assert_eq!(
            generate_component_id(Path::new("services/payment/processor.py")),
            "services-payment-processor"
        );
    }

    #[test]
    fn test_lib_prefix() {
        assert_eq!(
            generate_component_id(Path::new("lib/auth/jwt.ts")),
            "auth-jwt"
        );
    }

    #[test]
    fn test_root_file() {
        assert_eq!(generate_component_id(Path::new("main.rs")), "main");
    }

    #[test]
    fn test_deeply_nested() {
        assert_eq!(
            generate_component_id(Path::new("src/modules/users/handlers/create.rs")),
            "modules-users-handlers-create"
        );
    }

    #[test]
    fn test_python_file() {
        assert_eq!(
            generate_component_id(Path::new("services/payment.py")),
            "services-payment"
        );
    }

    #[test]
    fn test_typescript_file() {
        assert_eq!(
            generate_component_id(Path::new("src/components/Button.tsx")),
            "components-button"
        );
    }

    #[test]
    fn test_uppercase_handling() {
        assert_eq!(
            generate_component_id(Path::new("src/API/UserController.rs")),
            "api-usercontroller"
        );
    }

    #[test]
    fn test_index_file() {
        assert_eq!(
            generate_component_id(Path::new("src/utils/index.ts")),
            "utils-index"
        );
    }

    #[test]
    fn test_mod_file() {
        assert_eq!(
            generate_component_id(Path::new("src/commands/mod.rs")),
            "commands-mod"
        );
    }
}
