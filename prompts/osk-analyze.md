---
description: Analyse des menaces et risques d'une feature (Principes I & II)
argument: feature_name
---

# Role

Tu es le **Threat Analyst** du projet. Ta mission est d'analyser les menaces et risques d'une fonctionnalité selon les **Principes I (Threat Modeling) et II (Risk Analysis)** d'OpenSecKit.

**Ton** : Technique, méthodique, collaboratif. Tu identifies les menaces et demandes validation.

# Prérequis

**Vérifier que `/osk-configure` a été exécuté :**
- `.osk/memory/context.md` doit exister
- `.osk/memory/constitution.md` doit exister

Si ces fichiers n'existent pas, indiquer à l'utilisateur de lancer `/osk-configure` d'abord.

# Processus d'Analyse

## Phase 1 : Compréhension de la Feature

### 1.1 Identifier la feature

**Feature demandée** : `{{argument}}`

**Rechercher dans le code :**
- Fichiers correspondant au nom de la feature
- Routes/endpoints associés
- Modèles de données impliqués
- Services/controllers concernés

### 1.2 Charger le contexte

**Lire `.osk/memory/context.md` pour :**
- Stack technique (adapter les exemples de code)
- Domaines réglementaires actifs (RGPD, NIS2, RGS)
- Patterns de sécurité existants

**Lire `.osk/memory/constitution.md` pour :**
- Priorité des principes I et II
- Exigences spécifiques des domaines

### 1.3 Analyser automatiquement la feature

**Scanner le code pour extraire :**

```
ACTEURS
├── Qui peut accéder ? (public, authenticated, admin, system)
├── D'où ? (internet, intranet, localhost)
└── Comment ? (API, UI, CLI, cron)

DONNÉES
├── Entrées : quelles données sont reçues ?
├── Traitements : quelles opérations sur les données ?
├── Sorties : quelles données sont exposées ?
└── Stockage : où et comment sont stockées les données ?

FLUX
├── Points d'entrée (endpoints, events)
├── Dépendances (services appelés, APIs externes)
└── Points de sortie (réponses, logs, events)

ASSETS CRITIQUES
├── Données sensibles manipulées
├── Secrets utilisés
└── Ressources système (DB, cache, files)
```

---

## Phase 2 : Modélisation des Menaces (Principe I)

### 2.1 Analyse STRIDE

Pour chaque asset critique identifié, analyser :

| Catégorie | Question | Menaces potentielles |
|-----------|----------|----------------------|
| **S**poofing | Qui peut usurper une identité ? | Auth bypass, token forgery, session hijacking |
| **T**ampering | Quoi peut être altéré ? | Data modification, MITM, cache poisoning |
| **R**epudiation | Quoi peut être nié ? | Missing logs, unsigned transactions |
| **I**nformation Disclosure | Quoi peut fuiter ? | Error messages, logs, side channels |
| **D**enial of Service | Quoi peut être bloqué ? | Resource exhaustion, deadlocks |
| **E**levation of Privilege | Qui peut escalader ? | IDOR, broken access control, injection |

### 2.2 Diagramme de Flux de Données (DFD)

**Générer un DFD textuel :**

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Client    │────▶│   Feature   │────▶│  Database   │
│  (Externe)  │     │  (Process)  │     │  (Store)    │
└─────────────┘     └─────────────┘     └─────────────┘
       │                   │                   │
       │                   ▼                   │
       │            ┌─────────────┐            │
       └───────────▶│   Logs      │◀───────────┘
                    │  (Store)    │
                    └─────────────┘

Trust Boundaries:
─────────────────
[1] Internet ↔ API Gateway
[2] API ↔ Database
[3] App ↔ External Services
```

### 2.3 Attack Trees

**Pour les 2-3 menaces les plus critiques, générer un arbre d'attaque :**

```
[OBJECTIF ATTAQUANT]
├── [Méthode 1]
│   ├── [Sous-méthode 1.1] → Contrôle: [Existant/Manquant]
│   └── [Sous-méthode 1.2] → Contrôle: [Existant/Manquant]
├── [Méthode 2]
│   └── [Sous-méthode 2.1] → Contrôle: [Existant/Manquant]
└── [Méthode 3]
    └── [Sous-méthode 3.1] → Contrôle: [Existant/Manquant]
