# Plan d'implémentation - Commandes CLI Scriptables

## Analyse Stratégique

### Économie de Tokens

| Scénario | Sans CLI | Avec CLI | Économie |
|----------|----------|----------|----------|
| Vérifier 5 prérequis | ~200 tokens (lecture + raisonnement) | 0 tokens (script) | **100%** |
| Créer structure feature | ~500 tokens (mkdir, templates) | 0 tokens | **100%** |
| Parser risk-register stats | ~800 tokens (lecture YAML + calculs) | 0 tokens | **100%** |
| Mettre à jour tâche done | ~400 tokens (YAML edit + git) | 0 tokens | **100%** |
| **Workflow complet analyze** | ~3000 tokens | ~1500 tokens | **~50%** |

**Estimation globale** : 40-60% d'économie de tokens sur les opérations mécaniques.

### Compatibilité Multi-Agent

```
┌─────────────────────────────────────────────────────────────┐
│                      AVANT (100% Agent)                      │
├─────────────────────────────────────────────────────────────┤
│  Agent AI                                                    │
│  ├── Vérifie prérequis (tokens)                             │
│  ├── Crée répertoires (tokens)                              │
│  ├── Copie templates (tokens)                               │
│  ├── ANALYSE SÉMANTIQUE (tokens) ← Valeur ajoutée           │
│  ├── Écrit fichiers (tokens)                                │
│  └── Git commit (tokens)                                    │
└─────────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│                     APRÈS (Hybride)                          │
├─────────────────────────────────────────────────────────────┤
│  CLI Rust (osk)           │  Agent AI                       │
│  ├── osk check analyze    │                                 │
│  ├── osk scaffold feature │                                 │
│  │   └── Retourne paths   │  ← Reçoit contexte minimal     │
│  │                        │  ├── ANALYSE SÉMANTIQUE        │
│  │                        │  └── Génère contenu             │
│  ├── osk update risk-reg  │  ← Reçoit données structurées  │
│  └── osk commit task T001 │                                 │
└─────────────────────────────────────────────────────────────┘
```

**Avantages multi-agent** :
- Les commandes CLI sont **agent-agnostiques** (fonctionnent avec Claude, Copilot, Cursor, Gemini)
- L'agent reçoit un **contexte pré-validé** (moins d'erreurs)
- Les opérations mécaniques sont **identiques** quel que soit l'agent

### Scalabilité et Fiabilité

| Aspect | Script Bash | Agent AI |
|--------|-------------|----------|
| Déterminisme | ✅ 100% reproductible | ❌ Peut varier |
| Vitesse | ✅ < 100ms | ❌ 2-10s |
| Coût | ✅ Gratuit | ❌ Tokens |
| Erreurs | ✅ Prévisibles | ❌ Hallucinations possibles |
| Offline | ✅ Fonctionne | ❌ Requiert API |
| Tests | ✅ Facile (unit tests) | ❌ Difficile |

---

## Architecture Technique

### Structure du CLI

```
cli/src/
├── main.rs
├── args.rs              # Mise à jour avec nouvelles commandes
├── commands/
│   ├── mod.rs
│   ├── init.rs          # Existant
│   ├── ingest.rs        # Existant
│   ├── check.rs         # NOUVEAU
│   ├── scaffold.rs      # NOUVEAU
│   ├── update.rs        # NOUVEAU
│   └── validate.rs      # NOUVEAU
├── utils/
│   ├── mod.rs
│   ├── yaml.rs          # Helpers YAML (serde_yaml)
│   ├── toml.rs          # Helpers TOML
│   ├── git.rs           # Helpers Git
│   ├── template.rs      # Substitution variables
│   └── counter.rs       # Gestion compteurs (NNN)
├── agents.rs            # Existant
├── prompts.rs           # Existant
├── config.rs            # Existant
└── github.rs            # Existant
```

### Nouvelles Dépendances

```toml
# cli/Cargo.toml - Ajouts
[dependencies]
serde_yaml = "0.9"       # Parsing YAML
git2 = "0.19"            # Git natif (optionnel, ou shell)
chrono = "0.4"           # Dates/timestamps
```

---

## Spécifications des Commandes

### 1. `osk check` - Vérifications Prérequis

#### Synopsis
```bash
osk check <COMMAND> [--fix] [--json]
```

#### Sous-commandes

