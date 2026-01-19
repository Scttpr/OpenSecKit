# /osk-discover docs

Génère une documentation Markdown depuis le system-model, adaptée à l'audience.

## Synopsis

```
/osk-discover docs [--audience <security|developer|management|compliance>]
```

## Description

Transforme le system-model YAML en documentation Markdown lisible. Le contenu est adapté selon l'audience cible.

## Options

| Option | Description |
|--------|-------------|
| `--audience security` | Contrôles, menaces, trust boundaries, gaps |
| `--audience developer` | Architecture, data flows, composants, APIs |
| `--audience management` | Executive summary, risques, compliance |
| `--audience compliance` | Inventaire données, DPA, rétention |
| *(default)* | Document équilibré pour toutes audiences |

## Prérequis

- System-model complet (`.osk/system-model/index.yaml`)

## Output par Audience

| Audience | Fichier | Contenu principal |
|----------|---------|-------------------|
| security | `docs/security-overview.md` | Trust zones, contrôles, gaps |
| developer | `docs/architecture-guide.md` | Components, data flows, stack |
| management | `docs/executive-summary.md` | Métriques, risques, compliance |
| compliance | `docs/compliance-inventory.md` | Data register, DPA, rétention |
| general | `docs/architecture.md` | Vue équilibrée |

## Exemple (Security)

```
>>> /osk-discover docs --audience security

📄 Generating security documentation...

Loading system model...
✓ 9 sections loaded

Generating security-overview.md...
✓ Trust Architecture (3 zones, 5 boundaries)
✓ Security Controls (12 controls)
✓ Threat Surface (5 integrations)
✓ Gaps & Risks (3 items)

✅ Documentation generated: docs/security-overview.md
   👥 Audience: Security team
   📊 Sections: 5
```

## Exemple (Compliance)

```
>>> /osk-discover docs --audience compliance

📄 Generating compliance documentation...

✅ Documentation generated: docs/compliance-inventory.md

Contents:
- Data Processing Register (8 categories)
- PII Register (15 fields)
- Third-Party Register (5 processors)
- Control Evidence (12 controls)
- Gap Register (3 items)
```

## Exemple (Interactive)

```
>>> /osk-discover docs

📄 Documentation Generation
===========================

Who is the primary audience?

[1] Security team - Controls, threats, trust boundaries
[2] Developers - Architecture, data flows, components
[3] Management - Executive summary, risks, compliance
[4] Compliance/Audit - Data inventory, DPA, retention
[5] General - Balanced for all audiences

Choice: 3

Generating executive-summary.md...
```

## Voir aussi

- [`/osk-discover`](osk-discover.md) - Build/update le system-model
- [`/osk-discover validate`](osk-discover-validate.md) - Valider le modèle
