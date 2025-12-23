# Plan Multi-Agent OpenSecKit

> Migration d'une architecture mono-agent (Claude Code) vers une architecture multi-agent inspirée de GitHub SpecKit.

## Contexte

### État Actuel (v3.0.1)

OpenSecKit est aujourd'hui **fortement couplé à Claude Code** :

```
.claude/
├── commands/           # Slash commands (13 fichiers)
│   ├── osk-configure.md
│   ├── osk-analyze.md
│   └── ...
└── settings.local.json # Permissions outils
```

**Points de couplage identifiés :**

| Fichier | Dépendance | Impact |
|---------|------------|--------|
| `cli/src/commands/init.rs` | Hardcode `.claude/commands/` | Bloquant |
| `cli/src/commands/init.rs` | Hardcode `.claude/settings.local.json` | Bloquant |
| `prompts/*.md` | Syntaxe `/osk-*` dans les prompts | Moyen |
| `README.md` | Instructions spécifiques Claude | Faible |

**Points forts existants (réutilisables) :**

- Templates YAML/Handlebars → 100% agent-agnostiques
- Registry déclaratif (`registry.toml`) → Extensible
- Schemas de données → Réutilisables
- Workflow en 7 principes → Mappable à des rôles

---

## Architecture Cible

### Vue d'Ensemble

```
.osk/
├── registry.toml              # Enrichi avec support multi-agent
├── config.toml                # Configuration projet (inchangé)
├── memory/                    # Contexte persistant (inchangé)
├── specs/                     # Spécifications features (inchangé)
├── agents/                    # NOUVEAU: Définitions agents
│   ├── AGENTS.md              # Format universel
│   ├── roles.toml             # Rôles sécurité
│   ├── claude-code.yaml       # Adapter Claude Code
│   ├── copilot.yaml           # Adapter GitHub Copilot
│   ├── cursor.yaml            # Adapter Cursor
│   └── gemini.yaml            # Adapter Gemini
└── prompts/                   # Sources prompts (inchangé)

# Fichiers générés par agent
.claude/commands/*.md          # Si --agent=claude-code
.github/copilot-instructions.md # Si --agent=copilot
.cursor/rules/*.md             # Si --agent=cursor
```

### Modèle de Rôles Sécurité

Inspiré de SpecKit mais adapté au domaine sécurité :

| Rôle | Responsabilité | Commandes | Principes |
|------|----------------|-----------|-----------|
| **Risk Owner** | Gouvernance, configuration initiale | `/osk-configure`, `/osk-baseline` | - |
| **Security Architect** | Threat modeling, analyse risques | `/osk-analyze`, `/osk-specify` | I, II, III |
| **Security Engineer** | Exigences, hardening | `/osk-harden`, `/osk-plan`, `/osk-tasks` | IV, V, VI, VII |
| **Security Reviewer** | Implémentation, vérification | `/osk-implement` | Tous |
| **Privacy Analyst** | RGPD, protection données | `/osk-rgpd`, `/osk-pca-pra` | RGPD |
| **Compliance Officer** | RGS, NIS2, dashboard | `/osk-rgs`, `/osk-dashboard` | RGS, NIS2 |
| **SecOps Engineer** | Incidents, opérations | `/osk-incident` | Ops |

---

## Plan de Migration

### Phase 1 : Abstraction Agent (v3.1)

**Objectif :** Permettre l'initialisation pour différents agents sans casser la rétrocompatibilité.

#### Tâche 1.1 : Sélection Interactive d'Agent (comme SpecKit)

**Objectif :** Au lieu d'un simple flag `--agent`, proposer une sélection interactive avec `dialoguer::Select`.

**Fichier :** `cli/src/args.rs`

