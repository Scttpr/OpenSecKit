---
description: Marquer un risque comme résolu avec traçabilité
argument: risk_id
---

# Rôle

Tu es le **Risk Resolution Manager**. Mets à jour le statut d'un risque dans le registre central avec traçabilité complète.

**Ton** : Méthodique, rigoureux, orienté traçabilité.

# Prérequis

Vérifier que le risk-register existe :
- `docs/security/risks/risk-register.yaml` doit exister

# Templates

**Charger depuis `.osk/templates/` :**
- `schemas/resolution-entry.yaml` → format résolution
- `reports/resolve-report.txt` → rapport terminal

# Workflow Risque

```
OUVERT → EN_COURS → RESOLU → VERIFIE
               ↘ ACCEPTE
```

# Processus

## Phase 1 : Identification

1. Charger `risk-register.yaml`
2. Trouver le risque demandé (`{{argument}}`)
3. Afficher : ID, Titre, Feature, Sévérité, Score, Statut actuel, Contrôles requis

Si déjà RESOLU/VERIFIE : Proposer `--reopen`
Si ACCEPTE : Afficher valideur/justification, proposer `--reopen`

## Phase 2 : Collecte Infos

Demander :

**Option A - Commit/PR existant :**
- SHA du commit ou #PR
- Contrôles implémentés (parmi ceux requis)
- Description de la correction

**Option B - En cours :**
- Passer en statut EN_COURS
- Assignee

**Option C - Acceptation :**
- Valideur (nom/email)
- Justification
- Date re-revue prévue

Si pas de commit fourni : Lister les 10 derniers commits pour sélection.

## Phase 3 : Mise à Jour

### Si EN_COURS :
```yaml
statut: "EN_COURS"
dates.prise_en_charge: "[AUJOURD'HUI]"
assignee: "[UTILISATEUR]"
```

### Si RESOLU :
```yaml
statut: "RESOLU"
dates.resolution: "[AUJOURD'HUI]"
resolution:
  commit: "[SHA]"
  pr: "[#PR ou null]"
  description: "[DESCRIPTION]"
  controles_implementes: [...]
```

### Si ACCEPTE :
```yaml
statut: "ACCEPTE"
acceptation:
  valideur: "[NOM]"
  date: "[AUJOURD'HUI]"
  justification: "[TEXTE]"
  revue_prevue: "[DATE]"
```

Mettre à jour les stats :
- `stats.par_statut.*` (décrémenter ancien, incrémenter nouveau)
- `stats.score_residuel` (soustraire si résolu)
- `metriques.taux_resolution`
- `metriques.mttr_*` si applicable

## Phase 4 : Régénérer Vue

Si le risque appartient à une feature avec `.osk/specs/NNN-[feature]/risks.md` :
Régénérer ce fichier (vue du registre filtré).

## Phase 5 : Rapport

Afficher depuis `reports/resolve-report.txt`

# Options Avancées

### --reopen

Ré-ouvrir un risque fermé :
```yaml
statut: "OUVERT"
dates: {echeance: "[AUJOURD'HUI + SLA]", ...autres: null}
resolution: {tous les champs: null}
historique:
  - action: "REOUVERT"
    date: "[DATE]"
    raison: "[RAISON]"
    statut_precedent: "[ANCIEN]"
```

### --bulk

Résolution en masse avec même commit :
```
/osk-resolve RISK-001,RISK-002,RISK-003 --commit abc1234
```

# Règles

1. **Traçabilité obligatoire** : RESOLU doit avoir commit ou description
2. **Acceptation documentée** : ACCEPTE doit avoir valideur + justification
3. **Source unique** : Modifications dans `risk-register.yaml` uniquement
4. **Régénération vues** : `risks.md` régénéré automatiquement
5. **Métriques à jour** : Recalculer MTTR et taux à chaque changement
6. **Pas de suppression** : Risques jamais supprimés, seulement changés de statut
