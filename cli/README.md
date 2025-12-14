OpenSecKit CLI                                                Version 2.0.0


NAME
    osk - CLI pour générer des slash commands Claude Code

SYNOPSIS
    osk init [--force]
    osk ingest [-p PATH] [-o OUTPUT]


1. INSTALLATION

   $ git clone https://github.com/Scttpr/OpenSecKit
   $ cd OpenSecKit/cli
   $ cargo install --path .

   Vérification :
      $ osk --version


2. COMMANDES

   init      Initialiser + installer slash commands Claude Code
   ingest    Export contexte pour CI/CD ou copier-coller


3. WORKFLOW TYPIQUE

   [1] osk init              Initialise + installe slash commands
   [2] claude                Lancer Claude Code
   [3] /audit                Exécuter l'audit de sécurité
   [4] /spec "..."           Analyser une user story
   [5] /assess "..."         Évaluer conformité
   [6] /domain rgpd          Vérifier conformité RGPD

   Slash commands disponibles :
      /audit /spec /assess /domain /context /incident


4. USAGE DÉTAILLÉ

   4.1. init

      $ osk init              # Installation initiale
      $ osk init --force      # Mise à jour forcée

      Initialise le projet :
         - Crée la structure .osk/ et .claude/commands/
         - Télécharge templates, prompts et domaines
         - Installe les slash commands pour Claude Code
         - Détecte automatiquement la stack technique

      Options :
         --force / -f : Force la mise à jour des ressources

      Slash commands disponibles après init :
         /audit    - Audit de sécurité complet
         /spec     - Analyse specs pour user story
         /assess   - Évaluation conformité 7 principes
         /domain   - Conformité sectorielle (RGPD, NIS2, RGS)
         /context  - Extraction ADN technique
         /incident - Gestion de crise

      Mise à jour des slash commands :
         $ osk init --force

      Impact de --force :

      Écrasé (mis à jour) :
         .osk/prompts/           Prompts sources
         .osk/templates/         Templates
         .osk/domaines/          Domaines sectoriels
         .claude/commands/       Slash commands
         .osk/config.toml        Configuration (re-demandée)

      Préservé (jamais touché) :
         docs/security/          Vos documents de sécurité
         Code source du projet
         Historique git
         .osk/memory/            Mémoire conversations

      Attention : Si vous avez modifié manuellement les slash commands dans
      .claude/commands/, ces modifications seront perdues. Versionnez-les
      dans git si nécessaire.

   4.2. ingest

      $ osk ingest [options]

      Export du contexte pour CI/CD ou copier-coller.

      Options :
         -p, --path <chemin>     Chemin spécifique à ingérer
         -o, --output <fichier>  Fichier de sortie (défaut: stdout)

      Exemples :
         $ osk ingest > context.txt
         $ osk ingest -p src/ > context-src.txt


5. INTÉGRATION CI/CD

   Génération automatique du contexte dans votre pipeline.

   5.1. GitHub Actions

      Fichier : .github/workflows/security-context.yml

      name: Security Context
      on: [push, pull_request]

      jobs:
        generate:
          runs-on: ubuntu-latest
          steps:
            - uses: actions/checkout@v3
            - uses: actions-rs/toolchain@v1
            - run: cargo install --git https://github.com/Scttpr/OpenSecKit --path cli
            - run: osk ingest > security-context.txt
            - uses: actions/upload-artifact@v3
              with:
                name: security-context
                path: security-context.txt

   5.2. GitLab CI

      Fichier : .gitlab-ci.yml

      security-context:
        stage: security
        image: rust:latest
        script:
          - cargo install --git https://github.com/Scttpr/OpenSecKit --path cli
          - osk ingest > security-context.txt
        artifacts:
          paths: [security-context.txt]
          expire_in: 30 days

   Cas d'usage :
      - Traçabilité : un contexte par commit
      - Audit externe sans accès au code
      - Revue de sécurité continue


6. CONFIGURATION

   Fichier .osk/config.toml généré par osk init.

   Structure :

      [project]
      name = "mon-projet"
      stack = ["rust", "node"]

      [memory]
      max_tokens = 100000


7. FICHIERS GÉNÉRÉS

   .osk/
   ├── config.toml              Configuration
   ├── templates/               Templates téléchargés
   └── prompts/                 Prompts sources

   .claude/
   └── commands/                Slash commands installés par init
       ├── osk-audit.md         → /audit
       ├── osk-spec.md          → /spec
       ├── osk-assess.md        → /assess
       ├── osk-domain.md        → /domain
       ├── osk-context.md       → /context
       └── osk-incident.md      → /incident


8. DÉPANNAGE

   Erreur : Config introuvable
      $ osk init

   Templates obsolètes
      $ osk init --force

   Plus d'aide
      $ osk --help
      $ osk <commande> --help


---
OpenSecKit CLI v2.0.0
https://github.com/Scttpr/OpenSecKit