```rust
// AVANT
#[derive(Subcommand)]
pub enum Commands {
    Init {
        #[arg(long, short)]
        force: bool,
        #[arg(long, short)]
        default: bool,
    },
    // ...
}

// APRÈS
#[derive(Clone, Copy, Debug, PartialEq, clap::ValueEnum)]
pub enum Agent {
    ClaudeCode,
    Copilot,
    Cursor,
    Gemini,
}

#[derive(Subcommand)]
pub enum Commands {
    Init {
        #[arg(long, short)]
        force: bool,
        #[arg(long, short)]
        default: bool,
        /// Agent cible (si non spécifié, sélection interactive)
        #[arg(long, short)]
        agent: Option<Agent>,
        /// Installer tous les agents
        #[arg(long)]
        all_agents: bool,
    },
    // ...
}
```

**Fichier :** `cli/src/commands/init.rs`

```rust
use dialoguer::{theme::ColorfulTheme, Select};

// Détection des agents installés (comme SpecKit)
fn detect_available_agents() -> Vec<(&'static str, &'static str, bool)> {
    vec![
        ("Claude Code", "claude", which::which("claude").is_ok()),
        ("GitHub Copilot", "copilot", Path::new(".github").exists() || which::which("gh").is_ok()),
        ("Cursor", "cursor", which::which("cursor").is_ok() || Path::new(".cursor").exists()),
        ("Gemini", "gemini", which::which("gemini").is_ok()),
    ]
}

fn prompt_agent_selection() -> Result<Agent> {
    let agents = detect_available_agents();

    let items: Vec<String> = agents
        .iter()
        .map(|(name, _, available)| {
            if *available {
                format!("{} ✓", name)
            } else {
                format!("{} (non détecté)", name)
            }
        })
        .collect();

    let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Quel agent AI voulez-vous configurer ?")
        .items(&items)
        .default(0)
        .interact()?;

    match selection {
        0 => Ok(Agent::ClaudeCode),
        1 => Ok(Agent::Copilot),
        2 => Ok(Agent::Cursor),
        3 => Ok(Agent::Gemini),
        _ => Ok(Agent::ClaudeCode),
    }
}

pub fn run(client: &Client, force: bool, default: bool, agent: Option<Agent>, all_agents: bool) -> Result<()> {
    println!("🚀 Initialisation de OpenSecKit...\n");

    // Sélection de l'agent
    let selected_agents = if all_agents {
        vec![Agent::ClaudeCode, Agent::Copilot, Agent::Cursor, Agent::Gemini]
    } else if let Some(a) = agent {
        vec![a]
    } else if default {
        vec![Agent::ClaudeCode] // Défaut pour CI
    } else {
        vec![prompt_agent_selection()?]
    };

    // ... reste du code
}
```

**Sous-tâches :**

- [ ] Ajouter enum `Agent` dans `cli/src/args.rs` avec `#[derive(clap::ValueEnum)]`
- [ ] Ajouter `agent: Option<Agent>` et `all_agents: bool` à `Init`
- [ ] Ajouter dépendance `which = "6.0"` dans `Cargo.toml`
- [ ] Implémenter `detect_available_agents()`
- [ ] Implémenter `prompt_agent_selection()` avec `dialoguer::Select`
- [ ] Modifier signature de `run()` pour accepter les nouveaux paramètres
- [ ] Mettre à jour `commands/mod.rs` pour passer les paramètres

#### Tâche 1.2 : Output Condensé par Module

**Objectif :** Remplacer la liste verbeuse de fichiers par un résumé clair par module chargé.

**AVANT (verbose et illisible) :**
```
   📜 Registre des commandes mis à jour.
   ⚠️  Echec prompt 'prompts/osk-configure.md': ...
   ⚠️  Echec ressource 'templates/schemas/risk.yaml': ...
   ... (des dizaines de lignes)
```

**APRÈS (condensé et clair) :**
```
🚀 Initialisation de OpenSecKit v3.1.0

   📦 Modules chargés:
      ✓ Core          13 prompts, 8 schemas
      ✓ RGPD          3 templates
      ✓ RGS           2 templates
      ✓ NIS2          1 template

   🤖 Agent configuré: Claude Code
      → .claude/commands/ (13 slash commands)

✅ OpenSecKit prêt !
   Lancez: claude puis /osk-configure
```