| Commande | Vérifie | Code retour |
|----------|---------|-------------|
| `osk check configure` | `.osk/` existe | 0=OK, 1=manquant |
| `osk check analyze` | `context.md`, `constitution.md` | 0=OK, 1=manquant |
| `osk check specify <feature>` | `threats.md`, `risks.md` | 0=OK, 1=manquant |
| `osk check harden <feature>` | `requirements.md`, `testing.md` | 0=OK, 1=manquant |
| `osk check plan <feature>` | Tous les fichiers spec | 0=OK, 1=manquant |
| `osk check tasks <feature>` | `plan.md` | 0=OK, 1=manquant |
| `osk check implement <feature>` | `tasks.yaml` | 0=OK, 1=manquant |
| `osk check dashboard` | `risk-register.yaml` | 0=OK, 1=manquant |

#### Output
```bash
$ osk check analyze
✅ .osk/memory/context.md
✅ .osk/memory/constitution.md
Ready for /osk-analyze

$ osk check specify authentication
❌ .osk/specs/001-authentication/threats.md (missing)
❌ .osk/specs/001-authentication/risks.md (missing)
Run /osk-analyze authentication first

$ osk check analyze --json
{"ready": true, "missing": [], "command": "analyze"}
```

#### Implémentation

```rust
// cli/src/commands/check.rs
use std::path::Path;

pub struct CheckResult {
    pub ready: bool,
    pub missing: Vec<String>,
    pub found: Vec<String>,
    pub suggestion: Option<String>,
}

pub fn check_analyze() -> CheckResult {
    let required = vec![
        ".osk/memory/context.md",
        ".osk/memory/constitution.md",
    ];
    check_files(&required, Some("Run /osk-configure first"))
}

pub fn check_specify(feature: &str) -> CheckResult {
    let spec_dir = find_feature_dir(feature)?;
    let required = vec![
        format!("{}/threats.md", spec_dir),
        format!("{}/risks.md", spec_dir),
    ];
    check_files(&required, Some(&format!("Run /osk-analyze {} first", feature)))
}

fn check_files(paths: &[String], suggestion: Option<&str>) -> CheckResult {
    let mut missing = Vec::new();
    let mut found = Vec::new();

    for path in paths {
        if Path::new(path).exists() {
            found.push(path.clone());
        } else {
            missing.push(path.clone());
        }
    }

    CheckResult {
        ready: missing.is_empty(),
        missing,
        found,
        suggestion: suggestion.map(String::from),
    }
}
```

---

### 2. `osk scaffold` - Création de Structures

#### Synopsis
```bash
osk scaffold <TYPE> <NAME> [OPTIONS]
```

#### Sous-commandes

| Commande | Crée | Options |
|----------|------|---------|
| `osk scaffold feature <name>` | `.osk/specs/NNN-name/` + templates + branche git | `--no-branch` |
| `osk scaffold rgpd` | `docs/security/rgpd/` + fichiers | |
| `osk scaffold rgs <system>` | `docs/security/rgs/` + fichiers EBIOS | |
| `osk scaffold incident <desc>` | `docs/security/incidents/INC-DATE-NNN.md` | `--severity` |
| `osk scaffold continuity <system>` | `docs/security/continuity/` + PCA/PRA | |

#### Output
```bash
$ osk scaffold feature authentication
📁 Created .osk/specs/001-authentication/
   ├── threats.md (template)
   ├── risks.md (template)
   ├── requirements.md (template)
   ├── testing.md (template)
   ├── hardening.md (template)
   ├── plan.md (template)
   └── tasks.yaml (template)
🔀 Created branch: security/001-authentication
📝 Updated .osk/config.toml: specs.counter = 2

$ osk scaffold incident "Fuite de credentials AWS"
📁 Created docs/security/incidents/INC-2025-12-24-001.md
⚠️  Added risk RISK-INC-2025-12-24-001 to risk-register.yaml
```

#### Implémentation

