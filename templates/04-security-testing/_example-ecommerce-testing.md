---
title: "Exemple : tests de sécurité pour une plateforme e-commerce"
template_version: "1.0.0"
constitutional_principle: "IV - tests de sécurité continus"
project: "ShopSecure - Plateforme e-commerce"
ssdlc_phase: "implementation"
tags: ["example", "ecommerce", "security-testing", "sast", "dast", "sca"]
---

# Exemple : Tests de sécurité pour ShopSecure

**Projet** : ShopSecure - Plateforme e-commerce B2C
**Pipeline CI/CD** : GitHub Actions
**Outils** : Semgrep (SAST), OWASP ZAP (DAST), npm audit (SCA), Jest (tests régression)

---

## 1. SAST - Analyse statique du code

### Configuration Semgrep

**Fichier `.github/workflows/sast.yml`** :
```yaml
name: SAST Security Scan

on:
  pull_request:
  push:
    branches: [main, develop]

jobs:
  semgrep:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Run Semgrep
        uses: returntocorp/semgrep-action@v1
        with:
          config: >-
            p/security-audit
            p/owasp-top-ten
            p/nodejs
          generateSarif: true

      - name: Upload SARIF
        uses: github/codeql-action/upload-sarif@v2
        with:
          sarif_file: semgrep.sarif

      - name: Fail on HIGH findings
        run: |
          HIGH_COUNT=$(jq '[.runs[].results[] | select(.level=="error")] | length' semgrep.sarif)
          if [ "$HIGH_COUNT" -gt 0 ]; then
            echo "❌ $HIGH_COUNT vulnérabilités HIGH détectées"
            exit 1
          fi
```

---

### Résultats SAST typiques

**Vulnérabilités couramment détectées** :

1. **SQL Injection** (HIGH)
   - Localisation : `backend/routes/products.js`
   - Type : Concaténation directe d'entrée utilisateur dans requête SQL
   - Correction : Utiliser des requêtes paramétrées

2. **XSS via innerHTML** (HIGH)
   - Localisation : `frontend/src/components/ProductReview.js`
   - Type : Utilisation de `dangerouslySetInnerHTML` sans sanitisation
   - Correction : Sanitiser avec DOMPurify

3. **Secrets codés en dur** (CRITICAL)
   - Localisation : `backend/config/database.js`
   - Type : Clés API et mots de passe dans le code
   - Correction : Utiliser des variables d'environnement

---

## 2. DAST - Tests dynamiques

### Configuration OWASP ZAP

**Fichier `.github/workflows/dast.yml`** :
```yaml
name: DAST Security Scan

on:
  schedule:
    - cron: '0 2 * * *'  # Tous les jours à 2h
  workflow_dispatch:

jobs:
  zap-scan:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Deploy to staging
        run: |
          docker-compose -f docker-compose.staging.yml up -d
          sleep 30  # Attendre démarrage

      - name: ZAP Baseline Scan
        uses: zaproxy/action-baseline@v0.10.0
        with:
          target: 'http://localhost:3000'
          rules_file_name: '.zap/rules.tsv'
          cmd_options: '-a'

      - name: ZAP Full Scan (authenticated)
        uses: zaproxy/action-full-scan@v0.8.0
        with:
          target: 'http://localhost:3000'
          rules_file_name: '.zap/rules.tsv'
          cmd_options: '-a -j -n context.yaml'

      - name: Upload results
        uses: actions/upload-artifact@v3
        if: always()
        with:
          name: zap-reports
          path: |
            report_html.html
            report_json.json
```

---

### Contexte d'authentification ZAP

**Fichier `.zap/context.yaml`** :
```yaml
env:
  contexts:
    - name: "ShopSecure Authenticated"
      urls:
        - "http://localhost:3000"
      includePaths:
        - "http://localhost:3000/api/.*"
        - "http://localhost:3000/account/.*"
      excludePaths:
        - "http://localhost:3000/logout"
        - "http://localhost:3000/health"

      authentication:
        method: "json"
        parameters:
          loginUrl: "http://localhost:3000/api/auth/login"
          loginRequestData: '{"email":"test@example.com","password":"TestP@ssw0rd123"}'
        verification:
          method: "response"
          loggedInRegex: "\\Qtoken\\E"
          loggedOutRegex: "\\Qunauthorized\\E"

      users:
        - name: "testuser"
          credentials:
            email: "test@example.com"
            password: "TestP@ssw0rd123"

jobs:
  - type: spider
    parameters:
      maxDuration: 10

  - type: passiveScan-wait

  - type: activeScan
    parameters:
      maxDuration: 30
```

---

### Résultats DAST typiques

**Vulnérabilités couramment détectées** :

1. **CSRF manquant** (HIGH)
   - URL : `http://localhost:3000/api/orders`
   - Type : Pas de token anti-CSRF sur les endpoints sensibles
   - Correction : Implémenter middleware CSRF

2. **Configuration TLS faible** (MEDIUM)
   - URL : `https://staging.shopsecure.com`
   - Type : TLS 1.1 accepté (obsolète)
   - Correction : Configurer nginx pour TLS 1.3 uniquement

