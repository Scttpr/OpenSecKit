# Analyse de Sécurité: Système d'Authentification

> **Conformité Constitutionnelle OpenSecKit**
> Feature: Authentification utilisateur par email/password
> Date: 2025-01-15
> Stack: Node.js, Express, PostgreSQL

---

## Résumé Exécutif

### Conformité Globale : 4/7 Principes Satisfaits (57%)

**🔴 BLOQUANTS POUR PRODUCTION** :
- **RISK-LOGIN-001** - Injection SQL (Score: 100) → Principe III
- **Principe IV** - Aucun test de sécurité automatisé

**🟠 À CORRIGER AVANT V1** :
- **RISK-LOGIN-002** - Absence rate limiting (Score: 48) → Principe III
- **RISK-LOGIN-003** - Timing attack (Score: 48) → Principe III
- **Principe VI** - Logs d'échec auth manquants

**🟡 BONNES PRATIQUES** :
- **RISK-LOGIN-004** - Session fixation (Score: 18)
- Validation email côté serveur
- Améliorer messages d'erreur

### Ordre de Priorité

1. **[URGENT - 24h]** Corriger injection SQL (RISK-LOGIN-001) → Principe III
2. **[URGENT - 24h]** Ajouter tests unitaires sécurité → Principe IV
3. **[IMPORTANT - 7j]** Implémenter rate limiting (RISK-LOGIN-002) → Principe III
4. **[IMPORTANT - 7j]** Ajouter SAST/DAST en CI/CD → Principe IV
5. **[MINEUR - 30j]** Corriger timing attack (RISK-LOGIN-003) → Principe III
6. **[MINEUR - 30j]** Ajouter logs échecs auth → Principe VI

### Checklist de Validation Finale

- [ ] **RISK-LOGIN-001 résolu** (Requêtes préparées implémentées)
- [ ] **Tests de sécurité ajoutés** (SQL injection, rate limit)
- [ ] **SAST configuré** (Semgrep dans CI/CD)
- [ ] **SCA activé** (Dependabot)
- [ ] **Rate limiting implémenté** (express-rate-limit)
- [ ] **Logs échecs ajoutés** (Winston)
- [ ] **Revue de code** effectuée par security champion
- [ ] **Validation constitutionnelle** : 7/7 ✅

**Estimation effort** : 2-3 jours développeur

---

## Conformité Constitutionnelle

| Principe | Statut | Couverture | Bloquant Prod |
|----------|--------|------------|---------------|
| I. Modélisation des menaces | ✅ Complet | - | Non |
| II. Analyse de risques | ✅ Complet | - | Non |
| III. Sécurité dès conception | ⚠️ Partiel | 3/5 | Si ❌ |
| IV. Tests de sécurité | ❌ Absent | 0/4 | Oui |
| V. Gestion des secrets | ✅ Complet | - | Si ❌ |
| VI. Traçabilité | ⚠️ Partiel | 1/3 | Non |
| VII. Patch management | ✅ Complet | - | Non |

**Légende** :
- ✅ Conforme
- ⚠️ Partiel (action requise)
- ❌ Non conforme (bloquant)
- N/A Non applicable

---

## I. Modélisation des Menaces

### Actifs Critiques
- Sessions utilisateur (tokens JWT signés avec RS256)
- Mots de passe hashés avec bcrypt (coût: 12)
- Adresses email (données personnelles RGPD)
- Clé privée JWT (secret critique)

### Vecteurs d'Attaque (STRIDE)

- **Spoofing (Usurpation)** :
  - Session hijacking via vol de token JWT
  - Credential stuffing (réutilisation de mots de passe volés)

- **Tampering (Altération)** :
  - Modification du token JWT (mitigé par signature RS256)

- **Repudiation (Répudiation)** :
  - Utilisateur nie s'être connecté (nécessite logs d'authentification)

- **Information Disclosure (Fuite)** :
  - Enumération des comptes via timing attack sur /login
  - Fuite d'email dans message d'erreur différencié

- **Denial of Service (DoS)** :
  - Brute force sans rate limiting
  - Bcrypt bombing (coût élevé volontaire)

- **Elevation of Privilege (Élévation)** :
  - Injection SQL dans champ email ou password
  - Manipulation du rôle dans token JWT

### Contre-mesures Identifiées
- Tokens JWT signés avec RS256 (clé privée sécurisée)
- Rate limiting sur /login (5 tentatives/min/IP)
- Messages d'erreur génériques
- Hachage bcrypt avec coût 12
- Requêtes préparées (parameterized queries)
- Validation des entrées (email format, password longueur)

