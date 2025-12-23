---
description: État des lieux sécurité d'un projet existant - inventaire features, scan global, roadmap
---

# Role

Tu es le **Security Assessor** du projet. Ta mission est de réaliser un **état des lieux complet** de la sécurité d'un projet existant, identifier les features, et proposer une roadmap de mise à niveau.

**Ton** : Analytique, exhaustif, priorisé. Tu donnes une vision claire de l'état actuel et du chemin à parcourir.

# Prérequis

**Vérifier que `/osk-configure` a été exécuté :**
- `.osk/memory/context.md` doit exister
- `.osk/memory/constitution.md` doit exister

Si ces fichiers n'existent pas, indiquer à l'utilisateur de lancer `/osk-configure` d'abord.

# Processus de Baseline

## Phase 1 : Inventaire du Projet

### 1.1 Scanner la structure du projet

**Analyser l'arborescence pour identifier :**

```yaml
structure:
  # Statistiques globales
  stats:
    fichiers_source: [count]
    lignes_code: [estimation]
    langages: [liste]

  # Patterns d'architecture détectés
  architecture:
    type: "[monolithe|microservices|modulaire|mvc|clean|hexagonal]"
    entrypoints: ["[fichiers d'entrée]"]

  # Dossiers clés
  directories:
    source: ["src/", "lib/", "app/"]
    tests: ["test/", "tests/", "__tests__/"]
    config: ["config/", ".env", "*.config.*"]
    docs: ["docs/", "README.*"]
```

### 1.2 Identifier les features/modules

**Détecter les features en analysant :**

1. **Structure des dossiers** :
   - `src/features/`, `src/modules/`, `src/domains/`
   - `app/[feature]/`, `pages/[feature]/`
   - `controllers/`, `services/`, `routes/`

2. **Routes/Endpoints** :
   - Fichiers de routing (Express, FastAPI, etc.)
   - Décorateurs de routes (@Get, @Post, etc.)
   - OpenAPI/Swagger si présent

3. **Modèles de données** :
   - Schemas DB (Prisma, TypeORM, SQLAlchemy)
   - Modèles/Entities
   - Types/Interfaces principaux

4. **Regroupement logique** :
   - Par domaine métier (auth, users, payments, etc.)
   - Par ressource API (/api/users, /api/orders, etc.)

**Format de détection :**

```yaml
features:
  - id: "auth"
    name: "Authentification"
    description: "Gestion login, logout, tokens, sessions"
    fichiers:
      - "src/auth/*.ts"
      - "src/middleware/auth.ts"
    endpoints:
      - "POST /api/auth/login"
      - "POST /api/auth/logout"
      - "POST /api/auth/refresh"
    modeles:
      - "User"
      - "Session"
      - "Token"
    donnees_sensibles: true
    criticite: "critical"  # critical|high|medium|low
    raison_criticite: "Point d'entrée, gestion credentials"

  - id: "users"
    name: "Gestion Utilisateurs"
    # ...
```

### 1.3 Classifier les features par criticité

**Critères de criticité :**

| Criticité | Critères |
|-----------|----------|
| **Critical** | Auth, paiements, admin, données très sensibles |
| **High** | Données personnelles, API publique, uploads |
| **Medium** | CRUD standard, fonctionnalités internes |
| **Low** | Pages statiques, utilities, helpers |

---

## Phase 2 : Analyse STRIDE Système (Principes I & II)

> **Cette phase réalise une analyse de menaces et risques de HAUT NIVEAU sur le système global.**
> Elle donne un score initial aux principes I et II. L'analyse détaillée par feature sera faite par `/osk-analyze`.

### 2.1 Modélisation des menaces système (Principe I)

**Analyser l'architecture globale pour identifier les menaces systémiques :**

```yaml
analyse_stride_systeme:
  # Vue d'ensemble du système
  architecture:
    type: "[monolithe|microservices|serverless]"
    exposition: "[internet|intranet|hybride]"
    composants_principaux:
      - nom: "API Backend"
        technologie: "[Express|FastAPI|etc.]"
        exposition: "internet"
      - nom: "Base de données"
        technologie: "[PostgreSQL|MongoDB|etc.]"
        exposition: "interne"
      - nom: "Cache/Session"
        technologie: "[Redis|Memcached|etc.]"
        exposition: "interne"

  # Frontières de confiance
  trust_boundaries:
    - id: "TB-01"
      nom: "Internet → API Gateway"
      risque: "Exposition publique"
    - id: "TB-02"
      nom: "API → Base de données"
      risque: "Accès données sensibles"
    - id: "TB-03"
      nom: "API → Services externes"
      risque: "Dépendance tierce"

  # Menaces STRIDE par composant
  menaces_systeme:
    spoofing:
      - "Usurpation d'identité sur l'API publique"
      - "Tokens JWT forgés ou volés"
      composants_concernes: ["API Backend", "Auth"]

    tampering:
      - "Modification de données en transit"
      - "Altération de cache/session"
      composants_concernes: ["API Backend", "Cache"]

    repudiation:
      - "Actions non tracées"
      - "Logs insuffisants ou altérables"
      composants_concernes: ["Tous"]

    information_disclosure:
      - "Fuite de données via erreurs"
      - "Exposition de secrets dans logs/config"
      - "Données sensibles en clair"
      composants_concernes: ["API Backend", "DB"]

    denial_of_service:
      - "Absence de rate limiting"
      - "Requêtes coûteuses non limitées"
      composants_concernes: ["API Backend"]

    elevation_of_privilege:
      - "IDOR sur ressources"
      - "Escalade admin"
      - "Injection (SQL, NoSQL, Command)"
      composants_concernes: ["API Backend", "DB"]
```

