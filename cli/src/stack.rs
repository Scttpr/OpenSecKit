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
