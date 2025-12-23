# /osk-pca-pra

Plans de Continuité et Reprise d'Activité.

## Synopsis

```bash
>>> /osk-pca-pra
```

## Description

`/osk-pca-pra` génère les plans de continuité (PCA) et de reprise (PRA) d'activité.

## Fichiers Générés

| Fichier | Description |
|---------|-------------|
| `docs/security/continuity/PCA-*.md` | Plan de Continuité d'Activité |
| `docs/security/continuity/PRA-*.md` | Plan de Reprise d'Activité |

## Contenu

### PCA - Plan de Continuité d'Activité

- Processus critiques identifiés
- Scénarios de sinistre
- Procédures de basculement
- Communication de crise

### PRA - Plan de Reprise d'Activité

- RTO (Recovery Time Objective)
- RPO (Recovery Point Objective)
- Procédures de restauration
- Tests de reprise

## Prérequis

- `/osk-configure` exécuté
- `.osk/memory/context.md` présent
