# Contributing to OpenSecKit

Thanks for contributing to OpenSecKit!

## Quick Start

```bash
# Clone
git clone https://github.com/Scttpr/OpenSecKit.git
cd OpenSecKit

# Setup git hooks
git config core.hooksPath .githooks

# Build CLI
cd cli
cargo build

# Run tests
cargo test
```

## Development

### Prerequisites

- Rust 1.70+

### Git Hooks

The project uses a pre-commit hook for code quality. Enable it with:

```bash
git config core.hooksPath .githooks
```

This runs `cargo fmt --check` and `cargo clippy` before each commit.

### Code Style

```bash
# Format
cargo fmt

# Lint
cargo clippy -- -D warnings

# All checks
cargo fmt -- --check && cargo clippy -- -D warnings && cargo test
```

## Pull Requests

1. Fork the repo
2. Create a branch: `git checkout -b feat/my-feature`
3. Commit with [Conventional Commits](https://www.conventionalcommits.org/):
   - `feat:` new feature
   - `fix:` bug fix
   - `docs:` documentation
   - `refactor:` refactoring
   - `test:` tests
   - `ci:` CI/CD changes
   - `chore:` maintenance
4. Push and open a PR

### PR Checklist

- [ ] Tests pass (`cargo test`)
- [ ] Code formatted (`cargo fmt`)
- [ ] No clippy warnings
- [ ] Documentation updated if needed

## Project Structure

```
├── cli/                     # Rust CLI
│   ├── src/
│   │   ├── commands/        # Commands (init, check, scan...)
│   │   └── utils/           # Helpers
│   ├── templates/agents/    # Agent file templates
│   └── Cargo.toml
├── config/                  # Configuration files
│   ├── registry.toml        # Command registry
│   └── agents.toml          # Agent configurations
├── kit/                     # Core kit content
│   ├── discover/            # Discovery phase
│   │   └── prompts/
│   ├── comply/              # Compliance phase
│   │   ├── frameworks/      # RGPD, RGS frameworks
│   │   └── prompts/
│   └── secure/              # Security phase
│       └── prompts/
├── .githooks/               # Git hooks
│   └── pre-commit
└── docs/                    # Documentation
```

## Adding a Prompt

1. Create prompt in the appropriate kit folder:
   - Discovery: `kit/discover/prompts/`
   - Compliance: `kit/comply/frameworks/<framework>/prompts/`
   - Security: `kit/secure/prompts/`

2. Use the V4 frontmatter format:
   ```yaml
   ---
   description: "Short description for command picker"
   part: comply
   framework: rgpd  # if applicable
   phase: inventory
   model_sections: [index, data, actors]
   version: "5.0.0"
   knowledge:
     - reference/some-doc.md
   ---
   ```

3. Register in `config/registry.toml`

4. Test with `osk init --local`

## Questions

Open an [issue](https://github.com/Scttpr/OpenSecKit/issues) or a [discussion](https://github.com/Scttpr/OpenSecKit/discussions).
