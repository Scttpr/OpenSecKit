# /osk-harden

Mesures de durcissement pour une feature.

## Synopsis

```bash
>>> /osk-harden <feature>
```

## Description

`/osk-harden` définit les mesures de durcissement couvrant les secrets, le logging et le patch management.

## Arguments

| Argument | Description | Exemple |
|----------|-------------|---------|
| `feature` | Nom de la feature | `"authentication"` |

## Fichiers Générés

| Fichier | Description |
|---------|-------------|
| `.osk/specs/NNN-feature/hardening.md` | Mesures de durcissement |

## Principes Couverts

| Principe | Description |
|----------|-------------|
| V - Secrets Management | Gestion des secrets |
| VI - Audit Logging | Traçabilité |
| VII - Patch Management | Mises à jour |

## Prérequis

- `/osk-specify <feature>` exécuté
- `.osk/specs/NNN-feature/requirements.md` présent

## Prochaine étape

```bash
>>> /osk-plan "authentication"
```
