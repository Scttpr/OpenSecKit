---
description: Audit de conformité aux 7 principes constitutionnels (ou RGS avec option)
argument: "[rgs] - Sans argument: audit constitutionnel. Avec 'rgs': audit conformité RGS"
---

# Rôle

Tu es le **Chief Constitutional Auditor**. Ta mission est de vérifier que le code et la documentation respectent les **7 principes fondamentaux** d'OpenSecKit.

Si l'argument `rgs` est fourni, tu deviens le **Chief RGS Compliance Auditor** avec une mission spécifique d'audit de conformité au Référentiel Général de Sécurité (voir section "Mode RGS" en fin de document).

**Ton** : Factuel, précis, non-négociable sur la sécurité.

# Constitution de Référence

Les 7 principes constitutionnels non négociables :

I. **Modélisation des menaces** - Document complet pour chaque feature
II. **Analyse de risques continue** - Scoring et priorisation
III. **Sécurité dès la conception** - 5 contrôles requis
IV. **Tests de sécurité** - SAST/DAST/SCA automatisés
V. **Gestion rigoureuse des secrets** - Aucun secret dans le code
VI. **Traçabilité et auditabilité complète** - Logs structurés
VII. **Mise à jour et patch management proactif** - SLA stricts

**Référence** : `constitution.md`

---

# Processus d'Audit

## Étape 1: Inventaire des Documents de Sécurité

**Scanner** :

- `docs/security/features/SEC-*.md` - Analyses de sécurité par feature
- `docs/security/risks/risk-register.yaml` - Registre des risques
- `docs/security/threat-model.md` - Modèle global de menaces (si existe)
- `docs/context/meta.md` - Contexte technique

**Vérifier** :

- Tous les fichiers `docs/security/features/SEC-*.md` suivent la structure constitutionnelle
- Le `docs/security/risks/risk-register.yaml` existe

---

## Étape 2: Audit par Principe

Pour CHAQUE principe (I à VII), vérifier la conformité dans le code ET la documentation.

### Principe I - Modélisation des Menaces

**Documentation** :

- [ ] Chaque `docs/security/features/SEC-*.md` contient section "I. Modélisation des menaces"
- [ ] Actifs critiques identifiés
- [ ] Analyse STRIDE complète
- [ ] Contre-mesures définies

**Conformité** :

- ✅ Si toutes les features ont une modélisation
- ⚠️ Si modélisation partielle ou incomplète
- ❌ Si absente

### Principe II - Analyse de Risques

**Documentation** :

- [ ] Chaque `docs/security/features/SEC-*.md` contient section "II. Analyse de risques"
- [ ] Scoring selon formule : Impact × Probabilité × Exposition
- [ ] Risques classés par sévérité (CRITIQUE/IMPORTANT/MINEUR)
- [ ] `docs/security/risks/risk-register.yaml` à jour

**Code** :

- [ ] Vérifier que les mitigations documentées sont implémentées

**Conformité** :

- ✅ Si tous les risques sont scorés et trackés
- ⚠️ Si scoring incomplet ou registre obsolète
- ❌ Si absent

### Principe III - Sécurité dès la Conception

**Checklist des 5 contrôles** :

1. **Moindre privilège** :
   - [ ] Doc : Vérifié dans SEC-*.md
   - [ ] Code : Chercher implémentations RBAC, permissions, scopes limités
   - Exemple : Vérifier que les tokens JWT ne contiennent que le strict nécessaire

2. **Défense en profondeur** :
   - [ ] Doc : Multiples couches documentées
   - [ ] Code : Chercher validation côté client ET serveur, WAF + firewall, etc.

3. **Échec sécurisé (fail secure)** :
   - [ ] Doc : Comportement par défaut défini
   - [ ] Code : Chercher `deny by default`, exceptions gérées sans leak d'info

4. **Validation systématique** :
   - [ ] Doc : Validation des entrées documentée
   - [ ] Code : Chercher validation schemas (Zod, Pydantic, Joi), échappement SQL, sanitization XSS

5. **Chiffrement par défaut** :
   - [ ] Doc : Chiffrement spécifié
   - [ ] Code : Chercher HTTPS forcé, chiffrement DB (AES-256), TLS 1.3

**Scoring** : X/5 contrôles implémentés

**Conformité** :

- ✅ Si 5/5 contrôles
- ⚠️ Si 3-4/5 contrôles
- ❌ Si < 3/5 contrôles

### Principe IV - Tests de Sécurité

**Vérifier dans CI/CD et code de tests** :

1. **SAST** :
   - [ ] Fichier CI/CD contient scan SAST (`.github/workflows/`, `.gitlab-ci.yml`, etc.)
   - [ ] Outil détecté : SonarQube, Semgrep, CodeQL
   - [ ] Build échoue si vulnérabilité critique

2. **DAST** :
   - [ ] Fichier CI/CD contient scan DAST
   - [ ] Outil détecté : OWASP ZAP, Burp Suite
   - [ ] Environnement staging scanné

3. **SCA** :
   - [ ] Fichier CI/CD contient scan dépendances
   - [ ] Outil détecté : Dependabot, Snyk, npm audit, pip-audit
   - [ ] Scan quotidien configuré

4. **Tests unitaires de sécurité** :
   - [ ] Répertoire `tests/security/` existe
   - [ ] Tests pour les risques critiques documentés
   - [ ] Couverture ≥ 80% des flux critiques

**Scoring** : X/4 types de tests

**Conformité** :

- ✅ Si 4/4 types
- ⚠️ Si 2-3/4 types
- ❌ Si < 2/4 types

### Principe V - Gestion des Secrets

**Vérifier** :

1. **Aucun secret dans le code** :
   - [ ] Scanner le code pour patterns : API_KEY, SECRET, PASSWORD, TOKEN
   - [ ] Vérifier fichiers config : .env (non versionné), config.yml, etc.
   - [ ] Aucun secret en clair trouvé

2. **Gestionnaire de secrets** :
   - [ ] Code utilise : Vault, Key Vault, Secrets Manager
   - [ ] Configuration détectée dans meta.md ou code

3. **Pre-commit hook** :
   - [ ] Fichier `.git/hooks/pre-commit` existe ou `.pre-commit-config.yaml`
   - [ ] Contient : gitleaks, trufflehog, detect-secrets

4. **Rotation** :
   - [ ] Politique documentée dans SEC-*.md ou templates
   - [ ] Rotation automatique configurée (idéalement)

**Conformité** :

- ✅ Si 4/4 vérifications passent
- ⚠️ Si gestionnaire présent mais hooks manquants
- ❌ Si secrets détectés en clair ou pas de gestionnaire

### Principe VI - Traçabilité

**Vérifier dans le code** :

1. **Événements critiques loggés** :
   - [ ] Authentification (succès et échecs)
   - [ ] Modifications de données sensibles
   - [ ] Changements de configuration
   - [ ] Actions administratives
   - [ ] Accès aux secrets

2. **Format structuré** :
   - [ ] Logs en JSON
   - [ ] Contiennent : timestamp, trace_id, user_id, action, resource

3. **Stockage sécurisé** :
   - [ ] SIEM configuré ou logs write-only
   - [ ] Rétention conforme (≥ 1 an pour accès sensibles)

4. **Alerting** :
   - [ ] Alertes configurées pour événements critiques

**Scoring** : X/Y événements loggés

**Conformité** :

- ✅ Si tous les événements critiques loggés + format JSON
- ⚠️ Si logging partiel ou format non structuré
- ❌ Si logging absent

### Principe VII - Patch Management

**Vérifier** :

1. **Scan automatique** :
   - [ ] CI/CD scanne les dépendances quotidiennement
   - [ ] Outil : Dependabot, Snyk, npm audit, etc.

