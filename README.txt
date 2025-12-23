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
║                        Version 3.0.1                                     ║
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
   │  ┌─────────────────────────────────────────────────────────────┐│
   │  │              POUR CHAQUE FEATURE                            ││
   │  │                                                             ││
   │  │  /osk-analyze ──▶ /osk-specify ──▶ /osk-harden             ││
   │  │       │                                                     ││
   │  │       ▼                                                     ││
   │  │  /osk-plan ──▶ /osk-tasks                                  ││
   │  └─────────────────────────────────────────────────────────────┘│
   │                     │                                           │
   │                     ▼                                           │
   │  ┌─────────────────────────────────────────────────────────────┐│
   │  │              GESTION DES RISQUES                            ││
   │  │                                                             ││
   │  │  /osk-resolve ──────────────▶ risk-register.yaml (mise à   ││
   │  │       │                        jour statut avec traçabilité)││
   │  │       ▼                                                     ││
   │  │  OUVERT → EN_COURS → RESOLU → VERIFIE                      ││
   │  │                   ↘ ACCEPTE                                 ││
   │  └─────────────────────────────────────────────────────────────┘│
   │                     │                                           │
   │                     ▼                                           │
   │  ┌─────────────────────────────────────────────────────────────┐│
   │  │              CONFORMITÉ RÉGLEMENTAIRE                       ││
   │  │                                                             ││
   │  │  /osk-rgpd ──────────────▶ docs/security/rgpd/             ││
   │  │  /osk-rgs  ──────────────▶ docs/security/rgs/              ││
   │  └─────────────────────────────────────────────────────────────┘│
   │                     │                                           │
   │                     ▼                                           │
   │  /osk-dashboard (monitoring) + /osk-pca-pra (continuité)       │
   │                                                                 │
   └─────────────────────────────────────────────────────────────────┘


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

3. COMMANDES

   PHASE 0 : CONFIGURATION

      osk init              Crée .osk/config.toml et télécharge templates
      /osk-configure        Analyse le code, détecte domaines, pondère principes
      /osk-baseline         État des lieux sécurité + STRIDE système (I & II)

   PHASE 1-4 : ANALYSE PAR FEATURE

      /osk-analyze [feature]   Principes I, II  → threats.md, risks.md
      /osk-specify [feature]   Principes III, IV → requirements.md, testing.md
      /osk-harden [feature]    Principes V-VII → hardening.md
      /osk-plan [feature]      Tous → plan.md
      /osk-tasks [feature]     → tasks.md, tasks.yaml

   PHASE 5 : GESTION DES RISQUES

      /osk-resolve [RISK-ID]   Marquer un risque résolu (commit, PR, contrôles)
      /osk-resolve --reopen    Ré-ouvrir un risque fermé
      /osk-resolve --bulk      Résolution en masse

   PHASE 6 : CONFORMITÉ

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
   │   │       ├── risks.md            # Vue générée (risk-register = source)
   │   │       ├── requirements.md
   │   │       ├── testing.md
   │   │       ├── hardening.md
   │   │       ├── plan.md
   │   │       └── tasks.md
   │   └── templates/                  # Templates téléchargés
   │       ├── schemas/                # Schémas YAML
   │       ├── outputs/                # Templates fichiers générés
   │       └── reports/                # Rapports terminaux
   │
   └── docs/security/                  # Documentation finale (publiable)
       ├── risks/
       │   └── risk-register.yaml      # ★ SOURCE UNIQUE DES RISQUES ★
       ├── rgpd/
       │   ├── registre-traitements.md
       │   ├── dpia-global.md
       │   └── AUDIT-YYYY-MM-DD.md
       ├── rgs/
       │   ├── EBIOS-RM-[SYSTEME].md
       │   ├── DOSSIER-HOMOLOGATION.md
       │   └── AUDIT-YYYY-MM-DD.md
       └── continuity/
           ├── PCA-*.md
           └── PRA-*.md


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

5. RISK REGISTER - SOURCE UNIQUE

   Le fichier docs/security/risks/risk-register.yaml est la SOURCE UNIQUE
   pour tous les risques. Les fichiers .osk/specs/*/risks.md sont des VUES
   générées automatiquement.

   WORKFLOW DE RÉSOLUTION :

      OUVERT ────────▶ EN_COURS ────────▶ RESOLU ────────▶ VERIFIE
                              │
                              └──────────▶ ACCEPTE (risque accepté)

   Chaque transition est tracée avec :
   - Commit/PR de la correction
   - Contrôles implémentés
   - Date et auteur
   - Justification (si accepté)

   Commande : /osk-resolve RISK-AUTH-001


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

6. LES 7 PRINCIPES

   I    Threat Modeling      Analyse proactive des menaces (STRIDE)
   II   Risk Analysis        Scoring et priorisation des risques
   III  Security by Design   Exigences de sécurité dès la conception
   IV   Security Testing     Tests SAST/DAST/SCA automatisés
   V    Secrets Management   Aucun secret dans le code
   VI   Audit Logging        Logs immuables et centralisés
   VII  Patch Management     SLA stricts de mise à jour


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

7. ARCHITECTURE TEMPLATES

   Les prompts (skills) sont légers et référencent des templates :

   templates/
   ├── schemas/              # Schémas YAML (structures de données)
   │   ├── risk-register.yaml    # Structure du registre central
   │   ├── risk-entry.yaml       # Format d'un risque
   │   ├── requirement-entry.yaml
   │   ├── task-entry.yaml
   │   └── ...
   ├── outputs/              # Templates de fichiers générés
   │   ├── threats.md.tmpl
   │   ├── risks.md.tmpl
   │   ├── plan.md.tmpl
   │   └── ...
   └── reports/              # Rapports terminaux
       ├── analyze-report.txt
       ├── dashboard-report.txt
       └── ...


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

8. DOMAINES RÉGLEMENTAIRES

   RGPD     Protection des données personnelles       /osk-rgpd
   RGS      Référentiel Général de Sécurité (France)  /osk-rgs
   NIS2     Directive cybersécurité UE                (en cours)


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

9. LIENS

   constitution.md         Les 7 principes détaillés
   cli/README.txt          Documentation du CLI
   templates/README.md     Architecture des templates
   CONTRIBUTING.txt        Guide de contribution
   domaines/README.md      Domaines réglementaires
   CHANGELOG.txt           Historique des versions


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
OpenSecKit v3.0.1
https://github.com/Scttpr/OpenSecKit

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
