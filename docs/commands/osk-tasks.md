# /osk-tasks

Génération des tâches ordonnées.

## Synopsis

```bash
>>> /osk-tasks <feature>
```

## Description

`/osk-tasks` génère une liste de tâches atomiques et ordonnées pour implémenter le plan de sécurité.

## Arguments

| Argument | Description | Exemple |
|----------|-------------|---------|
| `feature` | Nom de la feature | `"authentication"` |

## Fichiers Générés

| Fichier | Description |
|---------|-------------|
| `.osk/specs/NNN-feature/tasks.md` | Liste lisible |
| `.osk/specs/NNN-feature/tasks.yaml` | Format structuré |

## Structure des Tâches

```yaml
tasks:
  - id: "T001"
    titre: "Implémenter rate limiting"
    description: "Ajouter rate limiting sur /api/auth/login"
    status: "todo"
    effort: "2h"
    fichiers:
      - "src/auth/login.ts"
    risques_adresses:
      - "RISK-AUTH-001"
    depends_on: []
    instructions: |
      1. Installer express-rate-limit
      2. Configurer 5 req/min par IP
      3. Ajouter tests
    criteres_done:
      - "Rate limiter actif"
      - "Tests passent"
```

## Prérequis

- `/osk-plan <feature>` exécuté
- `.osk/specs/NNN-feature/plan.md` présent

## Prochaine étape

```bash
>>> /osk-implement "authentication"
```
