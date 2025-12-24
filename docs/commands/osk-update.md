# osk update

Mettre à jour les fichiers de manière mécanique.

## Synopsis

```bash
osk update [--json] <subcommand> [options]
```

## Description

`osk update` effectue des mises à jour mécaniques sur les fichiers du projet : recalcul de statistiques, changement de statuts, régénération de rapports.

## Sous-commandes

### stats

Recalcule les statistiques du risk-register.

```bash
osk update stats
```

Met à jour les compteurs dans `docs/security/risks/risk-register.yaml`.

### task

Met à jour le statut d'une tâche.

```bash
osk update task <id> --done
```

| Option | Description |
|--------|-------------|
| `--done` | Marquer la tâche comme terminée |

### risk

Met à jour le statut d'un risque.

```bash
osk update risk <id> --status <STATUS>
```

| Status | Description |
|--------|-------------|
| `OUVERT` | Risque identifié, non traité |
| `EN_COURS` | Mitigation en cours |
| `RESOLU` | Risque mitigé |
| `ACCEPTE` | Risque accepté |
| `TRANSFERE` | Risque transféré |

### dashboard

Régénère le dashboard de sécurité.

```bash
osk update dashboard
```

Régénère `docs/security/dashboard.md` avec les métriques actuelles.

## Options globales

| Option | Description |
|--------|-------------|
| `--json` | Sortie au format JSON (pour agents AI) |

## Exemples

### Marquer une tâche terminée

```bash
osk update task T001 --done
# ✅ Task T001 marked as done
```

### Mettre à jour un risque

```bash
osk update risk RISK-AUTH-001 --status RESOLU
# 📝 Updating risk RISK-AUTH-001 to status RESOLU
```

### Sortie JSON

```bash
osk update --json task T002 --done
```

```json
{
  "success": true,
  "command": "update task",
  "updated_files": [],
  "changes": [
    {
      "field": "T002.status",
      "old_value": "todo",
      "new_value": "done"
    }
  ],
  "message": "Task T002 marked as done"
}
```

## Structure JSON

```typescript
interface UpdateResult {
  success: boolean;
  command: string;
  updated_files: string[];
  changes: Change[];
  message: string;
}

interface Change {
  field: string;
  old_value: string | null;
  new_value: string;
}
```

## Voir aussi

- [osk validate](osk-validate.md) - Valider la cohérence
- [osk-implement](osk-implement.md) - Exécuter les tâches