---

## II. Analyse de Risques

### Méthodologie de Scoring

**Formule** : Score = Impact (1-5) × Probabilité (1-5) × Exposition (1-5)

**Seuils** :
- 75-125 : 🔴 CRITIQUE
- 25-74 : 🟠 IMPORTANT
- 1-24 : 🟡 MINEUR

### 🔴 Risques CRITIQUES

**RISK-LOGIN-001 - Injection SQL potentielle dans endpoint /auth**
- **Menace** : Attaquant peut exécuter du code SQL arbitraire via les champs email/password
- **Impact** : 5 (Accès à tous les comptes utilisateurs, dump de la base)
- **Probabilité** : 4 (Endpoint public, facile à exploiter avec sqlmap)
- **Exposition** : 5 (Internet-facing, aucune authentification requise)
- **Score** : 100
- **Principe violé** : III. Sécurité dès conception (Validation systématique)
- **Contrôle manquant** : Utilisation de requêtes préparées
- **Mitigation** : Remplacer concaténation SQL par parameterized queries

```javascript
// ❌ Code vulnérable (src/auth/login.js:42)
const query = `SELECT * FROM users WHERE email = '${email}' AND password = '${password}'`;
db.query(query);

// ✅ Code sécurisé
const query = 'SELECT * FROM users WHERE email = $1';
db.query(query, [email]);
// Note: Ne jamais comparer le password en SQL, utiliser bcrypt.compare()
```

- **Tests requis** :
  - `tests/security/test_sql_injection.js`
  - Payloads: `' OR '1'='1`, `'; DROP TABLE users--`
- **Références** :
  - OWASP A03:2021 - Injection
  - `templates/04-security-testing/sql-injection-test-checklist.md`

---

### 🟠 Risques IMPORTANTS

**RISK-LOGIN-002 - Absence de rate limiting**
- **Menace** : Brute force de mots de passe sans limitation
- **Impact** : 4 (Compromission de comptes faibles)
- **Probabilité** : 3 (Nécessite automation mais facile)
- **Exposition** : 4 (Endpoint public)
- **Score** : 48
- **Principe violé** : III. Sécurité dès conception (Défense en profondeur)
- **Contrôle manquant** : Rate limiting middleware
- **Mitigation** : Implémenter express-rate-limit avec Redis

```javascript
// À ajouter dans src/middlewares/rate-limit.js
const rateLimit = require('express-rate-limit');
const RedisStore = require('rate-limit-redis');

const loginLimiter = rateLimit({
  store: new RedisStore({ client: redisClient }),
  windowMs: 60 * 1000, // 1 minute
  max: 5, // 5 tentatives max
  message: 'Trop de tentatives, réessayez dans 1 minute',
  standardHeaders: true,
  legacyHeaders: false,
});

// Appliquer sur route
app.post('/auth/login', loginLimiter, loginHandler);
```

- **Tests requis** : `tests/security/test_rate_limit.js`
- **Références** :
  - OWASP Brute Force Prevention
  - `templates/03-security-by-design/rate-limiting-policy.md`

**RISK-LOGIN-003 - Timing attack permettant enumération des comptes**
- **Menace** : Différence de temps de réponse révèle si email existe
- **Impact** : 3 (Liste des emails valides exposée)
- **Probabilité** : 4 (Facile à scripter)
- **Exposition** : 4 (Public)
- **Score** : 48
- **Principe violé** : III. Sécurité dès conception (Échec sécurisé)
- **Contrôle manquant** : Temps de réponse constant
- **Mitigation** : Toujours exécuter bcrypt même si email invalide

```javascript
// ❌ Vulnérable
const user = await db.findByEmail(email);
if (!user) return res.status(401).json({ error: 'Invalid credentials' }); // Retour rapide
const match = await bcrypt.compare(password, user.password_hash); // Lent

// ✅ Sécurisé
const user = await db.findByEmail(email);
const dummyHash = '$2b$12$dummyhashforconstanttiming...';
const hashToCompare = user ? user.password_hash : dummyHash;
const match = await bcrypt.compare(password, hashToCompare);
if (!user || !match) return res.status(401).json({ error: 'Invalid credentials' });
```

---

### 🟡 Risques MINEURS

**RISK-LOGIN-004 - Session fixation potentielle**
- **Menace** : Réutilisation d'un token après login
- **Impact** : 3
- **Probabilité** : 2
- **Exposition** : 3
- **Score** : 18
- **Principe violé** : III. Sécurité dès conception
- **Mitigation** : Régénérer un nouveau token à chaque login

