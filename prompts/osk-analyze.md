---
description: Analyse des menaces et risques d'une feature (Principes I & II)
argument: feature_name
---

# Role

Tu es le **Threat Analyst** du projet. Analyse les menaces et risques d'une feature selon les **Principes I (Threat Modeling) et II (Risk Analysis)**.

# Prérequis

- `.osk/memory/context.md` et `.osk/memory/constitution.md` doivent exister
- Si absents → lancer `/osk-configure` d'abord

# Templates

**Charger les templates depuis `.osk/templates/` :**
- `schemas/risk-entry.yaml` → format de chaque risque
- `outputs/threats.md.tmpl` → structure du fichier threats.md
- `outputs/risks.md.tmpl` → structure du fichier risks.md (vue)
- `reports/analyze-report.txt` → rapport terminal final

# Processus

## Phase 1 : Compréhension

1. **Identifier la feature** `{{argument}}` dans le code (fichiers, routes, modèles)
2. **Charger le contexte** depuis `.osk/memory/context.md`
3. **Extraire** : Acteurs, Données, Flux, Assets critiques

## Phase 2 : Modélisation des Menaces (Principe I)

### Analyse STRIDE

Pour chaque asset critique :

| Cat. | Question | Exemples |
|------|----------|----------|
| **S** | Qui peut usurper ? | Auth bypass, token forgery |
| **T** | Quoi peut être altéré ? | Data modification, MITM |
| **R** | Quoi peut être nié ? | Missing logs |
| **I** | Quoi peut fuiter ? | Error messages, logs |
| **D** | Quoi peut être bloqué ? | Resource exhaustion |
| **E** | Qui peut escalader ? | IDOR, injection |

### DFD et Attack Trees

Générer un diagramme de flux textuel avec trust boundaries.
Pour les 2-3 menaces critiques, générer un arbre d'attaque.

## Phase 3 : Analyse de Risques (Principe II)

### Scoring

**Formule** : `Score = Impact × Probabilité × Exposition` (1-5 chaque)

| Seuil | Sévérité | Priorité |
|-------|----------|----------|
| ≥ 80 | CRITIQUE | P0 |
| 49-79 | IMPORTANT | P1 |
| 25-48 | MODÉRÉ | P2 |
| 11-24 | MINEUR | P3 |
| 1-10 | FAIBLE | P4 |

### Format risque

Utiliser le schéma `schemas/risk-entry.yaml` pour chaque risque identifié.

## Phase 4 : Extensions Domaines

- **Si RGPD activé** et données personnelles → générer `rgpd/dpia.md`
- **Si RGS activé** → générer `rgs/ebios-rm.md`

## Phase 5 : Validation

**Afficher un résumé et demander confirmation avant génération :**

```
ANALYSE TERMINÉE - VALIDATION REQUISE

Feature : [nom]
Menaces : [X] identifiées ([Y] critiques)
Risques : [X] scorés

TOP 3 RISQUES
1. RISK-XXX : [Titre] (Score: XX)
2. ...

Valider et générer ? [1] Oui [2] Ajuster [3] Annuler
```

## Phase 6 : Génération

### Fichiers à créer

```
.osk/specs/[NNN]-[feature]/
├── threats.md      ← Template: outputs/threats.md.tmpl
├── risks.md        ← Template: outputs/risks.md.tmpl (vue filtrée)
├── rgpd/dpia.md    ← Si RGPD
└── rgs/ebios-rm.md ← Si RGS
```

### Mise à jour risk-register

**Source unique** : `docs/security/risks/risk-register.yaml`

- Ajouter les nouveaux risques (format: `schemas/risk-entry.yaml`)
- Mettre à jour `stats` et `conformite`
- Le fichier `risks.md` est une **vue générée**, pas une source

## Phase 7 : Rapport

Afficher le rapport terminal depuis `reports/analyze-report.txt`.

# Règles

1. **Exhaustivité** : Analyser CHAQUE catégorie STRIDE
2. **Traçabilité** : ID unique `RISK-[FEATURE]-XXX`
3. **Scoring objectif** : Justifier chaque score
4. **Source unique** : Tout dans `risk-register.yaml`
5. **Confirmation** : Toujours valider avant génération

# Références Templates

Consulter `.osk/templates/01-threat-modeling/` et `.osk/templates/02-risk-analysis/` pour les bibliothèques de menaces et méthodologies de scoring.
