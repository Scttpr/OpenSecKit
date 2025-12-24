# osk validate

Valider la cohérence des fichiers du projet.

## Synopsis

```bash
osk validate [--json] <subcommand> [options]
```

## Description

`osk validate` vérifie la cohérence et la complétude des fichiers de sécurité. Détecte les erreurs de syntaxe, les dépendances circulaires et les fichiers incomplets.

## Sous-commandes

### yaml

Valide la syntaxe de tous les fichiers YAML.

```bash
osk validate yaml
```

Vérifie :
- `docs/security/risks/risk-register.yaml`
- `.osk/specs/*/tasks.yaml`

### deps

Vérifie les dépendances des tâches (détection de cycles).

```bash
osk validate deps <feature>
```

Analyse le graphe de dépendances dans `tasks.yaml` pour détecter :
- Cycles circulaires (T001 → T002 → T001)
- Auto-références (T001 → T001)
- Dépendances vers tâches inconnues

### workflow

Vérifie la complétude du workflow d'une feature.

```bash
osk validate workflow <feature>
```

Vérifie que tous les fichiers requis existent et ne contiennent pas de TODO.

## Options globales

| Option | Description |
|--------|-------------|
| `--json` | Sortie au format JSON (pour agents AI) |

## Exemples

### Valider les dépendances

```bash
osk validate deps authentication
# 🔍 Validating dependencies in .osk/specs/001-auth/tasks.yaml...
#
#    5 tasks found
#
# ✅ No circular dependencies found
```

Avec un cycle détecté :

```bash
osk validate deps payment
# 🔍 Validating dependencies in .osk/specs/002-payment/tasks.yaml...
#
#    4 tasks found
#
# ❌ Circular dependency detected:
#    T003 → T004 → T003
```

### Valider les fichiers YAML

```bash
osk validate yaml
# ✅ docs/security/risks/risk-register.yaml
# ✅ .osk/specs/001-auth/tasks.yaml
# 📊 2 files validated, 0 errors
```

### Valider un workflow

```bash
osk validate workflow authentication
# 🔍 Validating workflow for authentication...
#
# ✅ threats.md
# ✅ risks.md
# ⚠️  requirements.md (empty or TODO)
# ❌ testing.md (missing)
# ❌ hardening.md (missing)
# ❌ plan.md (missing)
# ❌ tasks.yaml (missing)
#
# 📊 Workflow: 2/7 complete
# 💡 Next: Run /osk-specify authentication
```

### Sortie JSON

```bash
osk validate --json workflow auth
```

```json
{
  "valid": false,
  "command": "validate workflow",
  "checked_files": [
    {
      "path": ".osk/specs/001-auth/threats.md",
      "status": "valid",
      "error": null
    },
    {
      "path": ".osk/specs/001-auth/risks.md",
      "status": "incomplete",
      "error": "contains TODO or empty"
    },
    {
      "path": ".osk/specs/001-auth/requirements.md",
      "status": "missing",
      "error": "file not found"
    }
  ],
  "error_count": 5,
  "next_command": "/osk-analyze auth",
  "message": "Workflow: 2/7 complete"
}
```

## Structure JSON

```typescript
interface ValidateResult {
  valid: boolean;
  command: string;
  checked_files: FileStatus[];
  error_count: number;
  next_command: string | null;
  message: string;
}

interface FileStatus {
  path: string;
  status: "valid" | "invalid" | "missing" | "incomplete";
  error: string | null;
}
```

## Statuts de fichier

| Status | Description |
|--------|-------------|
| `valid` | Fichier présent avec contenu valide |
| `invalid` | Fichier présent mais syntaxe invalide |
| `missing` | Fichier absent |
| `incomplete` | Fichier présent mais contient TODO ou vide |
| `warning` | Avertissement (ex: dépendance vers tâche inconnue) |

## Voir aussi

- [osk check](osk-check.md) - Vérifier les prérequis
- [osk scaffold](osk-scaffold.md) - Créer des structures