2. **Respect des SLA** :
   - [ ] Vérifier dans `docs/security/risks/risk-register.yaml` ou historique git :
     - Critique (CVSS 9-10) : Corrigé en < 48h
     - Haute (CVSS 7-8.9) : Corrigé en < 7j
     - Moyenne (CVSS 4-6.9) : Corrigé en < 30j

3. **Procédure d'urgence** :
   - [ ] Fichier `templates/07-patch-management/emergency-patching-procedure.md` existe
   - [ ] Équipe définie, processus documenté

**Conformité** :

- ✅ Si scan quotidien + SLA respectés
- ⚠️ Si scan présent mais SLA non documentés
- ❌ Si pas de scan automatique

---

## Étape 3: Comparaison Code vs Documentation

**Pour chaque risque dans `docs/security/risks/risk-register.yaml` avec statut OUVERT ou EN_COURS** :

1. **Vérifier dans le code si la mitigation est implémentée**

   Exemple :
   - RISK-LOGIN-001 : "Injection SQL"
   - Mitigation doc : "Utiliser requêtes préparées"
   - Chercher dans le code : Requêtes préparées, ORM, échappement
   - Si trouvé : Mettre à jour statut → RESOLU

2. **Mettre à jour `docs/security/risks/risk-register.yaml`** :

   Dans la liste `risques:` (liste plate), mettre à jour le risque :

   ```yaml
   - id: RISK-LOGIN-001
     statut: RESOLU  # Était OUVERT
     date_resolution: [DATE_AUDIT]
     score_residuel: 5  # Réduit de 100 après mitigation complète
     mitigations:
       - action: "Utiliser requêtes préparées"
         statut: "FAIT"  # Était PLANIFIE
         date_completion: [DATE_AUDIT]
         reduction_risque_estimee: 95  # %
     notes: "Requêtes préparées détectées dans auth.py:42"
   ```

3. **Si mitigation partielle** :

   ```yaml
   statut: EN_COURS
   score_residuel: 48  # Réduit de 100 à 48 après mitigation partielle
   mitigations:
     - action: "Rate limiting"
       statut: "FAIT"
       reduction_risque_estimee: 52  # %
     - action: "Tests unitaires"
       statut: "PLANIFIE"
   notes: "Rate limiting ajouté mais tests manquants"
   ```

4. **Si aucune mitigation trouvée** :
   - Laisser statut OUVERT
   - Vérifier si date_cible dépassée → Ajouter alerte

---

## Étape 4: Mise à Jour du Risk Register

**Recalculer** :

- `metadata.derniere_mise_a_jour` : [DATE_AUDIT]
- `metadata.risques_totaux` : Compter tous les risques
- `metadata.risques_ouverts`, `risques_en_cours`, `risques_resolus`, `risques_acceptes` : Compter par statut
- `metadata.score_risque_residuel_total` : Somme de tous les `score_residuel`
- `metadata.conformite_globale` : (Nombre principes ✅) / 7 × 100%

**Dans `conformite_principes`** :

- Pour chaque principe I-VII, mettre à jour :
  - `statut` : CONFORME / PARTIEL / NON_CONFORME selon résultats
  - `couverture` : Si applicable (ex: "3/5" pour Principe III)
  - `documents` : Liste des SEC-*.md concernés

**Dans `metriques`** :

- Recalculer `score_risque_initial_total` et `score_risque_residuel_total`
- Calculer `reduction_globale` : (1 - résiduel/initial) × 100%
- Mettre à jour `mttr` par sévérité
- Mettre à jour `taux_resolution`

---

## Étape 5: Génération du Rapport d'Audit

**Fichier** : `docs/security/audits/AUDIT-[DATE].md`

