# Quickstart

Ce guide vous accompagne dans l'analyse de sécurité de votre première feature.

## 1. Configuration initiale

Après `osk init`, lancez Claude Code et configurez le projet :

```bash
claude
>>> /osk-configure
```

Cette commande :

- Analyse votre code source
- Détecte les domaines applicables (RGPD, RGS...)
- Pondère les 7 principes selon votre contexte
- Génère `.osk/memory/context.md` et `.osk/memory/constitution.md`

## 2. État des lieux (projets existants)

Pour un projet existant, faites un baseline :

```bash
>>> /osk-baseline
```

Cette commande :

- Inventorie les features existantes
- Effectue une analyse STRIDE au niveau système
- Crée une roadmap de sécurité

## 3. Analyser une feature

```bash
>>> /osk-analyze "authentication"
```

Génère :

- `.osk/specs/001-authentication/threats.md` - Menaces STRIDE
- `.osk/specs/001-authentication/risks.md` - Risques scorés
- `docs/security/risks/risk-register.yaml` - Mise à jour du registre

## 4. Définir les exigences

```bash
>>> /osk-specify "authentication"
```

Génère :

- `.osk/specs/001-authentication/requirements.md` - Exigences de sécurité
- `.osk/specs/001-authentication/testing.md` - Stratégie de tests

## 5. Durcissement

```bash
>>> /osk-harden "authentication"
```

Génère :

- `.osk/specs/001-authentication/hardening.md` - Mesures de durcissement

## 6. Planification

```bash
>>> /osk-plan "authentication"
>>> /osk-tasks "authentication"
```

Génère :

- `.osk/specs/001-authentication/plan.md` - Plan consolidé
- `.osk/specs/001-authentication/tasks.yaml` - Tâches ordonnées

## 7. Implémentation

```bash
>>> /osk-implement "authentication"
```

Exécute les tâches une par une et met à jour automatiquement le risk-register.

## 8. Monitoring

```bash
>>> /osk-dashboard
```

Affiche le tableau de bord de sécurité avec :

- Score de conformité aux 7 principes
- État des risques
- Recommandations prioritaires

## Workflow complet

```mermaid
graph TD
    A[osk init] --> B[/osk-configure]
    B --> C{Projet existant?}
    C -->|Oui| D[/osk-baseline]
    C -->|Non| E[/osk-analyze feature]
    D --> E
    E --> F[/osk-specify feature]
    F --> G[/osk-harden feature]
    G --> H[/osk-plan feature]
    H --> I[/osk-tasks feature]
    I --> J[/osk-implement feature]
    J --> K[/osk-dashboard]
    K --> L{Autre feature?}
    L -->|Oui| E
    L -->|Non| M[Conformité]
    M --> N[/osk-rgpd]
    M --> O[/osk-rgs]
```

## Prochaines étapes

- [:octicons-arrow-right-24: Comprendre les 7 Principes](../concepts/principles.md)
- [:octicons-arrow-right-24: Référence des commandes](../commands/index.md)
- [:octicons-arrow-right-24: Conformité RGPD](../domains/rgpd.md)