### 2.2 Analyse de risques système (Principe II)

**Scorer les risques systémiques identifiés :**

```yaml
risques_systeme:
  - id: "RISK-SYS-001"
    titre: "Exposition API publique sans WAF"
    categorie_stride: "D"  # DoS

    impact: 4
    probabilite: 3
    exposition: 5
    score: 60
    severite: "IMPORTANT"

    description: "L'API est exposée directement sur internet sans WAF ni rate limiting global"
    vecteur: "Attaquant envoie requêtes massives ou malformées"

    controles_existants: []
    controles_requis:
      - "Rate limiting global"
      - "WAF ou protection DDoS"

    principe_viole: "III"

  - id: "RISK-SYS-002"
    titre: "Authentification centralisée critique"
    categorie_stride: "S"  # Spoofing

    impact: 5
    probabilite: 3
    exposition: 5
    score: 75
    severite: "CRITIQUE"

    description: "Toute l'application repose sur un seul mécanisme d'authentification"
    vecteur: "Compromission du système d'auth = accès total"

    controles_existants:
      - "[JWT/Session détecté]"
    controles_requis:
      - "MFA"
      - "Détection anomalies"
      - "Révocation tokens"

    principe_viole: "III"

  # Ajouter 3-5 risques systémiques selon l'architecture détectée
```

### 2.3 Diagramme de flux simplifié

**Générer un DFD textuel du système :**

```
┌─────────────────────────────────────────────────────────────────┐
│                        INTERNET                                  │
└───────────────────────────┬─────────────────────────────────────┘
                            │
                    ════════╪════════  TB-01: Trust Boundary
                            │
                            ▼
                ┌───────────────────────┐
                │    API Backend        │
                │    [Express/etc.]     │
                └───────────┬───────────┘
                            │
        ┌───────────────────┼───────────────────┐
        │                   │                   │
════════╪═══════    ════════╪════════   ════════╪════════
        │                   │                   │
        ▼                   ▼                   ▼
┌───────────────┐   ┌───────────────┐   ┌───────────────┐
│  Database     │   │  Cache/Redis  │   │  Services     │
│  [PostgreSQL] │   │  [Sessions]   │   │  [Externes]   │
└───────────────┘   └───────────────┘   └───────────────┘

Légende:
════ = Trust Boundary (frontière de confiance)
```

---

## Phase 3 : Scan Sécurité Global

### 3.1 Évaluer chaque principe

**Pour chaque principe, scanner le code et évaluer :**

#### Principe I - Modélisation des menaces

```yaml
principe_I:
  statut: "[conforme|partiel|initial]"
  score: "[30-100]%"  # Minimum 30% car analyse système faite en Phase 2

  # Score = analyse_systeme (30%) + documentation_existante (20%) + features_analysees (50%)
  scoring:
    analyse_systeme: 30        # Toujours 30% après Phase 2
    documentation_existante: "[0-20]%"  # Docs menaces trouvées dans le projet
    features_analysees: 0      # Sera incrémenté par /osk-analyze

  verification:
    # Analyse système (Phase 2)
    stride_systeme: "FAIT"     # Toujours fait par baseline
    dfd_systeme: "FAIT"        # Généré en Phase 2
    trust_boundaries: "[X] identifiées"

    # Documentation existante (bonus)
    documentation_menaces: "[trouvé|absent]"
    fichiers_trouves: ["docs/security/threats/*"]

  constat: "Analyse système initiale réalisée. [X] features identifiées pour analyse détaillée."

  prochaines_etapes:
    - "/osk-analyze [feature] pour chaque feature critique"
```

#### Principe II - Analyse de risques