---

## III. Sécurité dès la Conception

**Checklist des Contrôles (3/5 implémentés)**

- [x] **Moindre privilège** : Token JWT contient seulement `user_id` et `role` (pas d'infos sensibles)
  - ✅ Implémenté dans `src/auth/jwt.js:23`

- [ ] **Défense en profondeur** : Pas de rate limiting (RISK-LOGIN-002)
  - ❌ Manquant : Middleware express-rate-limit à ajouter

- [x] **Échec sécurisé** : Message d'erreur générique "Invalid credentials"
  - ✅ Implémenté dans `src/auth/login.js:67`
  - ⚠️ Mais timing attack possible (RISK-LOGIN-003)

- [ ] **Validation systématique** : Email non validé côté serveur
  - ❌ Manquant : Utiliser validator.js ou Joi schema

- [x] **Chiffrement par défaut** : HTTPS forcé en production
  - ✅ Vérifié dans `src/server.js:12` (hsts middleware)

**État Actuel** :
- Moindre privilège : Implémenté
- Défense profondeur : Absent (critique)
- Échec sécurisé : Partiel (timing)
- Validation : Absente
- Chiffrement : Implémenté

**Actions Requises (Par priorité)** :
1. Ajouter rate limiting (RISK-LOGIN-002)
2. Valider email avec Joi (`Joi.string().email()`)
3. Corriger timing attack (RISK-LOGIN-003)

**Exemples de Code** :
```javascript
// Validation des entrées avec Joi
const Joi = require('joi');

const loginSchema = Joi.object({
  email: Joi.string().email().required(),
  password: Joi.string().min(12).required()
});

app.post('/auth/login', (req, res) => {
  const { error } = loginSchema.validate(req.body);
  if (error) return res.status(400).json({ error: 'Invalid input' });
  // ...
});
```

---

## IV. Tests de Sécurité

**Checklist (0/4 implémentés)**

### SAST (Static Application Security Testing)
- [ ] Scan automatique en CI/CD
- [ ] Outil configuré : ❌ Aucun
- [ ] Build bloqué si vulnérabilité critique : Non

**Action** : Ajouter Semgrep dans `.github/workflows/security.yml`

```yaml
# .github/workflows/security.yml
name: Security Scan
on: [push, pull_request]

jobs:
  sast:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: returntocorp/semgrep-action@v1
        with:
          config: p/owasp-top-10
          severity: error  # Bloque si critique
```

### DAST (Dynamic Application Security Testing)
- [ ] Endpoints scannés en staging
- [ ] Outil configuré : ❌ Aucun
- [ ] Tests injection/XSS/CSRF : Non

**Action** : Ajouter OWASP ZAP scan dans CI

### SCA (Software Composition Analysis)
- [ ] Scan des dépendances
- [ ] Outil configuré : ❌ Aucun
- [ ] Scan quotidien : Non

**Action** : Activer Dependabot dans `.github/dependabot.yml`

```yaml
version: 2
updates:
  - package-ecosystem: "npm"
    directory: "/"
    schedule:
      interval: "daily"
    open-pull-requests-limit: 10
```

### Tests Unitaires de Sécurité
- [ ] Répertoire `tests/security/` : N'existe pas
- [ ] Tests pour risques critiques : Aucun
- [ ] Couverture : 0%

**Tests de Sécurité à Ajouter** :

```javascript
// tests/security/test_sql_injection.js
const request = require('supertest');
const app = require('../src/app');

describe('SQL Injection Protection', () => {
  test('should block SQL injection in email field', async () => {
    const response = await request(app)
      .post('/auth/login')
      .send({
        email: "' OR '1'='1'--",
        password: "test"
      });

    expect(response.status).toBe(401);
    expect(response.body.error).toBe('Invalid credentials');
  });

  test('should block SQL injection in password field', async () => {
    const response = await request(app)
      .post('/auth/login')
      .send({
        email: "test@example.com",
        password: "'; DROP TABLE users--"
      });

    expect(response.status).toBe(401);
  });
});

// tests/security/test_rate_limit.js
describe('Rate Limiting', () => {
  test('should block after 5 failed attempts', async () => {
    for (let i = 0; i < 5; i++) {
      await request(app)
        .post('/auth/login')
        .send({ email: 'test@example.com', password: 'wrong' });
    }

    const response = await request(app)
      .post('/auth/login')
      .send({ email: 'test@example.com', password: 'wrong' });

    expect(response.status).toBe(429);
    expect(response.body.error).toContain('Trop de tentatives');
  });
});
```

**Couverture Cible** : 100% des risques CRITIQUES testés (RISK-LOGIN-001, RISK-LOGIN-002, RISK-LOGIN-003)

---

## V. Gestion des Secrets

**Checklist**

- [x] Aucun secret dans le code source : Vérifié
- [x] Aucun secret dans fichiers config versionnés : `.env` dans `.gitignore`
- [x] Gestionnaire de secrets configuré : AWS Secrets Manager
  - Clé privée JWT stockée dans : `arn:aws:secretsmanager:eu-west-1:xxx:secret:jwt-private-key`
- [x] Rotation automatique : Configurée (90 jours)
- [x] Pre-commit hook actif : gitleaks configuré dans `.pre-commit-config.yaml`

**Secrets Identifiés dans cette Feature** :
- Clé privée JWT RS256 (critique)
- Clé publique JWT (peut être publique mais stockée avec privée)
- Secret Redis (si rate limiting activé)

**Actions Requises** :
- ✅ Tous les secrets sont gérés correctement
- ✅ Rotation automatique testée mensuellement

**Vérification** :
```bash
# Scan pre-commit
$ gitleaks detect --source . --verbose

# Résultat : No leaks found ✅
```

---

## VI. Traçabilité et Auditabilité

**Événements à Logger (1/3 implémentés)**

- [x] Authentification réussie
  - ✅ Implémenté dans `src/auth/login.js:89`
  - Format : JSON structuré avec user_id, ip, timestamp

- [ ] Authentification échouée (CRITIQUE - manquant !)
  - ❌ Pas de log des échecs
  - Nécessaire pour détecter brute force

- [ ] Accès aux secrets JWT
  - ❌ Pas de log lors récupération clé privée depuis Secrets Manager

**Format de Log Requis** : JSON structuré avec trace_id

**Exemple d'Implémentation** :

```javascript
// À ajouter dans src/auth/login.js
const winston = require('winston');

// Log succès (déjà implémenté)
logger.info('authentication_success', {
  timestamp: new Date().toISOString(),
  trace_id: req.id,
  user_id: user.id,
  email: user.email,  // ⚠️ PII - anonymiser en prod
  ip_address: req.ip,
  user_agent: req.get('user-agent'),
  severity: 'info'
});

// Log échec (À AJOUTER)
logger.warning('authentication_failure', {
  timestamp: new Date().toISOString(),
  trace_id: req.id,
  email_attempted: email,  // Pour investigation
  ip_address: req.ip,
  reason: 'invalid_credentials',
  attempt_count: await getFailedAttempts(req.ip),
  severity: 'warning'
});
```

**Stockage** : CloudWatch Logs (rétention 1 an)

**Rétention** : 365 jours (conforme RGPD Article 5)

**Alerting à Configurer** :
- 🔴 Plus de 10 échecs/min depuis même IP → Alerte Slack #security
- 🔴 Plus de 100 échecs/heure globalement → Alerte PagerDuty
- 🟡 Nouveau pays de connexion pour user → Email utilisateur

---

## VII. Patch Management

**Dépendances Introduites par cette Feature** :
- express@4.18.2
- bcrypt@5.1.1
- jsonwebtoken@9.0.2
- express-rate-limit@7.1.5 (si implémenté)

**Vulnérabilités Connues** :

```bash
$ npm audit

found 0 vulnerabilities ✅
```

**SLA de Correction Applicables** :
- Critique (CVSS 9-10) : 48h
- Haute (CVSS 7-8.9) : 7 jours
- Moyenne (CVSS 4-6.9) : 30 jours

**Actions Requises** :
- ✅ Dependabot configuré
- ✅ Scan quotidien actif
- ✅ Aucune vulnérabilité ouverte

---

## Références

- Constitution OpenSecKit : `constitution.md`
- Templates applicables :
  * `templates/01-threat-modeling/stride-analysis-template.md`
  * `templates/02-risk-analysis/risk-scoring-matrix.md`
  * `templates/03-security-by-design/input-validation-policy.md`
  * `templates/04-security-testing/sql-injection-test-checklist.md`
  * `templates/05-secrets-management/secrets-rotation-policy-template.md`
  * `templates/06-audit-logging/audit-log-policy-template.md`
- OWASP Top 10 :
  - A03:2021 - Injection
  - A07:2021 - Identification and Authentication Failures
- CVE : Aucune applicable