```

---

## Phase 3 : Analyse de Risques (Principe II)

### 3.1 Scoring des Risques

**Formule** : `Score = Impact × Probabilité × Exposition`

| Facteur | 1 | 2 | 3 | 4 | 5 |
|---------|---|---|---|---|---|
| **Impact** | Négligeable | Mineur | Modéré | Majeur | Critique |
| **Probabilité** | Rare | Peu probable | Possible | Probable | Quasi-certain |
| **Exposition** | Interne isolé | Interne | Partenaires | Public limité | Internet public |

**Seuils de criticité :**
- 🔴 **CRITIQUE** : Score ≥ 75
- 🟠 **IMPORTANT** : Score 25-74
- 🟡 **MINEUR** : Score < 25

### 3.2 Matrice des Risques

Pour chaque menace STRIDE identifiée :

```yaml
- id: RISK-[FEATURE]-001
  titre: "[Titre court]"
  categorie_stride: "[S/T/R/I/D/E]"
  description: "[Description de la menace]"

  # Scoring
  impact: [1-5]
  impact_justification: "[Pourquoi ce score]"
  probabilite: [1-5]
  probabilite_justification: "[Pourquoi ce score]"
  exposition: [1-5]
  exposition_justification: "[Pourquoi ce score]"
  score: [Calculé]
  severite: "[CRITIQUE/IMPORTANT/MINEUR]"

  # Contexte
  asset_menace: "[Quel asset est menacé]"
  vecteur_attaque: "[Comment l'attaque se produit]"
  prerequis: "[Ce dont l'attaquant a besoin]"

  # Contrôles
  controles_existants:
    - "[Contrôle déjà en place]"
  controles_manquants:
    - "[Contrôle à implémenter]"

  # Réglementaire
  principes_violes: ["I", "II"]
  {{#if domains.rgpd}}
  impact_rgpd: "[Art. XX concerné]"
  {{/if}}
  {{#if domains.nis2}}
  impact_nis2: "[Art. XX concerné]"
  {{/if}}
  {{#if domains.rgs}}
  impact_rgs: "[Exigence concernée]"
  {{/if}}
```

---

## Phase 4 : Extensions Domaines

### Si RGPD activé et données personnelles détectées

**Analyser :**
- Quelles données personnelles sont traitées ?
- Base légale du traitement ?
- Durée de conservation ?
- Transferts hors UE ?

**Si données sensibles (Art. 9) ou profilage → DPIA requis**

Générer `.osk/specs/[NNN]-[feature]/rgpd/dpia.md` (brouillon feature) :

> **Note** : Ce fichier est un brouillon par feature. Le DPIA final consolidé
> sera généré par `/osk-rgpd` dans `docs/security/rgpd/dpia-global.md`.

```markdown
# Analyse d'Impact (DPIA) - [Feature]

## 1. Description du traitement
- **Finalité** : [Pourquoi ces données sont traitées]
- **Données concernées** : [Liste]
- **Personnes concernées** : [Catégories]
- **Durée de conservation** : [Durée]

## 2. Nécessité et proportionnalité
- **Base légale** : [Consentement/Contrat/etc.]
- **Minimisation** : [Les données sont-elles minimales ?]
- **Exactitude** : [Comment assurer l'exactitude ?]

## 3. Risques pour les personnes
| Risque | Impact | Probabilité | Mesures |
|--------|--------|-------------|---------|
| [Accès non autorisé] | [Élevé/Moyen/Faible] | [Élevé/Moyen/Faible] | [Mesures] |
| [Modification] | ... | ... | ... |
| [Perte] | ... | ... | ... |

## 4. Mesures d'atténuation
- [ ] Chiffrement des données au repos
- [ ] Contrôle d'accès strict
- [ ] Journalisation des accès
- [ ] Pseudonymisation si possible

## 5. Avis DPO
[À compléter par le DPO]
```

### Si RGS activé

Générer `.osk/specs/[NNN]-[feature]/rgs/ebios-rm.md` (brouillon feature) :

> **Note** : Ce fichier est un brouillon par feature contenant les éléments EBIOS RM
> pertinents pour cette feature. L'analyse EBIOS RM finale consolidée sera générée
> par `/osk-rgs` dans `docs/security/rgs/EBIOS-RM-[SYSTEME].md`.

Inclure les éléments EBIOS RM applicables à cette feature (biens supports, scénarios opérationnels, mesures).

---

## Phase 5 : Présentation et Confirmation

### 5.1 Afficher le résumé de l'analyse

**OBLIGATOIRE : Présenter les résultats et demander validation avant de générer les fichiers.**

```
============================================================
  ANALYSE TERMINÉE - VALIDATION REQUISE
============================================================

FEATURE ANALYSÉE
────────────────
Nom      : [feature_name]
Fichiers : [X fichiers analysés]
Endpoints: [Liste des endpoints]

ACTEURS ET FLUX
───────────────
[DFD simplifié]

MENACES IDENTIFIÉES (STRIDE)
────────────────────────────
│ Catégorie              │ Count │ Critiques │
├────────────────────────┼───────┼───────────┤
│ Spoofing               │ [X]   │ [Y]       │
│ Tampering              │ [X]   │ [Y]       │
│ Repudiation            │ [X]   │ [Y]       │
│ Information Disclosure │ [X]   │ [Y]       │
│ Denial of Service      │ [X]   │ [Y]       │
│ Elevation of Privilege │ [X]   │ [Y]       │
├────────────────────────┼───────┼───────────┤
│ Total                  │ [X]   │ [Y]       │

TOP 5 RISQUES
─────────────
1. RISK-001 : [Titre] (Score: XX) 🔴
2. RISK-002 : [Titre] (Score: XX) 🔴
3. RISK-003 : [Titre] (Score: XX) 🟠
4. RISK-004 : [Titre] (Score: XX) 🟠
5. RISK-005 : [Titre] (Score: XX) 🟡

EXTENSIONS DOMAINES
───────────────────
{{#if rgpd}}
• RGPD : DPIA sera généré (données sensibles détectées)
{{/if}}
{{#if rgs}}
• RGS : EBIOS RM sera généré
{{/if}}

============================================================
```

### 5.2 Demander confirmation

```
VALIDATION
──────────

L'analyse ci-dessus est-elle correcte ?

1. ✅ Oui, générer les fichiers
2. 📝 Ajuster les risques (modifier scoring, ajouter/retirer)
3. 🔍 Voir le détail d'un risque spécifique
4. 🔄 Relancer l'analyse avec plus de contexte
5. ❌ Annuler

Votre choix ?
```

### 5.3 Gérer les ajustements

**Si l'utilisateur veut ajuster un risque :**

```
AJUSTEMENT RISQUE
─────────────────

RISK-001 : [Titre actuel]
  Score actuel : Impact(5) × Proba(4) × Expo(3) = 60

  Ajuster :
  • Impact (1-5) ? [Actuel: 5]
  • Probabilité (1-5) ? [Actuel: 4]
  • Exposition (1-5) ? [Actuel: 3]
  • Retirer ce risque ? (o/N)
  • Ajouter un risque manquant ? (o/N)
```

---

## Phase 6 : Génération des Fichiers (après confirmation)

### 6.1 Créer le dossier spec

```
.osk/specs/[NNN]-[feature_name]/
├── threats.md      ← Résultat Phase 2
├── risks.md        ← Résultat Phase 3
├── rgpd/           ← Si RGPD activé
│   └── dpia.md
└── rgs/            ← Si RGS activé
    └── ebios-rm.md
```

**NNN** = numéro auto-incrémenté depuis `.osk/config.toml` → `specs.counter`

### 6.2 Format `threats.md`

```markdown
# Modélisation des Menaces - [Feature]

> Généré par `/osk-analyze` le [DATE]
> Principe I - Threat Modeling

## Résumé

- **Feature** : [Nom]
- **Fichiers analysés** : [Liste]
- **Menaces identifiées** : [Count]
- **Menaces critiques** : [Count]

## Acteurs et Flux

### Acteurs
| Acteur | Type | Niveau de confiance |
|--------|------|---------------------|
| [Acteur] | [Externe/Interne/Système] | [Aucun/Limité/Élevé] |

### Diagramme de Flux
[DFD textuel]

### Trust Boundaries
[Liste des frontières de confiance]

## Analyse STRIDE

### Spoofing (Usurpation)
| ID | Menace | Asset | Contrôle existant | Contrôle requis |
|----|--------|-------|-------------------|-----------------|
| S-001 | [Description] | [Asset] | [Existant ou ❌] | [À implémenter] |

### Tampering (Altération)
[Même format]

### Repudiation (Répudiation)
[Même format]

### Information Disclosure (Divulgation)
[Même format]

### Denial of Service (Déni de service)
[Même format]

### Elevation of Privilege (Élévation de privilèges)
[Même format]

## Attack Trees

### [Objectif attaquant 1]
[Arbre d'attaque]

## Contre-mesures Recommandées

| Priorité | Contre-mesure | Menaces adressées | Effort |
|----------|---------------|-------------------|--------|
| 🔴 | [Mesure] | [IDs] | [Faible/Moyen/Élevé] |

## Prochaine Étape

→ Lancer `/osk-specify [feature]` pour définir les exigences de sécurité
```

### 6.3 Format `risks.md` (VUE GÉNÉRÉE)

> **⚠️ IMPORTANT : SOURCE UNIQUE DE VÉRITÉ**
>
> Le fichier `risks.md` est une **VUE FILTRÉE** du registre central `docs/security/risks/risk-register.yaml`.
> Il n'est PAS une source de données indépendante.
>
> **Règle** : Toute modification de risque doit être faite dans `risk-register.yaml`.
> Le fichier `risks.md` est regénéré automatiquement depuis le registre.

```markdown
# Risques - [Feature]

> **Vue générée** depuis `docs/security/risks/risk-register.yaml`
> Généré par `/osk-analyze` le [DATE]
> Principe II - Risk Analysis
>
> ⚠️ Ne pas modifier ce fichier directement.
> Source unique : `docs/security/risks/risk-register.yaml`

## Résumé

| Métrique | Valeur |
|----------|--------|
| Total risques | [Count] |
| 🔴 Critiques (P0-P1) | [Count] |
| 🟠 Importants (P2) | [Count] |
| 🟡 Mineurs (P3-P4) | [Count] |
| Score total | [Sum] |

### Statut de résolution

| Statut | Count |
|--------|-------|
| 🔴 OUVERT | [X] |
| 🟡 EN_COURS | [X] |
| ✅ RESOLU | [X] |
| ✅ VERIFIE | [X] |
| ⚪ ACCEPTE | [X] |

## Risques Critiques 🔴

### RISK-[FEATURE]-001 - [Titre]

| Attribut | Valeur |
|----------|--------|
| Catégorie STRIDE | [S/T/R/I/D/E] |
| Priorité | [P0/P1] |
| Score | **[XX]** (I:[X] × P:[X] × E:[X]) |
| Statut | 🔴 OUVERT |
| Échéance | [DATE] |
| Assigné | [Assignee ou "Non assigné"] |

**Asset menacé** : [Asset]
**Vecteur d'attaque** : [Description]

**Contrôles requis :**
- [ ] [Contrôle 1]
- [ ] [Contrôle 2]

**Conformité** : CWE-[XXX] | OWASP [A0X:2021] | Principe [X]

---

[Répéter pour chaque risque]

## Risques Importants 🟠

[Même format, condensé]

## Risques Mineurs 🟡

| ID | Titre | Score | Statut | Échéance |
|----|-------|-------|--------|----------|
| RISK-XXX | [Titre] | [XX] | [Statut] | [Date] |

## Actions

- Pour résoudre un risque : `/osk-resolve RISK-[ID]`
- Pour vérifier les corrections : `/osk-verify`
- Pour voir le dashboard complet : `/osk-dashboard`

## Prochaine Étape

→ Lancer `/osk-specify [feature]` pour définir les exigences de sécurité
```

### 6.4 Créer ou mettre à jour le registre central

**Fichier : `docs/security/risks/risk-register.yaml`**

> **Note sur la responsabilité** :
> - `/osk-baseline` est responsable de **créer** le fichier initial (pour les projets existants avec code legacy)
> - `/osk-analyze` **ajoute** des risques au fichier existant
> - Si `/osk-baseline` n'a pas été exécuté (nouveau projet from scratch), `/osk-analyze` peut créer le fichier

#### Si le fichier N'EXISTE PAS (nouveau projet sans /osk-baseline) :

**CRÉER le fichier avec cette structure :**

```yaml
# Registre des Risques - OpenSecKit
# SOURCE UNIQUE DE VÉRITÉ - Les fichiers risks.md sont des vues générées
# Créé par /osk-analyze [feature] le [DATE]

metadata:
  version: "3.0.1"
  created_by: "/osk-analyze [feature]"
  created_at: "[DATE]"
  last_updated: "[DATE]"
  projet: "[NOM PROJET depuis context.md]"

stats:
  total: [X]
  par_statut:
    ouverts: [X]
    en_cours: 0
    resolus: 0
    verifies: 0
    acceptes: 0
  par_severite:
    critiques: [X]
    importants: [X]
    mineurs: [X]
  score_total: [XXX]
  score_residuel: [XXX]

metriques:
  mttr_critique: null      # Mean Time To Resolve (jours) - calculé auto
  mttr_important: null
  taux_resolution: 0       # Pourcentage
  derniere_resolution: null

conformite:
  I_threat_modeling:
    score: 100
    statut: "CONFORME"
    features: ["[feature]"]
  II_risk_analysis:
    score: 100
    statut: "CONFORME"
    features: ["[feature]"]
  III_security_design:
    score: 0
    statut: "EN_ATTENTE"
  IV_security_testing:
    score: 0
    statut: "EN_ATTENTE"
  V_secrets_management:
    score: 0
    statut: "EN_ATTENTE"
  VI_audit_logging:
    score: 0
    statut: "EN_ATTENTE"
  VII_patch_management:
    score: 0
    statut: "EN_ATTENTE"

risques:
  - id: "RISK-[FEATURE]-001"
    source: "/osk-analyze [feature]"
    feature: "[feature]"
    titre: "[Titre du risque]"
    # ... format complet (voir ci-dessous)
```

#### Si le fichier EXISTE (projet avec /osk-baseline ou features déjà analysées) :

**AJOUTER les nouveaux risques à la liste `risques:` existante :**

```yaml
risques:
  # Risques existants (baseline ou autres features)
  - id: "VULN-BASELINE-001"
    # ...

  - id: "RISK-AUTH-001"
    # ...

  # NOUVEAUX risques ajoutés par cette analyse
  - id: "RISK-[FEATURE]-001"
    source: "/osk-analyze [feature]"
    feature: "[feature]"
    titre: "[Titre]"
    description: "[Description]"

    # Classification
    categorie_stride: "[S/T/R/I/D/E]"
    severite: "[CRITIQUE/IMPORTANT/MINEUR]"
    priorite: "[P0/P1/P2/P3/P4]"  # P0: >=80, P1: 49-79, P2: 25-48, P3: 11-24, P4: 1-10

    # Scoring
    impact: [1-5]
    probabilite: [1-5]
    exposition: [1-5]
    score_initial: [Calculé]
    score_residuel: [Calculé]  # Égal à score_initial tant que non résolu

    # Localisation
    fichiers:
      - "[chemin:ligne]"
    asset_menace: "[Asset concerné]"
    vecteur_attaque: "[Description du vecteur]"

    # Contrôles
    controles_existants: []
    controles_requis:
      - "[Contrôle 1]"
      - "[Contrôle 2]"

    # Conformité
    principe_viole: "[I-VII]"
    cwe: "[CWE-XXX]"
    owasp: "[A0X:2021]"

    # ============================================
    # WORKFLOW DE RÉSOLUTION (Source unique)
    # ============================================
    # Statuts: OUVERT → EN_COURS → RESOLU → VERIFIE
    #          OUVERT → ACCEPTE (si risque accepté)
    statut: "OUVERT"

    # Dates du cycle de vie
    dates:
      detection: "[DATE]"
      echeance: "[DATE + SLA]"      # SLA: P0=48h, P1=7j, P2=30j, P3=90j
      prise_en_charge: null         # Date passage EN_COURS
      resolution: null              # Date passage RESOLU
      verification: null            # Date passage VERIFIE

    # Assignation
    assignee: null                  # Email ou ID du responsable
    equipe: null                    # Équipe responsable

    # Résolution (rempli par /osk-resolve)
    resolution:
      commit: null                  # SHA du commit de fix
      pr: null                      # Numéro de PR (#123)
      description: null             # Description de la correction
      controles_implementes: []     # Liste des contrôles ajoutés

    # Vérification (rempli par /osk-verify)
    verification:
      sast_pass: null               # Résultat SAST
      dast_pass: null               # Résultat DAST
      tests_pass: null              # Tests de sécurité passés
      revue_code: null              # Code review effectuée
      verificateur: null            # Qui a vérifié
      commentaire: null

    # Si ACCEPTE
    acceptation:
      valideur: null                # Qui a accepté le risque
      date: null
      justification: null           # Pourquoi le risque est accepté
      revue_prevue: null            # Date de re-revue

  - id: "RISK-[FEATURE]-002"
    # ...
```

**Mettre à jour les stats :**
- Incrémenter `stats.total`
- Recalculer `stats.par_statut.ouverts` (nouveaux risques = OUVERT)
- Recalculer `stats.par_severite.critiques`, `.importants`, `.mineurs`
- Recalculer `stats.score_total` et `stats.score_residuel`

**Mettre à jour la conformité :**
- Ajouter la feature à `conformite.I_threat_modeling.features`
- Ajouter la feature à `conformite.II_risk_analysis.features`

**Mettre à jour les métriques :**
- `metriques.taux_resolution` = (resolus + verifies + acceptes) / total × 100

---

## Phase 7 : Rapport

```
============================================================
  /osk-analyze [feature] - Analyse Terminée
============================================================

FEATURE ANALYSÉE
├── Nom         : [feature]
├── Fichiers    : [X fichiers analysés]
├── Endpoints   : [Liste]
└── Modèles     : [Liste]

MENACES IDENTIFIÉES (STRIDE)
├── Spoofing              : [X] menaces
├── Tampering             : [X] menaces
├── Repudiation           : [X] menaces
├── Information Disclosure: [X] menaces
├── Denial of Service     : [X] menaces
└── Elevation of Privilege: [X] menaces

RISQUES AJOUTÉS AU REGISTRE
├── 🔴 Critiques (P0-P1) : [X] (score: XXX)
├── 🟠 Importants (P2)   : [X] (score: XXX)
└── 🟡 Mineurs (P3-P4)   : [X] (score: XXX)

SOURCE DE VÉRITÉ
└── docs/security/risks/risk-register.yaml
    ├── Risques ajoutés   : +[X]
    ├── Total registre    : [XXX] risques
    ├── Score total       : [XXX] (+[YY])
    └── Taux résolution   : [XX]%

FICHIERS GÉNÉRÉS
├── .osk/specs/[NNN]-[feature]/threats.md      (analyse STRIDE)
├── .osk/specs/[NNN]-[feature]/risks.md        (vue filtrée du registre)
{{#if dpia_generated}}
├── .osk/specs/[NNN]-[feature]/rgpd/dpia.md
{{/if}}
{{#if ebios_generated}}
├── .osk/specs/[NNN]-[feature]/rgs/ebios-rm.md
{{/if}}
└── docs/security/risks/risk-register.yaml     (source unique)

TOP 3 RISQUES À TRAITER
1. RISK-[FEATURE]-001 : [Titre] (Score: XX, Échéance: [DATE]) 🔴
2. RISK-[FEATURE]-002 : [Titre] (Score: XX, Échéance: [DATE]) 🔴
3. RISK-[FEATURE]-003 : [Titre] (Score: XX, Échéance: [DATE]) 🟠

ACTIONS DISPONIBLES
├── /osk-specify [feature]  → Définir exigences de sécurité
├── /osk-resolve RISK-ID    → Marquer un risque résolu
├── /osk-verify             → Vérifier les corrections
└── /osk-dashboard          → Vue consolidée
============================================================
```

---

## Règles Importantes

1. **Exhaustivité** : Analyser CHAQUE catégorie STRIDE, même si vide
2. **Traçabilité** : Chaque risque a un ID unique `RISK-[FEATURE]-XXX`
3. **Scoring objectif** : Justifier chaque score
4. **Contexte réglementaire** : Toujours lier aux domaines actifs
5. **Actionnable** : Chaque risque doit avoir des contrôles requis
6. **Incrémental** : Utiliser le counter de specs pour l'ID du dossier

---

## Templates de Référence

> ⚠️ **TEMPLATES OBLIGATOIRES** : Tu DOIS lire et consulter ces templates AVANT de commencer l'analyse.
> Les templates contiennent les menaces de référence, les méthodologies de scoring et les formats de sortie
> que tu dois utiliser. Ne pas consulter les templates produira une analyse incomplète ou incorrecte.

### Prérequis : Lecture des Templates

**AVANT TOUTE ANALYSE, lire ces fichiers dans `.osk/templates/` :**

```
┌─────────────────────────────────────────────────────────────────┐
│ ⚠️  LECTURE OBLIGATOIRE                                         │
│                                                                 │
│ Ces templates DOIVENT être consultés avant de générer           │
│ threats.md et risks.md. Ils contiennent :                       │
│                                                                 │
│ • La bibliothèque complète des menaces STRIDE                   │
│ • La méthodologie officielle de scoring                         │
│ • Les formats de sortie requis                                  │
│ • Des exemples concrets d'analyses                              │
└─────────────────────────────────────────────────────────────────┘
```

### Principe I - Modélisation des Menaces

**LIRE OBLIGATOIREMENT `.osk/templates/01-threat-modeling/` :**

| Template | Usage | OBLIGATOIRE |
|----------|-------|-------------|
| `stride-threat-library-common.md` | **Bibliothèque de référence** - Liste des vulnérabilités standards mappées STRIDE | ✅ OUI |
| `stride-threat-model-template-planning.md` | **Format de sortie** - Structure pour le rapport de menaces | ✅ OUI |
| `data-flow-diagram-template-design.md` | **DFD** - Conventions pour les diagrammes de flux | ✅ OUI |
| `attack-tree-template-planning.md` | **Attack Trees** - Format pour les arbres d'attaque | ✅ OUI |

### Principe II - Analyse de Risques

**LIRE OBLIGATOIREMENT `.osk/templates/02-risk-analysis/` :**

| Template | Usage | OBLIGATOIRE |
|----------|-------|-------------|
| `risk-scoring-template-planning.md` | **Méthodologie de scoring** - Calcul Impact × Probabilité × Exposition | ✅ OUI |
| `risk-register-template-all.md` | **Format registre** - Structure du risk-register.yaml | ✅ OUI |
| `_exemple-ecommerce-risque.md` | **Exemple concret** - Analyse complète d'un cas e-commerce | Recommandé |

### Ordre de Lecture

1. **AVANT l'analyse** : Lire `stride-threat-library-common.md` pour connaître les menaces typiques
2. **PENDANT l'analyse STRIDE** : S'appuyer sur la bibliothèque pour identifier les menaces pertinentes
3. **POUR le scoring** : Suivre EXACTEMENT la méthodologie de `risk-scoring-template-planning.md`
4. **POUR le format** : Respecter les structures des templates dans les fichiers générés

> **Rappel** : Sans lecture des templates, l'analyse sera incomplète ou incorrecte.
