# Quick Start - Sécurisez votre projet en 10 minutes

**Objectif** : Obtenir un audit de sécurité actionnable et corriger vos premières vulnérabilités.

**Deux approches** :
- **Avec Claude Code + CLI** (5 minutes) - Recommandé (support complet)
- **Manuellement** (15 minutes) - Pour comprendre les principes

> **Note :** OpenSecKit est optimisé pour Claude Code. Les autres providers (Gemini, usage manuel) sont en **beta**.

---

## Contexte de l'exemple

Vous développez une **application web e-commerce** avec :
- Frontend React
- Backend Node.js + Express
- Base de données PostgreSQL
- API de paiement Stripe
- Hébergement cloud (AWS/Azure/GCP)

**Objectif** : Identifier et corriger les vulnérabilités critiques avant la mise en production.

---

## Approche 1 : Avec Claude Code (5 minutes) - Recommandé

**Prérequis** :
- Clé API Claude (`CLAUDE_API_KEY`)
- [Claude Code](https://claude.com/claude-code) installé

> **Pourquoi Claude Code ?** Support complet, modification automatique de fichiers, workflow fluide.

### Étape 1 : Installer et initialiser OpenSecKit

```bash
# Installer la CLI
git clone https://github.com/Scttpr/OpenSecKit
cd OpenSecKit/cli
cargo install --path .

# Initialiser dans votre projet e-commerce
cd mon-ecommerce/
export CLAUDE_API_KEY="votre-clé"
osk init

# Lors de l'initialisation :
# 1. Choisir "Claude" comme provider
# 2. Répondre "Oui" à "Activer l'intégration Claude Code ?"

# La CLI va :
# - Détecter votre stack (Node.js, React, PostgreSQL)
# - Télécharger les templates et prompts depuis GitHub
# - Créer .osk/config.toml
# - Générer docs/context/meta.md automatiquement
# - Créer .claude/commands/ avec les slash commands
```

### Étape 2 : Lancer l'audit avec Claude Code

```bash
# Lancer Claude Code
claude

# Dans l'interface, taper :
/osk-audit
```

**Ce qui se passe :**
1. Claude Code lit votre code
2. Applique le prompt d'audit OpenSecKit
3. Génère un rapport structuré
4. Affiche les vulnérabilités avec fichiers et lignes exactes

### Étape 3 : Analyser les résultats

Le rapport s'affiche dans le terminal et contient :

- **Score de conformité** : X/100
- **Top 3 risques critiques** identifiés dans votre code
- **Preuves techniques** (fichiers et lignes de code concernés)
- **Plan de remédiation** avec commandes `osk` à exécuter

**Exemple de résultat** :

```markdown
## Top 3 Risques Critiques

1. **Injection SQL** (Score 20/25) - `src/api/users.js:42`
   - Requête vulnérable détectée
   - Recommandation : Utiliser requêtes préparées

2. **Secrets exposés** (Score 18/25) - `.env` commité dans l'historique git
   - Token Stripe trouvé dans commit abc123
   - Recommandation : Révoquer + installer gitleaks

3. **Pas d'authentification sur /admin** (Score 15/25) - `src/routes/admin.js:12`
   - Endpoint sans protection
   - Recommandation : Ajouter middleware auth
```

### Étape 4 : Appliquer les corrections automatiquement

**Avantage de Claude Code :** Il modifie vos fichiers directement.

```bash
# Dans Claude Code (même session)
Corrige l'injection SQL dans src/api/users.js:42 en utilisant des requêtes préparées
```

Claude Code va :
1. Lire le fichier vulnérable
2. Générer le code sécurisé
3. Modifier le fichier automatiquement
4. Vous montrer le diff

**Avec Gemini ou autre (beta) :** Vous devez modifier manuellement.

### Résultat en 5 minutes

✅ Audit complet effectué via Claude Code
✅ Top 3 vulnérabilités identifiées avec fichiers et lignes exactes
✅ Corrections appliquées automatiquement
✅ Workflow fluide : Audit → Correction → Vérification

**Prochaine étape** : Relancer `/osk-audit` pour vérifier que tout est corrigé.

---

## Approche 2 : Manuelle (15 minutes) - Pour comprendre les principes

Si vous n'utilisez pas d'agent IA ou voulez comprendre les mécanismes sous-jacents, suivez cette approche pas à pas.

### Étape 1 : Modélisation des menaces (5 minutes)

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

### Avec l'approche Claude Code (5 minutes)

✅ **Audit complet** de votre code effectué
✅ **Top 3 vulnérabilités** identifiées avec fichiers et lignes exactes
✅ **Corrections appliquées** automatiquement dans vos fichiers
✅ **Workflow fluide** : tout dans la même session

**Impact** :
- 🛡️ Visibilité complète sur votre posture de sécurité
- 🎯 Priorisation automatique des risques
- 🤖 Corrections automatiques (pas de copier-coller)
- ⚡ Itération rapide : Audit → Fix → Vérification
- 📊 Rapport basé sur les 7 principes constitutionnels

### Avec l'approche manuelle (15 minutes)

✅ **Modélisation des menaces** : 3 menaces critiques identifiées
✅ **Analyse de risques** : Risques scorés et priorisés
✅ **Injection SQL** corrigée (vulnérabilité #1)
✅ **Détection de secrets** installée (gitleaks pre-commit hook)
✅ **Plan d'action** clair pour les 30 prochains jours

**Impact** :
- 🛡️ Réduction de **60% du risque** le plus critique
- 🚫 **0 secret** ne sera plus jamais committé
- 📋 Compréhension profonde des principes de sécurité
- 📚 Maîtrise des templates pour vos prochains projets

---

## Aide et ressources

### Vous êtes bloqué ?

- 📖 **Lire la constitution** : [constitution.md](constitution.md) - Comprendre le "pourquoi"
- ❓ **Consulter la FAQ** : [FAQ.md](FAQ.md) - Questions fréquentes (section agents IA)
- 💬 **Poser une question** : [GitHub Discussions](https://github.com/Scttpr/OpenSecKit/issues)

### Vous voulez aller plus loin ?

- **Utiliser d'autres prompts** : `/spec`, `/domain rgpd`, `/context`
- **Templates avancés** : Explorer `templates/` pour chaque principe
- **Domaines spécifiques** : Conformité RGPD, NIS2, RGS dans `domaines/`
- **CLI avancée** : [cli/README.md](cli/README.md) pour toutes les commandes

---

## Checklist finale

### Si vous avez utilisé l'approche Claude Code

- [ ] L'audit s'est exécuté via `/osk-audit`
- [ ] Vous avez lu le Top 3 des risques critiques
- [ ] Claude Code a modifié vos fichiers automatiquement
- [ ] Vous savez relancer `/osk-audit` pour vérifier les corrections

### Si vous avez utilisé l'approche manuelle

- [ ] Vous avez créé `threat-model.md` et `risk-analysis.md`
- [ ] Vous avez corrigé au moins 1 vulnérabilité critique
- [ ] Vous avez installé gitleaks en pre-commit hook
- [ ] Vous savez quoi faire cette semaine (étape 5)

**Dans tous les cas** :
- [ ] Vous avez bookmarké ce guide pour votre prochain projet
- [ ] Vous avez partagé OpenSecKit avec votre équipe

---

**Félicitations ! Votre application est déjà beaucoup plus sécurisée.** 🎉

**Prochaines étapes recommandées** :
- **Avec Claude Code** : `/osk-spec "description"` pour vos prochaines features, `/osk-domain rgpd` pour la conformité
- **Manuel** : [Configurer SAST dans votre CI/CD](templates/04-security-testing/sast-integration-guide-implementation.md)

> **Note :** L'approche Gemini ou CLI standalone (sans Claude Code) est en **beta**. Pas de modification automatique de fichiers.
