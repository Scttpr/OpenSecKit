# Quick start - Votre premier template en 10 minutes

**Objectif** : Sécuriser rapidement une application web en utilisant OpenSecKit.

**Temps estimé** : 10-15 minutes

---

## Contexte de l'exemple

Vous développez une **application web e-commerce** avec :
- Frontend React
- Backend Node.js + Express
- Base de données PostgreSQL
- API de paiement Stripe
- Hébergement cloud (AWS/Azure/GCP)

**Objectif** : Appliquer les principes de sécurité minimum avant la mise en production.

---

## Étape 1 : Modélisation des menaces (5 minutes)

### 1.1 Identifier vos actifs critiques

Les **actifs** sont ce que vous devez protéger :

- ✅ Base de données clients (emails, adresses, historique commandes)
- ✅ Tokens de paiement Stripe
- ✅ Sessions utilisateur (JWT tokens)
- ✅ Clés API backend

### 1.2 Utiliser le template STRIDE

```bash
# Copier le template
cp templates/01-threat-modeling/stride-threat-model-template-planning.md \
   mon-projet/docs/security/threat-model.md
```

### 1.3 Remplir rapidement (version minimale)

Ouvrez le fichier et remplissez :

**Assets** :
```markdown
## 3. Assets
| Asset | Type | Criticité | Justification |
|-------|------|-----------|---------------|
| Base de données clients | Données | Critique | Contient PII, emails, adresses |
| Tokens Stripe | Credentials | Critique | Accès paiements clients |
| Sessions utilisateur | Tokens | Élevée | Accès comptes utilisateurs |
```

**Menaces STRIDE** (top 3 minimum) :

```markdown
## 4. Menaces STRIDE

### Spoofing (Usurpation d'identité)
| # | Menace | Asset ciblé | Impact | Probabilité | Contre-mesure |
|---|--------|-------------|--------|-------------|---------------|
| T1 | Attaquant usurpe un utilisateur légitime | Sessions | Élevé | Moyenne | Multi-facteur auth (MFA) |

### Tampering (Altération)
| # | Menace | Asset ciblé | Impact | Probabilité | Contre-mesure |
|---|--------|-------------|--------|-------------|---------------|
| T2 | Injection SQL dans formulaire paiement | Base de données | Critique | Élevée | Requêtes préparées, ORM |

### Information Disclosure (Divulgation d'information)
| # | Menace | Asset ciblé | Impact | Probabilité | Contre-mesure |
|---|--------|-------------|--------|-------------|---------------|
| T3 | Logs exposent tokens Stripe | Tokens Stripe | Critique | Moyenne | Scrubbing des logs |
```

✅ **Vous avez identifié 3 menaces critiques en 5 minutes !**

---

## Étape 2 : Analyse de risques (3 minutes)

### 2.1 Scorer les risques

```bash
# Copier le template
cp templates/02-risk-analysis/risk-scoring-template-planning.md \
   mon-projet/docs/security/risk-analysis.md
```

### 2.2 Calculer le score de risque

**Formule** : `Score = Criticité × Probabilité × Exposition`

Pour **T2 (Injection SQL)** :
- Criticité = 5 (base de données compromise)
- Probabilité = 4 (vulnérabilité commune)
- Exposition = 5 (formulaire public sur internet)
- **Score = 5 × 4 × 5 = 100 / 125 = 80% de risque**

### 2.3 Remplir le registre des risques

```markdown
## Registre des risques

| ID | Menace | Score | Niveau | Validation requise | Contre-mesure | Statut |
|----|--------|-------|--------|-------------------|---------------|--------|
| T2 | Injection SQL | 20/25 | Critique | Direction | Requêtes préparées | À implémenter |
| T1 | Usurpation identité | 15/25 | Élevé | Security champion | MFA | À implémenter |
| T3 | Logs exposent secrets | 12/25 | Élevé | Security champion | Scrubbing logs | À implémenter |
```

✅ **Vous savez maintenant quoi prioriser !**

---

## Étape 3 : Actions immédiates (7 minutes)

### 3.1 Sécuriser la base de données (2 min)

**Problème** : T2 - Injection SQL

**Solution rapide** :

```javascript
// ❌ AVANT (vulnérable)
const query = `SELECT * FROM users WHERE email = '${req.body.email}'`;
db.query(query);

// ✅ APRÈS (sécurisé)
const query = 'SELECT * FROM users WHERE email = $1';
db.query(query, [req.body.email]);
```

**Template utilisé** : `templates/03-security-requirements/owasp-asvs-checklist-design.md` (section Injection)

---

### 3.2 Installer la détection de secrets (3 min)

**Problème** : T3 - Logs exposent les tokens Stripe

**Solution** :