**Fichier :** `cli/src/commands/init.rs`

```rust
#[derive(Default)]
struct InstallStats {
    prompts: usize,
    schemas: usize,
    outputs: usize,
    rgpd: usize,
    rgs: usize,
    nis2: usize,
    errors: Vec<String>,
}

fn install_resources(client: &Client, force: bool) -> Result<InstallStats> {
    let mut stats = InstallStats::default();

    // ... téléchargement des fichiers ...

    // Compter par catégorie au lieu de logger chaque fichier
    if item.path.starts_with("prompts/") {
        stats.prompts += 1;
    } else if item.path.starts_with("templates/schemas/") {
        stats.schemas += 1;
    } else if item.path.starts_with("templates/outputs/") {
        stats.outputs += 1;
    } else if item.path.starts_with("domaines/rgpd/") {
        stats.rgpd += 1;
    } else if item.path.starts_with("domaines/gouvernement-rgs/") {
        stats.rgs += 1;
    } else if item.path.starts_with("domaines/nis2/") {
        stats.nis2 += 1;
    }

    Ok(stats)
}

fn print_install_summary(stats: &InstallStats, agent: &Agent) {
    println!("\n   📦 Modules chargés:");
    println!("      ✓ Core          {} prompts, {} schemas", stats.prompts, stats.schemas);

    if stats.rgpd > 0 {
        println!("      ✓ RGPD          {} templates", stats.rgpd);
    }
    if stats.rgs > 0 {
        println!("      ✓ RGS           {} templates", stats.rgs);
    }
    if stats.nis2 > 0 {
        println!("      ✓ NIS2          {} templates", stats.nis2);
    }

    println!("\n   🤖 Agent configuré: {}", agent.display_name());
    match agent {
        Agent::ClaudeCode => println!("      → .claude/commands/ ({} slash commands)", stats.prompts),
        Agent::Copilot => println!("      → .github/copilot-instructions.md"),
        Agent::Cursor => println!("      → .cursor/rules/ ({} règles)", stats.prompts),
        Agent::Gemini => println!("      → .gemini/instructions.md"),
    }

    if !stats.errors.is_empty() {
        println!("\n   ⚠️  {} erreurs (voir --verbose pour détails)", stats.errors.len());
    }
}
```

**Sous-tâches :**

- [ ] Créer struct `InstallStats` pour collecter les compteurs
- [ ] Modifier `install_resources()` pour retourner `InstallStats`
- [ ] Supprimer les `println!` individuels par fichier
- [ ] Créer `print_install_summary()` pour affichage condensé
- [ ] Ajouter flag `--verbose` pour mode debug (optionnel)
- [ ] Implémenter `Agent::display_name()` pour affichage propre

#### Tâche 1.3 : Installation par Agent

**Objectif :** Chaque agent a sa propre fonction d'installation.

**Fichier :** `cli/src/commands/init.rs`

```rust
fn install_agent_config(agent: &Agent, stats: &InstallStats) -> Result<()> {
    match agent {
        Agent::ClaudeCode => install_claude_code(stats),
        Agent::Copilot => install_copilot(stats),
        Agent::Cursor => install_cursor(stats),
        Agent::Gemini => install_gemini(stats),
    }
}

fn install_claude_code(stats: &InstallStats) -> Result<()> {
    // Code existant: copier .osk/prompts/*.md → .claude/commands/*.md
    install_slash_commands(false)?;
    Ok(())
}

fn install_copilot(stats: &InstallStats) -> Result<()> {
    fs::create_dir_all(".github")?;
    let content = generate_copilot_instructions(stats)?;
    fs::write(".github/copilot-instructions.md", content)?;
    Ok(())
}

fn install_cursor(stats: &InstallStats) -> Result<()> {
    fs::create_dir_all(".cursor/rules")?;
    for prompt in &stats.prompt_names {
        let content = generate_cursor_rule(prompt)?;
        fs::write(format!(".cursor/rules/{}.md", prompt), content)?;
    }
    Ok(())
}

fn install_gemini(stats: &InstallStats) -> Result<()> {
    fs::create_dir_all(".gemini")?;
    let content = generate_gemini_instructions(stats)?;
    fs::write(".gemini/instructions.md", content)?;
    Ok(())
}
```

