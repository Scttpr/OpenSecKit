---
title: "Guide d'intégration DAST (Dynamic Application Security Testing)"
template_version: "1.0.0"
constitutional_principle: "IV - Security Testing"
ssdlc_phase: "implementation"
difficulty: "intermediate"
estimated_time: "2-3 hours"
tags: ["dast", "owasp-zap", "burp-suite", "security-testing", "ci-cd"]
---

# Guide d'intégration DAST (Dynamic Application Security Testing)

## Objectif

Configurer des tests de sécurité dynamiques (DAST) pour détecter les vulnérabilités pendant l'exécution de l'application.

**Principe** : Le DAST teste l'application en conditions réelles (boîte noire), sans accès au code source, simulant les attaques d'un attaquant externe.

---

## Différence SAST vs DAST

| Aspect | SAST (Static) | DAST (Dynamic) |
|--------|---------------|----------------|
| **Quand** | Analyse le code source | Teste l'application en exécution |
| **Accès** | Boîte blanche (code visible) | Boîte noire (comportement observable) |
| **Détecte** | Bugs de code, mauvaises pratiques | Vulnérabilités runtime, config |
| **Faux positifs** | Élevés (15-30%) | Faibles (5-10%) |
| **Exemples** | Injection SQL dans le code | Injection SQL exploitable |
| **Phase** | Développement, pre-commit | Staging, pre-production |

**Complémentarité** : SAST détecte tôt, DAST confirme l'exploitabilité. Les deux sont nécessaires.

---

## Prérequis

- [ ] Application déployée en environnement de test
- [ ] Application accessible via HTTP/HTTPS
- [ ] Credentials de test disponibles (pour scan authentifié)
- [ ] Base de données de test (données non sensibles)
- [ ] Pipeline CI/CD configuré

---

## Étape 1 : Choisir l'outil DAST

### Option A : OWASP ZAP (recommandé - open source)

**Avantages** :

- Gratuit et open source
- Facile à intégrer en CI/CD
- API complète pour automation
- Grande communauté
- Mode CLI et GUI

**Inconvénients** :

- Interface moins intuitive que Burp
- Moins de plugins que les outils commerciaux

**Quand choisir** : Projets open source, budgets limités, CI/CD automation

---

### Option B : Burp Suite Professional

**Avantages** :

- Interface très intuitive
- Scanner très puissant
- Extensions nombreuses (BApp Store)
- Support commercial

**Inconvénients** :

- Payant ($449/an/utilisateur)
- Automation limitée (Burp Suite Enterprise requis pour CI/CD)

**Quand choisir** : Tests manuels approfondis, pentests, budgets disponibles

---

### Option C : Acunetix / Netsparker (commercial)

**Avantages** :

- Très peu de faux positifs
- Interface moderne
- Scan rapide
- Support commercial

**Inconvénients** :

- Très coûteux ($4500+/an)
- Vendor lock-in

**Quand choisir** : Grandes entreprises, exigences de conformité strictes, scan de multiples applications

---

## Étape 2 : Installer OWASP ZAP

### Installation locale (pour tests manuels)

```bash
# macOS
brew install --cask owasp-zap

# Linux
wget https://github.com/zaproxy/zaproxy/releases/download/v2.14.0/ZAP_2_14_0_unix.sh
chmod +x ZAP_2_14_0_unix.sh
./ZAP_2_14_0_unix.sh

# Windows
# Télécharger depuis https://www.zaproxy.org/download/
```

### Installation Docker (pour CI/CD)

```bash
# Lancer ZAP en mode daemon
docker run -u zap -p 8080:8080 -i ghcr.io/zaproxy/zaproxy:stable zap.sh -daemon -host 0.0.0.0 -port 8080
```

---

## Étape 3 : Configuration de base

### Créer un contexte ZAP

**Contexte** : Définit le périmètre du scan (URLs incluses/exclues, authentification).

```yaml
# zap-context.yaml
env:
  contexts:
    - name: "Mon Application"
      urls:
        - "https://staging.monapp.com"
      includePaths:
        - "https://staging.monapp.com/api/.*"
        - "https://staging.monapp.com/admin/.*"
      excludePaths:
        - "https://staging.monapp.com/logout"
        - "https://staging.monapp.com/api/health"

  parameters:
    failOnError: true
    failOnWarning: false
    progressToStdout: true

jobs:
  - type: spider
    parameters:
      maxDuration: 5

  - type: passiveScan-wait

  - type: activeScan
    parameters:
      maxDuration: 10
      policy: "API-scan-minimal"
```

---

## Étape 4 : Scan non authentifié (baseline)

### Scan rapide (5-10 minutes)

