# Tests de Sécurité (Principe IV)

## Aperçu

Ce répertoire contient les outils pour appliquer le **Principe Constitutionnel IV : Tests de sécurité**.
Les tests automatisés sont **bloquants pour le déploiement** - aucune vulnérabilité CRITIQUE ou HAUTE ne doit atteindre la production.

### Philosophie

```
Code → SAST → Build → DAST → Deploy
  ↑      ↓              ↓
  └── SCA (continu) ────┘
```

**Règle fondamentale** : Tester tôt, tester souvent, bloquer les vulnérabilités.

---

## Contenu du Kit

| Fichier | Type de Test | Phase CI/CD | Description |
|:--------|:-------------|:------------|:------------|
| **[`sast-guide.md`](sast-guide.md)** | SAST | Build | Analyse statique du code source |
| **[`dast-guide.md`](dast-guide.md)** | DAST | Staging | Tests dynamiques sur application en cours d'exécution |
| **[`sca-guide.md`](sca-guide.md)** | SCA | Continu | Analyse des dépendances et vulnérabilités connues |
| **[`regression-tests-guide.md`](regression-tests-guide.md)** | Régression | Pre-release | Tests de non-régression sécurité |

---

## Guide de Démarrage (Workflow)

### Étape 1 : SCA - Dépendances (Priorité Haute)

**Pourquoi commencer ici ?** Meilleur ROI - détecte immédiatement les CVE connues dans vos dépendances.

* Utilisez : [`sca-guide.md`](sca-guide.md)
* **Outils recommandés** :
  - JavaScript/TypeScript : `npm audit`, Snyk, Dependabot
  - Python : `pip-audit`, Safety, Snyk
  - Rust : `cargo audit`
  - Go : `govulncheck`
* **Intégration** : PR checks + scan quotidien programmé

### Étape 2 : SAST - Code Source (Priorité Haute)

**Quand ?** À chaque commit/PR pour détecter les vulnérabilités avant merge.

* Utilisez : [`sast-guide.md`](sast-guide.md)
* **Outils recommandés** :
  - Multi-langage : Semgrep, SonarQube, CodeQL
  - JavaScript : ESLint security plugins
  - Python : Bandit
* **Configuration** : Bloquer les PR avec findings CRITICAL/HIGH

### Étape 3 : DAST - Application Live (Priorité Moyenne)

**Quand ?** Sur environnement de staging avant chaque release.

* Utilisez : [`dast-guide.md`](dast-guide.md)
* **Outils recommandés** :
  - OWASP ZAP (gratuit, CI/CD ready)
  - Burp Suite (manuel + automation)
  - Nuclei (templates communautaires)
* **Configuration** : Scan automatique post-déploiement staging

### Étape 4 : Tests de Régression (Pré-Production)

**Quand ?** Avant chaque release majeure pour valider les contrôles de sécurité.

* Utilisez : [`regression-tests-guide.md`](regression-tests-guide.md)
* **Couverture** :
  - Tests d'authentification (bypass, brute force)
  - Tests d'autorisation (IDOR, privilege escalation)
  - Tests d'injection (SQLi, XSS, command injection)
  - Tests de configuration (headers, TLS, cookies)

---

## Matrice des Outils par Stack

| Stack | SAST | DAST | SCA |
|:------|:-----|:-----|:----|
| **Node.js** | Semgrep, ESLint | ZAP, Burp | npm audit, Snyk |
| **Python** | Bandit, Semgrep | ZAP, Burp | pip-audit, Safety |
| **Java** | SpotBugs, Semgrep | ZAP, Burp | OWASP Dependency-Check |
| **Go** | gosec, Semgrep | ZAP | govulncheck |
| **Rust** | cargo-clippy | ZAP | cargo audit |
| **PHP** | Psalm, PHPStan | ZAP, Burp | Composer audit |

---

## Intégration CI/CD

### GitHub Actions (Exemple)

```yaml
name: Security Tests
on: [push, pull_request]

jobs:
  sca:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run SCA
        run: npm audit --audit-level=high

  sast:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Run Semgrep
        uses: returntocorp/semgrep-action@v1
        with:
          config: p/security-audit

  dast:
    runs-on: ubuntu-latest
    needs: [deploy-staging]
    steps:
      - name: Run ZAP Scan
        uses: zaproxy/action-full-scan@v0.8.0
        with:
          target: 'https://staging.example.com'
```

### Règles de Blocage

| Sévérité | SAST | DAST | SCA |
|:---------|:-----|:-----|:----|
| **CRITICAL** | Bloque PR | Bloque release | Bloque PR |
| **HIGH** | Bloque PR | Bloque release | Bloque PR |
| **MEDIUM** | Warning | Warning | Warning |
| **LOW** | Info | Info | Info |

---

## Utilisation avec une IA (Mode Agent)

Ce kit est optimisé pour être utilisé avec des assistants IA.

**Prompt suggéré pour l'IA :**
> "Agis comme un expert en tests de sécurité. Voici ma configuration CI/CD actuelle [INSÉRER CONFIG].
>
> 1. Identifie les lacunes dans ma couverture de tests de sécurité.
> 2. Propose une configuration SAST/DAST/SCA adaptée à mon stack [STACK].
> 3. Génère les fichiers de configuration nécessaires pour GitHub Actions."

**Pour analyser un rapport de vulnérabilités :**
> "Voici le rapport de scan SAST [INSÉRER RAPPORT]. Priorise les vulnérabilités et propose des correctifs pour les 3 plus critiques."

---

## SLA de Correction

| CVSS | Sévérité | SLA Correction |
|:-----|:---------|:---------------|
| 9.0-10.0 | CRITICAL | **48 heures** |
| 7.0-8.9 | HIGH | **7 jours** |
| 4.0-6.9 | MEDIUM | **30 jours** |
| 0.1-3.9 | LOW | Backlog |

---

## Métriques à Suivre

| Métrique | Cible | Mesure |
|:---------|:------|:-------|
| Couverture SAST | 100% du code | % fichiers scannés |
| Findings CRITICAL/HIGH ouverts | 0 | Count par sprint |
| Temps moyen de correction | < SLA | Jours depuis détection |
| Faux positifs | < 10% | Ratio findings/vrais positifs |
| Dépendances vulnérables | 0 CRITICAL/HIGH | Count SCA |

---

## Ressources et Outils

* [OWASP Testing Guide](https://owasp.org/www-project-web-security-testing-guide/)
* [Semgrep Rules Registry](https://semgrep.dev/explore)
* [OWASP ZAP](https://www.zaproxy.org/)
* [Snyk Vulnerability Database](https://snyk.io/vuln/)
* [GitHub Security Advisories](https://github.com/advisories)

---

**Prochaines étapes** : Après les tests de sécurité, procéder à la [Gestion des secrets (V)](../V-secrets-management/) et la [Journalisation d'audit (VI)](../VI-audit-logging/).