**Sous-tâches :**

- [ ] Créer `install_agent_config()` dispatcher
- [ ] Refactorer `install_slash_commands()` en `install_claude_code()`
- [ ] Implémenter `install_copilot()`
- [ ] Implémenter `install_cursor()`
- [ ] Implémenter `install_gemini()`

#### Tâche 1.4 : Template GitHub Copilot

**Fichier à générer :** `.github/copilot-instructions.md`

```markdown
# OpenSecKit Instructions for GitHub Copilot

## Project Context
This project uses OpenSecKit for security-as-code workflows.
Configuration: .osk/config.toml
Specifications: .osk/specs/

## Available Commands
When the user asks about security analysis, use the /osk-analyze workflow...

## Security Principles
1. Threat Modeling (STRIDE)
2. Risk Analysis (scoring 1-125)
...
```

**Sous-tâches :**

- [ ] Créer template `templates/agents/copilot-instructions.tmpl`
- [ ] Implémenter `generate_copilot_instructions()` dans CLI
- [ ] Tester avec GitHub Copilot Chat

#### Tâche 1.5 : Template Cursor

**Fichiers à générer :** `.cursor/rules/*.md`

Cursor utilise des fichiers de règles par contexte :

```
.cursor/
└── rules/
    ├── osk-security.md      # Règles générales sécurité
    ├── osk-analyze.md       # Règles pour analyse
    └── osk-implement.md     # Règles pour implémentation
```

**Sous-tâches :**

- [ ] Créer templates `templates/agents/cursor-*.tmpl`
- [ ] Implémenter `generate_cursor_rules()` dans CLI
- [ ] Tester avec Cursor AI

#### Tâche 1.6 : Mise à jour Registry

**Fichier :** `registry.toml`

```toml
[meta]
version = "3.1.0"
supported_agents = ["claude-code", "copilot", "cursor", "gemini"]

[commands.analyze]
url = "https://raw.githubusercontent.com/..."
version = "3.1.0"
phase = "analyze"
principles = ["I", "II"]
# NOUVEAU
agents = ["claude-code", "copilot", "cursor", "gemini"]
roles = ["security-architect"]
```

**Sous-tâches :**

- [ ] Ajouter champ `supported_agents` dans `[meta]`
- [ ] Ajouter champ `agents` à chaque commande
- [ ] Ajouter champ `roles` à chaque commande
- [ ] Mettre à jour parsing TOML dans CLI

#### Tâche 1.7 : Documentation

- [ ] Mettre à jour README.md avec nouvelles options
- [ ] Créer `docs/multi-agent.md`
- [ ] Mettre à jour `docs/commands/index.md`

---

### Phase 2 : Rôles Spécialisés (v3.2)

**Objectif :** Définir des rôles sécurité spécialisés avec délégation de commandes.

#### Tâche 2.1 : Fichier AGENTS.md

**Fichier :** `.osk/agents/AGENTS.md`

Format universel lisible par tous les agents :

