# osk scaffold

Créer des structures de fichiers pour les workflows.

## Synopsis

```bash
osk scaffold [--json] <subcommand> [options]
```

## Description

`osk scaffold` génère automatiquement les structures de fichiers nécessaires pour démarrer un workflow. Crée les templates avec les bonnes conventions de nommage.

## Sous-commandes

### feature

Crée la structure complète pour une nouvelle feature de sécurité.

```bash
osk scaffold feature <name> [--no-branch]
```

| Option | Description |
|--------|-------------|
| `--no-branch` | Ne pas créer de branche git |

**Fichiers créés :**
- `.osk/specs/NNN-<slug>/threats.md`
- `.osk/specs/NNN-<slug>/risks.md`
- `.osk/specs/NNN-<slug>/requirements.md`
- `.osk/specs/NNN-<slug>/testing.md`
- `.osk/specs/NNN-<slug>/hardening.md`
- `.osk/specs/NNN-<slug>/plan.md`
- `.osk/specs/NNN-<slug>/tasks.yaml`

### incident

Crée un rapport d'incident.

```bash
osk scaffold incident <description> [--severity LEVEL]
```

| Option | Description |
|--------|-------------|
| `--severity` | CRITIQUE, IMPORTANT, MODERE, MINEUR (défaut: IMPORTANT) |

**Fichier créé :** `docs/security/incidents/INC-YYYY-MM-DD-NNN.md`

### rgpd

Crée la structure pour la conformité RGPD.

```bash
osk scaffold rgpd
```

**Fichiers créés :**
- `docs/security/rgpd/registre-traitements.md`
- `docs/security/rgpd/dpia-global.md`
- `docs/security/rgpd/procedure-violation.md`

### rgs

Crée la structure pour la conformité RGS.

```bash
osk scaffold rgs <system>
```

**Fichiers créés :**
- `docs/security/rgs/EBIOS-RM-<system>.md`
- `docs/security/rgs/DOSSIER-HOMOLOGATION-<system>.md`
- `docs/security/rgs/MCS-<system>.md`

## Options globales

| Option | Description |
|--------|-------------|
| `--json` | Sortie au format JSON (pour agents AI) |

## Exemples

### Créer une feature

```bash
osk scaffold feature "user authentication"
# 📁 Created .osk/specs/001-user-authentication/
#    ├── threats.md
#    ├── risks.md
#    ├── requirements.md
#    ├── testing.md
#    ├── hardening.md
#    ├── plan.md
#    ├── tasks.yaml
# 🔀 Created branch: security/001-user-authentication
# ✅ Feature 'user authentication' scaffolded as 001-user-authentication
# 💡 Next: Run /osk-analyze user authentication
```

### Sortie JSON

```bash
osk scaffold --json feature "api gateway" --no-branch
```

```json
{
  "success": true,
  "command": "scaffold feature",
  "created_dir": ".osk/specs/002-api-gateway",
  "created_files": [
    ".osk/specs/002-api-gateway/threats.md",
    ".osk/specs/002-api-gateway/risks.md",
    ".osk/specs/002-api-gateway/requirements.md",
    ".osk/specs/002-api-gateway/testing.md",
    ".osk/specs/002-api-gateway/hardening.md",
    ".osk/specs/002-api-gateway/plan.md",
    ".osk/specs/002-api-gateway/tasks.yaml"
  ],
  "branch": null,
  "next_command": "/osk-analyze api gateway",
  "message": "Feature 'api gateway' scaffolded as 002-api-gateway"
}
```

### Créer un incident

```bash
osk scaffold incident "Fuite de données détectée" --severity CRITIQUE
```

## Structure JSON

```typescript
interface ScaffoldResult {
  success: boolean;
  command: string;
  created_dir: string | null;
  created_files: string[];
  branch: string | null;
  next_command: string | null;
  message: string;
}
```

## Voir aussi

- [osk check](osk-check.md) - Vérifier les prérequis
- [osk validate](osk-validate.md) - Valider la cohérence