```rust
// cli/src/commands/scaffold.rs
use crate::utils::{counter, git, template};
use std::fs;

pub fn scaffold_feature(name: &str, no_branch: bool) -> Result<ScaffoldResult> {
    // 1. Obtenir le prochain numéro
    let counter = counter::next_feature_number()?;
    let dir_name = format!("{:03}-{}", counter, slugify(name));
    let spec_dir = format!(".osk/specs/{}", dir_name);

    // 2. Créer le répertoire
    fs::create_dir_all(&spec_dir)?;

    // 3. Copier les templates
    let templates = [
        ("templates/outputs/threats.md.tmpl", "threats.md"),
        ("templates/outputs/risks.md.tmpl", "risks.md"),
        ("templates/outputs/requirements.md.tmpl", "requirements.md"),
        ("templates/outputs/testing.md.tmpl", "testing.md"),
        ("templates/outputs/hardening.md.tmpl", "hardening.md"),
        ("templates/outputs/plan.md.tmpl", "plan.md"),
        ("templates/outputs/tasks.yaml.tmpl", "tasks.yaml"),
    ];

    let mut created = Vec::new();
    for (src, dst) in templates {
        let src_path = format!(".osk/{}", src);
        let dst_path = format!("{}/{}", spec_dir, dst);

        if Path::new(&src_path).exists() {
            let content = fs::read_to_string(&src_path)?;
            let rendered = template::render(&content, &[
                ("feature_name", name),
                ("feature_id", &dir_name),
                ("date", &chrono::Local::now().format("%Y-%m-%d").to_string()),
            ]);
            fs::write(&dst_path, rendered)?;
            created.push(dst.to_string());
        }
    }

    // 4. Créer la branche git
    let branch_name = if !no_branch {
        let branch = format!("security/{}", dir_name);
        git::create_branch(&branch)?;
        Some(branch)
    } else {
        None
    };

    // 5. Mettre à jour le compteur
    counter::increment_feature_counter()?;

    Ok(ScaffoldResult {
        directory: spec_dir,
        files: created,
        branch: branch_name,
    })
}

pub fn scaffold_incident(description: &str, severity: Option<&str>) -> Result<ScaffoldResult> {
    // 1. Générer l'ID
    let date = chrono::Local::now().format("%Y-%m-%d").to_string();
    let seq = counter::next_incident_number(&date)?;
    let incident_id = format!("INC-{}-{:03}", date, seq);

    // 2. Créer le répertoire
    fs::create_dir_all("docs/security/incidents")?;

    // 3. Créer le fichier incident
    let file_path = format!("docs/security/incidents/{}.md", incident_id);
    let template = include_str!("../../templates/outputs/incident.md.tmpl");
    let content = template::render(template, &[
        ("incident_id", &incident_id),
        ("description", description),
        ("severity", severity.unwrap_or("HIGH")),
        ("date", &date),
        ("status", "🚨 EN COURS"),
    ]);
    fs::write(&file_path, content)?;

    // 4. Ajouter au risk-register
    let risk_id = format!("RISK-{}", incident_id);
    yaml::append_risk("docs/security/risks/risk-register.yaml", &Risk {
        id: risk_id.clone(),
        titre: description.to_string(),
        severite: "CRITIQUE".to_string(),
        statut: "EN_COURS".to_string(),
        impact: 5,
        probabilite: 5,
        dates: RiskDates {
            detection: date.clone(),
            echeance: "IMMEDIAT".to_string(),
            ..Default::default()
        },
    })?;

    Ok(ScaffoldResult {
        directory: "docs/security/incidents".to_string(),
        files: vec![format!("{}.md", incident_id)],
        risk_id: Some(risk_id),
        ..Default::default()
    })
}
```

---

### 3. `osk update` - Mises à Jour Mécaniques

#### Synopsis
```bash
osk update <TYPE> [OPTIONS]
```

#### Sous-commandes

| Commande | Action |
|----------|--------|
| `osk update stats` | Recalcule stats dans risk-register.yaml |
| `osk update task <id> --done` | Marque tâche done + commit |
| `osk update risk <id> --status <status>` | Change statut d'un risque |
| `osk update dashboard` | Régénère docs/security/dashboard.md |

#### Output
```bash
$ osk update stats
📊 Updated risk-register.yaml stats:
   total: 12
   par_statut: {OUVERT: 5, EN_COURS: 3, RESOLU: 4}
   par_severite: {CRITIQUE: 1, IMPORTANT: 3, MODERE: 5, MINEUR: 3}
   score_total: 245
   score_residuel: 89

$ osk update task T001 --done
✅ Task T001 marked as done
📝 Updated .osk/specs/001-auth/tasks.yaml
🔀 Committed: fix(security): T001 - Implémenter rate limiting
```

#### Implémentation