```markdown
# OpenSecKit Security Agents

## Overview
OpenSecKit uses specialized security roles to ensure comprehensive coverage.

## Roles

### Risk Owner
**Responsibility:** Project governance and initial security configuration
**Commands:** /osk-configure, /osk-baseline
**Artifacts:** .osk/config.toml, .osk/memory/context.md

### Security Architect
**Responsibility:** Threat modeling (STRIDE) and risk analysis
**Commands:** /osk-analyze, /osk-specify
**Artifacts:** threats.md, risks.md, requirements.md
**Principles:** I (Threat Modeling), II (Risk Analysis), III (Security Requirements)

### Security Engineer
**Responsibility:** Security hardening and implementation planning
**Commands:** /osk-harden, /osk-plan, /osk-tasks
**Artifacts:** hardening.md, tasks.yaml
**Principles:** IV-VII

### Security Reviewer
**Responsibility:** Implementation verification and risk resolution
**Commands:** /osk-implement
**Updates:** risk-register.yaml (status: OUVERT → RESOLU → VERIFIE)

### Privacy Analyst
**Responsibility:** GDPR compliance and data protection
**Commands:** /osk-rgpd, /osk-pca-pra
**Domains:** RGPD

### Compliance Officer
**Responsibility:** Regulatory compliance (RGS, NIS2) and reporting
**Commands:** /osk-rgs, /osk-dashboard
**Domains:** RGS, NIS2

### SecOps Engineer
**Responsibility:** Incident response and operational security
**Commands:** /osk-incident
**Artifacts:** incident reports, playbooks

## Coordination

Agents coordinate through the `.osk/specs/` directory:
1. Risk Owner configures → Security Architect analyzes
2. Security Architect specifies → Security Engineer hardens
3. Security Engineer plans → Security Reviewer implements
4. All roles → Compliance Officer dashboards
```

**Sous-tâches :**

- [ ] Créer fichier `.osk/agents/AGENTS.md`
- [ ] Valider format avec plusieurs agents (Claude, Copilot)
- [ ] Ajouter au registry pour distribution

#### Tâche 2.2 : Fichier roles.toml

**Fichier :** `.osk/agents/roles.toml`

```toml
[roles.risk-owner]
name = "Risk Owner"
description = "Project governance and initial security configuration"
commands = ["configure", "baseline"]
principles = []

[roles.security-architect]
name = "Security Architect"
description = "Threat modeling (STRIDE) and risk analysis"
commands = ["analyze", "specify"]
principles = ["I", "II", "III"]

[roles.security-engineer]
name = "Security Engineer"
description = "Security hardening and implementation planning"
commands = ["harden", "plan", "tasks"]
principles = ["IV", "V", "VI", "VII"]

[roles.security-reviewer]
name = "Security Reviewer"
description = "Implementation verification and risk resolution"
commands = ["implement"]
principles = ["I", "II", "III", "IV", "V", "VI", "VII"]

[roles.privacy-analyst]
name = "Privacy Analyst"
description = "GDPR compliance and data protection"
commands = ["rgpd", "pca-pra"]
domains = ["rgpd"]

[roles.compliance-officer]
name = "Compliance Officer"
description = "Regulatory compliance and reporting"
commands = ["rgs", "dashboard"]
domains = ["rgs", "nis2"]

[roles.secops-engineer]
name = "SecOps Engineer"
description = "Incident response and operational security"
commands = ["incident"]
```

**Sous-tâches :**

- [ ] Créer fichier `roles.toml`
- [ ] Ajouter parsing dans CLI
- [ ] Générer documentation automatique des rôles

#### Tâche 2.3 : Adapters par Agent

**Structure :**

```
.osk/agents/
├── AGENTS.md           # Format universel
├── roles.toml          # Définition rôles
├── claude-code.yaml    # Adapter Claude Code
├── copilot.yaml        # Adapter Copilot
├── cursor.yaml         # Adapter Cursor
└── gemini.yaml         # Adapter Gemini
```

**Exemple `claude-code.yaml` :**

```yaml
agent: claude-code
version: "3.2.0"
format: slash-command
output_dir: .claude/commands

command_mapping:
  configure: /osk-configure
  analyze: /osk-analyze
  specify: /osk-specify
  harden: /osk-harden
  plan: /osk-plan
  tasks: /osk-tasks
  implement: /osk-implement
  rgpd: /osk-rgpd
  rgs: /osk-rgs
  dashboard: /osk-dashboard
  pca-pra: /osk-pca-pra
  incident: /osk-incident
  baseline: /osk-baseline

settings:
  file: .claude/settings.local.json
  permissions:
    - "Bash(git:*)"
    - "Bash(npm:*)"
    - "Read"
    - "Write"
    - "WebFetch"
```

