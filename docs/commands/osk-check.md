# osk check

Vérifier les prérequis avant d'exécuter une commande.

## Synopsis

```bash
osk check [--json] <subcommand>
```

## Description

`osk check` vérifie que tous les fichiers et configurations nécessaires sont présents avant d'exécuter une commande du workflow. Utile pour :

- Valider l'état du projet avant une étape
- Diagnostiquer les fichiers manquants
- Automatiser les vérifications dans les scripts

## Sous-commandes

| Commande | Description |
|----------|-------------|
| `configure` | Vérifie les prérequis pour `/osk-configure` |
| `analyze` | Vérifie les prérequis pour `/osk-analyze` |
| `specify <feature>` | Vérifie les prérequis pour `/osk-specify` |
| `harden <feature>` | Vérifie les prérequis pour `/osk-harden` |
| `plan <feature>` | Vérifie les prérequis pour `/osk-plan` |
| `tasks <feature>` | Vérifie les prérequis pour `/osk-tasks` |
| `implement <feature>` | Vérifie les prérequis pour `/osk-implement` |
| `dashboard` | Vérifie les prérequis pour `/osk-dashboard` |

## Options

| Option | Description |
|--------|-------------|
| `--json` | Sortie au format JSON (pour agents AI) |

## Exemples

### Vérification standard

```bash
osk check configure
# ✅ .osk
# ✅ .osk/config.toml
# 📊 Ready: All prerequisites met
```

### Sortie JSON

```bash
osk check --json analyze
```

```json
{
  "ready": true,
  "command": "osk-analyze",
  "found": [
    ".osk/config.toml",
    "docs/security/risks/risk-register.yaml"
  ],
  "missing": [],
  "suggestion": null
}
```

### Prérequis manquants

```bash
osk check --json specify authentication
```

```json
{
  "ready": false,
  "command": "osk-specify",
  "found": [".osk/specs/001-authentication"],
  "missing": [
    ".osk/specs/001-authentication/threats.md",
    ".osk/specs/001-authentication/risks.md"
  ],
  "suggestion": "Run /osk-analyze authentication first"
}
```

## Structure JSON

```typescript
interface CheckResult {
  ready: boolean;           // true si tous les prérequis sont présents
  command: string;          // nom de la commande vérifiée
  found: string[];          // fichiers/dossiers trouvés
  missing: string[];        // fichiers/dossiers manquants
  suggestion: string | null; // suggestion si non prêt
}
```

## Voir aussi

- [osk validate](osk-validate.md) - Valider la cohérence des fichiers
- [osk scaffold](osk-scaffold.md) - Créer des structures de fichiers