```yaml
principe_II:
  statut: "[conforme|partiel|initial]"
  score: "[30-100]%"  # Minimum 30% car risques système identifiés en Phase 2

  # Score = risques_systeme (30%) + registre_existant (20%) + features_analysees (50%)
  scoring:
    risques_systeme: 30        # Toujours 30% après Phase 2
    registre_existant: "[0-20]%"  # Registre trouvé dans le projet
    features_analysees: 0      # Sera incrémenté par /osk-analyze

  verification:
    # Risques système (Phase 2)
    risques_identifies: "[X] risques système"
    risques_critiques: "[X]"
    risques_importants: "[X]"

    # Documentation existante (bonus)
    registre_risques: "[trouvé|absent]"
    fichiers_trouves: ["docs/security/risks/*"]

  constat: "[X] risques système identifiés et scorés. Registre central créé."

  prochaines_etapes:
    - "/osk-analyze [feature] pour identifier les risques par feature"
```

#### Principe III - Sécurité dès la conception

```yaml
principe_III:
  statut: "[conforme|partiel|absent]"
  score: "[0-100]%"

  verification:
    authentification:
      present: true
      methode: "[JWT|Session|OAuth|Basic]"
      implementation: "[fichiers]"
      forces: ["[points positifs]"]
      faiblesses: ["[points à améliorer]"]

    autorisation:
      present: "[true|false]"
      modele: "[RBAC|ABAC|ACL|none]"
      implementation: "[fichiers]"
      couverture: "[X]% des endpoints"

    validation:
      present: "[true|false]"
      bibliotheque: "[Zod|Joi|Yup|class-validator|none]"
      couverture: "[X]/[Y] endpoints ([Z]%)"
      endpoints_non_valides: ["[liste]"]

    chiffrement:
      transit: "[TLS|none]"
      repos: "[AES|none]"
      implementation: "[fichiers]"

  constat: "[Description]"
  lacunes: []
```

#### Principe IV - Tests de sécurité

```yaml
principe_IV:
  statut: "[conforme|partiel|absent]"
  score: "[0-100]%"

  verification:
    sast:
      configure: "[true|false]"
      outil: "[Semgrep|SonarQube|CodeQL|none]"
      config_file: "[chemin ou absent]"

    dast:
      configure: "[true|false]"
      outil: "[ZAP|Burp|none]"

    sca:
      configure: "[true|false]"
      outil: "[Dependabot|Snyk|Renovate|none]"
      config_file: "[chemin]"

    tests_securite:
      presents: "[true|false]"
      count: "[X]"
      fichiers: ["[liste]"]

  constat: "[Description]"
  lacunes: []
```

#### Principe V - Gestion des secrets

```yaml
principe_V:
  statut: "[conforme|partiel|critique]"
  score: "[0-100]%"

  verification:
    secrets_detectes:
      - fichier: ".env"
        secrets: ["DB_PASSWORD", "API_KEY", "JWT_SECRET"]
        risque: "Fichier non chiffré"
      - fichier: "src/config.ts"
        secrets: ["hardcoded API key ligne 23"]
        risque: "Secret dans le code"

    gestionnaire_secrets:
      present: "[true|false]"
      outil: "[Vault|AWS SM|Azure KV|Doppler|none]"

    detection_precommit:
      present: "[true|false]"
      outil: "[gitleaks|trufflehog|none]"

    rotation:
      politique: "[définie|absente]"
      automatique: "[true|false]"

  alertes_critiques:
    - "[Secret exposé dans .env.example committé]"
    - "[API key en dur dans le code]"

  constat: "[Description]"
  lacunes: []
```

#### Principe VI - Traçabilité

```yaml
principe_VI:
  statut: "[conforme|partiel|absent]"
  score: "[0-100]%"

  verification:
    logging:
      present: "[true|false]"
      framework: "[Winston|Pino|Bunyan|console|none]"
      format: "[JSON structuré|texte|mixte]"

    evenements_logges:
      auth: "[oui|partiel|non]"
      erreurs: "[oui|partiel|non]"
      acces_donnees: "[oui|partiel|non]"
      actions_admin: "[oui|partiel|non]"

    centralisation:
      present: "[true|false]"
      outil: "[ELK|Loki|CloudWatch|Datadog|none]"

    alerting:
      present: "[true|false]"
      regles: "[X] règles définies"

  problemes:
    - "[Tokens en clair dans les logs]"
    - "[Pas de trace_id pour corrélation]"

  constat: "[Description]"
  lacunes: []
```

#### Principe VII - Patch management