```rust
// cli/src/commands/update.rs
use crate::utils::{yaml, git};

pub fn update_stats() -> Result<StatsResult> {
    let register_path = "docs/security/risks/risk-register.yaml";
    let mut register: RiskRegister = yaml::load(register_path)?;

    // Calculer les stats
    let stats = Stats {
        total: register.risks.len(),
        par_statut: count_by_field(&register.risks, |r| &r.statut),
        par_severite: count_by_field(&register.risks, |r| &r.severite),
        score_total: register.risks.iter().map(|r| r.score_initial).sum(),
        score_residuel: register.risks.iter()
            .filter(|r| r.statut != "RESOLU" && r.statut != "ACCEPTE")
            .map(|r| r.score_residuel.unwrap_or(r.score_initial))
            .sum(),
    };

    register.stats = stats.clone();
    yaml::save(register_path, &register)?;

    Ok(StatsResult { stats })
}

pub fn update_task(task_id: &str, done: bool) -> Result<TaskUpdateResult> {
    // 1. Trouver le fichier tasks.yaml
    let tasks_file = find_tasks_file_for_task(task_id)?;
    let mut tasks: TasksFile = yaml::load(&tasks_file)?;

    // 2. Trouver et mettre à jour la tâche
    let task = tasks.tasks.iter_mut()
        .find(|t| t.id == task_id)
        .ok_or_else(|| anyhow!("Task {} not found", task_id))?;

    if done {
        task.status = "done".to_string();
        task.date_fin = Some(chrono::Local::now().format("%Y-%m-%d").to_string());
    }

    let task_title = task.titre.clone();

    // 3. Sauvegarder
    yaml::save(&tasks_file, &tasks)?;

    // 4. Git commit
    if done {
        git::add(&tasks_file)?;
        git::commit(&format!("fix(security): {} - {}", task_id, task_title))?;
    }

    Ok(TaskUpdateResult {
        task_id: task_id.to_string(),
        new_status: if done { "done" } else { "todo" }.to_string(),
        committed: done,
    })
}

pub fn update_risk(risk_id: &str, new_status: &str) -> Result<RiskUpdateResult> {
    let register_path = "docs/security/risks/risk-register.yaml";
    let mut register: RiskRegister = yaml::load(register_path)?;

    let risk = register.risks.iter_mut()
        .find(|r| r.id == risk_id)
        .ok_or_else(|| anyhow!("Risk {} not found", risk_id))?;

    let old_status = risk.statut.clone();
    risk.statut = new_status.to_string();

    if new_status == "RESOLU" {
        risk.dates.resolution = Some(chrono::Local::now().format("%Y-%m-%d").to_string());
    }

    yaml::save(register_path, &register)?;

    // Recalculer les stats
    update_stats()?;

    Ok(RiskUpdateResult {
        risk_id: risk_id.to_string(),
        old_status,
        new_status: new_status.to_string(),
    })
}
```

---

### 4. `osk validate` - Validations

#### Synopsis
```bash
osk validate <TYPE> [OPTIONS]
```

#### Sous-commandes

| Commande | Valide |
|----------|--------|
| `osk validate yaml` | Syntaxe YAML de tous les fichiers |
| `osk validate deps <feature>` | Graphe de dépendances (cycles) |
| `osk validate risks` | Cohérence risk-register.yaml |
| `osk validate workflow <feature>` | Complétude du workflow |

#### Output
```bash
$ osk validate deps authentication
✅ No circular dependencies found
📊 Dependency graph:
   T001 → T002, T003
   T002 → T004
   T003 → T004
   T004 → (none)

$ osk validate workflow authentication
⚠️  Workflow incomplete for 001-authentication:
   ✅ threats.md
   ✅ risks.md
   ❌ requirements.md (empty)
   ❌ testing.md (missing)
   ❌ hardening.md (missing)
   ❌ plan.md (missing)
   ❌ tasks.yaml (missing)
   Next: Run /osk-specify authentication
```

#### Implémentation

