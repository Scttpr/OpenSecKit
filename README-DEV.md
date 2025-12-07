# Guide de développement OpenSecKit

## Installation des dépendances (locales uniquement)

```bash
# Tout installer en une commande
./scripts/setup-dev.sh
```

Ce script installe aussi un **hook pre-commit** qui exécute automatiquement `npm test` avant chaque commit.

## Usage quotidien

### Tester tout avant un commit

```bash
npm test
```

### Commandes individuelles

```bash
# Liens morts
npm run check:links:all

# Frontmatter YAML
source .venv/bin/activate  # Activer le virtualenv Python
npm run check:frontmatter

# Rust linting
npm run check:rust
```

## Structure

- `node_modules/` - Dépendances Node.js (local, gitignored)
- `.venv/` - Environnement Python (local, gitignored)
- `scripts/` - Scripts de dev
- `package.json` - Config Node.js
- `requirements-dev.txt` - Dépendances Python

Tout est installé **localement dans le projet**, rien de global sur ta machine.
