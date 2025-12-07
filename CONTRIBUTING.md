# Contributing to OpenSecKit

Merci de contribuer à OpenSecKit ! Ce guide vous aidera à configurer votre environnement de développement.

## Prérequis

- **Rust** et **Cargo** (pour la CLI)
- **Node.js** (pour les outils de validation)
- **Python 3** (pour les scripts de validation)
- **Git**

## Setup de l'environnement de développement

### Installation rapide

```bash
# Cloner le repo
git clone https://github.com/Scttpr/OpenSecKit
cd OpenSecKit

# Installer toutes les dépendances localement
./scripts/setup-dev.sh
```

Ce script installe automatiquement un **hook pre-commit** qui exécute `npm test` avant chaque commit pour garantir que toutes les vérifications passent.

### Installation manuelle

**Node.js (local) :**

```bash
npm install
```

Cela installe `markdown-link-check` dans `node_modules/`.

**Python (virtualenv local) :**

```bash
python3 -m venv .venv
source .venv/bin/activate
pip install -r requirements-dev.txt
```

**Rust (CLI) :**

```bash
cd cli
cargo build
```

## Hook pre-commit

Un hook pre-commit est automatiquement installé par `./scripts/setup-dev.sh`. Il exécute `npm test` avant chaque commit.

**Pour bypass le hook (déconseillé) :**

```bash
git commit --no-verify -m "votre message"
```

## Vérifications avant commit

### Tout tester en une commande

```bash
npm test
# ou
npm run lint
```

### Tests individuels

**Vérifier les liens morts :**

```bash
npm run check:links:all
```

**Valider le frontmatter YAML des templates :**

```bash
# Activer le virtualenv si pas déjà fait
source .venv/bin/activate
npm run check:frontmatter
```

**Linter le code Rust :**

```bash
npm run check:rust
# ou manuellement
cd cli
cargo fmt -- --check
cargo clippy
```

## Structure du projet

```
OpenSecKit/
├── cli/                    # CLI Rust
├── templates/              # Templates de sécurité
├── domaines/               # Extensions sectorielles
├── prompts/                # Prompts pour agents IA
├── scripts/                # Scripts de développement
├── .github/workflows/      # CI/CD
├── package.json            # Dépendances Node.js (dev)
├── requirements-dev.txt    # Dépendances Python (dev)
└── .venv/                  # Virtual env Python (local, gitignored)
```

## Workflow de contribution

1. Fork le repo
2. Créer une branche : `git checkout -b feature/ma-feature`
3. Installer les dépendances : `./scripts/setup-dev.sh`
4. Faire vos modifications
5. Tester : `npm test`
6. Commit : `git commit -m "feat: ma feature"`
7. Push : `git push origin feature/ma-feature`
8. Ouvrir une Pull Request

## Conventions

### Templates

Chaque template doit avoir un frontmatter YAML avec :

```yaml
---
title: "Titre du template"
constitutional_principle: "I" # ou II, III, IV, V, VI, VII
ssdlc_phase: "planning" # ou design, implementation, all
---
```

### Code Rust

- Formater avec `cargo fmt`
- Pas de warnings Clippy
- Suivre les conventions Rust standard

### Commits

Format recommandé (conventional commits) :

- `feat:` Nouvelle fonctionnalité
- `fix:` Correction de bug
- `docs:` Documentation
- `refactor:` Refactoring
- `test:` Tests
- `chore:` Maintenance

## CI/CD

Les workflows GitHub Actions s'exécutent automatiquement sur chaque PR :

- ✅ Validation des liens (markdown-link-check)
- ✅ Validation du frontmatter (python-frontmatter)
- ✅ Linting Rust (cargo fmt, clippy)
- ✅ Security checks

Vous pouvez les tester localement avec les commandes ci-dessus.

## Besoin d'aide ?

- **Issues :** [GitHub Issues](https://github.com/Scttpr/OpenSecKit/issues)
- **Discussions :** [GitHub Discussions](https://github.com/Scttpr/OpenSecKit/issues)