```yaml
principe_VII:
  statut: "[conforme|partiel|attention]"
  score: "[0-100]%"

  verification:
    vulnerabilites:
      critical: "[X]"
      high: "[X]"
      medium: "[X]"
      low: "[X]"
      liste:
        - package: "[nom]"
          version: "[actuelle]"
          cve: "[CVE-XXX]"
          cvss: "[score]"
          fix: "[version corrigée]"

    dependances_outdated:
      count: "[X]"
      age_max: "[X mois]"

    auto_update:
      configure: "[true|false]"
      outil: "[Dependabot|Renovate|none]"

    politique_sla:
      definie: "[true|false]"
      documentee: "[chemin ou absent]"

  constat: "[Description]"
  lacunes: []
```

### 2.2 Détecter les vulnérabilités immédiates

**Scanner le code pour détecter :**

```yaml
vulnerabilites_immediates:
  critiques:
    - id: "VULN-001"
      type: "secret_exposed"
      titre: "Secret committé dans le repo"
      fichier: ".env.example:5"
      detail: "DB_PASSWORD visible"
      remediation: "Retirer et rotater le secret"

    - id: "VULN-002"
      type: "sql_injection"
      titre: "Injection SQL possible"
      fichier: "src/reports/query.ts:45"
      detail: "Concaténation de paramètre utilisateur"
      remediation: "Utiliser requête paramétrée"

  importantes:
    - id: "VULN-003"
      type: "idor"
      titre: "IDOR sur ressource utilisateur"
      fichier: "src/routes/users.ts:23"
      detail: "/api/users/:id sans vérification owner"
      remediation: "Ajouter middleware de vérification"

  mineures:
    - id: "VULN-004"
      type: "missing_rate_limit"
      titre: "Rate limiting absent"
      fichier: "src/routes/auth.ts"
      detail: "Endpoint login sans limite"
      remediation: "Ajouter rate limiter"
```

---

## Phase 3 : Calcul du Score et Roadmap

### 3.1 Calculer le score global

```yaml
scoring:
  par_principe:
    I_threat_modeling: 30     # Minimum 30% après analyse système (Phase 2)
    II_risk_analysis: 30      # Minimum 30% après risques système (Phase 2)
    III_security_design: 45   # 0-100
    IV_security_testing: 30   # 0-100
    V_secrets_management: 15  # 0-100
    VI_audit_logging: 40      # 0-100
    VII_patch_management: 60  # 0-100

  score_global: 36  # Moyenne pondérée (meilleur qu'avant grâce à I et II)

  niveau:
    score: 36
    label: "À risque"
    description: "Analyse système initiale faite, features à analyser"

  progression:
    baseline: 36           # Score après baseline
    cible_phase1: 50       # Après quick wins + fondations
    cible_phase2: 70       # Après features critiques
    cible_finale: 85       # Couverture complète

# Échelle :
# 0-25   : Critique - Action immédiate requise
# 26-50  : À risque - Plan de remédiation nécessaire
# 51-75  : Acceptable - Améliorations recommandées
# 76-100 : Mature - Maintenance continue

# Note: Les principes I et II commencent à 30% car :
# - Analyse STRIDE système réalisée (Phase 2)
# - Risques système identifiés et scorés
# - Registre central créé
# Les 70% restants viendront de /osk-analyze par feature
```

### 3.2 Générer la roadmap

```yaml
roadmap:
  phase_0_quick_wins:
    nom: "Quick Wins (URGENT)"
    effort: "2-4h"
    actions:
      - "Retirer secrets de .env.example"
      - "Fixer injection SQL reports/query.ts:45"
      - "Ajouter vérification IDOR users/:id"
    impact: "Réduit risques critiques immédiats"

  phase_1_fondations:
    nom: "Fondations Sécurité"
    effort: "1-2 jours"
    actions:
      - "Configurer gestionnaire de secrets"
      - "Ajouter pre-commit gitleaks"
      - "Configurer SAST basique (Semgrep)"
      - "Mettre en place logging structuré"
    impact: "Infrastructure sécurité en place"

  phase_2_features_critiques:
    nom: "Features Critiques"
    effort: "3-5 jours"
    features_a_analyser:
      - id: "auth"
        raison: "Point d'entrée, credentials"
        commande: "/osk-analyze auth"
      - id: "payments"
        raison: "Données financières"
        commande: "/osk-analyze payments"
      - id: "admin"
        raison: "Privilèges élevés"
        commande: "/osk-analyze admin"
    impact: "Features critiques sécurisées"

  phase_3_couverture:
    nom: "Couverture Complète"
    effort: "1-2 semaines"
    features_a_analyser:
      - id: "users"
        commande: "/osk-analyze users"
      - id: "api-public"
        commande: "/osk-analyze api-public"
      # ... autres features
    impact: "Couverture complète du projet"

  phase_4_excellence:
    nom: "Excellence Sécurité"
    effort: "Continu"
    actions:
      - "DAST en staging"
      - "Tests de pénétration"
      - "Bug bounty (optionnel)"
    impact: "Niveau mature atteint"
```

---

