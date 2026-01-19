# /osk-discover

Build or update the system model (adaptive - detects existing model).

## Synopsis

```
/osk-discover
```

## Description

The main discovery command. It **adapts** to the current state:

- **No model exists**: Full 3-phase guided discovery
- **Model exists + code changed**: Incremental update via git diff
- **Model exists + no changes**: Offers context update

## Adaptive Flow

```
/osk-discover
      │
      ▼
┌─────────────────────────┐
│ Model exists?           │
└─────────────────────────┘
      │
      ├── NO ──► Full Discovery
      │
      ▼ YES
┌─────────────────────────┐
│ Code changed since      │
│ last_commit?            │
└─────────────────────────┘
      │
      ├── YES ──► Incremental Update
      │
      ▼ NO
┌─────────────────────────┐
│ "Model up to date"      │
│ Update context? [Y/N]   │
└─────────────────────────┘
```

## Prérequis

- `osk init` exécuté (`.osk/config.toml` existe)

## CLI Utilisées

| Commande | Usage |
|----------|-------|
| `osk scan --json` | Liste des fichiers (full discovery) |
| `osk changes --json` | Fichiers modifiés (incremental) |
| `osk id <path>` | Génération des IDs composants |
| `osk validate system-model` | Validation finale |

## Output

```
.osk/system-model/
├── index.yaml          # Vue d'ensemble + METADATA
├── business.yaml       # Contexte métier
├── architecture.yaml   # Composants, services
├── data.yaml           # Inventaire données
├── actors.yaml         # Utilisateurs, systèmes
├── boundaries.yaml          # Frontières de confiance
├── integrations.yaml   # Intégrations externes
├── controls.yaml       # Contrôles existants
├── tooling.yaml        # Outillage développement
├── team.yaml           # Structure équipe
└── gaps.yaml           # Gaps identifiés
```

## Metadata (index.yaml header)

```yaml
# First lines of index.yaml - easy to find for updates
metadata:
  generated_at: "2026-01-17T14:30:00Z"    # ISO 8601 timestamp
  last_commit: "abc123def456"              # Git commit SHA
  model_version: "4.0.0"                   # OpenSecKit version
  discovery_mode: "full"                   # "full" or "incremental"
```

## Exemple (Full Discovery)

```
>>> /osk-discover

ℹ️ No existing system model found.
Starting full discovery...

🔍 Phase 1: Quick Scan
======================
✓ Stack: Node.js + TypeScript (90%)
✓ Framework: NestJS (85%)
✓ CI/CD: GitHub Actions (100%)
✓ 127 files analyzed

🔬 Phase 2: Deep Analysis
=========================
✓ 12 components identified
✓ 5 external integrations
✓ 3 trust boundaries

📋 Phase 3: Interview
=====================
? Business domain?
  [1] E-commerce
  [2] SaaS
  [3] API Platform

...

✅ Discovery Complete
- Model: .osk/system-model/
- Commit: abc123
- Gaps: 3 (run /osk-discover validate --resolve)
```

## Exemple (Incremental Update)

```
>>> /osk-discover

ℹ️ Existing model found (commit: abc123)
Checking for changes...

📝 Changes Since abc123
=======================
├── + src/api/orders.rs (added)
├── ~ src/services/payment.rs (modified)
└── - src/legacy/old.rs (deleted)

📋 Model Impact:
├── architecture.yaml: +1, ~1, -1 components
└── gaps.yaml: +1 new gap

[A]pply all | [R]eview each | [C]ancel

Choice: A

✅ Update Complete
- Commit: def456
- Mode: incremental
```

## Exemple (Model Up to Date)

```
>>> /osk-discover

ℹ️ Model up to date (commit: def456)
No code changes detected.

Update operational context? [Y/N]: Y

What would you like to update?
[1] Hosting (provider, regions)
[2] Team (owner, contacts)
[3] Tooling (CI/CD, monitoring)
[4] Business (domain, criticality)

...
```

## Voir aussi

- [`/osk-discover validate`](osk-discover-validate.md) - Valider (ou `--resolve` pour corriger)
- [`/osk-discover docs`](osk-discover-docs.md) - Générer documentation