```bash
# Baseline scan (passive + spider)
docker run -t ghcr.io/zaproxy/zaproxy:stable zap-baseline.py \
  -t https://staging.monapp.com \
  -r zap-report.html \
  -J zap-report.json
```

**Détecte** :

- Headers de sécurité manquants (CSP, X-Frame-Options)
- Cookies non sécurisés
- Versions de serveur exposées
- Liens HTTP en clair

---

### Scan complet (30-60 minutes)

```bash
# Full scan (active scanning)
docker run -t ghcr.io/zaproxy/zaproxy:stable zap-full-scan.py \
  -t https://staging.monapp.com \
  -r zap-report.html \
  -J zap-report.json \
  -z "-config scanner.strength=MEDIUM"
```

**Détecte en plus** :

- Injections SQL
- XSS (Cross-Site Scripting)
- Path Traversal
- CSRF
- Vulnérabilités d'authentification

---

## Étape 5 : Scan authentifié

**Pourquoi** : Tester les pages protégées par authentification (dashboard admin, APIs privées).

### Configuration pour authentification par formulaire

```yaml
# zap-context-auth.yaml
env:
  contexts:
    - name: "Authenticated App"
      urls:
        - "https://staging.monapp.com"
      authentication:
        method: "formBased"
        parameters:
          loginUrl: "https://staging.monapp.com/login"
          loginRequestData: "username={%username%}&password={%password%}"
        verification:
          method: "response"
          loggedInRegex: "\\QLogout\\E"
          loggedOutRegex: "\\QLogin\\E"
      users:
        - name: "test-user"
          credentials:
            username: "testuser@example.com"
            password: "TestPassword123!"
```

### Configuration pour authentification par token (API)

```yaml
# zap-context-jwt.yaml
env:
  contexts:
    - name: "API with JWT"
      urls:
        - "https://staging.monapp.com/api"
      authentication:
        method: "scriptBased"
        parameters:
          loginUrl: "https://staging.monapp.com/api/auth/login"
          loginRequestData: '{"email":"{%email%}","password":"{%password%}"}'
        verification:
          method: "response"
          loggedInRegex: "\\Qtoken\\E"
      users:
        - name: "api-user"
          credentials:
            email: "test@example.com"
            password: "TestPassword123!"
```

**Note** : Pour des scénarios d'authentification complexes, consultez la [documentation ZAP Authentication](https://www.zaproxy.org/docs/authentication/).

---

## Étape 6 : Intégration CI/CD

### GitHub Actions

**Créer `.github/workflows/dast.yml`** :

```yaml
name: DAST Security Scan

on:
  schedule:
    - cron: '0 2 * * *'  # Tous les jours à 2h du matin
  workflow_dispatch:  # Déclenchement manuel

jobs:
  zap-scan:
    runs-on: ubuntu-latest

    steps:
      - name: Checkout code
        uses: actions/checkout@v3

      - name: ZAP Baseline Scan
        uses: zaproxy/action-baseline@v0.10.0
        with:
          target: 'https://staging.monapp.com'
          rules_file_name: '.zap/rules.tsv'
          cmd_options: '-a'

      - name: Upload ZAP Report
        uses: actions/upload-artifact@v3
        if: always()
        with:
          name: zap-scan-report
          path: |
            report_html.html
            report_json.json

      - name: Fail on High Severity
        run: |
          HIGH_COUNT=$(jq '.site[0].alerts | map(select(.riskcode == "3")) | length' report_json.json)
          if [ "$HIGH_COUNT" -gt 0 ]; then
            echo "❌ $HIGH_COUNT vulnérabilités HIGH détectées"
            exit 1
          fi
```

---

### GitLab CI

**Ajouter dans `.gitlab-ci.yml`** :

```yaml
dast:
  stage: security
  image: ghcr.io/zaproxy/zaproxy:stable
  script:
    - zap-baseline.py -t $STAGING_URL -r zap-report.html -J zap-report.json
  artifacts:
    reports:
      dast: zap-report.json
    paths:
      - zap-report.html
  allow_failure: false
  only:
    - main
    - develop
```

---

### Jenkins

**Jenkinsfile** :

```groovy
stage('DAST Scan') {
    steps {
        sh '''
            docker run --rm \
              -v $(pwd):/zap/wrk:rw \
              ghcr.io/zaproxy/zaproxy:stable \
              zap-full-scan.py \
              -t https://staging.monapp.com \
              -r zap-report.html \
              -J zap-report.json
        '''

        publishHTML([
            reportDir: '.',
            reportFiles: 'zap-report.html',
            reportName: 'ZAP Security Report'
        ])
    }
}
```

---

