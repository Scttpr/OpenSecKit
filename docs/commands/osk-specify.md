# /osk-specify

Définition des exigences de sécurité et stratégie de tests.

## Synopsis

```bash
>>> /osk-specify <feature>
```

## Description

`/osk-specify` définit les exigences de sécurité et la stratégie de tests pour une feature analysée.

## Arguments

| Argument | Description | Exemple |
|----------|-------------|---------|
| `feature` | Nom de la feature | `"authentication"` |

## Fichiers Générés

| Fichier | Description |
|---------|-------------|
| `.osk/specs/NNN-feature/requirements.md` | Exigences de sécurité |
| `.osk/specs/NNN-feature/testing.md` | Stratégie de tests |

## Principes Couverts

| Principe | Description |
|----------|-------------|
| III - Security by Design | Exigences de sécurité |
| IV - Security Testing | Stratégie SAST/DAST/SCA |

## Prérequis

- `/osk-analyze <feature>` exécuté
- `.osk/specs/NNN-feature/threats.md` présent
- `.osk/specs/NNN-feature/risks.md` présent

## Prochaine étape

```bash
>>> /osk-harden "authentication"
```