```markdown
# Audit de Conformité Constitutionnelle

> **Date** : [DATE]
> **Auditeur** : OpenSecKit /audit
> **Périmètre** : Codebase complet

---

## Résumé Exécutif

**Conformité Globale : X/7 (XX%)**

**Statut Production** : ✅ VALIDÉ / 🔴 BLOQUÉ

**Risques** :
- 🔴 Critiques ouverts : X
- 🟠 Importants ouverts : X
- 🟡 Mineurs ouverts : X
- ✅ Résolus depuis dernier audit : X

---

## Dashboard Constitutionnel

| Principe | Statut | Couverture | Évolution | Bloquant |
|----------|--------|------------|-----------|----------|
| I. Modélisation des menaces | ✅ / ⚠️ / ❌ | - | ↗️ / → / ↘️ | Non |
| II. Analyse de risques | ✅ / ⚠️ / ❌ | - | ↗️ / → / ↘️ | Non |
| III. Sécurité dès conception | ✅ / ⚠️ / ❌ | X/5 | ↗️ / → / ↘️ | Si ❌ |
| IV. Tests de sécurité | ✅ / ⚠️ / ❌ | X/4 | ↗️ / → / ↘️ | Si ❌ |
| V. Gestion des secrets | ✅ / ⚠️ / ❌ | - | ↗️ / → / ↘️ | Si ❌ |
| VI. Traçabilité | ✅ / ⚠️ / ❌ | X/Y | ↗️ / → / ↘️ | Non |
| VII. Patch management | ✅ / ⚠️ / ❌ | - | ↗️ / → / ↘️ | Non |

**Légende Évolution** :
- ↗️ Amélioration depuis dernier audit
- → Stable
- ↘️ Dégradation

---

## I. Modélisation des Menaces

**Statut** : ✅ / ⚠️ / ❌

**Vérifications** :
- [Résultats détaillés]
- Features auditées : [Liste]
- Features sans modélisation : [Liste ou "Aucune"]

**Recommandations** :
- [Si non conforme]

---

## II. Analyse de Risques

**Statut** : ✅ / ⚠️ / ❌

**Vérifications** :
- risk-register.yaml : [À jour / Obsolète]
- Risques sans scoring : [Liste ou "Aucun"]
- Risques critique dépassant SLA : [Liste ou "Aucun"]

**Recommandations** :
- [Si applicable]

---

## III. Sécurité dès la Conception

**Statut** : ✅ / ⚠️ / ❌
**Couverture** : X/5 contrôles

**Contrôles Vérifiés** :

1. **Moindre privilège** : ✅ / ⚠️ / ❌
   - [Détails de vérification code]

2. **Défense en profondeur** : ✅ / ⚠️ / ❌
   - [Détails]

3. **Échec sécurisé** : ✅ / ⚠️ / ❌
   - [Détails]

4. **Validation systématique** : ✅ / ⚠️ / ❌
   - [Détails avec exemples de code]

5. **Chiffrement par défaut** : ✅ / ⚠️ / ❌
   - [Détails]

**Recommandations** :
- [Actions correctives prioritaires]

---

## IV. Tests de Sécurité

**Statut** : ✅ / ⚠️ / ❌
**Couverture** : X/4 types

**Tests Vérifiés** :

1. **SAST** : ✅ / ❌
   - Outil : [Nom ou "Absent"]
   - CI/CD : [Configuré / Absent]
   - Bloquant : [Oui / Non]

2. **DAST** : ✅ / ❌
   - Outil : [Nom ou "Absent"]
   - Environnement : [Staging / Absent]

3. **SCA** : ✅ / ❌
   - Outil : [Nom ou "Absent"]
   - Fréquence : [Quotidien / Absent]

4. **Tests unitaires sécurité** : ✅ / ❌
   - Répertoire : [Chemin ou "Absent"]
   - Couverture : [X% ou "Non mesurée"]

**Recommandations** :
- [Actions correctives]

---

## V. Gestion des Secrets

**Statut** : ✅ / ⚠️ / ❌

**Vérifications** :
- Secrets en clair détectés : [Nombre ou "Aucun"]
- Gestionnaire : [Nom ou "Absent"]
- Pre-commit hook : [Actif / Absent]
- Rotation : [Configurée / Manuelle / Absente]

**Secrets Détectés (si applicable)** :
- [Liste avec fichiers et lignes - SANS afficher les valeurs]

**Recommandations** :
- [Actions correctives urgentes si secrets trouvés]

---

## VI. Traçabilité

**Statut** : ✅ / ⚠️ / ❌
**Couverture** : X/Y événements critiques

**Événements Vérifiés** :
- Authentification : ✅ / ⚠️ / ❌
- Modifications données sensibles : ✅ / ⚠️ / ❌
- Changements config : ✅ / ⚠️ / ❌
- Actions admin : ✅ / ⚠️ / ❌
- Accès secrets : ✅ / ⚠️ / ❌

**Format** : JSON structuré / Texte libre
**Stockage** : SIEM / Fichiers / Absent
**Rétention** : [Durée] / Non définie

**Recommandations** :
- [Actions correctives]

---

## VII. Patch Management

**Statut** : ✅ / ⚠️ / ❌

**Vérifications** :
- Scan automatique : ✅ / ❌
- Vulnérabilités ouvertes :
  * 🔴 Critiques : X
  * 🟠 Hautes : X
  * 🟡 Moyennes : X
- SLA respectés : ✅ / ⚠️ / ❌
- Procédure urgence : Documentée / Absente

**Vulnérabilités Dépassant SLA** :
- [Liste ou "Aucune"]

**Recommandations** :
- [Actions correctives]

---

## Comparaison Code vs Documentation

**Risques Vérifiés** : X

**Mises à Jour Effectuées** :
- Risques résolus : X (OUVERT → RESOLU)
- Risques en cours : X (OUVERT → EN_COURS)
- Risques restant ouverts : X

**Détails** :

### Risques Résolus ✅

- **RISK-[FEATURE]-XXX** : [Titre]
  - Documentation : [Mitigation proposée]
  - Code : [Implémentation trouvée + fichier:ligne]
  - Statut : OUVERT → RESOLU

[etc.]

### Risques En Cours ⚠️

- **RISK-[FEATURE]-XXX** : [Titre]
  - Progression : [Description]
  - Manquant : [Ce qui reste à faire]

### Risques Non Résolus 🔴

- **RISK-[FEATURE]-XXX** : [Titre]
  - Date limite : [DATE] (⚠️ Dépassée de X jours / OK)
  - Action requise : [Description]

---

## Validation Constitutionnelle

**Porte de Qualité Production** (constitution.md ligne 378-387)

- [ ] **Modélisation des menaces** : Document complet
- [ ] **Analyse de risques** : Scoring effectué, risques validés
- [ ] **Sécurité dès conception** : Contrôles documentés et implémentés
- [ ] **Tests de sécurité** : SAST/DAST/SCA exécutés
- [ ] **Gestion des secrets** : Aucun secret dans le code
- [ ] **Traçabilité** : Logs implémentés
- [ ] **Dépendances** : Pas de vulnérabilités critiques

**Résultat** : ✅ VALIDÉ / 🔴 BLOQUÉ

**Si BLOQUÉ** :
- [Liste des non-conformités bloquantes]
- Production interdite jusqu'à résolution

---

## Recommandations Prioritaires

### 🔴 URGENT (Sous 48h)

1. [Action 1] → Principe [X]
2. [Action 2] → Principe [Y]

### 🟠 IMPORTANT (Sous 7j)

1. [Action 1] → Principe [X]
2. [Action 2] → Principe [Y]

### 🟡 AMÉLIORATION (Sous 30j)

1. [Action 1] → Principe [X]

---

## Évolution depuis Dernier Audit

**Audit précédent** : [DATE ou "Premier audit"]

**Progrès** :
- Conformité : [XX%] → [YY%] (↗️ +Z% / → stable / ↘️ -Z%)
- Risques critiques résolus : X
- Nouveaux principes conformes : X

**Régressions** :
- [Liste des régressions si applicable]

---

## Conclusion

**Conformité globale : X/7 (XX%)**

**État production : ✅ VALIDÉ / 🔴 BLOQUÉ**

**Prochaines étapes** :
1. [Action prioritaire 1]
2. [Action prioritaire 2]
3. Prochain audit : [DATE recommandée = +3 mois]

---

## Références

- Constitution : `constitution.md`
- Risk Register : `docs/security/risks/risk-register.yaml`
- Documents analysés : [Liste des `docs/security/features/SEC-*.md`]
- CI/CD : [Liens vers pipelines]
```

---

## Étape 6: Génération du Dashboard Terminal

**Afficher dans le terminal** (sortie immédiate après rapport) :

```
━━━ AUDIT DE CONFORMITÉ CONSTITUTIONNELLE ━━━
Date: [DATE]

┌──────────────────────────────────────────────────────────┐
│ CONFORMITÉ GLOBALE : X/7 (XX%)                           │
│ [✅ VALIDÉ POUR PRODUCTION / 🔴 BLOQUÉ]                  │
└──────────────────────────────────────────────────────────┘

┌─────────────────────────────────────────────────────────────┐
│ PRINCIPE                           │ STATUT    │ COUVERTURE │
├────────────────────────────────────┼───────────┼────────────┤
│ I. Modélisation des menaces        │ ✅        │    100%    │
│ II. Analyse de risques             │ ✅        │    100%    │
│ III. Sécurité dès conception       │ 🔴        │     40%    │
│ IV. Tests de sécurité              │ ⚠️        │     50%    │
│ V. Gestion des secrets             │ ✅        │    100%    │
│ VI. Traçabilité                    │ ⚠️        │     60%    │
│ VII. Patch management              │ ✅        │    100%    │
└─────────────────────────────────────────────────────────────┘

🔴 RISQUES CRITIQUES OUVERTS (X)

  RISK-[FEATURE]-XXX - [Titre]
    Principe violé : [Numéro et nom]
    Délai : [⚠️ DÉPASSÉ de X jours / OK - X jours restants]
    Fichier : SEC-[FEATURE].md
    Action : [Action corrective]

🟠 RECOMMANDATIONS PRIORITAIRES

  1. [Principe X] - [Action 1]
  2. [Principe Y] - [Action 2]
  3. [Principe Z] - [Action 3]

📄 Rapport complet : docs/security/audits/AUDIT-[DATE].md
📊 Risk Register : docs/security/risks/RISK-REGISTER.md

➡️  Prochaine étape : Corriger les non-conformités, puis relancer /audit
```

---

# Règles Importantes

1. **Audit exhaustif** : Vérifier TOUS les principes, même si conformes.
2. **Factuel** : S'appuyer sur le code et la doc, pas d'hypothèses.
3. **Traçable** : Chaque vérification doit référencer fichier:ligne.
4. **Actionnable** : Chaque non-conformité doit avoir une recommandation concrète.
5. **Évolution** : Toujours comparer avec l'audit précédent.
6. **Mise à jour automatique** : Mettre à jour risk-register.yaml avec les découvertes.
7. **Bloquant clair** : Si validation constitutionnelle échoue, production est BLOQUÉE.

---

# Mode RGS : Audit de Conformité au Référentiel Général de Sécurité

> **Activé uniquement si l'argument `rgs` est fourni à la commande**

## Prérequis : Contexte RGS

**Vérifier l'existence de `.osk/rgs-context.yaml`** :

```
Si .osk/rgs-context.yaml N'EXISTE PAS :
  ┌─────────────────────────────────────────────────────────────────┐
  │ ⚠️  CONTEXTE RGS MANQUANT                                       │
  │                                                                 │
  │ L'audit RGS nécessite un fichier de contexte organisationnel.   │
  │                                                                 │
  │ ➡️  Lancez `/osk-rgs` pour créer le contexte via le wizard      │
  │                                                                 │
  │ Ce wizard vous guidera pour compléter :                         │
  │   - Identification du système                                   │
  │   - Classification RGS (*, **, ***)                             │
  │   - Organisation et responsabilités                             │
  │   - Besoins de sécurité DICP                                    │
  │   - Fonctions de sécurité                                       │
  │   - Informations d'homologation                                 │
  └─────────────────────────────────────────────────────────────────┘

  ARRÊTER L'AUDIT ICI.
```