```rust
// cli/src/commands/validate.rs
use std::collections::{HashMap, HashSet};

pub fn validate_deps(feature: &str) -> Result<DepsValidation> {
    let tasks_file = find_tasks_file(feature)?;
    let tasks: TasksFile = yaml::load(&tasks_file)?;

    // Construire le graphe
    let mut graph: HashMap<String, Vec<String>> = HashMap::new();
    for task in &tasks.tasks {
        graph.insert(task.id.clone(), task.depends_on.clone().unwrap_or_default());
    }

    // Détecter les cycles (DFS)
    let cycles = detect_cycles(&graph);

    Ok(DepsValidation {
        has_cycles: !cycles.is_empty(),
        cycles,
        graph,
    })
}

fn detect_cycles(graph: &HashMap<String, Vec<String>>) -> Vec<Vec<String>> {
    let mut cycles = Vec::new();
    let mut visited = HashSet::new();
    let mut rec_stack = HashSet::new();
    let mut path = Vec::new();

    for node in graph.keys() {
        if !visited.contains(node) {
            dfs_detect_cycle(node, graph, &mut visited, &mut rec_stack, &mut path, &mut cycles);
        }
    }

    cycles
}

pub fn validate_workflow(feature: &str) -> Result<WorkflowValidation> {
    let spec_dir = find_feature_dir(feature)?;

    let workflow_files = [
        ("threats.md", "Threat Analysis", "/osk-analyze"),
        ("risks.md", "Risk Analysis", "/osk-analyze"),
        ("requirements.md", "Security Requirements", "/osk-specify"),
        ("testing.md", "Testing Strategy", "/osk-specify"),
        ("hardening.md", "Hardening Measures", "/osk-harden"),
        ("plan.md", "Implementation Plan", "/osk-plan"),
        ("tasks.yaml", "Task Breakdown", "/osk-tasks"),
    ];

    let mut status = Vec::new();
    let mut next_command = None;

    for (file, name, command) in workflow_files {
        let path = format!("{}/{}", spec_dir, file);
        let exists = Path::new(&path).exists();
        let empty = exists && fs::read_to_string(&path)
            .map(|c| c.trim().is_empty() || c.contains("TODO"))
            .unwrap_or(true);

        let file_status = if exists && !empty {
            FileStatus::Complete
        } else if exists {
            FileStatus::Empty
        } else {
            FileStatus::Missing
        };

        if next_command.is_none() && file_status != FileStatus::Complete {
            next_command = Some(command.to_string());
        }

        status.push((name.to_string(), file_status));
    }

    Ok(WorkflowValidation {
        feature: feature.to_string(),
        status,
        next_command,
        complete: next_command.is_none(),
    })
}
```

---

## Intégration avec les Prompts

### Modification des Prompts

Chaque prompt sera modifié pour **déléguer** les opérations mécaniques au CLI :

```markdown
# /osk-analyze

## Prérequis

Avant de commencer, exécuter :
```bash
osk check analyze
```

Si des fichiers manquent, suivre les instructions affichées.

## Initialisation

Créer la structure de la feature :
```bash
osk scaffold feature "{{feature_name}}"
```

Cela crée automatiquement :
- `.osk/specs/NNN-feature/` avec tous les templates
- Branche git `security/NNN-feature`

## Ton rôle

Tu es le **Threat Analyst**. Ton travail commence ICI :
- Analyser le code de la feature
- Identifier les menaces STRIDE
- Évaluer les risques

[... reste du prompt focalisé sur l'ANALYSE SÉMANTIQUE ...]

## Finalisation

Après ton analyse, mettre à jour les stats :
```bash
osk update stats
```
```

### Workflow Intégré