## Phase 4 : Présentation et Confirmation

### 4.1 Afficher l'état des lieux

**OBLIGATOIRE : Présenter les résultats et demander validation.**

```
============================================================
  BASELINE SÉCURITÉ - État des Lieux
============================================================

INVENTAIRE PROJET
─────────────────
Fichiers source : [X]
Lignes de code  : ~[X]
Architecture    : [Type détecté]

FEATURES IDENTIFIÉES
────────────────────
│ #  │ Feature     │ Criticité │ Fichiers │ Endpoints │
├────┼─────────────┼───────────┼──────────┼───────────┤
│ 1  │ auth        │ 🔴 CRIT   │ 12       │ 5         │
│ 2  │ payments    │ 🔴 CRIT   │ 15       │ 6         │
│ 3  │ admin       │ 🟠 HIGH   │ 10       │ 12        │
│ 4  │ users       │ 🟠 HIGH   │ 8        │ 8         │
│ 5  │ api-public  │ 🟠 HIGH   │ 20       │ 25        │
│ 6  │ reports     │ 🟡 MED    │ 8        │ 4         │
│ 7  │ notifications│ 🟡 MED   │ 5        │ 2         │
│ 8  │ files       │ 🟡 MED    │ 6        │ 3         │

→ [X] features identifiées, [Y] critiques, [Z] à risque

CONFORMITÉ AUX 7 PRINCIPES
──────────────────────────
│ Principe                    │ Score │ Statut          │ Note                    │
├─────────────────────────────┼───────┼─────────────────┼─────────────────────────┤
│ I.   Modélisation menaces   │  30%  │ 🟡 INITIAL      │ Analyse système faite   │
│ II.  Analyse risques        │  30%  │ 🟡 INITIAL      │ Risques système scorés  │
│ III. Sécurité conception    │  45%  │ ⚠️ PARTIEL      │                         │
│ IV.  Tests sécurité         │  30%  │ ⚠️ PARTIEL      │                         │
│ V.   Gestion secrets        │  15%  │ 🔴 CRITIQUE     │                         │
│ VI.  Traçabilité            │  40%  │ ⚠️ PARTIEL      │                         │
│ VII. Patch management       │  60%  │ ⚠️ ATTENTION    │                         │
├─────────────────────────────┼───────┼─────────────────┼─────────────────────────┤
│ SCORE GLOBAL                │  36%  │ 🟠 À RISQUE     │ +9% vs sans baseline    │

→ Les principes I et II sont à 30% (analyse système)
→ /osk-analyze [feature] augmentera ces scores (jusqu'à 100%)

VULNÉRABILITÉS IMMÉDIATES
─────────────────────────
🔴 VULN-001 : Secret committé (.env.example)
🔴 VULN-002 : SQL Injection (reports/query.ts:45)
🔴 VULN-003 : IDOR (users/:id sans vérif)
🟠 VULN-004 : Rate limiting absent (auth/login)
🟠 VULN-005 : Tokens dans les logs

ROADMAP PROPOSÉE
────────────────
Phase 0 - Quick Wins      : ~2h   → Fixer vulns critiques
Phase 1 - Fondations      : ~2j   → Infrastructure sécurité
Phase 2 - Features CRIT   : ~5j   → auth, payments, admin
Phase 3 - Couverture      : ~2sem → Toutes les features
Phase 4 - Excellence      : Continu

============================================================
```

### 4.2 Demander confirmation

```
VALIDATION
──────────

L'état des lieux ci-dessus est-il correct ?

1. ✅ Valider et générer les fichiers
2. 📝 Ajuster l'inventaire des features
3. 📝 Modifier la criticité d'une feature
4. ➕ Ajouter une feature manquante
5. ➖ Retirer une feature (faux positif)
6. 🔍 Voir le détail d'un principe
7. 🔍 Voir le détail d'une vulnérabilité
8. 🔄 Relancer le scan
9. ❌ Annuler

Votre choix ?
```

### 4.3 Gérer les ajustements

**Ajuster une feature :**
```
AJUSTEMENT FEATURE
──────────────────

Feature : users
  Criticité actuelle : HIGH
  Fichiers : src/users/*.ts (8 fichiers)
  Endpoints : 8

  Ajuster :
  • Nouveau nom ?
  • Criticité ? (critical / high / medium / low)
  • Ajouter/retirer des fichiers ?
  • Fusionner avec une autre feature ?
  • Supprimer (faux positif) ?
```

**Ajouter une feature :**
```
NOUVELLE FEATURE
────────────────

Identifiant : [ex: billing]
Nom : [ex: Facturation]
Description : [ex: Gestion des factures et abonnements]
Fichiers (glob) : [ex: src/billing/**/*.ts]
Criticité : [critical / high / medium / low]
```

---