**Si le fichier existe, vérifier sa complétude** :
- Scanner pour les champs contenant `[À COMPLÉTER]`
- Si champs critiques incomplets → Suggérer `/osk-rgs` en mode complétion

---

## Documents de Référence RGS

**Templates à charger** (dans `domaines/gouvernement-rgs/templates/`) :

1. `rgs-audit-checklist.md` - Checklist des exigences par fonction RGS
2. `rgs-homologation-dossier-template.md` - Structure du dossier d'homologation
3. `rgs-anssi-cryptography-requirements.md` - Exigences cryptographiques
4. `rgs-franceconnect-auth-requirements.md` - Exigences FranceConnect
5. `rgs-integrity-requirements.md` - Exigences intégrité (Fonction 2)
6. `rgs-tracability-requirements.md` - Exigences traçabilité (Fonction 4)
7. `rgs-mcs-maintenance.md` - Procédures MCS post-homologation

**Documents projet à scanner** :
- `docs/security/features/SEC-*.md` - Analyses de sécurité par feature
- `docs/security/risks/risk-register.yaml` - Registre des risques
- `docs/context/meta.md` - Contexte technique du projet
- `.osk/rgs-context.yaml` - Contexte organisationnel RGS

---

## Processus d'Audit RGS

### Règle Fondamentale : Traçabilité des Preuves

**CHAQUE vérification DOIT être accompagnée de preuves traçables** au format :

```
preuve:
  type: doc | code | config | externe
  fichier: "chemin/vers/fichier.ext"
  ligne: 42  # ou plage "42-58"
  extrait: "code ou texte pertinent (max 100 chars)"
```

**Types de preuves** :
- `doc` : Documentation (SEC-*.md, meta.md, etc.)
- `code` : Code source (*.py, *.js, *.ts, etc.)
- `config` : Configuration (*.yaml, *.json, .env.example, etc.)
- `externe` : Document externe (rapport pentest, certificat, etc.)

**Sans preuve fichier:ligne, une vérification est considérée NON VÉRIFIABLE.**

---

### Étape 1 : Extraction du Contexte

**Lire `.osk/rgs-context.yaml`** et extraire :

```yaml
niveau_rgs_cible: classification.niveau_rgs  # RGS* | RGS** | RGS***
classification_donnees: classification.classification_donnees
besoins_dicp:
  disponibilite: besoins_securite.disponibilite.niveau  # 0-4
  integrite: besoins_securite.integrite.niveau
  confidentialite: besoins_securite.confidentialite.niveau
  preuve: besoins_securite.preuve.niveau
statut_homologation: homologation.statut
```

**Adapter l'audit au niveau RGS cible** :
- RGS* : Exigences de base
- RGS** : Exigences renforcées (+ PSCE qualifiés)
- RGS*** : Exigences maximales (+ certifications gouvernementales)

---

### Étape 2 : Audit par Fonction RGS

Pour CHAQUE fonction RGS (selon Annexes B du RGS), vérifier la conformité.

#### Fonction 1 : Authentification (Annexe B2)

**Exigences selon niveau** :

| Exigence | RGS* | RGS** | RGS*** |
|----------|------|-------|--------|
| Mots de passe ≥ 12 caractères | ✓ | ✓ | ✓ |
| MFA obligatoire | - | ✓ | ✓ |
| Certificats qualifiés | - | - | ✓ |
| FranceConnect (si applicable) | Faible | Substantiel | Élevé |

**Vérifier dans `docs/security/features/SEC-*.md`** :
- [ ] Section authentification présente
- [ ] Méthode d'authentification documentée
- [ ] Niveau eIDAS spécifié (si FranceConnect)
- [ ] Politique de mots de passe conforme

**Vérifier dans le code** :
- [ ] Implémentation MFA (si RGS** ou ***)
- [ ] Verrouillage après X tentatives
- [ ] Expiration des sessions

**Collecter les preuves** (OBLIGATOIRE) :

```yaml
AUTH-001:  # Politique mots de passe
  statut: CONFORME | PARTIEL | NON_CONFORME
  preuves:
    - type: doc
      fichier: "docs/security/features/SEC-AUTH.md"
      ligne: 45
      extrait: "Mot de passe minimum 14 caractères"
    - type: code
      fichier: "src/auth/validators.py"
      ligne: 23-28
      extrait: "MIN_PASSWORD_LENGTH = 14"
    - type: config
      fichier: ".env.example"
      ligne: 12
      extrait: "PASSWORD_MIN_LENGTH=14"

AUTH-002:  # MFA
  statut: CONFORME | PARTIEL | NON_CONFORME
  preuves:
    - type: code
      fichier: "src/auth/mfa.py"
      ligne: 15-42
      extrait: "class TOTPAuthenticator"
    - type: doc
      fichier: "docs/security/features/SEC-AUTH.md"
      ligne: 78
      extrait: "MFA TOTP obligatoire pour admin"

AUTH-003:  # Sessions
  statut: CONFORME | PARTIEL | NON_CONFORME
  preuves:
    - type: config
      fichier: "config/security.yaml"
      ligne: 8
      extrait: "session_timeout: 1800"
```

**Conformité** :
- ✅ CONFORME : Toutes exigences du niveau cible respectées + preuves complètes
- ⚠️ PARTIEL : Exigences partiellement respectées ou preuves incomplètes
- ❌ NON_CONFORME : Exigences critiques manquantes ou non vérifiables

---

#### Fonction 2 : Intégrité (Annexe B3)

**Exigences selon niveau** :

| Exigence | RGS* | RGS** | RGS*** |
|----------|------|-------|--------|
| Validation des entrées | ✓ | ✓ | ✓ |
| Hash SHA-256+ | ✓ | ✓ | ✓ |
| Signature électronique | - | Avancée | Qualifiée |
| Horodatage qualifié | - | ✓ | ✓ |

**Vérifier dans `docs/security/features/SEC-*.md`** :
- [ ] Contrôles d'intégrité documentés
- [ ] Format de signature spécifié (PAdES, XAdES si applicable)
- [ ] TSA (Time Stamping Authority) référencée si RGS**+

**Vérifier dans le code** :
- [ ] Validation systématique des entrées
- [ ] Algorithmes de hash conformes (pas MD5/SHA1)
- [ ] Signature de documents implémentée (si applicable)

**Collecter les preuves** (OBLIGATOIRE) :

```yaml
INT-001:  # Validation des entrées
  statut: CONFORME | PARTIEL | NON_CONFORME
  preuves:
    - type: code
      fichier: "src/api/schemas.py"
      ligne: 12-45
      extrait: "class UserInputSchema(BaseModel)"
    - type: code
      fichier: "src/db/queries.py"
      ligne: 28
      extrait: "cursor.execute(query, params)  # Requête préparée"

INT-002:  # Algorithmes de hash
  statut: CONFORME | PARTIEL | NON_CONFORME
  preuves:
    - type: code
      fichier: "src/utils/crypto.py"
      ligne: 15
      extrait: "hashlib.sha256(data).hexdigest()"
    - type: doc
      fichier: "docs/security/features/SEC-DATA.md"
      ligne: 67
      extrait: "Hash SHA-256 pour vérification intégrité"

INT-003:  # Signature électronique (si applicable)
  statut: CONFORME | PARTIEL | NON_CONFORME | N/A
  preuves:
    - type: code
      fichier: "src/documents/signer.py"
      ligne: 34-56
      extrait: "def sign_document_pades(pdf, certificate)"
```

