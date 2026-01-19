---
title: "Configuration de la détection de secrets"
template_version: "1.0.0"
constitutional_principle: "V - Gestion des secrets"
ssdlc_phase: "implementation"
difficulty: "beginner"
estimated_time: "30 minutes"
tags: ["secrets", "gitleaks", "trufflehog", "pre-commit", "ci-cd"]
---

# Configuration de la détection de secrets

## Objectif

Empêcher tout commit contenant des secrets en clair dans le code source, en configurant une détection automatique pré-commit et en CI/CD.

**Principe** : La prévention technique élimine le risque humain. Un développeur ne devrait jamais pouvoir commiter accidentellement un secret.

---

## Prérequis

- [ ] Git installé
- [ ] Accès au référentiel du projet
- [ ] Permissions pour modifier les hooks git
- [ ] (Optionnel) Accès au pipeline CI/CD

---

## Étape 1 : Scanner le code existant

### Option A : Gitleaks (recommandé - rapide et précis)

**Installation** :

```bash
# macOS
brew install gitleaks

# Linux
wget https://github.com/gitleaks/gitleaks/releases/download/v8.18.1/gitleaks_8.18.1_linux_x64.tar.gz
tar -xzf gitleaks_8.18.1_linux_x64.tar.gz
sudo mv gitleaks /usr/local/bin/

# Windows
# Télécharger depuis https://github.com/gitleaks/gitleaks/releases
```

**Scanner tout l'historique git** :

```bash
cd /path/to/your/project
gitleaks detect --source . --verbose --report-path gitleaks-report.json
```

**Interpréter les résultats** :

```json
{
  "Description": "AWS Access Key",
  "StartLine": 42,
  "EndLine": 42,
  "StartColumn": 15,
  "EndColumn": 35,
  "Match": "AKIAIOSFODNN7EXAMPLE",
  "Secret": "AKIAIOSFODNN7EXAMPLE",
  "File": "config/aws.js",
  "Commit": "a1b2c3d4",
  "Author": "dev@example.com",
  "Date": "2025-11-01"
}
```

**Actions si secrets trouvés** :

1. **Ne pas paniquer** : Identifier chaque secret trouvé
2. **Vérifier s'il est actif** : Tester si le secret fonctionne encore
3. **Si actif** :
   - Révoquer immédiatement le secret côté provider
   - Générer un nouveau secret
   - Le stocker dans le gestionnaire de secrets
   - Supprimer de l'historique git (voir Étape 5)
4. **Si inactif/test** : Supprimer quand même de l'historique (risque de réutilisation)

---

### Option B : TruffleHog (détection d'entropie)

**Installation** :

```bash
pip install trufflehog
```

**Scanner** :

```bash
trufflehog git file://. --only-verified
```

**Avantage** : Détecte les secrets même sans pattern connu (haute entropie)
**Inconvénient** : Plus de faux positifs que gitleaks

---

## Étape 2 : Configurer le pre-commit hook

### Option A : Gitleaks en pre-commit hook (simple)

**Créer le hook** :

```bash
cat > .git/hooks/pre-commit << 'EOF'
#!/bin/sh
# Gitleaks pre-commit hook

if command -v gitleaks &> /dev/null
then
    gitleaks protect --staged --verbose
    exitCode=$?

    if [ $exitCode -eq 1 ]; then
        echo "⚠️  SECRET DÉTECTÉ - Commit bloqué"
        echo "Supprimez le secret du fichier avant de commiter"
        echo "Si c'est un faux positif, ajoutez-le au .gitleaksignore"
        exit 1
    fi
else
    echo "⚠️  gitleaks n'est pas installé - hook ignoré"
    echo "Installez avec: brew install gitleaks"
fi
EOF

chmod +x .git/hooks/pre-commit
```

**Tester** :

```bash
# Créer un fichier avec un faux secret
echo 'AWS_KEY=AKIAIOSFODNN7EXAMPLE' > test-secret.txt
git add test-secret.txt
git commit -m "test"
# Devrait être bloqué
```

---

### Option B : Framework pre-commit (multi-outils)

**Installation** :

```bash
pip install pre-commit
```

