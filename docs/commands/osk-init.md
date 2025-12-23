# osk init

Initialise un projet avec OpenSecKit.

## Synopsis

```bash
osk init [--force]
```

## Description

`osk init` prépare votre projet pour utiliser OpenSecKit :

1. Crée la structure `.osk/`
2. Télécharge les templates et prompts
3. Installe les slash commands dans `.claude/commands/`
4. Détecte automatiquement la stack technique

## Options

| Option | Description |
|--------|-------------|
| `--force`, `-f` | Force la mise à jour des ressources |

## Exemples

```bash
# Installation initiale
cd mon-projet/
osk init

# Mise à jour forcée
osk init --force
```

## Fichiers Créés

### Structure `.osk/`

```
.osk/
├── config.toml          # Configuration projet
├── registry.toml        # Registre des commandes
├── constitution.md      # Constitution sécurité
├── prompts/             # Prompts sources
├── templates/           # Templates
├── domaines/            # Domaines réglementaires
├── memory/              # Mémoire contextuelle
└── specs/               # Spécifications par feature
```

### Structure `.claude/`

```
.claude/
└── commands/            # Slash commands installés
    ├── osk-configure.md
    ├── osk-baseline.md
    ├── osk-analyze.md
    └── ...
```

## Comportement avec `--force`

| Fichiers | Comportement |
|----------|--------------|
| `.osk/prompts/` | ✅ Écrasé |
| `.osk/templates/` | ✅ Écrasé |
| `.claude/commands/` | ✅ Écrasé |
| `.osk/config.toml` | ✅ Re-demandé |
| `docs/security/` | ❌ Préservé |
| `.osk/memory/` | ❌ Préservé |
| `.osk/specs/` | ❌ Préservé |

!!! warning "Attention"
    Si vous avez modifié manuellement les slash commands dans `.claude/commands/`, ces modifications seront perdues avec `--force`.

## Prochaine étape

Après `osk init`, lancez Claude Code et configurez le projet :

```bash
claude
>>> /osk-configure
```