**Conformité** :
- ✅ CONFORME : Intégrité garantie selon niveau cible + preuves complètes
- ⚠️ PARTIEL : Contrôles présents mais preuves incomplètes
- ❌ NON_CONFORME : Pas de garantie d'intégrité ou non vérifiable

---

#### Fonction 3 : Confidentialité (Annexe B4)

**Exigences selon niveau** :

| Exigence | RGS* | RGS** | RGS*** |
|----------|------|-------|--------|
| TLS 1.2+ | ✓ | ✓ | ✓ |
| TLS 1.3 | - | ✓ | ✓ |
| Chiffrement repos AES-256 | ✓ | ✓ | ✓ |
| HSM pour clés | - | - | ✓ |
| Hébergement SecNumCloud | - | - | ✓ |

**Vérifier dans `docs/context/meta.md`** :
- [ ] Stack technique identifiée
- [ ] Hébergeur spécifié
- [ ] Certifications hébergeur (HDS, SecNumCloud si applicable)

**Vérifier dans `docs/security/features/SEC-*.md`** :
- [ ] Chiffrement en transit documenté
- [ ] Chiffrement au repos documenté
- [ ] Gestion des clés documentée

**Vérifier dans le code et configuration** :
- [ ] TLS configuré (version, cipher suites)
- [ ] Chiffrement base de données
- [ ] Pas de données sensibles en clair

**Collecter les preuves** (OBLIGATOIRE) :

```yaml
CONF-001:  # TLS
  statut: CONFORME | PARTIEL | NON_CONFORME
  preuves:
    - type: config
      fichier: "nginx/ssl.conf"
      ligne: 5-8
      extrait: "ssl_protocols TLSv1.3; ssl_ciphers ..."
    - type: config
      fichier: "docker-compose.yml"
      ligne: 45
      extrait: "FORCE_SSL=true"

CONF-002:  # Chiffrement au repos
  statut: CONFORME | PARTIEL | NON_CONFORME
  preuves:
    - type: config
      fichier: "terraform/database.tf"
      ligne: 23
      extrait: "storage_encrypted = true"
    - type: doc
      fichier: "docs/context/meta.md"
      ligne: 34
      extrait: "PostgreSQL avec chiffrement AES-256"

CONF-003:  # Hébergement (si RGS***)
  statut: CONFORME | PARTIEL | NON_CONFORME | N/A
  preuves:
    - type: externe
      fichier: "certificats/secnumcloud-ovh.pdf"
      ligne: null
      extrait: "Certification SecNumCloud OVH valide jusqu'au..."
    - type: doc
      fichier: ".osk/rgs-context.yaml"
      ligne: 89
      extrait: "hebergeur: OVHcloud SecNumCloud"
```

**Conformité** :
- ✅ CONFORME : Confidentialité garantie selon niveau cible + preuves complètes
- ⚠️ PARTIEL : Chiffrement présent mais preuves incomplètes
- ❌ NON_CONFORME : Données sensibles non protégées ou non vérifiable

---

#### Fonction 4 : Traçabilité (Annexe B5)

**Exigences obligatoires (tous niveaux)** :

| Exigence | RGS* | RGS** | RGS*** |
|----------|------|-------|--------|
| Logs JSON structurés | ✓ | ✓ | ✓ |
| Rétention ≥ 3 ans | ✓ | ✓ | ✓ |
| Protection intégrité logs | - | ✓ | ✓ |
| Horodatage RFC 3161 | - | - | ✓ |

**Événements obligatoires à logger** :
- Authentification (succès/échecs)
- Accès aux données sensibles
- Modifications de données
- Actions administratives
- Erreurs de sécurité

**Vérifier dans `docs/security/features/SEC-*.md`** :
- [ ] Stratégie de logging documentée
- [ ] Événements critiques identifiés
- [ ] Durée de rétention spécifiée

**Vérifier dans le code** :
- [ ] Logger structuré (JSON)
- [ ] Champs obligatoires : timestamp, trace_id, user_id, action, resource
- [ ] Pas de données sensibles dans les logs

**Collecter les preuves** (OBLIGATOIRE) :

```yaml
TRACE-001:  # Logger structuré
  statut: CONFORME | PARTIEL | NON_CONFORME
  preuves:
    - type: code
      fichier: "src/utils/logger.py"
      ligne: 12-25
      extrait: "class JSONFormatter(logging.Formatter)"
    - type: config
      fichier: "config/logging.yaml"
      ligne: 8
      extrait: "format: json"

TRACE-002:  # Événements authentification
  statut: CONFORME | PARTIEL | NON_CONFORME
  preuves:
    - type: code
      fichier: "src/auth/handlers.py"
      ligne: 45
      extrait: "logger.info('auth_success', user_id=user.id)"
    - type: code
      fichier: "src/auth/handlers.py"
      ligne: 52
      extrait: "logger.warning('auth_failure', email=email)"

TRACE-003:  # Rétention 3 ans
  statut: CONFORME | PARTIEL | NON_CONFORME
  preuves:
    - type: config
      fichier: "terraform/cloudwatch.tf"
      ligne: 15
      extrait: "retention_in_days = 1095  # 3 ans"
    - type: doc
      fichier: "docs/security/features/SEC-LOGGING.md"
      ligne: 34
      extrait: "Rétention : 3 ans conformément RGS"

TRACE-004:  # Protection intégrité logs (si RGS**+)
  statut: CONFORME | PARTIEL | NON_CONFORME | N/A
  preuves:
    - type: config
      fichier: "terraform/s3-logs.tf"
      ligne: 28
      extrait: "object_lock_enabled = true  # WORM"
```

**Conformité** :
- ✅ CONFORME : Traçabilité complète selon niveau cible + preuves complètes
- ⚠️ PARTIEL : Logs présents mais preuves incomplètes
- ❌ NON_CONFORME : Traçabilité insuffisante ou non vérifiable

---

### Étape 3 : Vérification de la Chaîne de Confiance

**Fournisseurs (depuis `.osk/rgs-context.yaml`)** :

Pour chaque fournisseur listé :
- [ ] Localisation UE vérifiée
- [ ] Certifications conformes au niveau RGS cible
- [ ] DPA (Data Processing Agreement) signé

**Certifications requises par niveau** :
- RGS* : ISO 27001 recommandé
- RGS** : ISO 27001 obligatoire, HDS si données santé
- RGS*** : SecNumCloud obligatoire

---

### Étape 4 : Calcul du Score de Conformité

**Algorithme de scoring** :

```python
def calculer_score_rgs(resultats_audit):
    poids = {
        "authentification": 25,
        "integrite": 25,
        "confidentialite": 30,
        "tracabilite": 20
    }

    score_total = 0
    for fonction, resultat in resultats_audit.items():
        if resultat == "CONFORME":
            score_total += poids[fonction]
        elif resultat == "PARTIEL":
            score_total += poids[fonction] * 0.5
        # NON_CONFORME = 0

    return score_total  # Sur 100
```

**Seuils de conformité** :
- ≥ 90% : Prêt pour homologation
- 70-89% : Améliorations requises avant homologation
- < 70% : Non conforme, audit bloquant

---

### Étape 5 : Génération des Livrables RGS

#### 5.1 Rapport d'Audit RGS

**Fichier** : `docs/security/audits/AUDIT-RGS-[DATE].md`

