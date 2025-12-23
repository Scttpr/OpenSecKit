# CLI

Référence du CLI OpenSecKit.

## Installation

```bash
# Via Cargo
cargo install osk

# Depuis les sources
git clone https://github.com/Scttpr/OpenSecKit
cd OpenSecKit/cli
cargo install --path .
```

## Commandes

### init

Initialise un projet avec OpenSecKit.

```bash
osk init [OPTIONS]
```

**Options :**

| Option | Description |
|--------|-------------|
| `--force`, `-f` | Force la mise à jour |
| `--default` | Mode non-interactif |

**Exemple :**

```bash
cd mon-projet/
osk init
osk init --force  # Mise à jour
```

### ingest

Exporte le contexte du projet.

```bash
osk ingest [OPTIONS]
```

**Options :**

| Option | Description |
|--------|-------------|
| `-p`, `--path <PATH>` | Chemin à ingérer |
| `-o`, `--output <FILE>` | Fichier de sortie |

**Exemple :**

```bash
osk ingest > context.txt
osk ingest -p src/ > context-src.txt
```

## Configuration

Fichier `.osk/config.toml` :

```toml
[project]
name = "mon-projet"

[stack]
detected = ["rust", "node"]
custom = []

[domains]
active = ["rgpd", "rgs"]

[domains.rgpd]
enabled = true
niveau = "standard"

[domains.rgs]
enabled = true
niveau = "standard"

[principles]
I_threat_modeling = "high"
II_risk_analysis = "high"
III_security_design = "medium"
IV_security_testing = "high"
V_secrets_management = "high"
VI_audit_logging = "medium"
VII_patch_management = "high"
```

## Intégration CI/CD

### GitHub Actions

```yaml
name: Security Context
on: [push, pull_request]

jobs:
  generate:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions-rs/toolchain@v1
      - run: cargo install --git https://github.com/Scttpr/OpenSecKit --path cli
      - run: osk ingest > security-context.txt
      - uses: actions/upload-artifact@v4
        with:
          name: security-context
          path: security-context.txt
```

### GitLab CI

```yaml
security-context:
  stage: security
  image: rust:latest
  script:
    - cargo install --git https://github.com/Scttpr/OpenSecKit --path cli
    - osk ingest > security-context.txt
  artifacts:
    paths: [security-context.txt]
    expire_in: 30 days
```

## Dépannage

| Problème | Solution |
|----------|----------|
| Config introuvable | `osk init` |
| Templates obsolètes | `osk init --force` |
| Erreur compilation | Vérifier Rust 1.70+ |

```bash
osk --help
osk <commande> --help
```
