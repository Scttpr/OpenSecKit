# /osk-analyze

Analyse des menaces et risques d'une feature.

## Synopsis

```bash
>>> /osk-analyze <feature>
```

## Description

`/osk-analyze` effectue une analyse STRIDE complète et génère les risques scorés pour une feature.

## Arguments

| Argument | Description | Exemple |
|----------|-------------|---------|
| `feature` | Nom de la feature à analyser | `"authentication"` |

## Actions

1. **Analyse STRIDE** - Identification des menaces par catégorie
2. **Scoring des risques** - Calcul Impact × Probabilité × Exposition
3. **Mise à jour du registre** - Ajout au `risk-register.yaml`

## Fichiers Générés

| Fichier | Description |
|---------|-------------|
| `.osk/specs/NNN-feature/threats.md` | Analyse STRIDE |
| `.osk/specs/NNN-feature/risks.md` | Risques scorés (vue) |
| `docs/security/risks/risk-register.yaml` | Registre mis à jour |

## Principes Couverts

| Principe | Description |
|----------|-------------|
| I - Threat Modeling | Analyse STRIDE |
| II - Risk Analysis | Scoring et priorisation |

## Exemple

```bash
>>> /osk-analyze "authentication"
```

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
/osk-analyze authentication - Analyse Terminée
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

MENACES IDENTIFIÉES (STRIDE)
├── Spoofing              : 3 menaces
├── Tampering             : 2 menaces
├── Repudiation           : 1 menace
├── Information Disclosure: 2 menaces
├── Denial of Service     : 2 menaces
└── Elevation of Privilege: 3 menaces

RISQUES AJOUTÉS AU REGISTRE
├── Critiques (P0-P1) : 2 (score: 180)
├── Importants (P2)   : 4 (score: 140)
└── Mineurs (P3-P4)   : 7 (score: 85)

TOP 3 RISQUES À TRAITER
1. RISK-AUTH-001 : Brute force login (Score: 100, P0)
2. RISK-AUTH-002 : Session fixation (Score: 80, P0)
3. RISK-AUTH-003 : Password in logs (Score: 60, P1)

PROCHAINES ÉTAPES
├── /osk-specify authentication
└── /osk-harden authentication
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## Prérequis

- `/osk-configure` exécuté
- `.osk/memory/context.md` présent

## Prochaine étape

```bash
>>> /osk-specify "authentication"
```