```markdown
# Audit de Conformité RGS

> **Date** : [DATE]
> **Système** : [systeme.nom depuis rgs-context.yaml]
> **Niveau cible** : [classification.niveau_rgs]
> **Auditeur** : OpenSecKit /audit rgs

---

## Résumé Exécutif

**Score de Conformité : XX/100**

**Statut Homologation** :
- 🟢 PRÊT POUR HOMOLOGATION (≥90%)
- 🟡 AMÉLIORATIONS REQUISES (70-89%)
- 🔴 NON CONFORME (<70%)

---

## Dashboard RGS

| Fonction | Statut | Score | Exigences |
|----------|--------|-------|-----------|
| 1. Authentification (B2) | ✅/⚠️/❌ | XX/25 | [Niveau atteint] |
| 2. Intégrité (B3) | ✅/⚠️/❌ | XX/25 | [Niveau atteint] |
| 3. Confidentialité (B4) | ✅/⚠️/❌ | XX/30 | [Niveau atteint] |
| 4. Traçabilité (B5) | ✅/⚠️/❌ | XX/20 | [Niveau atteint] |

---

## Détail par Fonction

### Fonction 1 : Authentification (Annexe B2)

**Statut** : [CONFORME/PARTIEL/NON_CONFORME]
**Score** : XX/25

**Exigences vérifiées** :

| ID | Exigence | Statut | Preuves |
|----|----------|--------|---------|
| AUTH-001 | Politique mots de passe ≥14 chars | ✅ | 3 preuves |
| AUTH-002 | MFA obligatoire (RGS**+) | ✅ | 2 preuves |
| AUTH-003 | Sessions sécurisées | ⚠️ | 1 preuve |

**Détail des preuves** :

```
AUTH-001 - Politique mots de passe ✅
├─ [doc]    docs/security/features/SEC-AUTH.md:45
│           "Mot de passe minimum 14 caractères"
├─ [code]   src/auth/validators.py:23-28
│           "MIN_PASSWORD_LENGTH = 14"
└─ [config] .env.example:12
            "PASSWORD_MIN_LENGTH=14"

AUTH-002 - MFA obligatoire ✅
├─ [code]   src/auth/mfa.py:15-42
│           "class TOTPAuthenticator"
└─ [doc]    docs/security/features/SEC-AUTH.md:78
            "MFA TOTP obligatoire pour admin"

AUTH-003 - Sessions sécurisées ⚠️
└─ [config] config/security.yaml:8
            "session_timeout: 1800"
    ⚠️ MANQUANT: Preuve code d'implémentation
```

**Non-conformités** :
- AUTH-003 : Preuve d'implémentation manquante dans le code

**Recommandations** :
- Localiser l'implémentation du timeout session et documenter

[Répéter pour chaque fonction avec le même format]

---

## Chaîne de Confiance

**Fournisseurs audités** :

| Fournisseur | Type | Certifications | Conformité |
|-------------|------|----------------|------------|
| [Nom] | [Type] | [Certs] | ✅/❌ |

---

## Écarts avec le Niveau Cible

### Bloquants (à corriger avant homologation)

1. **[Écart 1]**
   - Fonction : [X]
   - Exigence : [Description]
   - Impact : [Criticité]
   - Recommandation : [Action]

### Non-bloquants (améliorations recommandées)

1. **[Écart 1]**
   - [Détails]

---

## Éléments Manquants pour Dossier d'Homologation

**Documents OSK générés** : ✅
- [ ] Analyses `docs/security/features/SEC-*.md`
- [ ] Risk register `docs/security/risks/risk-register.yaml`
- [ ] Contexte technique `docs/context/meta.md`

**Documents externes requis** : ⚠️
- [ ] Analyse de risques EBIOS RM - [Statut]
- [ ] Rapport de pentest - [Statut]
- [ ] PRA/PCA - [Statut]
- [ ] Politique de sécurité - [Statut]
- [ ] Contrats/DPA fournisseurs - [Statut]

---

## Prochaines Étapes

1. **Corriger les non-conformités bloquantes**
   - [Action 1]
   - [Action 2]

2. **Compléter les documents manquants**
   - [Document 1]

3. **Planifier l'homologation**
   - Date cible : [depuis rgs-context.yaml]
   - Commission : [À définir]

---

## Références

- Contexte RGS : `.osk/rgs-context.yaml`
- Risk Register : `docs/security/risks/risk-register.yaml`
- Templates RGS : `domaines/gouvernement-rgs/templates/`
- RGS v2.0 : [Lien ANSSI]
```

---

#### 5.2 Assemblage Automatique du Dossier d'Homologation

**Si score ≥ 70%**, proposer de générer le dossier d'homologation **pré-rempli automatiquement**.

##### 5.2.1 Scan des Sources Disponibles

**Scanner TOUTES les sources suivantes** et calculer le taux de pré-remplissage :

```yaml
sources:
  # Section 1 : Synthèse Exécutive
  section_1_identification:
    - source: ".osk/rgs-context.yaml"
      champs: [systeme.nom, systeme.code, systeme.description, systeme.url]
      injection: "Section 1.1 - Identification du Système"
    - source: ".osk/rgs-context.yaml"
      champs: [classification.niveau_rgs, classification.classification_donnees]
      injection: "Section 1.3 - Classification RGS"
    - source: ".osk/rgs-context.yaml"
      champs: [besoins_securite.disponibilite, .integrite, .confidentialite, .preuve]
      injection: "Section 1.4 - Besoins DICP"

  # Section 2 : Description du Système
  section_2_architecture:
    - source: "docs/context/meta.md"
      champs: [stack_technique, architecture, composants]
      injection: "Section 2.2 - Vue d'ensemble Architecture"
    - source: "docs/context/meta.md"
      champs: [stack.frontend, stack.backend, stack.database, stack.infrastructure]
      injection: "Section 2.4 - Stack Technologique"

  # Section 3 : Modélisation des Menaces
  section_3_menaces:
    - source: "docs/security/rgs/EBIOS-RM-*.md"
      champs: [atelier_1, atelier_2, atelier_3, scenarios_strategiques]
      injection: "Section 3.1 - Résumé EBIOS Risk Manager"
    - source: "docs/security/features/SEC-*.md"
      champs: [section_stride, menaces_identifiees, contre_mesures]
      injection: "Section 3.2 - Analyse STRIDE (agrégée)"

  # Section 4 : Architecture de Sécurité
  section_4_securite:
    - source: "docs/security/features/SEC-*.md"
      champs: [authentification, integrite, confidentialite, tracabilite]
      injection: "Section 4.1 - Fonctions de Sécurité RGS"
    - source: "Résultats audit (cette session)"
      champs: [preuves_collectees_par_fonction]
      injection: "Section 4 - Preuves d'implémentation"

  # Section 5 : Analyse et Traitement des Risques
  section_5_risques:
    - source: "docs/security/risks/risk-register.yaml"
      champs: [risques[], metadata, metriques]
      injection: "Section 5.2 - Registre des Risques"
    - source: "docs/security/rgs/EBIOS-RM-*.md"
      champs: [atelier_4, atelier_5, plan_traitement]
      injection: "Section 5 - Scénarios opérationnels"

  # Section 6 : Mesures de Sécurité
  section_6_mesures:
    - source: "Résultats audit (cette session)"
      champs: [mesures_techniques, mesures_organisationnelles]
      injection: "Section 6.1-6.2 - Mesures implémentées"
    - source: "Résultats audit (cette session)"
      champs: [conformite_rgs_par_fonction]
      injection: "Section 6.3 - Matrice de Conformité"

  # Section 7 : Tests et Validation
  section_7_tests:
    - source: ".github/workflows/*.yml"
      champs: [sast_config, dast_config, sca_config]
      injection: "Section 7.1 - Tests automatisés"
    - source: "docs/security/audits/AUDIT-*.md"
      champs: [historique_audits, resultats]
      injection: "Section 7.2 - Historique audits"

  # Section 8 : PRA/PCA
  section_8_continuite:
    - source: "docs/security/continuity/PCA-*.md"
      champs: [fonctions_critiques, rto, rpo, organisation_crise]
      injection: "Section 8.3 - Continuité d'Activité"
    - source: "docs/security/continuity/PRA-*.md"
      champs: [strategie_backup, sites_reprise, tests_dr]
      injection: "Section 8.1-8.2 - Sauvegarde et Reprise"

  # Section 9 : Supply Chain
  section_9_fournisseurs:
    - source: ".osk/rgs-context.yaml"
      champs: [fournisseurs.hebergement, .forge, .ci_cd, .autres]
      injection: "Section 9.1 - Fournisseurs Critiques"

  # Section 10-11 : MCS et Incidents
  section_10_11_mcs:
    - source: "docs/security/rgs/MCS-*.md"
      champs: [surveillance, patch_management, revues]
      injection: "Section 10 - MCS"
    - source: "docs/security/incidents/INC-*.md"
      champs: [historique_incidents]
      injection: "Section 11 - Incidents"

  # Section 12 : Décision
  section_12_decision:
    - source: ".osk/rgs-context.yaml"
      champs: [organisation.rssi, organisation.autorite_homologation]
      injection: "Section 12 - Signatures"

  # Documents externes (non générables)
  externes_requis:
    - "Rapport de pentest (prestataire PASSI)"
    - "DPA fournisseurs signés"
    - "Politique de sécurité organisationnelle"
    - "Certificats (SecNumCloud, HDS si applicable)"
```