## Étape 7 : Configurer les règles de scan

### Créer `.zap/rules.tsv`

**Format** : `[RISK_LEVEL]\t[RULE_ID]\t[ACTION]\t[URL_PATTERN]`

```tsv
# Ignorer les faux positifs
IGNORE 10021 IGNORE https://staging.monapp.com/health
IGNORE 10096 IGNORE https://staging.monapp.com/public/.*

# Alerter mais ne pas bloquer
WARN 10035 WARN .*
WARN 10063 WARN .*

# Bloquer le pipeline si détecté
FAIL 40018 FAIL .* # SQL Injection
FAIL 40012 FAIL .* # XSS Reflected
FAIL 40014 FAIL .* # XSS Stored
```

---

## Étape 8 : Analyser les résultats

### Interpréter le rapport ZAP

**Niveaux de risque** :

- **High (3)** : Vulnérabilité critique exploitable (SQL injection, RCE)
- **Medium (2)** : Vulnérabilité exploitable mais impact limité (XSS, CSRF)
- **Low (1)** : Mauvaise pratique (headers manquants)
- **Informational (0)** : Information (versions exposées)

### Prioriser les corrections

**Matrice de priorisation** :

| Risque | Confiance | Priorité | SLA |
|--------|-----------|----------|-----|
| High | High | P0 - Critique | 48h |
| High | Medium | P1 - Élevée | 7j |
| Medium | High | P1 - Élevée | 7j |
| Medium | Medium | P2 - Moyenne | 30j |
| Low | * | P3 - Faible | 90j |

---

## Étape 9 : Gérer les faux positifs

### Vérification manuelle

**Checklist pour chaque alerte** :

- [ ] Reproduire l'alerte manuellement
- [ ] Vérifier que la vulnérabilité est réellement exploitable
- [ ] Identifier le code source responsable
- [ ] Proposer une correction

**Exemple : SQL Injection détectée** :

```bash
# Test manuel
curl "https://staging.monapp.com/api/users?id=1' OR '1'='1"

# Si erreur SQL visible → Vulnérabilité confirmée
# Si erreur générique ou filtrage → Faux positif
```

---

### Configurer les exclusions

**Créer `.zap/exclusions.conf`** :

```ini
[exclude_urls]
# Health checks
/health
/metrics
/ping

# Static assets
/public/*
/assets/*
*.js
*.css
*.png

[exclude_params]
# Timestamps (génèrent des faux positifs)
timestamp
_t
cache_buster
```

---

## Checklist de validation

- [ ] DAST configuré en CI/CD (scan automatique)
- [ ] Scan baseline passe sans alerte HIGH
- [ ] Scan authentifié couvre toutes les pages protégées
- [ ] Faux positifs identifiés et exclus
- [ ] Rapports ZAP archivés et analysés
- [ ] Processus de correction des vulnérabilités défini
- [ ] SLA de correction respectés (HIGH < 48h)
- [ ] Équipe formée à l'interprétation des rapports

---

## Métriques de succès

- **Couverture** : > 80% des endpoints testés
- **Faux positifs** : < 10% des alertes
- **Temps de scan** : < 30 minutes en baseline, < 2h en full scan
- **Vulnérabilités HIGH** : 0 en production
- **Temps de correction** : < 48h pour HIGH, < 7j pour MEDIUM

---

## Dépannage

### Scan trop lent

**Solutions** :

- Réduire la profondeur du spider (`-config spider.maxDepth=3`)
- Limiter le nombre de threads (`-config scanner.threadPerHost=2`)
- Utiliser des policies de scan ciblées (API-scan, AJAX-scan)

### Trop de faux positifs

**Solutions** :

- Utiliser le mode "Attack" plutôt que "Insane"
- Configurer les exclusions (`.zap/rules.tsv`)
- Activer uniquement les plugins pertinents pour votre stack

### Authentification échoue

**Solutions** :

- Vérifier les logs ZAP (`zap.log`)
- Tester l'authentification manuellement via ZAP GUI
- Vérifier que les cookies/tokens sont bien propagés

---

## Ressources

- [OWASP ZAP Documentation](https://www.zaproxy.org/docs/)
- [ZAP Automation Framework](https://www.zaproxy.org/docs/automate/automation-framework/)
- [ZAP Authentication Guide](https://www.zaproxy.org/docs/authentication/)
- [Burp Suite Documentation](https://portswigger.net/burp/documentation)
- [OWASP DAST Best Practices](https://owasp.org/www-community/Vulnerability_Scanning_Tools)

---

**Prochaine étape** : Configurer le scan de dépendances avec [sca-dependency-scanning.md](sca-dependency-scanning.md)
