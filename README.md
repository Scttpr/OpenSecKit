# OpenSecKit V3

**Security as Code, AI-Ready**

OpenSecKit structure la sécurité de vos projets via 7 principes fondamentaux et des workflows guidés par IA.

## Installation

```bash
# CLI Rust
cargo install osk

# Initialiser un projet
osk init
```

## Workflow Principal

```
┌─────────────────────────────────────────────────────────────────┐
│                      WORKFLOW OPENSECKIT V3                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  osk init ──▶ /osk-configure ──▶ /osk-baseline (si code existant)│
│                     │                                           │
│                     ▼                                           │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │              POUR CHAQUE FEATURE                         │    │
│  │                                                         │    │
│  │  /osk-analyze ──▶ /osk-specify ──▶ /osk-harden          │    │
│  │       │                                                 │    │
│  │       ▼                                                 │    │
│  │  /osk-plan ──▶ /osk-tasks                               │    │
│  └─────────────────────────────────────────────────────────┘    │
│                     │                                           │
│                     ▼                                           │
│  ┌─────────────────────────────────────────────────────────┐    │
│  │              CONFORMITÉ RÉGLEMENTAIRE                    │    │
│  │                                                         │    │
│  │  /osk-rgpd ──────────────▶ docs/security/rgpd/          │    │
│  │  /osk-rgs  ──────────────▶ docs/security/rgs/           │    │
│  └─────────────────────────────────────────────────────────┘    │
│                     │                                           │
│                     ▼                                           │
│  /osk-dashboard (monitoring) + /osk-pca-pra (continuité)        │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

## Commandes

### Phase 0 : Configuration

| Commande | Description |
|----------|-------------|
| `osk init` | Crée `.osk/config.toml` et télécharge les templates |
| `/osk-configure` | Analyse le code, détecte les domaines, pondère les principes |
| `/osk-baseline` | État des lieux sécurité (projets avec code existant) |

### Phase 1-4 : Analyse par Feature

| Commande | Principes | Génère |
|----------|-----------|--------|
| `/osk-analyze [feature]` | I, II | `threats.md`, `risks.md`, brouillons DPIA/EBIOS |
| `/osk-specify [feature]` | III, IV | `requirements.md`, `testing.md` |
| `/osk-harden [feature]` | V, VI, VII | `hardening.md` |
| `/osk-plan [feature]` | Tous | `plan.md` |
| `/osk-tasks [feature]` | - | `tasks.md`, `tasks.yaml` |

### Phase 5 : Conformité

| Commande | Domaine | Génère |
|----------|---------|--------|
| `/osk-rgpd` | RGPD | Registre Art. 30, DPIA global, procédures |
| `/osk-rgpd audit` | RGPD | Rapport d'audit conformité |
| `/osk-rgs` | RGS | EBIOS RM, contexte homologation |
| `/osk-rgs renew` | RGS | Rapport de ré-homologation |

### Utilitaires

| Commande | Description |
|----------|-------------|
| `/osk-dashboard` | Vue consolidée de la posture sécurité |
| `/osk-pca-pra` | Plans de Continuité et Reprise d'Activité |

## Structure des Fichiers

```
projet/
├── .osk/                           # Espace de travail interne
│   ├── config.toml                 # Configuration (source de vérité)
│   ├── memory/
│   │   ├── context.md              # Faits techniques détectés
│   │   └── constitution.md         # Principes pondérés
│   ├── specs/                      # Brouillons par feature
│   │   └── NNN-feature/
│   │       ├── threats.md
│   │       ├── risks.md
│   │       ├── requirements.md
│   │       ├── testing.md
│   │       ├── hardening.md
│   │       ├── plan.md
│   │       ├── rgpd/dpia.md        # Brouillon DPIA
│   │       └── rgs/ebios.md        # Brouillon EBIOS
│   └── templates/                  # Templates téléchargés
│
└── docs/security/                  # Documentation finale (publiable)
    ├── risks/
    │   └── risk-register.yaml      # Registre central des risques
    ├── rgpd/
    │   ├── registre-traitements.md # Art. 30
    │   ├── dpia-global.md          # DPIA consolidé
    │   ├── procedure-violation.md  # Art. 33-34
    │   └── AUDIT-YYYY-MM-DD.md
    ├── rgs/
    │   ├── EBIOS-RM-[SYSTEME].md   # EBIOS consolidé
    │   ├── DOSSIER-HOMOLOGATION.md
    │   └── AUDIT-YYYY-MM-DD.md
    └── continuity/
        ├── PCA-*.md
        └── PRA-*.md
```

## Les 7 Principes

| # | Principe | Description |
|---|----------|-------------|
| I | Threat Modeling | Analyse proactive des menaces (STRIDE) |
| II | Risk Analysis | Scoring et priorisation des risques |
| III | Security by Design | Exigences de sécurité dès la conception |
| IV | Security Testing | Tests SAST/DAST/SCA automatisés |
| V | Secrets Management | Aucun secret dans le code |
| VI | Audit Logging | Logs immuables et centralisés |
| VII | Patch Management | SLA stricts de mise à jour |

## Domaines Réglementaires

| Domaine | Description | Commande |
|---------|-------------|----------|
| RGPD | Protection des données personnelles | `/osk-rgpd` |
| RGS | Référentiel Général de Sécurité (France) | `/osk-rgs` |
| NIS2 | Directive cybersécurité UE | *(en cours)* |

## Liens

- [Structure des fichiers](schemas/FILE-STRUCTURE.md)
- [Les 7 principes](constitution.md)
- [Domaines réglementaires](domaines/README.md)