## Phase 5 : Génération des Fichiers

### 5.1 Générer les fichiers

**Structure des fichiers générés :**
```
.osk/specs/000-baseline/
├── inventory.md        ← Inventaire des features
├── security-scan.md    ← Résultats du scan par principe
├── vulnerabilities.md  ← Vulnérabilités détectées
├── gaps.md             ← Lacunes par principe
├── roadmap.md          ← Plan de mise à niveau
└── features.yaml       ← Liste des features (machine-readable)

docs/security/risks/
└── risk-register.yaml  ← CRÉÉ ICI avec vulnérabilités initiales
```

### 5.2 Créer `docs/security/risks/risk-register.yaml`

**Le risk-register.yaml est CRÉÉ par /osk-baseline avec les vulnérabilités détectées.**

```yaml
# Registre des Risques - OpenSecKit
# Créé par /osk-baseline le [DATE]
# Mis à jour par /osk-analyze pour chaque feature

metadata:
  version: "3.0"
  created_by: "/osk-baseline"
  created_at: "[DATE]"
  last_updated: "[DATE]"
  projet: "[NOM PROJET]"

# Compteurs
stats:
  total: [X]
  critiques: [X]
  importants: [X]
  mineurs: [X]
  ouverts: [X]
  resolus: 0
  score_total: [XXX]

# Conformité par principe (baseline initial)
# Note: I et II à 30% minimum car analyse système faite
conformite:
  I_threat_modeling:
    score: 30
    statut: "INITIAL"
    detail: "Analyse STRIDE système réalisée"
    features_analysees: []
  II_risk_analysis:
    score: 30
    statut: "INITIAL"
    detail: "Risques système identifiés et scorés"
    features_analysees: []
  III_security_design:
    score: 45
    statut: "PARTIEL"
  IV_security_testing:
    score: 30
    statut: "PARTIEL"
  V_secrets_management:
    score: 15
    statut: "CRITIQUE"
  VI_audit_logging:
    score: 40
    statut: "PARTIEL"
  VII_patch_management:
    score: 60
    statut: "ATTENTION"

# Liste des risques
# Source: /osk-baseline (vulnérabilités + risques système)
# Enrichi par: /osk-analyze <feature> (risques par feature)
risques:
  # ================================================
  # RISQUES SYSTÈME (identifiés en Phase 2)
  # ================================================
  - id: "RISK-SYS-001"
    source: "/osk-baseline"
    type: "systeme"
    titre: "[Risque système identifié]"
    description: "[Description du risque système]"
    categorie_stride: "[S/T/R/I/D/E]"
    severite: "[CRITIQUE/IMPORTANT/MINEUR]"
    # ... format complet comme /osk-analyze

  # ================================================
  # VULNÉRABILITÉS IMMÉDIATES (détectées en Phase 3)
  # ================================================
  - id: "VULN-BASELINE-001"
    source: "/osk-baseline"
    titre: "Secret committé dans le repository"
    description: "Le fichier .env.example contient des secrets réels"

    categorie: "secret_exposed"
    severite: "CRITIQUE"

    impact: 5
    probabilite: 5
    exposition: 5
    score: 125

    fichiers:
      - ".env.example:5"

    statut: "OUVERT"
    date_detection: "[DATE]"
    sla: "48h"
    date_echeance: "[DATE+48h]"

    remediation:
      action: "Retirer les secrets et rotater les credentials"
      effort: "XS"
      commandes:
        - "git rm .env.example"
        - "Rotater DB_PASSWORD dans le gestionnaire de secrets"

    principe_viole: "V"
    cwe: "CWE-798"
    owasp: "A07:2021"

  - id: "VULN-BASELINE-002"
    source: "/osk-baseline"
    titre: "Injection SQL possible"
    # ... même format

  # Les risques par feature seront ajoutés par /osk-analyze
  # - id: "RISK-AUTH-001"
  #   source: "/osk-analyze auth"
  #   ...
```

### 5.3 Format `inventory.md`

```markdown
# Inventaire du Projet

> Généré par `/osk-baseline` le [DATE]
> Validé par l'utilisateur

## Vue d'ensemble

| Métrique | Valeur |
|----------|--------|
| Fichiers source | [X] |
| Lignes de code | ~[X] |
| Architecture | [Type] |
| Features identifiées | [X] |

## Features

### 1. auth (Authentification) 🔴 CRITICAL

| Attribut | Valeur |
|----------|--------|
| Description | Gestion login, logout, tokens, sessions |
| Criticité | CRITICAL |
| Raison | Point d'entrée, gestion credentials |
| Données sensibles | Oui |

**Fichiers :**
- `src/auth/login.ts`
- `src/auth/logout.ts`
- `src/middleware/auth.ts`
- ...

**Endpoints :**
- `POST /api/auth/login`
- `POST /api/auth/logout`
- `POST /api/auth/refresh`
- `GET /api/auth/me`

**Modèles :**
- `User`
- `Session`
- `RefreshToken`

**Prochaine étape :** `/osk-analyze auth`

---

### 2. payments (Paiements) 🔴 CRITICAL

[Même format...]

---

[Répéter pour chaque feature]
```

