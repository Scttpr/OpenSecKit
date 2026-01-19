---
title: "Template de tests de régression sécurité"
template_version: "1.0.0"
constitutional_principle: "IV - tests de sécurité continus"
ssdlc_phase: "design"
difficulty: "intermediate"
estimated_time: "2-3 hours"
tags: ["regression-testing", "security-testing", "automation", "ci-cd"]
---

# Template de tests de régression sécurité

## Métadonnées du projet

**Projet** : [Nom du projet]
**Date de création** : [AAAA-MM-JJ]
**Responsable** : [Nom du security champion]
**Dernière révision** : [AAAA-MM-JJ]
**Version** : 1.0.0

---

## Objectif

Définir une suite de tests automatisés pour vérifier que les contrôles de sécurité restent actifs après chaque modification du code.

**Principe** : "Ce qui n'est pas testé est cassé". Les tests de régression garantissent qu'une correction de bug ou une nouvelle feature ne réintroduit pas de vulnérabilités.

---

## 1. Périmètre des tests

### Tests à inclure

- [ ] Authentification et autorisation
- [ ] Validation des entrées utilisateur
- [ ] Gestion des sessions
- [ ] Protection CSRF
- [ ] Headers de sécurité HTTP
- [ ] Chiffrement des données sensibles
- [ ] Détection d'injections (SQL, XSS, Command)
- [ ] Contrôles d'accès (RBAC/ABAC)
- [ ] Rate limiting et anti-brute force
- [ ] Logging des événements de sécurité

---

## 2. Authentification et autorisation

### Test 2.1 : Accès sans authentification bloqué

**Vulnérabilité testée** : Broken Access Control (OWASP A01:2021)

**Scénario** :
```
GIVEN un utilisateur non authentifié
WHEN il tente d'accéder à /api/users/profile
THEN il reçoit une erreur 401 Unauthorized
```

**Tests à implémenter** :
- [ ] Bloquer l'accès aux routes protégées sans authentification
- [ ] Rejeter les tokens JWT expirés
- [ ] Rejeter les tokens JWT modifiés ou invalides

---

### Test 2.2 : Escalade de privilèges impossible

**Vulnérabilité testée** : Privilege Escalation

**Scénario** :
```
GIVEN un utilisateur avec rôle "user"
WHEN il tente d'accéder à /api/admin/users
THEN il reçoit une erreur 403 Forbidden
```

