# Structure des Fichiers

Organisation des fichiers générés par OpenSecKit.

## Vue d'ensemble

```
projet/
├── .osk/                           # Espace de travail interne
│   ├── config.toml                 # Configuration projet
│   ├── registry.toml               # Registre des commandes
│   ├── agents.toml                 # Configuration multi-agent
│   ├── memory/                     # Mémoire contextuelle
│   │   ├── context.md              # Faits techniques
│   │   └── constitution.md         # Principes pondérés
│   ├── specs/                      # Brouillons par feature
│   │   └── NNN-feature/
│   │       ├── threats.md
│   │       ├── risks.md
│   │       ├── requirements.md
│   │       ├── testing.md
│   │       ├── hardening.md
│   │       ├── plan.md
│   │       └── tasks.yaml
│   ├── templates/                  # Templates
│   │   ├── agents/                # Templates Tera par agent
│   │   ├── schemas/
│   │   ├── outputs/
│   │   └── reports/
│   ├── prompts/                    # Prompts sources
│   └── domaines/                   # Domaines réglementaires
│
├── AGENTS.md                       # Format universel (tous agents)
│
├── .claude/commands/               # Claude Code
│   ├── osk-configure.md
│   └── ...
│
├── .github/                        # GitHub Copilot
│   └── copilot-instructions.md
│
├── .cursor/rules/                  # Cursor
│   ├── osk-analyze.md
│   └── ...
│
├── .gemini/                        # Gemini
│   └── instructions.md
│
└── docs/security/                  # Documentation finale
    ├── risks/
    │   └── risk-register.yaml      # ★ SOURCE UNIQUE ★
    ├── rgpd/
    │   ├── registre-traitements.md
    │   ├── dpia-global.md
    │   └── AUDIT-*.md
    ├── rgs/
    │   ├── EBIOS-RM-*.md
    │   ├── DOSSIER-HOMOLOGATION-*.md
    │   └── MCS-*.md
    ├── continuity/
    │   ├── PCA-*.md
    │   └── PRA-*.md
    └── dashboard.md
```

## `.osk/` - Espace Interne

### config.toml

Configuration du projet générée par `osk init`.

### memory/

Mémoire contextuelle pour les LLMs :

- `context.md` - Faits techniques détectés par `/osk-configure`
- `constitution.md` - Principes pondérés selon le contexte

### specs/

Brouillons de spécifications par feature :

| Fichier | Commande | Description |
|---------|----------|-------------|
| `threats.md` | `/osk-analyze` | Analyse STRIDE |
| `risks.md` | `/osk-analyze` | Vue des risques |
| `requirements.md` | `/osk-specify` | Exigences |
| `testing.md` | `/osk-specify` | Stratégie tests |
| `hardening.md` | `/osk-harden` | Durcissement |
| `plan.md` | `/osk-plan` | Plan consolidé |
| `tasks.yaml` | `/osk-tasks` | Tâches ordonnées |

### templates/

Templates utilisés par les prompts :

- `schemas/` - Structures YAML
- `outputs/` - Templates de fichiers générés
- `reports/` - Rapports terminaux

## `docs/security/` - Documentation Finale

### risks/

**risk-register.yaml** - Source unique de vérité pour les risques.

!!! warning "Important"
    Les fichiers `.osk/specs/*/risks.md` sont des **vues générées**. Ne les modifiez pas directement.

### rgpd/

Documentation RGPD :

- `registre-traitements.md` - Art. 30
- `dpia-global.md` - Art. 35
- `AUDIT-YYYY-MM-DD.md` - Rapports d'audit

### rgs/

Documentation RGS :

- `EBIOS-RM-{systeme}.md` - Analyse de risques
- `DOSSIER-HOMOLOGATION-{systeme}.md` - Homologation
- `MCS-{systeme}.md` - Maintien en condition

### continuity/

Plans de continuité :

- `PCA-*.md` - Plan de Continuité d'Activité
- `PRA-*.md` - Plan de Reprise d'Activité

## Fichiers Agent

Générés automatiquement par `osk init` selon l'agent sélectionné.

### AGENTS.md

Format universel compatible avec tous les agents AI. Toujours généré.

### .claude/commands/

Slash commands pour Claude Code (un fichier par commande).

### .github/copilot-instructions.md

Instructions consolidées pour GitHub Copilot.

### .cursor/rules/

Règles de projet pour Cursor (un fichier par commande).

### .gemini/instructions.md

Instructions pour Google Gemini.

!!! tip "Mise à jour"
    Utilisez `osk init --force` pour mettre à jour les fichiers agent.
