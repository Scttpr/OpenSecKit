# osk init

Initialise un projet avec OpenSecKit.

## Synopsis

```bash
osk init [OPTIONS]
```

## Description

`osk init` prépare votre projet pour utiliser OpenSecKit :

1. Crée la structure `.osk/`
2. Télécharge les templates et prompts
3. Détecte automatiquement la stack technique
4. Configure l'agent AI de votre choix (Claude Code, Copilot, Cursor, Gemini)

## Options

| Option | Description |
|--------|-------------|
| `--force`, `-f` | Force la mise à jour des ressources |
| `--default`, `-d` | Mode non-interactif avec valeurs par défaut (pour CI) |
| `--agent <AGENT>`, `-a` | Agent AI cible : `claude-code`, `copilot`, `cursor`, `gemini` |
| `--all-agents` | Installe la configuration pour tous les agents |

## Agents Supportés

| Agent | Fichiers générés |
|-------|------------------|
| `claude-code` | `.claude/commands/*.md` (slash commands) |
| `copilot` | `.github/copilot-instructions.md` |
| `cursor` | `.cursor/rules/*.md` (une règle par commande) |
| `gemini` | `.gemini/instructions.md` |

En plus des fichiers spécifiques à chaque agent, `AGENTS.md` est généré à la racine du projet (format universel compatible avec tous les agents).

## Exemples

```bash
# Installation interactive (sélection d'agent)
cd mon-projet/
osk init

# Installation avec agent spécifique
osk init --agent copilot
osk init --agent cursor

# Installation pour tous les agents
osk init --all-agents

# Mode CI (non-interactif, Claude Code par défaut)
osk init --default

# Mise à jour forcée
osk init --force --agent claude-code
```

## Sélection Interactive

Sans l'option `--agent`, une sélection interactive s'affiche :

```
? Quel agent AI voulez-vous configurer ?
❯ Claude Code ✓
  GitHub Copilot ✓
  Cursor (non détecté)
  Gemini (non détecté)
```

Les agents installés sur le système sont marqués avec ✓.

## Output

L'installation affiche un résumé condensé par module :

```
🚀 Initialisation de OpenSecKit v3.1.0

   📦 Modules chargés:
      ✓ Core          13 prompts, 11 schemas, 12 outputs
      ✓ RGPD          8 fichiers
      ✓ RGS           12 fichiers

   🤖 Agent(s) configuré(s):
      ✓ Claude Code → .claude/commands/ (13 slash commands)

✅ OpenSecKit prêt !
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

### Fichiers Agent (selon choix)

```
# Claude Code
.claude/commands/*.md

# GitHub Copilot
.github/copilot-instructions.md

# Cursor
.cursor/rules/*.md

# Gemini
.gemini/instructions.md

# Universel (toujours généré)
AGENTS.md
```

## Comportement avec `--force`

| Fichiers | Comportement |
|----------|--------------|
| `.osk/prompts/` | ✅ Écrasé |
| `.osk/templates/` | ✅ Écrasé |
| Fichiers agent | ✅ Écrasé |
| `.osk/config.toml` | ✅ Re-demandé |
| `docs/security/` | ❌ Préservé |
| `.osk/memory/` | ❌ Préservé |
| `.osk/specs/` | ❌ Préservé |

!!! warning "Attention"
    Si vous avez modifié manuellement les fichiers agent, ces modifications seront perdues avec `--force`.

## Prochaine étape

Après `osk init`, lancez votre agent AI et configurez le projet :

=== "Claude Code"
    ```bash
    claude
    >>> /osk-configure
    ```

=== "GitHub Copilot"
    ```
    Ouvrez VS Code avec GitHub Copilot et demandez :
    "Analyse la sécurité de ce projet avec OpenSecKit"
    ```

=== "Cursor"
    ```
    Ouvrez Cursor, les règles OSK sont chargées automatiquement
    ```
