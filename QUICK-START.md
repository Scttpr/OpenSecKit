OpenSecKit - Quick Start                                     Version 2.0.0


NAME
    Quick Start - Sécurisez votre projet en 10 minutes

OBJECTIF
    Audit de sécurité et corrections de vulnérabilités


1. APPROCHE 1 : AVEC CLAUDE CODE (5 MINUTES) - RECOMMANDÉ

   1.1. Installer

      $ git clone https://github.com/Scttpr/OpenSecKit
      $ cd OpenSecKit/cli
      $ cargo install --path .

   1.2. Initialiser

      $ cd mon-projet/
      $ osk init

   1.3. Auditer

      $ claude
      >>> /audit

   1.4. Corriger

      Claude Code affiche les vulnérabilités. Demandez-lui de les corriger :

      >>> Corrige l'injection SQL dans src/api/users.js:42

   1.5. Vérifier

      >>> /audit

   Résultat : Audit complet + corrections automatiques en 5 minutes.


2. APPROCHE 2 : MANUELLE (15 MINUTES)

   2.1. Modélisation menaces (5 min)

      $ cp templates/01-threat-modeling/stride-threat-model-template-planning.md \
           docs/security/threat-model.md

      Identifiez :
         - Actifs critiques (DB, secrets, sessions)
         - Top 3 menaces STRIDE
         - Contre-mesures

   2.2. Analyse risques (3 min)

      $ cp templates/02-risk-analysis/risk-scoring-template-planning.md \
           docs/security/risk-analysis.md

      Scorez chaque risque : Criticité × Probabilité × Exposition

   2.3. Actions immédiates (7 min)

      Sécuriser la DB : Requêtes préparées

      Installer détection secrets :

         $ brew install gitleaks
         $ gitleaks detect --source . --verbose

      Hook pre-commit :

         $ cat > .git/hooks/pre-commit << 'EOF'
         #!/bin/sh
         gitleaks protect --staged --verbose
         EOF
         $ chmod +x .git/hooks/pre-commit

      Planifier MFA : Ajouter dans le backlog


3. PROCHAINES ÉTAPES

   Cette semaine :
      [1] Configurer SAST (Semgrep/SonarQube)
      [2] Ajouter logging sécurisé
      [3] Configurer Dependabot

   Ce mois :
      [4] Implémenter MFA
      [5] Gestionnaire de secrets (Vault)
      [6] Audit complet

   Voir templates/ pour détails.


4. AIDE

   constitution.md     Les 7 principes
   FAQ.md              Questions fréquentes
   cli/README.md       Toutes les commandes


5. PROCHAINES ÉTAPES

   Avec Claude Code : /spec "description" pour vos features
   Manuel : Configurer SAST (templates/04-security-testing/)


---
OpenSecKit Quick Start v2.0.0
https://github.com/Scttpr/OpenSecKit