**Créer `.pre-commit-config.yaml`** :

```yaml
repos:
  - repo: https://github.com/gitleaks/gitleaks
    rev: v8.18.1
    hooks:
      - id: gitleaks

  - repo: https://github.com/pre-commit/pre-commit-hooks
    rev: v4.5.0
    hooks:
      - id: detect-private-key  # Détecte les clés SSH/RSA
      - id: check-merge-conflict
      - id: trailing-whitespace

  - repo: https://github.com/Yelp/detect-secrets
    rev: v1.4.0
    hooks:
      - id: detect-secrets
        args: ['--baseline', '.secrets.baseline']
```

**Installer les hooks** :

```bash
pre-commit install
```

**Tester** :

```bash
pre-commit run --all-files
```

**Avantages** :

- Facile à maintenir
- Standardisé dans l'équipe
- Pas besoin de modifier `.git/hooks/` manuellement

---

## Étape 3 : Gérer les faux positifs

### Créer `.gitleaksignore`

```bash
# .gitleaksignore
# Ignorer les faux positifs détectés par gitleaks

# Exemple de secret test (pas réel)
test-dummy.txt:generic-dummy:1234567890abcdef

# Clé publique (pas sensible)
*.pub

# Fixtures de test
tests/fixtures/**
```

**Format** : `fichier:type-secret:hash`

---

### Créer un baseline (detect-secrets)

```bash
# Générer un baseline des secrets existants (à migrer)
detect-secrets scan > .secrets.baseline

# Auditer le baseline
detect-secrets audit .secrets.baseline
```

**Utilité** : Permet de tracker les secrets connus à migrer sans bloquer les commits

---

## Étape 4 : Configurer la détection en CI/CD

### GitHub Actions

**Créer `.github/workflows/secrets-scan.yml`** :

```yaml
name: Secrets Detection

on:
  pull_request:
    branches: [main, develop]
  push:
    branches: [main, develop]

jobs:
  gitleaks:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
        with:
          fetch-depth: 0  # Nécessaire pour scanner tout l'historique

      - name: Run Gitleaks
        uses: gitleaks/gitleaks-action@v2
        env:
          GITHUB_TOKEN: ${{ secrets.GITHUB_TOKEN }}
```

---

### GitLab CI

**Ajouter dans `.gitlab-ci.yml`** :

```yaml
secrets-detection:
  stage: test
  image: zricethezav/gitleaks:latest
  script:
    - gitleaks detect --source . --verbose --no-git
  allow_failure: false  # Bloque le pipeline si secrets détectés
```

---

### Jenkins

**Ajouter dans `Jenkinsfile`** :

```groovy
stage('Secrets Detection') {
    steps {
        sh 'gitleaks detect --source . --verbose --report-path gitleaks-report.json'
    }
}
```

---

## Étape 5 : Supprimer les secrets de l'historique git

### Option A : BFG Repo-Cleaner (recommandé - simple)

**Installation** :

```bash
# Télécharger BFG
wget https://repo1.maven.org/maven2/com/madgag/bfg/1.14.0/bfg-1.14.0.jar
```

**Utilisation** :

```bash
# Cloner un miroir du repo
git clone --mirror git@github.com:user/repo.git

# Supprimer un fichier contenant des secrets
java -jar bfg.jar --delete-files credentials.json repo.git

# Nettoyer l'historique
cd repo.git
git reflog expire --expire=now --all
git gc --prune=now --aggressive

# Pousser les changements (ATTENTION : réécriture d'historique)
git push --force
```

---

### Option B : git filter-repo (plus puissant)

**Installation** :

```bash
pip install git-filter-repo
```

**Supprimer un fichier** :

```bash
git filter-repo --path config/secrets.yml --invert-paths
```

**Supprimer un pattern de texte** :

```bash
git filter-repo --replace-text <(echo "AKIAIOSFODNN7EXAMPLE==>REDACTED")
```

---

### ⚠️ Avertissements

1. **Communiquer à l'équipe** : Réécriture d'historique = tous doivent re-cloner
2. **Coordonner** : Faire en dehors des heures de travail
3. **Vérifier les forks** : Secrets peuvent persister dans les forks
4. **Révoquer quand même** : Même après suppression, considérer le secret compromis