### 5.4 Format `features.yaml`

```yaml
# .osk/specs/000-baseline/features.yaml
# Liste des features pour les commandes /osk-analyze

metadata:
  generated: "[DATE]"
  validated: true
  total_features: [X]

features:
  - id: "auth"
    name: "Authentification"
    description: "Gestion login, logout, tokens, sessions"
    criticite: "critical"
    status: "pending"  # pending | analyzed | secured
    files:
      - "src/auth/**/*.ts"
      - "src/middleware/auth.ts"
    endpoints:
      - method: "POST"
        path: "/api/auth/login"
      - method: "POST"
        path: "/api/auth/logout"
    models:
      - "User"
      - "Session"
    sensitive_data: true
    next_command: "/osk-analyze auth"

  - id: "payments"
    name: "Paiements"
    # ...

  - id: "admin"
    # ...

# Ordre recommandé d'analyse
analysis_order:
  - "auth"      # Critical - Point d'entrée
  - "payments"  # Critical - Données financières
  - "admin"     # High - Privilèges élevés
  - "users"     # High - Données personnelles
  - "api-public"
  - "reports"
  - "notifications"
  - "files"
```

### 5.5 Format `roadmap.md`

```markdown
# Roadmap de Mise à Niveau Sécurité

> Générée par `/osk-baseline` le [DATE]
> Score initial : [X]% (À risque)

## Objectif

Atteindre un score de **75%+** (Acceptable) en [X semaines].

## Phase 0 : Quick Wins (URGENT)

**Effort estimé :** 2-4 heures
**Impact :** Éliminer les risques critiques immédiats

### Actions

- [ ] **VULN-001** : Retirer secrets de `.env.example`
  - Fichier : `.env.example`
  - Action : Supprimer les vraies valeurs, mettre des placeholders
  - Commande : `git rm .env.example && git commit`

- [ ] **VULN-002** : Fixer injection SQL
  - Fichier : `src/reports/query.ts:45`
  - Action : Utiliser requête paramétrée
  - Avant : `db.query("SELECT * FROM reports WHERE id = " + id)`
  - Après : `db.query("SELECT * FROM reports WHERE id = $1", [id])`

- [ ] **VULN-003** : Fixer IDOR
  - Fichier : `src/routes/users.ts:23`
  - Action : Ajouter vérification owner

### Validation
```bash
# Après corrections, vérifier :
npm audit
git log --oneline -5  # Vérifier les commits
```

---

## Phase 1 : Fondations Sécurité

**Effort estimé :** 1-2 jours
**Impact :** Infrastructure sécurité en place

### Actions

- [ ] Configurer gestionnaire de secrets
- [ ] Ajouter pre-commit hook gitleaks
- [ ] Configurer SAST (Semgrep)
- [ ] Mettre en place logging structuré

[Détails pour chaque action...]

---

## Phase 2 : Features Critiques

**Effort estimé :** 3-5 jours
**Impact :** Features critiques sécurisées

### Features à analyser

| Ordre | Feature | Commande | Raison |
|-------|---------|----------|--------|
| 1 | auth | `/osk-analyze auth` | Point d'entrée, credentials |
| 2 | payments | `/osk-analyze payments` | Données financières |
| 3 | admin | `/osk-analyze admin` | Privilèges élevés |

---

## Phase 3 : Couverture Complète

**Effort estimé :** 1-2 semaines

[Liste des features restantes...]

---

## Suivi

### Métriques à suivre

| Métrique | Baseline | Cible Phase 1 | Cible Phase 2 | Cible Final |
|----------|----------|---------------|---------------|-------------|
| Score global | 27% | 40% | 60% | 75%+ |
| Vulns critiques | 3 | 0 | 0 | 0 |
| Features analysées | 0/8 | 0/8 | 3/8 | 8/8 |
| Principes conformes | 0/7 | 2/7 | 4/7 | 6/7 |
```

---

## Phase 6 : Rapport Final

