# Changelog

Historique des versions OpenSecKit.

## Version 3.2.0 - 2025-12-24

### Nouvelles fonctionnalités

- **Commandes CLI scriptables**
    - `osk check <command>` : vérifier les prérequis d'une commande
    - `osk scaffold feature|incident|rgpd|rgs` : créer des structures de fichiers
    - `osk update stats|task|risk|dashboard` : mises à jour mécaniques
    - `osk validate yaml|deps|workflow` : valider la cohérence

- **Sortie JSON pour agents AI**
    - Flag `--json` sur toutes les commandes scriptables
    - Format structuré facilement parsable
    - Économie de tokens (~40-60%)

- **Détection de cycles dans les dépendances**
    - `osk validate deps <feature>` analyse le graphe de tâches
    - Algorithme DFS avec coloration
    - Détecte auto-références et cycles complexes
    - Avertit sur dépendances vers tâches inconnues

### Structures JSON

```json
// osk check --json configure
{"ready": true, "command": "...", "found": [...], "missing": [...]}

// osk scaffold --json feature "auth"
{"success": true, "created_dir": "...", "created_files": [...], "next_command": "..."}

// osk validate --json workflow "auth"
{"valid": false, "checked_files": [...], "error_count": 7, "next_command": "..."}
```

### Améliorations

- **Nouveaux modules utils**
    - `counter.rs` : numérotation auto-incrémentée des features
    - `git.rs` : helpers git (create_branch, commit, etc.)
    - `yaml.rs` : validation et manipulation YAML
    - `template.rs` : substitution de variables dans templates

- **Templates embarqués** pour scaffold feature
    - threats.md, risks.md, requirements.md
    - testing.md, hardening.md, plan.md, tasks.yaml

---

## Version 3.1.0 - 2025-12-23

### Nouvelles fonctionnalités

- **Support Multi-Agent**
    - Claude Code (slash commands)
    - GitHub Copilot (instructions)
    - Cursor (rules)
    - Gemini (instructions)
    - AGENTS.md (format universel)

- **Architecture data-driven**
    - `agents.toml` : configuration déclarative des agents
    - Templates Tera dans `templates/agents/`
    - Ajouter un agent = config + template (zéro code Rust)
    - 3 formats : `slash-command`, `single-file`, `rules-dir`

- **Sélection interactive d'agent**
    - `osk init` affiche un menu de sélection
    - Détection automatique des agents installés
    - Option `--agent <nom>` pour sélection directe
    - Option `--all-agents` pour tous les agents

- **Nouveaux modules CLI**
    - `agents.rs` : gestion multi-agent
    - `prompts.rs` : parsing des prompts (frontmatter, steps, outputs)

### Améliorations

- **Output condensé** lors de `osk init`
    - Résumé par module au lieu de liste de fichiers
    - Affichage clair des agents configurés

- **Nouvelles dépendances Rust**
    - `tera` : moteur de templates
    - `regex` : parsing frontmatter
    - `which` : détection agents

---

## Version 3.0.1 - 2025-12-23

### Nouvelles fonctionnalités

- **`/osk-implement`** - Nouvelle commande pour exécuter les tâches
    - Exécute les tâches de `tasks.yaml` une par une
    - Un commit par tâche (traçabilité maximale)
    - Mise à jour automatique du `risk-register.yaml`
    - Options : `--auto`, `--dry-run`, `--task T001`

- **Analyse STRIDE système** dans `/osk-baseline`
    - Principes I et II démarrent à 30%
    - Identification précoce des menaces architecturales

### Améliorations

- **Risk Register - Source unique**
    - `docs/security/risks/risk-register.yaml` = SOURCE UNIQUE
    - Les fichiers `.osk/specs/*/risks.md` sont des vues générées

- **Architecture Templates refactorisée**
    - Prompts réduits de ~800 à ~100 lignes
    - Templates séparés : schemas/, outputs/, reports/
    - Réduction totale : -88%

- **Dashboard étendu**
    - Scanne tous les dossiers `docs/security/*`
    - Inclut conformité RGPD, RGS, continuité

### Changements

- `/osk-resolve` remplacé par `/osk-implement`

---

## Version 3.0.0 - 2025-12-23

### Breaking changes

- **Refonte des commandes**
    - ❌ Supprimées : `/security`, `/audit`
    - ✅ Nouvelles : `/osk-configure`, `/osk-baseline`, `/osk-analyze`, `/osk-specify`, `/osk-harden`, `/osk-plan`, `/osk-tasks`

- **Structure des fichiers**
    - Avant : `docs/security/features/SEC-*.md`
    - Après : `.osk/specs/NNN-feature/` + `docs/security/`

### Nouvelles fonctionnalités

- **Workflow orienté feature**
- **CLI `--default`** pour mode non-interactif
- **Scripts de test** : `test-local.sh`, `check-links.sh`

---

## Version 2.0.0 - 2025-01-15

### Breaking changes

- Réduction des commandes : 6 → 4
- Nouveau Risk Register centralisé

### Nouvelles fonctionnalités

- `/security` - Analyse constitutionnelle complète
- `/audit` - Vérification conformité
- `/dashboard` - Vue consolidée
- Risk Register avec scoring et lifecycle

---

## Liens

- [GitHub Releases](https://github.com/Scttpr/OpenSecKit/releases)
- [Guide de migration](getting-started/installation.md)
