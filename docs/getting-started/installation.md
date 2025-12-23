# Installation

## Prérequis

- [Rust](https://rustup.rs/) (1.70+)
- [Claude Code](https://claude.ai/claude-code) installé
- Git

## Installation du CLI

### Via Cargo (recommandé)

```bash
cargo install osk
```

### Depuis les sources

```bash
git clone https://github.com/Scttpr/OpenSecKit
cd OpenSecKit/cli
cargo install --path .
```

### Vérification

```bash
osk --version
```

## Initialiser un projet

```bash
cd mon-projet/
osk init
```

Cette commande :

1. Crée la structure `.osk/`
2. Télécharge les templates et prompts
3. Installe les slash commands dans `.claude/commands/`
4. Détecte automatiquement la stack technique

## Mise à jour

Pour mettre à jour les slash commands et templates :

```bash
osk init --force
```

!!! warning "Attention"
    L'option `--force` écrase les fichiers suivants :

    - `.osk/prompts/` - Prompts sources
    - `.osk/templates/` - Templates
    - `.claude/commands/` - Slash commands

    Vos documents dans `docs/security/` et `.osk/specs/` sont **préservés**.

## Structure créée

```
mon-projet/
├── .osk/
│   ├── config.toml          # Configuration projet
│   ├── registry.toml        # Registre des commandes
│   ├── prompts/             # Prompts sources
│   ├── templates/           # Templates
│   ├── domaines/            # Domaines réglementaires
│   ├── memory/              # Mémoire contextuelle
│   └── specs/               # Spécifications par feature
│
├── .claude/
│   └── commands/            # Slash commands installés
│       ├── osk-configure.md
│       ├── osk-analyze.md
│       └── ...
│
└── docs/security/           # Documentation générée
    ├── risks/
    ├── rgpd/
    ├── rgs/
    └── continuity/
```

## Prochaine étape

Une fois installé, lancez Claude Code et commencez par configurer votre projet :

```bash
claude
>>> /osk-configure
```

[:octicons-arrow-right-24: Guide Quickstart](quickstart.md)
