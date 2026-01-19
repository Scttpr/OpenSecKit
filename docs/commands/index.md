# Commandes

OpenSecKit V4 organise les commandes en **3 phases** : Discover → Comply → Secure.

## Vue d'ensemble V4

```mermaid
graph TB
    subgraph "Part 1: DISCOVER"
        A[osk init] --> B[/osk-discover]
        B --> E[/osk-discover validate]
    end

    subgraph "Part 2: COMPLY"
        E --> F[/osk-comply rgpd]
        E --> G[/osk-comply rgs]
        E --> H[/osk-comply nis2]
    end

    subgraph "Part 3: SECURE"
        E --> I[/osk-secure specify]
        I --> J[/osk-secure clarify]
        J --> K[/osk-secure plan]
        K --> L[/osk-secure tasks]
        L --> M[/osk-secure implement]
    end

    B -.-> |system-model| F
    B -.-> |system-model| I
```

## Part 1: Discover

Analyse automatique du codebase pour créer le **system-model**.

| Commande | Description | Output |
|----------|-------------|--------|
| [`/osk-discover`](osk-discover.md) | Build/update model (adaptive) | `system-model/` |
| [`/osk-discover validate`](osk-discover-validate.md) | Validation (ou `--resolve`) | Rapport de gaps |
| [`/osk-discover docs`](osk-discover-docs.md) | Documentation (`--audience`) | `docs/*.md` |

## Part 2: Comply

Évaluation de conformité aux référentiels.

| Commande | Framework | Output |
|----------|-----------|--------|
| [`/osk-comply rgpd`](osk-comply-rgpd.md) | RGPD | Assessment, registre traitements |
| [`/osk-comply rgs`](osk-comply-rgs.md) | RGS | Assessment, dossier homologation |
| `/osk-comply nis2` | NIS2 | Assessment (à venir) |
| `/osk-comply iso27001` | ISO 27001 | Assessment (à venir) |
| `/osk-comply soc2` | SOC 2 | Assessment (à venir) |
| `/osk-comply list` | - | Liste des frameworks disponibles |
| `/osk-comply status` | - | Statut de conformité global |

## Part 3: Secure

Workflow de sécurité par feature (style SpecKit).

| Commande | Description | Output |
|----------|-------------|--------|
| `/osk-secure specify` | Analyse STRIDE + exigences | `security-spec.md`, `threats.md` |
| `/osk-secure clarify` | Résolution ambiguïtés | Spec mise à jour |
| `/osk-secure plan` | Plan d'implémentation | `security-plan.md` |
| `/osk-secure tasks` | Tâches ordonnées | `tasks.yaml` |
| `/osk-secure implement` | Exécution + risk-register | Code + mise à jour registre |

## CLI Utilitaires

Commandes scriptables avec sortie JSON pour agents AI.

| Commande | Description | Utilisé par |
|----------|-------------|-------------|
| `osk init` | Initialise le projet | Setup initial |
| `osk scan --json` | Liste fichiers (respecte .gitignore) | discover init/update |
| `osk id <path>` | Génère ID composant | discover init |
| `osk changes --json` | Fichiers modifiés depuis dernier scan | discover update |
| `osk validate system-model` | Valide le system-model | discover validate |
| `osk check <command>` | Vérifie prérequis | Tous |
| `osk update <target>` | Mises à jour mécaniques | Tous |

## Structure des Fichiers

### `.osk/system-model/` (Part 1)

```
.osk/system-model/
├── index.yaml          # Vue d'ensemble (<200 lignes)
├── business.yaml       # Contexte métier
├── architecture.yaml   # Composants, services
├── data.yaml           # Inventaire données
├── actors.yaml         # Utilisateurs, systèmes
├── boundaries.yaml          # Frontières de confiance
├── integrations.yaml   # Intégrations externes
├── controls.yaml       # Contrôles existants
└── gaps.yaml           # Gaps identifiés
```

### `.osk/compliance/` (Part 2)

```
.osk/compliance/
├── assessment-rgpd.yaml
├── assessment-rgs.yaml
└── ...
```

### `.osk/specs/` (Part 3)

```
.osk/specs/
└── NNN-feature/
    ├── security-spec.md
    ├── threats.md
    ├── requirements.md
    ├── testing.md
    ├── hardening.md
    ├── plan.md
    ├── tasks.md
    └── tasks.yaml
```

## Utilisation

```bash
# Initialiser le projet
osk init

# Dans Claude Code / agent AI
>>> /osk-discover

# Évaluer conformité RGPD
>>> /osk-comply rgpd

# Sécuriser une feature
>>> /osk-secure specify "authentication"
```

## Migration V3 → V4

| Commande V3 | Commande V4 |
|-------------|-------------|
| `/osk-configure` | `/osk-discover` |
| `/osk-baseline` | `/osk-discover` |
| `/osk-analyze` | `/osk-secure specify` |
| `/osk-specify` | `/osk-secure specify` |
| `/osk-harden` | `/osk-secure specify` |
| `/osk-plan` | `/osk-secure plan` |
| `/osk-tasks` | `/osk-secure tasks` |
| `/osk-implement` | `/osk-secure implement` |
| `/osk-rgpd` | `/osk-comply rgpd` |
| `/osk-rgs` | `/osk-comply rgs` |
