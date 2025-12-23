╔══════════════════════════════════════════════════════════════════════════╗
║                                                                          ║
║   ██████╗ ██████╗ ███████╗███╗   ██╗███████╗███████╗ ██████╗            ║
║  ██╔═══██╗██╔══██╗██╔════╝████╗  ██║██╔════╝██╔════╝██╔════╝            ║
║  ██║   ██║██████╔╝█████╗  ██╔██╗ ██║███████╗█████╗  ██║                 ║
║  ██║   ██║██╔═══╝ ██╔══╝  ██║╚██╗██║╚════██║██╔══╝  ██║                 ║
║  ╚██████╔╝██║     ███████╗██║ ╚████║███████║███████╗╚██████╗            ║
║   ╚═════╝ ╚═╝     ╚══════╝╚═╝  ╚═══╝╚══════╝╚══════╝ ╚═════╝            ║
║                                                                          ║
║                    ██╗  ██╗██╗████████╗                                  ║
║                    ██║ ██╔╝██║╚══██╔══╝                                  ║
║                    █████╔╝ ██║   ██║                                     ║
║                    ██╔═██╗ ██║   ██║                                     ║
║                    ██║  ██╗██║   ██║                                     ║
║                    ╚═╝  ╚═╝╚═╝   ╚═╝                                     ║
║                                                                          ║
║                        Version 3.0.0                                     ║
║                  Security as Code, AI-Ready                              ║
║                                                                          ║
╚══════════════════════════════════════════════════════════════════════════╝


NAME
    OpenSecKit - Framework de sécurité applicative

SYNOPSIS
    CLI Rust + Templates + Intégration Claude Code

DESCRIPTION
    OpenSecKit structure la sécurité via 7 principes fondamentaux
    et des workflows guidés par IA.


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

1. INSTALLATION

   # CLI Rust
   $ cargo install osk

   # Initialiser un projet
   $ cd mon-projet/
   $ osk init


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

2. WORKFLOW PRINCIPAL

   ┌─────────────────────────────────────────────────────────────────┐
   │                      WORKFLOW OPENSECKIT V3                      │
   ├─────────────────────────────────────────────────────────────────┤
   │                                                                 │
   │  osk init ──▶ /osk-configure ──▶ /osk-baseline (projet existant)│
   │                     │                                           │
   │                     ▼                                           │
   │  ┌─────────────────────────────────────────────────────────┐    │
   │  │              POUR CHAQUE FEATURE                         │    │
   │  │                                                         │    │
   │  │  /osk-analyze ──▶ /osk-specify ──▶ /osk-harden          │    │
   │  │       │                                                 │    │
   │  │       ▼                                                 │    │
   │  │  /osk-plan ──▶ /osk-tasks                               │    │
   │  └─────────────────────────────────────────────────────────┘    │
   │                     │                                           │
   │                     ▼                                           │
   │  ┌─────────────────────────────────────────────────────────┐    │
   │  │              CONFORMITÉ RÉGLEMENTAIRE                    │    │
   │  │                                                         │    │
   │  │  /osk-rgpd ──────────────▶ docs/security/rgpd/          │    │
   │  │  /osk-rgs  ──────────────▶ docs/security/rgs/           │    │
   │  └─────────────────────────────────────────────────────────┘    │
   │                     │                                           │
   │                     ▼                                           │
   │  /osk-dashboard (monitoring) + /osk-pca-pra (continuité)        │
   │                                                                 │
   └─────────────────────────────────────────────────────────────────┘


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

3. COMMANDES

   PHASE 0 : CONFIGURATION

      osk init              Crée .osk/config.toml et télécharge templates
      /osk-configure        Analyse le code, détecte domaines, pondère principes
      /osk-baseline         État des lieux sécurité (projets existants)

   PHASE 1-4 : ANALYSE PAR FEATURE

      /osk-analyze [feature]   Principes I, II  → threats.md, risks.md
      /osk-specify [feature]   Principes III, IV → requirements.md, testing.md
      /osk-harden [feature]    Principes V-VII → hardening.md
      /osk-plan [feature]      Tous → plan.md
      /osk-tasks [feature]     → tasks.md, tasks.yaml

   PHASE 5 : CONFORMITÉ

      /osk-rgpd             RGPD : Registre Art. 30, DPIA, procédures
      /osk-rgpd audit       RGPD : Rapport d'audit conformité
      /osk-rgs              RGS : EBIOS RM, contexte homologation
      /osk-rgs renew        RGS : Rapport de ré-homologation

   UTILITAIRES

      /osk-dashboard        Vue consolidée de la posture sécurité
      /osk-pca-pra          Plans de Continuité et Reprise d'Activité


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

4. STRUCTURE DES FICHIERS

   projet/
   ├── .osk/                           # Espace de travail interne
   │   ├── config.toml                 # Configuration (source de vérité)
   │   ├── memory/
   │   │   ├── context.md              # Faits techniques détectés
   │   │   └── constitution.md         # Principes pondérés
   │   ├── specs/                      # Brouillons par feature
   │   │   └── NNN-feature/
   │   │       ├── threats.md
   │   │       ├── risks.md
   │   │       ├── requirements.md
   │   │       ├── testing.md
   │   │       ├── hardening.md
   │   │       ├── plan.md
   │   │       ├── rgpd/dpia.md        # Brouillon DPIA
   │   │       └── rgs/ebios.md        # Brouillon EBIOS
   │   └── templates/                  # Templates téléchargés
   │
   └── docs/security/                  # Documentation finale (publiable)
       ├── risks/
       │   └── risk-register.yaml      # Registre central des risques
       ├── rgpd/
       │   ├── registre-traitements.md # Art. 30
       │   ├── dpia-global.md          # DPIA consolidé
       │   ├── procedure-violation.md  # Art. 33-34
       │   └── AUDIT-YYYY-MM-DD.md
       ├── rgs/
       │   ├── EBIOS-RM-[SYSTEME].md   # EBIOS consolidé
       │   ├── DOSSIER-HOMOLOGATION.md
       │   └── AUDIT-YYYY-MM-DD.md
       └── continuity/
           ├── PCA-*.md
           └── PRA-*.md


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

5. LES 7 PRINCIPES

   I    Threat Modeling      Analyse proactive des menaces (STRIDE)
   II   Risk Analysis        Scoring et priorisation des risques
   III  Security by Design   Exigences de sécurité dès la conception
   IV   Security Testing     Tests SAST/DAST/SCA automatisés
   V    Secrets Management   Aucun secret dans le code
   VI   Audit Logging        Logs immuables et centralisés
   VII  Patch Management     SLA stricts de mise à jour


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

6. DOMAINES RÉGLEMENTAIRES

   RGPD     Protection des données personnelles       /osk-rgpd
   RGS      Référentiel Général de Sécurité (France)  /osk-rgs
   NIS2     Directive cybersécurité UE                (en cours)


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

7. LIENS

   constitution.md         Les 7 principes détaillés
   cli/README.txt          Documentation du CLI
   CONTRIBUTING.txt        Guide de contribution
   domaines/README.md      Domaines réglementaires


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
OpenSecKit v3.0.0
https://github.com/Scttpr/OpenSecKit

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
