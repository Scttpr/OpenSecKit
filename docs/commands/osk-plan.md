# /osk-plan

Plan d'implémentation consolidé.

## Synopsis

```bash
>>> /osk-plan <feature>
```

## Description

`/osk-plan` consolide toutes les analyses en un plan d'implémentation actionnable.

## Arguments

| Argument | Description | Exemple |
|----------|-------------|---------|
| `feature` | Nom de la feature | `"authentication"` |

## Fichiers Générés

| Fichier | Description |
|---------|-------------|
| `.osk/specs/NNN-feature/plan.md` | Plan consolidé |

## Prérequis

- `/osk-harden <feature>` exécuté
- Tous les fichiers de spec présents

## Prochaine étape

```bash
>>> /osk-tasks "authentication"
```
