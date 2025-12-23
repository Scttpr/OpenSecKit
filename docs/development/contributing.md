# Contribuer

Guide de contribution à OpenSecKit.

## Setup

```bash
git clone https://github.com/Scttpr/OpenSecKit
cd OpenSecKit
./scripts/setup-dev.sh
```

Installe :

- Dépendances Node.js
- Environnement Python
- Hook pre-commit

## Workflow

1. Fork le repo
2. Créer une branche : `git checkout -b feature/ma-feature`
3. Setup : `./scripts/setup-dev.sh`
4. Faire les modifications
5. Tests : `npm test`
6. Commit : `git commit -m "feat: ma feature"`
7. Push : `git push origin feature/ma-feature`
8. Ouvrir une Pull Request

## Tests

```bash
# Tout tester
npm test

# Tests individuels
npm run check:links:all
source .venv/bin/activate && npm run check:frontmatter
npm run check:rust
```

## Conventions

### Commits

Format [Conventional Commits](https://www.conventionalcommits.org/) :

| Préfixe | Usage |
|---------|-------|
| `feat:` | Nouvelle fonctionnalité |
| `fix:` | Correction bug |
| `docs:` | Documentation |
| `refactor:` | Refactoring |
| `test:` | Tests |
| `chore:` | Maintenance |

### Rust

```bash
cargo fmt     # Formatage
cargo clippy  # Linting
```

### Templates

Frontmatter requis :

```yaml
---
title: "Titre"
constitutional_principle: "I"  # I-VII
ssdlc_phase: "planning"        # planning, design, implementation, all
---
```

## Checklist PR

- [ ] `cargo build` compile sans erreur
- [ ] `npm test` passe
- [ ] `cargo fmt` appliqué
- [ ] `cargo clippy` sans warnings
- [ ] Liens markdown valides
- [ ] Frontmatter YAML valide
- [ ] Commits au format conventional
- [ ] Documentation à jour

## Support

- [Issues](https://github.com/Scttpr/OpenSecKit/issues)
- [Discussions](https://github.com/Scttpr/OpenSecKit/discussions)
