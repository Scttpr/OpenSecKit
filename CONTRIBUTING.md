# Contributing to OpenSecKit

Merci de contribuer à OpenSecKit !

## Quick Start

```bash
# Clone
git clone https://github.com/Scttpr/OpenSecKit.git
cd OpenSecKit

# Build CLI
cd cli
cargo build

# Run tests
cargo test
```

## Development

### Prerequisites

- Rust 1.70+
- Node.js 20+ (for docs validation)
- Python 3.x (for MkDocs)

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

1. Fork le repo
2. Crée une branche: `git checkout -b feat/ma-feature`
3. Commit avec [Conventional Commits](https://www.conventionalcommits.org/):
   - `feat:` nouvelle fonctionnalité
   - `fix:` correction de bug
   - `docs:` documentation
   - `refactor:` refactoring
   - `test:` ajout de tests
   - `ci:` changements CI/CD
   - `chore:` maintenance
4. Push et ouvre une PR

### Checklist PR

- [ ] Tests passent (`cargo test`)
- [ ] Code formaté (`cargo fmt`)
- [ ] Pas de warnings clippy
- [ ] Documentation mise à jour si nécessaire

## Structure du Projet

```
├── cli/                 # CLI Rust
│   ├── src/
│   │   ├── commands/    # Commandes (init, check, scaffold...)
│   │   └── utils/       # Helpers
│   └── Cargo.toml
├── prompts/             # Prompts OSK
├── templates/           # Templates Tera
├── domaines/            # Contenu RGPD/RGS/NIS2
└── docs/                # Documentation MkDocs
```

## Ajouter un Prompt

1. Créer `prompts/osk-<nom>.md` avec frontmatter:
   ```yaml
   ---
   description: "Description courte"
   argument: "nom_argument"  # optionnel
   ---
   ```
2. Documenter dans `docs/commands/`
3. Tester avec `osk init`

## Questions

Ouvre une [issue](https://github.com/Scttpr/OpenSecKit/issues) ou une [discussion](https://github.com/Scttpr/OpenSecKit/discussions).
