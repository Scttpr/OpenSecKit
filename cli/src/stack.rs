use ignore::WalkBuilder;
use std::collections::HashSet;

pub fn detect() -> String {
    let mut stack = HashSet::new();

    let walker = WalkBuilder::new(".")
        .max_depth(Some(3))
        .git_ignore(true)
        .build();

    for result in walker {
        match result {
            Ok(entry) => {
                let file_name = entry.file_name().to_string_lossy();

                if file_name == "Cargo.toml" {
                    stack.insert("Rust");
                }

                if file_name == "package.json" {
                    stack.insert("Node.js");
                }
                if file_name == "tsconfig.json" {
                    stack.insert("TypeScript");
                }
                if file_name == "next.config.js" || file_name == "next.config.ts" {
                    stack.insert("Next.js");
                }
                if file_name == "deno.json" {
                    stack.insert("Deno");
                }

                if file_name == "requirements.txt"
                    || file_name == "Pipfile"
                    || file_name == "pyproject.toml"
                    || file_name == "uv.lock"
                {
                    stack.insert("Python");
                }

                if file_name == "go.mod" {
                    stack.insert("Go");
                }

                if file_name == "composer.json" {
                    stack.insert("PHP");
                }
                if file_name == "artisan" {
                    stack.insert("Laravel");
                }

                if file_name == "pom.xml" {
                    stack.insert("Java (Maven)");
                }
                if file_name == "build.gradle" || file_name == "build.gradle.kts" {
                    stack.insert("Java/Kotlin (Gradle)");
                }

                if file_name == "Gemfile" {
                    stack.insert("Ruby");
                }

                if file_name == "Dockerfile" || file_name == "docker-compose.yml" {
                    stack.insert("Docker");
                }
                if file_name.starts_with("k8s")
                    || file_name.ends_with(".helm")
                    || file_name == "Chart.yaml"
                {
                    stack.insert("Kubernetes");
                }
                if file_name.contains("terraform") || file_name.ends_with(".tf") {
                    stack.insert("Terraform");
                }
            }
            Err(_) => continue,
        }
    }

    let mut stack_vec: Vec<String> = stack.into_iter().map(String::from).collect();
    stack_vec.sort();
    stack_vec.join(", ")
}

fn detect_in(base: &std::path::Path) -> String {
    let mut stack = HashSet::new();

    let walker = WalkBuilder::new(base)
        .max_depth(Some(3))
        .git_ignore(true)
        .build();

    for result in walker {
        match result {
            Ok(entry) => {
                let file_name = entry.file_name().to_string_lossy();

                if file_name == "Cargo.toml" {
                    stack.insert("Rust");
                }
                if file_name == "package.json" {
                    stack.insert("Node.js");
                }
                if file_name == "tsconfig.json" {
                    stack.insert("TypeScript");
                }
                if file_name == "next.config.js" || file_name == "next.config.ts" {
                    stack.insert("Next.js");
                }
                if file_name == "deno.json" {
                    stack.insert("Deno");
                }
                if file_name == "requirements.txt"
                    || file_name == "Pipfile"
                    || file_name == "pyproject.toml"
                    || file_name == "uv.lock"
                {
                    stack.insert("Python");
                }
                if file_name == "go.mod" {
                    stack.insert("Go");
                }
                if file_name == "composer.json" {
                    stack.insert("PHP");
                }
                if file_name == "artisan" {
                    stack.insert("Laravel");
                }
                if file_name == "pom.xml" {
                    stack.insert("Java (Maven)");
                }
                if file_name == "build.gradle" || file_name == "build.gradle.kts" {
                    stack.insert("Java/Kotlin (Gradle)");
                }
                if file_name == "Gemfile" {
                    stack.insert("Ruby");
                }
                if file_name == "Dockerfile" || file_name == "docker-compose.yml" {
                    stack.insert("Docker");
                }
                if file_name.starts_with("k8s")
                    || file_name.ends_with(".helm")
                    || file_name == "Chart.yaml"
                {
                    stack.insert("Kubernetes");
                }
                if file_name.contains("terraform") || file_name.ends_with(".tf") {
                    stack.insert("Terraform");
                }
            }
            Err(_) => continue,
        }
    }

    let mut stack_vec: Vec<String> = stack.into_iter().map(String::from).collect();
    stack_vec.sort();
    stack_vec.join(", ")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_detect_empty_dir() {
        let dir = tempdir().unwrap();
        let result = detect_in(dir.path());
        assert!(result.is_empty());
    }

    #[test]
    fn test_detect_rust() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("Cargo.toml"), "[package]").unwrap();
        let result = detect_in(dir.path());
        assert_eq!(result, "Rust");
    }

    #[test]
    fn test_detect_node_typescript() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("package.json"), "{}").unwrap();
        fs::write(dir.path().join("tsconfig.json"), "{}").unwrap();
        let result = detect_in(dir.path());
        assert!(result.contains("Node.js"));
        assert!(result.contains("TypeScript"));
    }

    #[test]
    fn test_detect_python() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("requirements.txt"), "flask").unwrap();
        let result = detect_in(dir.path());
        assert_eq!(result, "Python");
    }

    #[test]
    fn test_detect_go() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("go.mod"), "module test").unwrap();
        let result = detect_in(dir.path());
        assert_eq!(result, "Go");
    }

    #[test]
    fn test_detect_docker() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("Dockerfile"), "FROM alpine").unwrap();
        let result = detect_in(dir.path());
        assert_eq!(result, "Docker");
    }

    #[test]
    fn test_detect_multiple() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("Cargo.toml"), "").unwrap();
        fs::write(dir.path().join("Dockerfile"), "").unwrap();
        fs::write(dir.path().join("main.tf"), "").unwrap();
        let result = detect_in(dir.path());
        assert!(result.contains("Rust"));
        assert!(result.contains("Docker"));
        assert!(result.contains("Terraform"));
    }

    #[test]
    fn test_detect_nested() {
        let dir = tempdir().unwrap();
        fs::create_dir_all(dir.path().join("backend")).unwrap();
        fs::write(dir.path().join("backend/go.mod"), "").unwrap();
        let result = detect_in(dir.path());
        assert_eq!(result, "Go");
    }
}