3. **Headers de sécurité manquants** (LOW)
   - Type : CSP, X-Frame-Options, HSTS manquants
   - Correction : Ajouter headers de sécurité via middleware

---

## 3. SCA - Scan des dépendances

### Configuration npm audit

**Fichier `.github/workflows/sca.yml`** :
```yaml
name: SCA Dependency Scan

on:
  pull_request:
  push:
    branches: [main]
  schedule:
    - cron: '0 3 * * *'  # Quotidien à 3h

jobs:
  npm-audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3

      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: 18

      - name: Install dependencies
        run: npm ci

      - name: Run npm audit
        run: |
          npm audit --audit-level=moderate --json > npm-audit.json
          cat npm-audit.json | jq .

      - name: Check for critical vulnerabilities
        run: |
          CRITICAL=$(jq '.metadata.vulnerabilities.critical' npm-audit.json)
          HIGH=$(jq '.metadata.vulnerabilities.high' npm-audit.json)

          if [ "$CRITICAL" -gt 0 ]; then
            echo "❌ $CRITICAL vulnérabilités CRITICAL détectées"
            exit 1
          fi

          if [ "$HIGH" -gt 5 ]; then
            echo "⚠️ $HIGH vulnérabilités HIGH détectées (limite: 5)"
            exit 1
          fi

      - name: Upload audit report
        uses: actions/upload-artifact@v3
        if: always()
        with:
          name: npm-audit-report
          path: npm-audit.json
```

---

### Résultats SCA typiques

**Vulnérabilités couramment détectées** :

1. **Prototype Pollution in lodash** (HIGH)
   - Package : `lodash@4.17.15`
   - Chemin : `express > body-parser > lodash`
   - CVSS : 7.4
   - Correction : Mise à jour vers `lodash@4.17.21`

2. **SQL Injection in sequelize** (CRITICAL)
   - Package : `sequelize@6.25.0`
   - CVSS : 9.8
   - Correction : Mise à jour urgente vers `sequelize@6.28.1`
   - SLA : 48h (criticité CRITICAL)

---

## 4. Tests de régression sécurité

### Configuration des tests

**Tests à implémenter** :

1. **Tests d'authentification**
   - Vérifier le rejet des mots de passe faibles
   - Vérifier le blocage après 5 tentatives échouées
   - Vérifier l'invalidation de session après logout

2. **Tests d'autorisation**
   - Vérifier qu'un utilisateur ne peut pas accéder aux routes admin
   - Vérifier la protection contre IDOR (accès aux ressources d'autres utilisateurs)

3. **Tests de validation d'entrées**
   - Tester les payloads SQL injection
   - Tester les payloads XSS
   - Vérifier l'échappement des données

---

## 5. Métriques de sécurité

### Dashboard de métriques

| Métrique | Cible | Actuel | Statut |
|----------|-------|--------|--------|
| **SAST - Vulnérabilités critiques** | 0 | 0 | ✅ |
| **SAST - Vulnérabilités hautes** | < 5 | 2 | ✅ |
| **DAST - Vulnérabilités critiques** | 0 | 0 | ✅ |
| **DAST - Vulnérabilités hautes** | < 3 | 1 | ✅ |
| **SCA - Dépendances critiques** | 0 | 0 | ✅ |
| **SCA - Dépendances hautes** | < 5 | 3 | ✅ |
| **Couverture tests sécurité** | > 80% | 87% | ✅ |
| **Temps de correction (CRITICAL)** | < 48h | 24h | ✅ |
| **Temps de correction (HIGH)** | < 7j | 4j | ✅ |

---

### Tendance sur 3 mois

```
Vulnérabilités détectées par mois
────────────────────────────────────
Sept    Oct     Nov
────────────────────────────────────
CRITICAL:  1 → 0 → 0  ✅
HIGH:      8 → 5 → 2  ✅
MEDIUM:   15 → 12 → 8 ✅
LOW:      25 → 22 → 18 ✅
```

---

## 6. Intégration continue

### Workflow complet

**Fichier `.github/workflows/security-pipeline.yml`** :
```yaml
name: Security Pipeline

on:
  pull_request:
  push:
    branches: [main, develop]

jobs:
  # Parallèle
  sast:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: returntocorp/semgrep-action@v1

  sca:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: npm ci && npm audit

  unit-tests:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: npm ci && npm test

  # Séquentiel après tests
  security-tests:
    needs: [sast, sca, unit-tests]
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - run: npm ci && npm run test:security

  # Nightly DAST
  dast:
    if: github.event_name == 'schedule'
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - uses: zaproxy/action-full-scan@v0.8.0
```

---

## 7. Checklist de validation

- [ ] Pipeline SAST configuré et actif
- [ ] Pipeline DAST configuré (scans quotidiens)
- [ ] Pipeline SCA configuré (Dependabot ou npm audit)
- [ ] Tests de régression sécurité implémentés
- [ ] Métriques suivies dans un dashboard
- [ ] SLA de correction définis et respectés
- [ ] Équipe formée aux outils de sécurité
- [ ] Documentation à jour

---

**Prochaine étape** : Implémenter la gestion des secrets (Principe V)