##### 5.2.2 Affichage du Taux de Pré-remplissage

**Afficher avant génération** :

```
┌─────────────────────────────────────────────────────────────────┐
│ 📋 ASSEMBLAGE DU DOSSIER D'HOMOLOGATION                         │
│                                                                 │
│ Score de conformité : XX/100                                    │
│                                                                 │
│ Sources détectées et taux de pré-remplissage par section :      │
└─────────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────────┐
│ SECTION                              │ PRÉ-REMPLI │ SOURCE       │
├──────────────────────────────────────┼────────────┼──────────────┤
│ 1. Synthèse Exécutive                │    100%    │ rgs-context  │
│ 2. Description du Système            │     85%    │ meta.md      │
│ 3. Modélisation des Menaces          │     90%    │ EBIOS + SEC  │
│ 4. Architecture de Sécurité          │     95%    │ Audit + SEC  │
│ 5. Analyse des Risques               │    100%    │ risk-register│
│ 6. Mesures de Sécurité               │     90%    │ Audit        │
│ 7. Tests et Validation               │     60%    │ CI/CD        │
│ 8. PRA/PCA                           │    100%    │ PCA + PRA    │
│ 9. Supply Chain                      │     80%    │ rgs-context  │
│ 10. MCS                              │     50%    │ MCS-*.md     │
│ 11. Incidents                        │      0%    │ -            │
│ 12. Décision d'Homologation          │     70%    │ rgs-context  │
├──────────────────────────────────────┼────────────┼──────────────┤
│ TOTAL PRÉ-REMPLISSAGE                │     82%    │              │
└──────────────────────────────────────────────────────────────────┘

⚠️  DOCUMENTS EXTERNES REQUIS (non générables)
    • Rapport de pentest PASSI
    • DPA fournisseurs signés
    • Certificats hébergeur

Générer le dossier pré-rempli ?
```

##### 5.2.3 Génération du Dossier Assemblé

**Fichier généré** : `docs/security/rgs/DOSSIER-HOMOLOGATION-[SYSTÈME]-[DATE].md`

**Processus d'assemblage** :

1. **Charger le template** `domaines/gouvernement-rgs/templates/rgs-homologation-dossier-template.md`

2. **Pour chaque section, INJECTER le contenu réel** :

```markdown
## SECTION 1 : SYNTHÈSE EXÉCUTIVE

### 1.1 Identification du Système
<!-- INJECTÉ DEPUIS .osk/rgs-context.yaml -->

| Champ | Valeur |
|-------|--------|
| **Nom du Système** | [systeme.nom] |
| **Code Système** | [systeme.code] |
| **Description** | [systeme.description] |
| **URL Production** | [systeme.url] |
| **RSSI** | [organisation.rssi.nom] ([organisation.rssi.email]) |
| **Autorité d'Homologation** | [organisation.autorite_homologation.nom] |

> ✅ Section pré-remplie automatiquement depuis `.osk/rgs-context.yaml`

### 1.3 Classification RGS
<!-- INJECTÉ DEPUIS .osk/rgs-context.yaml -->

**Niveau RGS** : [classification.niveau_rgs]
**Classification données** : [classification.classification_donnees]
**Réglementations** : [classification.reglementations[].join(", ")]

### 1.4 Besoins DICP
<!-- INJECTÉ DEPUIS .osk/rgs-context.yaml -->

| Besoin | Niveau | Justification | RTO/RPO |
|--------|--------|---------------|---------|
| Disponibilité | [besoins_securite.disponibilite.niveau]/4 | [.justification] | RTO: [.rto], RPO: [.rpo] |
| Intégrité | [besoins_securite.integrite.niveau]/4 | [.justification] | - |
| Confidentialité | [besoins_securite.confidentialite.niveau]/4 | [.justification] | - |
| Preuve | [besoins_securite.preuve.niveau]/4 | [.justification] | Rétention: [.retention] |

---

## SECTION 3 : MODÉLISATION DES MENACES

### 3.1 Résumé EBIOS Risk Manager
<!-- INJECTÉ DEPUIS docs/security/rgs/EBIOS-RM-*.md -->

[CONTENU EXTRAIT DE L'ANALYSE EBIOS]

- **Atelier 1** : [Résumé biens essentiels]
- **Atelier 2** : [Résumé sources de risques]
- **Atelier 3** : [Top 3 scénarios stratégiques]
- **Atelier 4** : [Nombre scénarios opérationnels]
- **Atelier 5** : [Budget traitement risques]

> ✅ Section pré-remplie depuis `docs/security/rgs/EBIOS-RM-[SYSTÈME]-[DATE].md`

### 3.2 Analyse STRIDE (Agrégée)
<!-- INJECTÉ DEPUIS docs/security/features/SEC-*.md -->

| Catégorie | Menaces | Statut |
|-----------|---------|--------|
| Spoofing | [Agrégé depuis SEC-*.md] | X/Y mitigées |
| Tampering | [Agrégé depuis SEC-*.md] | X/Y mitigées |
| Repudiation | [Agrégé depuis SEC-*.md] | X/Y mitigées |
| Info Disclosure | [Agrégé depuis SEC-*.md] | X/Y mitigées |
| DoS | [Agrégé depuis SEC-*.md] | X/Y mitigées |
| Elevation | [Agrégé depuis SEC-*.md] | X/Y mitigées |

> ✅ Agrégé automatiquement depuis [X] fichiers SEC-*.md

---

## SECTION 4 : ARCHITECTURE DE SÉCURITÉ

### 4.1 Fonctions de Sécurité RGS
<!-- INJECTÉ DEPUIS Résultats Audit RGS -->

#### Fonction 1 : Authentification (Annexe B2)

**Statut** : [Résultat audit]
**Score** : [XX/25]

**Preuves d'implémentation** :
```
AUTH-001 ✅ [doc] docs/security/features/SEC-AUTH.md:45
AUTH-002 ✅ [code] src/auth/mfa.py:15-42
AUTH-003 ✅ [config] config/security.yaml:8
```

> ✅ Preuves collectées lors de l'audit RGS du [DATE]

[RÉPÉTER POUR CHAQUE FONCTION]

---

## SECTION 5 : ANALYSE ET TRAITEMENT DES RISQUES

### 5.2 Registre des Risques
<!-- INJECTÉ DEPUIS docs/security/risks/risk-register.yaml -->

| ID | Risque | Gravité | Vraisemblance | Score | Statut |
|----|--------|---------|---------------|-------|--------|
[POUR CHAQUE risque DANS risk-register.yaml]
| [id] | [titre] | [gravite] | [vraisemblance] | [score_residuel] | [statut] |
[FIN POUR]

**Métriques** :
- Risques totaux : [metadata.risques_totaux]
- Risques ouverts : [metadata.risques_ouverts]
- Score résiduel total : [metriques.score_risque_residuel_total]

> ✅ Extrait de `docs/security/risks/risk-register.yaml` (dernière MAJ: [date])

---

## SECTION 8 : PRA/PCA

### 8.1 Stratégie de Sauvegarde
<!-- INJECTÉ DEPUIS docs/security/continuity/PRA-*.md -->

[CONTENU EXTRAIT DU PRA]

### 8.3 Continuité d'Activité
<!-- INJECTÉ DEPUIS docs/security/continuity/PCA-*.md -->

[CONTENU EXTRAIT DU PCA]

> ✅ Extrait de `docs/security/continuity/PCA-[SYSTÈME]-[DATE].md`

---

## SECTION 9 : SUPPLY CHAIN

### 9.1 Fournisseurs Critiques
<!-- INJECTÉ DEPUIS .osk/rgs-context.yaml -->

| Fournisseur | Type | Localisation | Certifications | DPA |
|-------------|------|--------------|----------------|-----|
[POUR CHAQUE fournisseur DANS rgs-context.fournisseurs]
| [nom] | [type] | [localisation] | [certifications] | [dpa_signe ? "✅" : "⚠️"] |
[FIN POUR]

> ✅ Extrait de `.osk/rgs-context.yaml`

---

## SECTIONS À COMPLÉTER MANUELLEMENT

Les sections suivantes nécessitent des documents externes :

### Section 7.1 : Rapport de Pentest
<!-- ⚠️ DOCUMENT EXTERNE REQUIS -->

> ⚠️ **À COMPLÉTER** : Joindre le rapport de test d'intrusion
> - Prestataire : [Nom prestataire PASSI]
> - Date : [Date du pentest]
> - Fichier : `audits/[DATE]-pentest-[PRESTATAIRE].pdf`

### Section 12 : Signatures
<!-- ⚠️ SIGNATURES REQUISES -->

> ⚠️ **À COMPLÉTER** : Signatures manuscrites ou électroniques

**Recommandation RSSI** :
- Signé : _________________ Date : _________
- [organisation.rssi.nom], RSSI

**Décision Autorité d'Homologation** :
- Signé : _________________ Date : _________
- [organisation.autorite_homologation.nom]
```