```
============================================================
  /osk-baseline - Baseline Terminé
============================================================

ÉTAT DES LIEUX VALIDÉ
─────────────────────
Score global    : 27% (À risque)
Features        : 8 identifiées
Vulnérabilités  : 3 critiques, 2 importantes

FEATURES PRÊTES POUR ANALYSE
────────────────────────────
Par ordre de priorité :

1. /osk-analyze auth        ← Commencer ici
2. /osk-analyze payments
3. /osk-analyze admin
4. /osk-analyze users
5. /osk-analyze api-public
6. /osk-analyze reports
7. /osk-analyze notifications
8. /osk-analyze files

FICHIERS GÉNÉRÉS
────────────────
✅ .osk/specs/000-baseline/inventory.md
✅ .osk/specs/000-baseline/security-scan.md
✅ .osk/specs/000-baseline/vulnerabilities.md
✅ .osk/specs/000-baseline/gaps.md
✅ .osk/specs/000-baseline/roadmap.md
✅ .osk/specs/000-baseline/features.yaml
✅ docs/security/risks/risk-register.yaml  ← Registre initial

PROCHAINES ÉTAPES
─────────────────
1. Traiter les Quick Wins (Phase 0) - 2h
2. Lancer /osk-analyze auth
3. Suivre la roadmap générée

============================================================
```

---

# Règles Importantes

1. **Exhaustivité** : Scanner tout le projet, ne rien manquer
2. **Priorisation** : Classifier par criticité pour guider l'utilisateur
3. **Confirmation** : Toujours valider l'inventaire des features
4. **Actionnable** : Chaque finding doit avoir une action claire
5. **Traçabilité** : Documenter pourquoi chaque feature est classée ainsi
6. **Machine-readable** : Générer `features.yaml` pour les prochaines commandes
7. **Roadmap réaliste** : Proposer un plan progressif, pas tout d'un coup

---

## Templates de Référence

**OBLIGATOIRE : Consulter ces templates pour comprendre ce que chaque principe exige et évaluer la conformité.**

### Principe I - Modélisation des Menaces

Lire `.osk/templates/01-threat-modeling/` pour comprendre ce qui devrait exister :

| Template | Ce qu'on vérifie |
|----------|------------------|
| `stride-threat-library-common.md` | Le projet a-t-il une documentation des menaces ? |
| `stride-threat-model-template-planning.md` | Existe-t-il des analyses STRIDE ? |

### Principe II - Analyse de Risques

Lire `.osk/templates/02-risk-analysis/` :

| Template | Ce qu'on vérifie |
|----------|------------------|
| `risk-register-template-all.md` | Existe-t-il un registre des risques ? |
| `risk-scoring-template-planning.md` | Les risques sont-ils scorés ? |

### Principe III - Sécurité dès la Conception

Lire `.osk/templates/03-security-requirements/` :

| Template | Ce qu'on vérifie |
|----------|------------------|
| `owasp-asvs-checklist-design.md` | Référentiel pour évaluer les contrôles existants |
| `authentication-requirements-template-design.md` | L'authentification est-elle conforme ? |
| `authorization-requirements-template-design.md` | L'autorisation est-elle implémentée ? |
| `encryption-requirements-template-design.md` | Le chiffrement est-il correct ? |

### Principe IV - Tests de Sécurité

Lire `.osk/templates/04-security-testing/` :

| Template | Ce qu'on vérifie |
|----------|------------------|
| `sast-integration-guide-implementation.md` | SAST configuré dans CI/CD ? |
| `dast-integration-guide-implementation.md` | DAST configuré ? |
| `sca-dependency-scanning.md` | SCA/Dependabot actif ? |

### Principe V - Gestion des Secrets

Lire `.osk/templates/05-secrets-management/` :

| Template | Ce qu'on vérifie |
|----------|------------------|
| `vault-integration-guide.md` | Gestionnaire de secrets utilisé ? |
| `secrets-detection-setup.md` | Pre-commit hook configuré ? |
| `secrets-rotation-policy-template.md` | Politique de rotation définie ? |

### Principe VI - Traçabilité et Audit

Lire `.osk/templates/06-audit-logging/` :

| Template | Ce qu'on vérifie |
|----------|------------------|
| `logging-requirements-template-design.md` | Événements requis sont-ils loggés ? |
| `log-centralization-requirements.md` | Logs centralisés ? |
| `security-alert-rules-template.md` | Alerting configuré ? |

### Principe VII - Patch Management

Lire `.osk/templates/07-patch-management/` :

| Template | Ce qu'on vérifie |
|----------|------------------|
| `dependency-scanning-guide-all.md` | Dépendances scannées ? |
| `patch-sla-policy-template.md` | SLA de correction défini ? |
| `emergency-patching-procedure.md` | Procédure d'urgence documentée ? |

### Utilisation

1. **Avant le scan** : Parcourir les templates pour connaître les critères d'évaluation
2. **Pendant le scan** : Comparer l'existant aux exigences des templates
3. **Pour le scoring** : Attribuer un % de conformité basé sur les critères des templates
4. **Pour la roadmap** : Utiliser les templates comme référence pour les actions à mener
