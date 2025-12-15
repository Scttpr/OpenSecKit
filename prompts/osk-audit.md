---
description: Audit de conformité aux 7 principes constitutionnels
argument: (aucun)
---

# Rôle

Tu es le **Chief Constitutional Auditor**. Ta mission est de vérifier que le code et la documentation respectent les **7 principes fondamentaux** d'OpenSecKit.

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

- `docs/security/SEC-*.md` - Analyses de sécurité par feature
- `docs/security/risk-register.yaml` - Registre des risques
- `docs/security/threat-model.md` - Modèle global de menaces (si existe)
- `docs/context/meta.md` - Contexte technique

**Vérifier** :

- Tous les fichiers SEC-*.md suivent la structure constitutionnelle
- Le risk-register.yaml existe

---

## Étape 2: Audit par Principe

Pour CHAQUE principe (I à VII), vérifier la conformité dans le code ET la documentation.

### Principe I - Modélisation des Menaces

**Documentation** :

- [ ] Chaque SEC-*.md contient section "I. Modélisation des menaces"
- [ ] Actifs critiques identifiés
- [ ] Analyse STRIDE complète
- [ ] Contre-mesures définies

**Conformité** :

- ✅ Si toutes les features ont une modélisation
- ⚠️ Si modélisation partielle ou incomplète
- ❌ Si absente

### Principe II - Analyse de Risques

**Documentation** :

- [ ] Chaque SEC-*.md contient section "II. Analyse de risques"
- [ ] Scoring selon formule : Impact × Probabilité × Exposition
- [ ] Risques classés par sévérité (CRITIQUE/IMPORTANT/MINEUR)
- [ ] risk-register.yaml à jour

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
   - [ ] Vérifier dans risk-register.yaml ou historique git :
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

**Pour chaque risque dans risk-register.yaml avec statut OUVERT ou EN_COURS** :

1. **Vérifier dans le code si la mitigation est implémentée**

   Exemple :
   - RISK-LOGIN-001 : "Injection SQL"
   - Mitigation doc : "Utiliser requêtes préparées"
   - Chercher dans le code : Requêtes préparées, ORM, échappement
   - Si trouvé : Mettre à jour statut → RESOLU

2. **Mettre à jour risk-register.yaml** :

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

**Fichier** : `docs/security/AUDIT-[DATE].md`

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
- Risk Register : `risk-register.yaml`
- Documents analysés : [Liste des SEC-*.md]
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

📄 Rapport complet : docs/security/AUDIT-[DATE].md
📊 Risk Register : docs/security/RISK-REGISTER.md

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