```bash
# Installer gitleaks
brew install gitleaks  # macOS
# ou wget https://github.com/gitleaks/gitleaks/releases/...

# Scanner le code existant
gitleaks detect --source . --verbose

# Installer le pre-commit hook
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/sh
gitleaks protect --staged --verbose
EOF
chmod +x .git/hooks/pre-commit
```

**Template utilisé** : `templates/05-secrets-management/secrets-detection-setup.md`

---

### 3.3 Activer MFA (2 min - planning)

**Problème** : T1 - Usurpation d'identité

**Action** : Ajouter dans votre backlog :

```markdown
## User Story : Multi-factor authentication

En tant qu'utilisateur, je veux activer l'authentification à deux facteurs
pour protéger mon compte contre l'usurpation.

**Acceptance criteria** :
- [ ] Option MFA disponible dans les paramètres utilisateur
- [ ] Support TOTP (Google Authenticator, Authy)
- [ ] Codes de récupération générés
- [ ] MFA obligatoire pour admins
```

**Template à utiliser plus tard** : `templates/03-security-requirements/authentication-requirements-template-design.md`

---

## Étape 4 : Validation constitutionnelle (2 minutes)

### Checklist minimale avant production

Vérifiez que vous respectez les 7 principes :

- [x] **I. Modélisation des menaces** : `threat-model.md` créé ✅
- [x] **II. Analyse de risques** : Risques scorés, top 3 identifiés ✅
- [x] **III. Sécurité dès la conception** : Injection SQL corrigée ✅
- [ ] **IV. Tests de sécurité** : À faire (voir étape 5)
- [x] **V. Gestion des secrets** : Détection pré-commit installée ✅
- [ ] **VI. Journalisation d'audit** : À faire (voir étape 5)
- [ ] **VII. Patch management** : À configurer (voir étape 5)

**Score** : 4/7 principes appliqués en 10 minutes !

---

## Étape 5 : Prochaines étapes (après le quick start)

### Cette semaine

1. **Configurer SAST** (15 min) :
   ```bash
   # Ajouter SonarQube/Semgrep dans votre CI/CD
   ```
   Template : `templates/04-security-testing/sast-integration-guide-implementation.md`

2. **Ajouter logging sécurisé** (30 min) :
   Template : `templates/06-audit-logging/logging-requirements-template-design.md`

3. **Configurer Dependabot** (5 min) :
   ```yaml
   # .github/dependabot.yml
   version: 2
   updates:
     - package-ecosystem: "npm"
       directory: "/"
       schedule:
         interval: "daily"
   ```
   Template : `templates/07-patch-management/dependency-scanning-guide-all.md`

### Ce mois

4. **Implémenter MFA** : Voir template `authentication-requirements-template-design.md`
5. **Configurer un gestionnaire de secrets** : Vault/Key Vault (template `vault-integration-guide.md`)
6. **Audit complet** : Vérifier tous les templates restants

---

## Résumé : Ce que vous avez accompli

En **10-15 minutes**, vous avez :

✅ Identifié vos **3 menaces critiques**
✅ Scoré les **risques** pour prioriser
✅ Corrigé l'**injection SQL** (vulnérabilité #1)
✅ Installé la **détection de secrets**
✅ Créé un **plan d'action** clair

**Impact** :
- 🛡️ Réduction de **60% du risque** le plus critique (injection SQL)
- 🚫 **0 secret** ne sera plus jamais committé accidentellement
- 📋 Roadmap sécurité claire pour les 30 prochains jours

---

## Aide et ressources

### Vous êtes bloqué ?

- 📖 **Lire la constitution** : [constitution.md](constitution.md) - Comprendre le "pourquoi"
- ❓ **Consulter la FAQ** : [FAQ.md](FAQ.md) - Questions fréquentes
- 💬 **Poser une question** : Ouvrir une issue GitHub avec tag `question`

### Vous voulez aller plus loin ?

- **Cas d'usage par rôle** : Voir [README.md](README.md#cas-dusage-par-rôle)
- **Templates avancés** : Explorer `templates/` pour chaque principe
- **Domaines spécifiques** : RGPD, NIS2, RGS dans `domaines/`

---

## Checklist finale

Avant de fermer ce guide, vérifiez que :

- [ ] Vous avez créé `threat-model.md` et `risk-analysis.md`
- [ ] Vous avez corrigé au moins 1 vulnérabilité critique
- [ ] Vous avez installé gitleaks localement
- [ ] Vous savez quoi faire cette semaine (étape 5)
- [ ] Vous avez bookmarké ce guide pour votre prochain projet

---

**Félicitations ! Votre application est déjà beaucoup plus sécurisée.** 🎉

*Temps total : 10-15 minutes | Impact : Réduction majeure des risques critiques*

**Prochaine étape recommandée** : [Configurer SAST dans votre CI/CD](templates/04-security-testing/sast-integration-guide-implementation.md)