**Exemple `copilot.yaml` :**

```yaml
agent: copilot
version: "3.2.0"
format: instructions
output_file: .github/copilot-instructions.md

command_mapping:
  # Copilot n'a pas de slash commands natifs
  # On mappe vers des instructions contextuelles
  configure: "When asked to configure security..."
  analyze: "When asked to analyze threats..."

context_files:
  - .osk/config.toml
  - .osk/memory/context.md
  - docs/security/risks/risk-register.yaml
```

**Sous-tâches :**

- [ ] Créer `claude-code.yaml`
- [ ] Créer `copilot.yaml`
- [ ] Créer `cursor.yaml`
- [ ] Créer `gemini.yaml`
- [ ] Implémenter lecture adapters dans CLI

#### Tâche 2.4 : Commande `osk agents`

Nouvelle commande CLI pour lister/gérer les agents :

```bash
# Lister agents supportés
osk agents list

# Afficher config agent actuel
osk agents show

# Changer d'agent
osk agents switch copilot

# Installer tous les agents
osk agents install --all
```

**Sous-tâches :**

- [ ] Créer `cli/src/commands/agents.rs`
- [ ] Implémenter sous-commandes (list, show, switch, install)
- [ ] Ajouter à `cli/src/main.rs`
- [ ] Écrire tests

---

### Phase 3 : Orchestration (v4.0)

**Objectif :** Permettre la coordination multi-agent et l'interopérabilité.

#### Tâche 3.1 : Context Service API

Service optionnel pour gérer le contexte de manière centralisée :

```
.osk/
└── context-service/
    ├── config.toml      # Configuration service
    └── api.yaml         # Spécification OpenAPI
```

**Endpoints proposés :**

```yaml
openapi: 3.0.0
info:
  title: OpenSecKit Context Service
  version: 4.0.0

paths:
  /context/load:
    post:
      summary: Load project context
      requestBody:
        content:
          application/json:
            schema:
              type: object
              properties:
                project: { type: string }
                feature: { type: string }
                domains: { type: array, items: { type: string } }
      responses:
        200:
          description: Context loaded
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/Context'

  /risks:
    get:
      summary: Query risk register
      parameters:
        - name: status
          in: query
          schema:
            type: string
            enum: [OUVERT, RESOLU, ACCEPTE, VERIFIE]
    post:
      summary: Add or update risk
      requestBody:
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/Risk'

  /sync:
    post:
      summary: Synchronize state between agents
      description: Used when multiple agents work concurrently
```

**Sous-tâches :**

- [ ] Définir schéma OpenAPI complet
- [ ] Implémenter serveur (Rust/Axum ou Python/FastAPI)
- [ ] Créer client CLI (`osk context serve`)
- [ ] Documenter intégration

#### Tâche 3.2 : Coordination Multi-Agent

Mécanisme pour éviter les conflits quand plusieurs agents travaillent :

```toml
# .osk/agents/coordination.toml
[locking]
enabled = true
backend = "file"  # ou "redis", "api"
lock_file = ".osk/.locks"

[sync]
auto_sync = true
sync_interval = "5s"
conflict_resolution = "last-write-wins"  # ou "merge", "manual"

[notifications]
enabled = true
channels = ["file"]  # ou "webhook", "slack"
```

**Sous-tâches :**

- [ ] Implémenter système de locks
- [ ] Créer mécanisme de sync
- [ ] Gérer résolution conflits
- [ ] Tester avec 2+ agents simultanés

#### Tâche 3.3 : MCP Server (Model Context Protocol)

