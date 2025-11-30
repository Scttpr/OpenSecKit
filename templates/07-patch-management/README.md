# Modèles de Gestion des Correctifs

## Aperçu

Ce répertoire contient des modèles pour le **Principe Constitutionnel VII : Gestion des Correctifs** - identification et remédiation rapides des vulnérabilités dans les dépendances et l'infrastructure.

## Modèles disponibles

| Modèle | Description |
|--------|-------------|
| [guide-analyse-dependances-tous.md](dependency-scanning-guide-all.md) | Analyse de vulnérabilités SCA/dépendances |
| [patch-sla-policy-template.md](patch-sla-policy-template.md) | SLA pour correction des vulnérabilités |
| [emergency-patching-procedure.md](emergency-patching-procedure.md) | Processus et workflow de correction |

## Principes Clés

- ✅ **Analyser les dépendances régulièrement** (quotidien dans CI/CD)
- ✅ **Corriger rapidement les vulnérabilités critiques** (24 heures)
- ✅ **Automatiser autant que possible** (Dependabot, fusion automatique)
- ✅ **Suivre les métriques de correction** (temps moyen de correction)

## SLA de Remédiation

| Criticité | Délai |
|-----------|-------|
| Critique | 24 heures |
| Élevé | 7 jours |
| Moyen | 30 jours |
| Faible | 90 jours |

---

**Tous les Modèles de Base Complets !** Procéder à la création d'extensions par domaine et guides de processus.
