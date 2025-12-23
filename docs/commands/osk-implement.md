# /osk-implement

Implémentation des tâches avec mise à jour automatique du risk-register.

## Synopsis

```bash
>>> /osk-implement <feature> [options]
```

## Description

`/osk-implement` exécute les tâches de sécurité une par une, crée des commits atomiques et met à jour automatiquement le risk-register.

## Arguments

| Argument | Description | Exemple |
|----------|-------------|---------|
| `feature` | Nom de la feature | `"authentication"` |

## Options

| Option | Description |
|--------|-------------|
| `--auto` | Exécute sans confirmation |
| `--dry-run` | Affiche sans modifier |
| `--task T001` | Exécute une tâche spécifique |

## Processus

Pour chaque tâche :

1. **Affichage** - Montre la tâche et ses instructions
2. **Implémentation** - Applique les changements
3. **Validation** - Vérifie les critères de done
4. **Mise à jour** - Met à jour `tasks.yaml` et `risk-register.yaml`
5. **Commit** - Crée un commit atomique

## Mise à jour du Risk Register

Quand une tâche est terminée, les risques associés sont mis à jour :

```yaml
- id: "RISK-AUTH-001"
  statut: "RESOLU"  # Modifié automatiquement
  resolution:
    commit: "abc123"
    description: "T001 - Implémenter rate limiting"
    controles_implementes:
      - "Rate limiting 5 req/min"
  dates:
    resolution: "2025-01-15"
```

## Exemple

```bash
>>> /osk-implement "authentication"
```

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
TÂCHE 1/5 : T001
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
Implémenter rate limiting

Fichiers : src/auth/login.ts
Effort   : 2h
Risques  : RISK-AUTH-001

Instructions :
1. Installer express-rate-limit
2. Configurer 5 req/min par IP
3. Ajouter tests
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

[Implémentation...]

✅ T001 terminée
   Commit: abc123
   Risques résolus: RISK-AUTH-001
```

## Rapport Final

```
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
/osk-implement authentication - Terminé
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

RÉSUMÉ
──────
Tâches exécutées : 5/5
Commits créés    : 5

IMPACT SUR LE RISK REGISTER
───────────────────────────
Risques résolus     : 4
Score résiduel      : 405 → 125 (↓280)
Taux résolution     : 45% → 78%

PROCHAINES ÉTAPES
─────────────────
→ /osk-dashboard pour voir l'impact global
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
```

## Règles

1. **Ordre des dépendances** - Respecte `depends_on`
2. **Un commit par tâche** - Traçabilité maximale
3. **Mise à jour immédiate** - Risk-register après chaque tâche
4. **Validation** - Vérifie critères done avant de continuer
5. **Patterns projet** - Respecte le style de code existant

## Prérequis

- `/osk-tasks <feature>` exécuté
- `.osk/specs/NNN-feature/tasks.yaml` présent

## Prochaine étape

```bash
>>> /osk-dashboard
```