**Tests à implémenter** :
- [ ] Empêcher l'escalade de privilèges
- [ ] Empêcher les attaques IDOR (accès aux ressources d'autres utilisateurs)

---

## 3. Validation des entrées

### Test 3.1 : Injection SQL bloquée

**Vulnérabilité testée** : SQL Injection (OWASP A03:2021)

**Scénario** :
```
GIVEN un endpoint de recherche /api/users?search=<query>
WHEN un attaquant envoie search=' OR '1'='1
THEN l'injection est détectée et bloquée
```

**Payloads à tester** :
- `' OR '1'='1`
- `'; DROP TABLE users--`
- `' UNION SELECT * FROM passwords--`
- `admin'--`
- `1' AND '1'='1`

**Tests à implémenter** :
- [ ] Vérifier que les injections SQL sont bloquées ou sanitizées
- [ ] Vérifier qu'aucune erreur SQL n'est exposée dans la réponse

---

### Test 3.2 : XSS bloqué

**Vulnérabilité testée** : Cross-Site Scripting (OWASP A03:2021)

**Payloads à tester** :
- `<script>alert("XSS")</script>`
- `<img src=x onerror=alert("XSS")>`
- `<svg/onload=alert("XSS")>`
- `javascript:alert("XSS")`
- `<iframe src="javascript:alert('XSS')"></iframe>`

**Tests à implémenter** :
- [ ] Vérifier que les payloads XSS sont échappés ou supprimés
- [ ] Vérifier qu'aucun script malveillant n'est exécuté

---

### Test 3.3 : Path Traversal bloqué

**Vulnérabilité testée** : Path Traversal

**Payloads à tester** :
- `../../../etc/passwd`
- `..\\..\\..\\windows\\system32\\config\\sam`
- `....//....//....//etc/passwd`
- `/var/www/../../etc/passwd`

**Tests à implémenter** :
- [ ] Vérifier que les tentatives de path traversal sont bloquées
- [ ] Retourner une erreur 400 avec message approprié

---

## 4. Gestion des sessions

### Test 4.1 : Session fixation impossible

**Tests à implémenter** :
- [ ] Régénérer l'ID de session après login
- [ ] Invalider la session après logout
- [ ] Appliquer un timeout de session

---

## 5. Protection CSRF

### Test 5.1 : CSRF token requis

**Tests à implémenter** :
- [ ] Rejeter les requêtes sans token CSRF
- [ ] Accepter les requêtes avec token CSRF valide
- [ ] Rejeter les requêtes avec token CSRF invalide

---

## 6. Headers de sécurité HTTP

### Test 6.1 : Headers de sécurité présents

**Headers à vérifier** :
- [ ] `Content-Security-Policy` : politique de sécurité du contenu
- [ ] `X-Frame-Options` : protection contre le clickjacking
- [ ] `X-Content-Type-Options` : protection contre le MIME sniffing
- [ ] `Strict-Transport-Security` : application HTTPS
- [ ] `X-XSS-Protection` : protection XSS du navigateur
- [ ] `Referrer-Policy` : contrôle des informations de référence
- [ ] `Permissions-Policy` : contrôle des fonctionnalités du navigateur

**Tests à implémenter** :
- [ ] Vérifier la présence de tous les headers de sécurité
- [ ] Vérifier que les informations du serveur ne sont pas exposées
- [ ] Vérifier que `X-Powered-By` est supprimé

---

## 7. Chiffrement des données sensibles

### Test 7.1 : Mots de passe chiffrés en base

**Tests à implémenter** :
- [ ] Vérifier que les mots de passe ne sont jamais stockés en clair
- [ ] Vérifier que les mots de passe utilisent un algorithme de hachage sécurisé (bcrypt, argon2)
- [ ] Vérifier que les données PII sensibles sont chiffrées en base

---

## 8. Rate limiting et anti-brute force

### Test 8.1 : Rate limiting actif

**Tests à implémenter** :
- [ ] Bloquer les tentatives de connexion excessives (>5 en 5 minutes)
- [ ] Retourner une erreur 429 après dépassement du seuil
- [ ] Appliquer le rate limiting sur les endpoints API critiques

---

## 9. Logging des événements de sécurité

### Test 9.1 : Événements de sécurité loggés

**Tests à implémenter** :
- [ ] Logger les tentatives de connexion échouées
- [ ] Logger les tentatives d'escalade de privilèges
- [ ] Logger les tentatives d'injection détectées
- [ ] Inclure les métadonnées pertinentes (IP, timestamp, user ID)

---

## 10. Tests de non-régression

### Test 10.1 : Vulnérabilités passées ne sont pas réintroduites

**Processus** :
- [ ] Créer un registre des CVE corrigés
- [ ] Créer un test pour chaque vulnérabilité corrigée
- [ ] Documenter la date de correction et le contexte

**Exemple de registre** :
```
CVE-2023-12345: SQL injection dans recherche utilisateur (corrigé le 2023-05-15)
CVE-2023-67890: IDOR dans mise à jour profil (corrigé le 2023-06-20)
```

---

## 11. Intégration CI/CD

### Configuration GitHub Actions

```yaml
# .github/workflows/security-tests.yml
name: Security Regression Tests

on:
  pull_request:
  push:
    branches: [main, develop]

jobs:
  security-tests:
    runs-on: ubuntu-latest

    steps:
      - uses: actions/checkout@v3

      - name: Setup Node.js
        uses: actions/setup-node@v3
        with:
          node-version: 18

      - name: Install dependencies
        run: npm ci

      - name: Run security regression tests
        run: npm run test:security
        env:
          NODE_ENV: test

      - name: Upload test results
        if: always()
        uses: actions/upload-artifact@v3
        with:
          name: security-test-results
          path: coverage/

      - name: Fail on security test failures
        run: |
          if grep -q "FAIL.*security" test-results.xml; then
            echo "❌ Security tests failed"
            exit 1
          fi
```

---

## 12. Métriques de succès

**KPIs à suivre** :

- **Couverture des tests de sécurité** : > 80%
- **Temps d'exécution** : < 5 minutes
- **Taux de faux positifs** : < 5%
- **Taux de succès** : 100% (aucun test ne doit échouer en production)
- **Fréquence d'exécution** : À chaque PR + quotidien

---

## 13. Maintenance

### Quand ajouter de nouveaux tests

- [ ] Après la correction d'une vulnérabilité
- [ ] Lors de l'ajout d'une nouvelle fonctionnalité sensible
- [ ] Suite à un rapport de pentest
- [ ] Après un incident de sécurité
- [ ] Lors de l'ajout d'une nouvelle API publique

### Revue trimestrielle

- [ ] Vérifier que tous les tests passent
- [ ] Ajouter des tests pour nouvelles vulnérabilités OWASP Top 10
- [ ] Mettre à jour les payloads de test (nouvelles techniques d'attaque)
- [ ] Archiver les tests obsolètes

---

## Checklist de validation

- [ ] Au moins 1 test par catégorie OWASP Top 10
- [ ] Tests d'authentification couvrent tous les flux
- [ ] Tests d'injection couvrent SQL, XSS, Command Injection
- [ ] Tests de contrôle d'accès couvrent RBAC/IDOR
- [ ] Tests de session couvrent fixation, timeout, logout
- [ ] Tests de headers vérifient tous les headers de sécurité
- [ ] Tests de rate limiting vérifient login et API
- [ ] Tests de logging vérifient les événements critiques
- [ ] Tests intégrés en CI/CD
- [ ] Documentation à jour

---

## Ressources

- [OWASP Testing Guide](https://owasp.org/www-project-web-security-testing-guide/)
- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Jest Security Testing](https://jestjs.io/docs/getting-started)
- [Playwright Security Testing](https://playwright.dev/)
- [NIST SSDF](https://csrc.nist.gov/publications/detail/sp/800-218/final)

---

**Prochaine étape** : Intégrer le scan DAST avec [dast-integration-guide-implementation.md](dast-integration-guide-implementation.md)
