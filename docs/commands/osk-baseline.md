# /osk-baseline

État des lieux sécurité pour projets existants.

## Synopsis

```bash
>>> /osk-baseline
```

## Description

`/osk-baseline` effectue un inventaire complet de sécurité et une analyse STRIDE au niveau système. Utilisez cette commande pour les projets existants avant d'analyser les features individuelles.

## Actions

1. **Inventaire des features** - Scan du code pour identifier les fonctionnalités
2. **Analyse STRIDE système** - Menaces au niveau architectural
3. **Création de la roadmap** - Plan de remédiation priorisé

## Fichiers Générés

| Fichier | Description |
|---------|-------------|
| `.osk/specs/000-baseline/inventory.md` | Inventaire des features |
| `.osk/specs/000-baseline/features.yaml` | Liste structurée |
| `.osk/specs/000-baseline/roadmap.md` | Roadmap de sécurité |

## Principes Couverts

Cette commande initialise les **Principes I et II à 30%** grâce à l'analyse STRIDE système.

| Principe | Couverture |
|----------|------------|
| I - Threat Modeling | 30% (système) |
| II - Risk Analysis | 30% (système) |

## Prérequis

- `/osk-configure` exécuté
- `.osk/memory/context.md` présent

## Prochaine étape

```bash
>>> /osk-analyze "feature-name"
```

Analysez chaque feature identifiée dans l'inventaire.
