---
title: "Guide d'analyse des dépendances - Toutes phases"
template_version: "1.0.0"
constitutional_principle: "VII - Gestion proactive des correctifs"
ssdlc_phase: "all"
last_updated: "2025-01-15"
reviewers:
  - security-champion-team
description: "Guide d'analyse de composition logicielle (SCA) pour analyser les dépendances et identifier les vulnérabilités connues, garantissant une correction rapide."
tags:
  - sca
  - dependency-scanning
  - vulnerabilities
  - patching
difficulty: "beginner"
estimated_time: "1-2 hours"
prerequisites:
  - "Compréhension des dépendances du projet"
  - "Pipeline CI/CD configuré"
related_templates:
  - "patch-sla-policy-template.md"
  - "../security-testing/sca-integration-guide-all.md"
compliance_frameworks:
  - "PCI-DSS Requirement 6.2"
  - "NIST SSDF"
---

# Guide d'analyse des dépendances - Toutes phases

## 1. Pourquoi analyser les dépendances ?

**81% des bases de code contiennent au moins une dépendance vulnérable** (Synopsys 2024)

**Bénéfices** :
- Identifier les bibliothèques vulnérables avant déploiement
- Analyse automatisée dans le CI/CD
- Remédiation priorisée (scores CVSS)
- Conformité des licences

---

## 2. Sélection des outils

| Outil | Support langages | Avantages | Inconvénients |
|------|------------------|-----------|---------------|
| **Dependabot** (GitHub) | Nombreux | Gratuit, PRs automatiques, intégration GitHub | GitHub uniquement |
| **Snyk** | Nombreux | Convivial développeurs, guide de correction | Payant pour dépôts privés |
| **OWASP Dependency-Check** | Java, .NET, Ruby, Node.js | Gratuit, open-source | Analyses plus lentes |
| **npm audit** / **pip-audit** | JS, Python | Intégré, rapide | Spécifique à un langage |

**Recommandé** : ☑ Dependabot (GitHub) + `npm audit` / `pip-audit` dans le CI/CD

---

## 3. Configuration de GitHub Dependabot

**Activer Dependabot** (`.github/dependabot.yml`) :

```yaml
version: 2
updates:
  - package-ecosystem: "npm"
    directory: "/"
    schedule:
      interval: "weekly"
    open-pull-requests-limit: 5
    # Auto-merge des mises à jour patch
    labels:
      - "dependencies"
    reviewers:
      - "security-champions"

  - package-ecosystem: "pip"
    directory: "/"
    schedule:
      interval: "weekly"
```

**Dependabot automatiquement** :
- Analyse `package.json`, `requirements.txt` hebdomadairement
- Ouvre des PRs pour les dépendances vulnérables
- Met à jour vers des versions sécurisées

---

## 4. Intégration CI/CD

**GitHub Actions** (`.github/workflows/dependency-scan.yml`) :

```yaml
name: Analyse des vulnérabilités des dépendances

on:
  pull_request:
  push:
    branches: [main]
  schedule:
    - cron: '0 3 * * *' # Quotidien à 3h du matin

jobs:
  npm-audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-node@v4
      - run: npm audit --audit-level=high # Échoue sur high/critical

  pip-audit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: actions/setup-python@v4
      - run: pip install pip-audit
      - run: pip-audit --require-hashes --desc # Sortie détaillée
```

---

## 5. SLA de remédiation

| Sévérité | Action | Délai |
|----------|--------|-------|
| **Critique** (CVSS 9.0-10.0) | Patcher immédiatement, hotfix release | 24 heures |
| **Élevée** (CVSS 7.0-8.9) | Patcher dans le prochain sprint | 7 jours |
| **Moyenne** (CVSS 4.0-6.9) | Planifier le patch, pas urgent | 30 jours |
| **Faible** (CVSS 0.1-3.9) | Suivre, patcher de manière opportuniste | 90 jours |

---

## 6. Patching automatisé

**Auto-merge Dependabot** (pour les mises à jour patch/minor) :

```yaml
# .github/workflows/dependabot-auto-merge.yml
name: Dependabot Auto-Merge

on: pull_request

jobs:
  auto-merge:
    if: github.actor == 'dependabot[bot]'
    runs-on: ubuntu-latest
    steps:
      - uses: dependabot/fetch-metadata@v1
        id: metadata
      - if: steps.metadata.outputs.update-type == 'version-update:semver-patch'
        run: gh pr merge --auto --squash "$PR_URL"
        env:
          PR_URL: ${{github.event.pull_request.html_url}}
          GITHUB_TOKEN: ${{secrets.GITHUB_TOKEN}}
```

---

## 7. Checklist d'implémentation

- [ ] Activer Dependabot (`.github/dependabot.yml`)
- [ ] Ajouter l'analyse des dépendances au CI/CD
- [ ] Définir les SLA de remédiation
- [ ] Configurer l'auto-merge pour les mises à jour patch
- [ ] Revue hebdomadaire des PRs Dependabot
- [ ] Documenter le processus de patching

---

## Ressources associées

- [Template de SLA pour les vulnérabilités](patch-sla-policy-template.md)
- [Documentation GitHub Dependabot](https://docs.github.com/en/code-security/dependabot)