---

## Étape 6 : Former l'équipe

### Bonnes pratiques à enseigner

**✅ À faire** :

- Utiliser des variables d'environnement (injectées au runtime)
- Utiliser le gestionnaire de secrets du projet
- Vérifier avant de commiter (`git diff --staged`)
- Ne jamais désactiver le pre-commit hook

**❌ À ne jamais faire** :

- Commiter un fichier `.env`
- Hardcoder une clé API dans le code
- Partager des secrets par Slack/email
- Utiliser `git commit --no-verify` (bypass le hook)

### Checklist d'onboarding nouveau développeur

- [ ] Installer gitleaks localement
- [ ] Installer les pre-commit hooks (`pre-commit install`)
- [ ] Lire la politique de gestion des secrets
- [ ] Connaître le processus en cas de commit accidentel de secret
- [ ] Avoir accès au gestionnaire de secrets du projet

---

## Étape 7 : Monitoring et alerting

### GitGuardian (SaaS - gratuit pour open source)

**Setup** :

1. Créer un compte sur <https://gitguardian.com>
2. Connecter le repo GitHub/GitLab
3. Recevoir des alertes temps réel en cas de leak

**Avantages** :

- Détection en temps réel (même si bypass du pre-commit hook)
- Dashboard centralisé
- Alertes Slack/email

---

### Gitleaks en monitoring continu

**Cron job quotidien** :

```bash
# crontab -e
0 2 * * * cd /path/to/repo && git pull && gitleaks detect --source . --verbose --report-path /var/log/gitleaks/$(date +\%Y-\%m-\%d).json && cat /var/log/gitleaks/$(date +\%Y-\%m-\%d).json | mail -s "Gitleaks Daily Report" security@example.com
```

---

## Checklist de validation

- [ ] Gitleaks installé localement par tous les développeurs
- [ ] Pre-commit hook configuré et actif
- [ ] Pipeline CI/CD bloque les PR contenant des secrets
- [ ] `.gitleaksignore` créé pour gérer les faux positifs
- [ ] Historique git scanné et nettoyé
- [ ] Équipe formée aux bonnes pratiques
- [ ] Monitoring continu en place (GitGuardian ou cron)
- [ ] Procédure de révocation documentée

---

## Métriques de succès

- **Taux de blocage** : 100% des commits avec secrets bloqués
- **Faux positifs** : < 5% des détections
- **Temps de révocation** : < 1h après détection d'un leak
- **Conformité** : 100% des développeurs ont gitleaks installé

---

## Dépannage

### "gitleaks not found" malgré installation

**Solution** :

```bash
# Vérifier le PATH
echo $PATH
which gitleaks

# Ajouter au PATH si nécessaire
export PATH="$PATH:/usr/local/bin"
```

### Faux positifs constants

**Solution** :

1. Identifier le pattern faussement détecté
2. L'ajouter au `.gitleaksignore`
3. Ou créer une configuration custom gitleaks :

```toml
# .gitleaks.toml
[allowlist]
  description = "Allowlist"
  regexes = [
    '''EXAMPLE_API_KEY''',  # Clés d'exemple
  ]
  paths = [
    '''tests/fixtures/.*''',  # Dossier de tests
  ]
```

### Pre-commit hook ignoré

**Solution** :

```bash
# Vérifier que le hook est exécutable
chmod +x .git/hooks/pre-commit

# Vérifier qu'il n'y a pas d'erreur dans le script
.git/hooks/pre-commit
```

---

## Ressources

- [Gitleaks Documentation](https://github.com/gitleaks/gitleaks)
- [TruffleHog Documentation](https://github.com/trufflesecurity/trufflehog)
- [Detect Secrets](https://github.com/Yelp/detect-secrets)
- [GitGuardian](https://www.gitguardian.com/)
- [OWASP Secrets Management](https://cheatsheetseries.owasp.org/cheatsheets/Secrets_Management_Cheat_Sheet.html)

---

**Prochaine étape** : Intégrer un gestionnaire de secrets avec [vault-integration-guide.md](vault-integration-guide.md)
