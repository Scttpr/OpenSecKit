# /osk-configure

Configuration intelligente du projet.

## Synopsis

```bash
>>> /osk-configure
```

## Description

`/osk-configure` analyse votre code source et configure les principes de sécurité selon le contexte du projet.

## Actions

1. **Scan du code** - Détecte la stack technique (langages, frameworks, bases de données)
2. **Détection des domaines** - Identifie les réglementations applicables (RGPD, RGS, NIS2)
3. **Pondération des principes** - Ajuste l'importance de chaque principe selon le contexte
4. **Génération du contexte** - Crée les fichiers de mémoire

## Fichiers Générés

| Fichier | Description |
|---------|-------------|
| `.osk/memory/context.md` | Faits techniques détectés |
| `.osk/memory/constitution.md` | Principes pondérés |

## Exemple de sortie

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
/osk-configure - Configuration Terminée
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

STACK DÉTECTÉE
├── Langages  : TypeScript, Python
├── Frameworks: Express, FastAPI
├── Bases     : PostgreSQL, Redis
└── Cloud     : AWS

DOMAINES ACTIVÉS
├── RGPD      : ✅ (données personnelles détectées)
├── RGS       : ❌ (pas de pattern gouv.fr)
└── NIS2      : ❌

PONDÉRATION DES PRINCIPES
├── I   Threat Modeling     : HIGH
├── II  Risk Analysis       : HIGH
├── III Security by Design  : HIGH
├── IV  Security Testing    : HIGH
├── V   Secrets Management  : HIGH
├── VI  Audit Logging       : MEDIUM
└── VII Patch Management    : HIGH

FICHIERS GÉNÉRÉS
├── .osk/memory/context.md
└── .osk/memory/constitution.md

PROCHAINE ÉTAPE
→ /osk-baseline (projet existant)
→ /osk-analyze "feature" (nouveau projet)
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## Prérequis

- `osk init` exécuté
- `.osk/config.toml` présent

## Prochaine étape

=== "Projet existant"
    ```bash
    >>> /osk-baseline
    ```

=== "Nouveau projet"
    ```bash
    >>> /osk-analyze "ma-feature"
    ```