Exposer OpenSecKit comme serveur MCP pour intégration native :

```json
{
  "name": "openseckit",
  "version": "4.0.0",
  "tools": [
    {
      "name": "osk_analyze",
      "description": "Run STRIDE threat analysis on a feature",
      "parameters": {
        "feature": { "type": "string", "required": true }
      }
    },
    {
      "name": "osk_risks",
      "description": "Query or update the risk register",
      "parameters": {
        "action": { "type": "string", "enum": ["list", "add", "update"] },
        "risk_id": { "type": "string" }
      }
    }
  ]
}
```

**Sous-tâches :**

- [ ] Créer `mcp-server/` directory
- [ ] Implémenter serveur MCP (TypeScript ou Python)
- [ ] Exposer toutes les commandes OSK comme tools
- [ ] Tester avec Claude Desktop et autres clients MCP

---

## Résumé des Livrables

### Phase 1 (v3.1)

| # | Livrable | Type | Priorité |
|---|----------|------|----------|
| 1.1 | Sélection interactive d'agent | Code | Haute |
| 1.2 | Output condensé par module | Code | Haute |
| 1.3 | Installation par agent (dispatcher) | Code | Haute |
| 1.4 | Template GitHub Copilot | Template | Haute |
| 1.5 | Template Cursor | Template | Moyenne |
| 1.6 | Registry enrichi (`agents`, `roles`) | Config | Moyenne |
| 1.7 | Documentation multi-agent | Docs | Moyenne |

### Phase 2 (v3.2)

| Livrable | Type | Priorité |
|----------|------|----------|
| AGENTS.md universel | Template | Haute |
| roles.toml | Config | Haute |
| Adapters YAML par agent | Config | Moyenne |
| Commande `osk agents` | Code | Moyenne |

### Phase 3 (v4.0)

| Livrable | Type | Priorité |
|----------|------|----------|
| Context Service API | Code | Moyenne |
| Coordination multi-agent | Code | Basse |
| MCP Server | Code | Moyenne |

---

## Critères de Validation

### Phase 1

- [ ] `osk init` affiche la sélection interactive d'agent
- [ ] `osk init` affiche un résumé condensé par module (pas la liste de fichiers)
- [ ] `osk init` (sans flag) → Claude Code par défaut
- [ ] `osk init --agent copilot` génère `.github/copilot-instructions.md`
- [ ] `osk init --agent cursor` génère `.cursor/rules/*.md`
- [ ] `osk init --all-agents` génère tous les formats
- [ ] `osk init --default` fonctionne pour CI (non-interactif)
- [ ] Détection des agents installés fonctionne (✓ vs non détecté)
- [ ] Tests unitaires passent
- [ ] Documentation à jour

### Phase 2

- [ ] AGENTS.md est lisible par Claude, Copilot, Cursor
- [ ] Rôles sont correctement mappés aux commandes
- [ ] `osk agents list` affiche tous les agents supportés
- [ ] Adapters génèrent les bons fichiers

### Phase 3

- [ ] Context Service démarre et répond aux requêtes
- [ ] Deux agents peuvent travailler sans conflit
- [ ] MCP Server expose les tools correctement
- [ ] Intégration Claude Desktop fonctionne

---

## Risques et Mitigations

| Risque | Impact | Probabilité | Mitigation |
|--------|--------|-------------|------------|
| Breaking changes CLI | Haut | Moyen | Rétrocompatibilité par défaut |
| Formats agents évoluent | Moyen | Haut | Adapters versionnés |
| Conflits multi-agent | Haut | Moyen | Locking + merge strategy |
| Maintenance multiple formats | Moyen | Haut | Génération depuis source unique |

---

## Prochaines Étapes

1. **Valider ce plan** avec les parties prenantes
2. **Prioriser** les tâches Phase 1
3. **Créer une branche** `feature/multi-agent`
4. **Implémenter** tâche par tâche avec tests
5. **Release** v3.1.0 après validation Phase 1
