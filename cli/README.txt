╔══════════════════════════════════════════════════════════════════════════╗
║                         OPENSECKIT - CLI                                 ║
║              CLI pour générer des slash commands                         ║
║                          Version 3.0.0                                   ║
╚══════════════════════════════════════════════════════════════════════════╝


NAME
    osk - OpenSecKit Command Line Interface

SYNOPSIS
    osk init [--force]
    osk ingest [-p PATH] [-o OUTPUT]


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

1. INSTALLATION

   $ git clone https://github.com/Scttpr/OpenSecKit
   $ cd OpenSecKit/cli
   $ cargo install --path .

   Vérification :
      $ osk --version


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

2. COMMANDES

   init      Initialiser + installer slash commands Claude Code
   ingest    Export contexte pour CI/CD ou copier-coller


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

3. WORKFLOW V3

   [1] osk init                   Initialise le projet + installe slash commands
   [2] claude                     Lancer Claude Code
   [3] /osk-configure             Configuration intelligente (analyse code)
   [4] /osk-baseline              État des lieux sécurité (projets existants)
   [5] /osk-analyze "feature"     Analyse menaces et risques
   [6] /osk-specify "feature"     Exigences de sécurité
   [7] /osk-harden "feature"      Durcissement
   [8] /osk-plan "feature"        Plan d'implémentation
   [9] /osk-tasks "feature"       Génération des tâches

   Slash commands disponibles :

      Workflow principal :
      /osk-configure      - Configuration intelligente (analyse code, domaines)
      /osk-baseline       - État des lieux sécurité (projets existants)
      /osk-analyze        - Analyse menaces et risques (Principes I & II)
      /osk-specify        - Exigences et tests (Principes III & IV)
      /osk-harden         - Durcissement (Principes V, VI & VII)
      /osk-plan           - Plan d'implémentation consolidé
      /osk-tasks          - Génération des tâches ordonnées

      Domaines réglementaires :
      /osk-rgpd           - Conformité RGPD et registre des traitements
      /osk-rgs            - Conformité RGS et EBIOS RM

      Monitoring et continuité :
      /osk-dashboard      - Tableau de bord et métriques clés
      /osk-pca-pra        - Plans de continuité et reprise


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

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
         .osk/specs/             Spécifications features

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


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

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


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

6. CONFIGURATION

   Fichier .osk/config.toml généré par osk init.

   Structure :

      [project]
      name = "mon-projet"

      [stack]
      detected = ["rust", "node"]
      custom = []

      [domains]
      active = ["rgpd", "rgs"]

      [domains.rgpd]
      enabled = true
      niveau = "standard"

      [domains.rgs]
      enabled = true
      niveau = "standard"


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

7. FICHIERS GÉNÉRÉS

   Par osk init :

   .osk/
   ├── config.toml              Configuration projet
   ├── registry.toml            Registre des commandes
   ├── constitution.md          Constitution sécurité
   ├── prompts/                  Prompts sources
   ├── templates/                Templates
   ├── domaines/                 Domaines réglementaires
   ├── memory/                   Mémoire contextuelle
   │   ├── context.md           Contexte projet (/osk-configure)
   │   └── constitution.md      Constitution pondérée (/osk-configure)
   └── specs/                    Spécifications par feature
       └── NNN-feature/         Dossier feature (/osk-analyze)
           ├── analysis.md      Analyse menaces et risques
           ├── requirements.md  Exigences de sécurité
           ├── hardening.md     Mesures de durcissement
           ├── plan.md          Plan d'implémentation
           ├── tasks.md         Tâches ordonnées
           ├── rgpd/dpia.md     Brouillon DPIA
           └── rgs/ebios.md     Brouillon EBIOS

   .claude/
   └── commands/                Slash commands installés
       ├── osk-configure.md     → /osk-configure
       ├── osk-baseline.md      → /osk-baseline
       ├── osk-analyze.md       → /osk-analyze
       ├── osk-specify.md       → /osk-specify
       ├── osk-harden.md        → /osk-harden
       ├── osk-plan.md          → /osk-plan
       ├── osk-tasks.md         → /osk-tasks
       ├── osk-rgpd.md          → /osk-rgpd
       ├── osk-rgs.md           → /osk-rgs
       ├── osk-dashboard.md     → /osk-dashboard
       └── osk-pca-pra.md       → /osk-pca-pra

   Par les slash commands (via Claude Code) :

   docs/security/
   ├── risks/                   Registre des risques
   │   └── risk-register.yaml   Fichier consolidé
   ├── rgpd/                    Documents RGPD (/osk-rgpd)
   │   ├── registre-traitements.md
   │   ├── dpia-global.md       DPIA consolidé
   │   └── procedure-violation.md
   ├── rgs/                     Documents RGS (/osk-rgs)
   │   ├── EBIOS-RM-*.md        Analyse EBIOS consolidée
   │   ├── DOSSIER-HOMOLOGATION-*.md
   │   └── MCS-*.md
   ├── continuity/              Plans PCA/PRA (/osk-pca-pra)
   │   ├── PCA-*.md
   │   └── PRA-*.md
   └── DASHBOARD.md             Tableau de bord (/osk-dashboard)


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

8. DÉPANNAGE

   Erreur : Config introuvable
      $ osk init

   Templates obsolètes
      $ osk init --force

   Plus d'aide
      $ osk --help
      $ osk <commande> --help


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
OpenSecKit CLI v3.0.0
https://github.com/Scttpr/OpenSecKit

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