##### 5.2.4 Résumé Post-Génération

**Afficher après génération** :

```
┌─────────────────────────────────────────────────────────────────┐
│ ✅ DOSSIER D'HOMOLOGATION GÉNÉRÉ                                │
└─────────────────────────────────────────────────────────────────┘

📄 Fichier : docs/security/rgs/DOSSIER-HOMOLOGATION-[SYSTÈME]-[DATE].md

📊 CONTENU ASSEMBLÉ

   Sources utilisées :
   ├─ .osk/rgs-context.yaml          → Sections 1, 9, 12
   ├─ docs/context/meta.md           → Section 2
   ├─ docs/security/rgs/EBIOS-RM-*.md → Section 3
   ├─ docs/security/features/SEC-*.md → Sections 3, 4
   ├─ docs/security/risks/risk-register.yaml → Section 5
   ├─ Résultats audit RGS            → Sections 4, 6, 7
   ├─ docs/security/continuity/PCA-*.md → Section 8
   └─ docs/security/continuity/PRA-*.md → Section 8

   Taux de pré-remplissage : 82%

⚠️  ACTIONS RESTANTES

   1. [ ] Joindre rapport pentest     → Section 7
   2. [ ] Joindre DPA fournisseurs    → Section 9 (annexe)
   3. [ ] Joindre certificats         → Section 9 (annexe)
   4. [ ] Signatures RSSI + Autorité  → Section 12
   5. [ ] Revue finale par RSSI

📋 Le dossier est prêt à ~82%. Complétez les sections marquées
   [⚠️ À COMPLÉTER] avant soumission à la commission.
```

---

### Étape 6 : Dashboard Terminal RGS

**Affichage immédiat après audit** :

```
━━━━━━━ AUDIT DE CONFORMITÉ RGS ━━━━━━━
Système: [systeme.nom]
Niveau cible: [RGS* | RGS** | RGS***]
Date: [DATE]

┌──────────────────────────────────────────────────────────┐
│ SCORE DE CONFORMITÉ : XX/100                              │
│ [🟢 PRÊT POUR HOMOLOGATION / 🟡 AMÉLIORATIONS / 🔴 NON]  │
└──────────────────────────────────────────────────────────┘

┌───────────────────────────────────────────────────────────────────┐
│ FONCTION RGS                   │ STATUT │ SCORE │ PREUVES │ NIVEAU│
├────────────────────────────────┼────────┼───────┼─────────┼───────┤
│ 1. Authentification (B2)       │   ✅   │ 25/25 │   8/8   │ RGS** │
│ 2. Intégrité (B3)              │   ⚠️   │ 12/25 │   4/6   │ RGS*  │
│ 3. Confidentialité (B4)        │   ✅   │ 30/30 │   7/7   │ RGS** │
│ 4. Traçabilité (B5)            │   ⚠️   │ 15/20 │   5/8   │ RGS*  │
└───────────────────────────────────────────────────────────────────┘

🔴 NON-CONFORMITÉS BLOQUANTES (X)

  [FONCTION] - [Description courte]
    Exigence : [Exigence RGS non respectée]
    Impact : [Criticité]
    Action : [Correction requise]

⚠️  PREUVES MANQUANTES (X)

  INT-002 - Hash SHA-256
    ✗ Preuve code manquante
    → Localiser implémentation hashage dans le code source

  TRACE-003 - Rétention 3 ans
    ✗ Preuve config manquante
    → Documenter configuration rétention logs

📋 DOCUMENTS MANQUANTS POUR HOMOLOGATION

  ⚠️ Analyse EBIOS RM - Non fournie
  ⚠️ Rapport pentest - Non fourni
  ✅ Analyses SEC-*.md - Présentes (X preuves doc)
  ✅ Risk register - À jour

📄 Rapport complet : docs/security/audits/AUDIT-RGS-[DATE].md
   └─ Inclut le détail des XX preuves collectées (fichier:ligne)
📊 Contexte RGS : .osk/rgs-context.yaml

➡️  Prochaines étapes :
    1. Compléter les preuves manquantes (X)
    2. Corriger les non-conformités bloquantes
    3. Fournir les documents externes manquants
    4. Relancer /audit rgs
    5. Préparer la commission d'homologation
```

---

## Règles Spécifiques Mode RGS

1. **Contexte obligatoire** : Ne JAMAIS auditer sans `.osk/rgs-context.yaml`
2. **Niveau adaptatif** : Ajuster les exigences au niveau RGS cible
3. **Traçabilité juridique** : Chaque vérification doit être prouvable
4. **Chaîne de confiance** : Vérifier les fournisseurs, pas seulement le code
5. **Documents externes** : Identifier clairement ce qui ne peut pas être généré par OSK
6. **Ébauche actionnable** : Le dossier d'homologation doit être exploitable par une vraie commission
7. **MCS anticipé** : Rappeler les obligations post-homologation (template MCS)
8. **Preuves fichier:ligne OBLIGATOIRES** :
   - Chaque exigence vérifiée DOIT avoir au moins une preuve avec `fichier:ligne`
   - Format : `[type] chemin/fichier.ext:ligne "extrait"`
   - Types acceptés : `[doc]`, `[code]`, `[config]`, `[externe]`
   - Sans preuve → statut `NON VÉRIFIABLE` (équivalent NON_CONFORME)
   - Le rapport DOIT inclure la section "Détail des preuves" pour chaque fonction
   - Le dashboard terminal DOIT afficher le ratio preuves collectées/attendues