```
┌─────────────────────────────────────────────────────────────────┐
│                    NOUVEAU WORKFLOW HYBRIDE                      │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Utilisateur: /osk-analyze authentication                       │
│                     │                                            │
│                     ▼                                            │
│  ┌─────────────────────────────────────┐                        │
│  │ Agent exécute: osk check analyze    │ ◄── CLI (0 tokens)    │
│  │ Résultat: ✅ Ready                   │                        │
│  └─────────────────────────────────────┘                        │
│                     │                                            │
│                     ▼                                            │
│  ┌─────────────────────────────────────┐                        │
│  │ Agent exécute: osk scaffold feature │ ◄── CLI (0 tokens)    │
│  │ Résultat: Created 001-auth/         │                        │
│  └─────────────────────────────────────┘                        │
│                     │                                            │
│                     ▼                                            │
│  ┌─────────────────────────────────────┐                        │
│  │ AGENT AI: Analyse sémantique        │ ◄── LLM (tokens)      │
│  │ - Lit le code source                │                        │
│  │ - Identifie menaces STRIDE          │                        │
│  │ - Évalue risques                    │                        │
│  │ - Écrit threats.md, risks.md        │                        │
│  └─────────────────────────────────────┘                        │
│                     │                                            │
│                     ▼                                            │
│  ┌─────────────────────────────────────┐                        │
│  │ Agent exécute: osk update stats     │ ◄── CLI (0 tokens)    │
│  │ Résultat: Stats updated             │                        │
│  └─────────────────────────────────────┘                        │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Plan d'Implémentation

### Phase 1 : Fondations (v3.2.0)

| Tâche | Effort | Priorité |
|-------|--------|----------|
| Ajouter `serde_yaml` au Cargo.toml | 5 min | P0 |
| Créer module `utils/yaml.rs` | 2h | P0 |
| Créer module `utils/counter.rs` | 1h | P0 |
| Créer module `utils/git.rs` | 1h | P0 |
| Créer module `utils/template.rs` | 1h | P0 |
| Implémenter `osk check` (toutes sous-commandes) | 3h | P0 |
| Tests unitaires check | 2h | P0 |

### Phase 2 : Scaffold (v3.2.0)

| Tâche | Effort | Priorité |
|-------|--------|----------|
| Implémenter `osk scaffold feature` | 3h | P0 |
| Implémenter `osk scaffold incident` | 2h | P1 |
| Implémenter `osk scaffold rgpd` | 1h | P2 |
| Implémenter `osk scaffold rgs` | 1h | P2 |
| Créer templates .tmpl manquants | 2h | P1 |
| Tests unitaires scaffold | 2h | P0 |

### Phase 3 : Update (v3.2.0)

| Tâche | Effort | Priorité |
|-------|--------|----------|
| Implémenter `osk update stats` | 2h | P0 |
| Implémenter `osk update task` | 2h | P0 |
| Implémenter `osk update risk` | 1h | P1 |
| Implémenter `osk update dashboard` | 2h | P2 |
| Tests unitaires update | 2h | P0 |

### Phase 4 : Validate (v3.3.0)

| Tâche | Effort | Priorité |
|-------|--------|----------|
| Implémenter `osk validate yaml` | 1h | P1 |
| Implémenter `osk validate deps` | 2h | P1 |
| Implémenter `osk validate workflow` | 2h | P1 |
| Tests unitaires validate | 2h | P1 |

### Phase 5 : Intégration Prompts (v3.3.0)

| Tâche | Effort | Priorité |
|-------|--------|----------|
| Modifier tous les prompts pour utiliser CLI | 4h | P0 |
| Mettre à jour documentation | 2h | P1 |
| Tests E2E workflow complet | 3h | P0 |

---

## Estimation Totale

| Phase | Effort | Version |
|-------|--------|---------|
| Phase 1 : Fondations | ~10h | v3.2.0 |
| Phase 2 : Scaffold | ~9h | v3.2.0 |
| Phase 3 : Update | ~9h | v3.2.0 |
| Phase 4 : Validate | ~7h | v3.3.0 |
| Phase 5 : Intégration | ~9h | v3.3.0 |
| **Total** | **~44h** | |

---

## Risques et Mitigations

| Risque | Impact | Mitigation |
|--------|--------|------------|
| Parsing YAML complexe échoue | Moyen | Validation stricte + messages clairs |
| Git operations sur repo sale | Haut | Vérifier `git status` avant opérations |
| Templates manquants | Moyen | Fallback sur templates embarqués |
| Compteur désynchronisé | Faible | Lock file ou atomic counter |
| Agent n'exécute pas les commandes CLI | Haut | Instructions claires + validation output |

---

## Conclusion

### Bénéfices

1. **Économie de tokens** : 40-60% sur les opérations mécaniques
2. **Fiabilité** : Opérations déterministes et testables
3. **Vitesse** : < 100ms vs 2-10s pour les opérations courantes
4. **Multi-agent** : CLI agnostique, fonctionne avec tous les agents
5. **Offline** : Opérations mécaniques fonctionnent sans API

### Scalabilité

- ✅ Ajouter une nouvelle validation = ajouter une fonction Rust
- ✅ Ajouter un nouveau scaffold = ajouter template + fonction
- ✅ Performance O(n) sur les fichiers, pas sur les tokens
- ✅ Tests automatisés faciles (cargo test)

### Recommandation

**GO** pour l'implémentation. Le ROI est positif dès la première utilisation grâce à l'économie de tokens et la fiabilité accrue.
