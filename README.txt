┌─────────────────────────────────────────────────────────────────────────┐
│                                                                         │
│   ██████╗ ██████╗ ███████╗███╗   ██╗███████╗███████╗ ██████╗          │
│  ██╔═══██╗██╔══██╗██╔════╝████╗  ██║██╔════╝██╔════╝██╔════╝          │
│  ██║   ██║██████╔╝█████╗  ██╔██╗ ██║███████╗█████╗  ██║               │
│  ██║   ██║██╔═══╝ ██╔══╝  ██║╚██╗██║╚════██║██╔══╝  ██║               │
│  ╚██████╔╝██║     ███████╗██║ ╚████║███████║███████╗╚██████╗          │
│   ╚═════╝ ╚═╝     ╚══════╝╚═╝  ╚═══╝╚══════╝╚══════╝ ╚═════╝          │
│                                                                         │
│                    ██╗  ██╗██╗████████╗                                │
│                    ██║ ██╔╝██║╚══██╔══╝                                │
│                    █████╔╝ ██║   ██║                                   │
│                    ██╔═██╗ ██║   ██║                                   │
│                    ██║  ██╗██║   ██║                                   │
│                    ╚═╝  ╚═╝╚═╝   ╚═╝                                   │
│                                                                         │
│                        Version 2.0.0                                    │
│                  Security as Code, AI-Ready                             │
│                                                                         │
└─────────────────────────────────────────────────────────────────────────┘


NAME
    OpenSecKit - Framework de sécurité applicative

SYNOPSIS
    Templates + CLI + Intégration Claude Code

DESCRIPTION
    Framework complet pour intégrer la sécurité dans vos projets.

    - 40 templates markdown organisés selon 7 principes
    - CLI Rust pour automatiser la génération de contexte
    - Intégration Claude Code via slash commands
    - Agnostique du langage


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

1. DÉMARRAGE RAPIDE

   Installation :

      $ git clone https://github.com/Scttpr/OpenSecKit
      $ cd OpenSecKit/cli
      $ cargo install --path .

   Utilisation :

      $ cd /mon-projet
      $ osk init
      $ claude
      >>> /audit

   Voir QUICK-START.txt pour le guide complet.


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

2. LES 7 PRINCIPES

    I    - Modélisation des menaces
    II   - Analyse de risques
    III  - Sécurité dès la conception
    IV   - Tests de sécurité
    V    - Gestion des secrets
    VI   - Journalisation d'audit
    VII  - Patch management

   Voir constitution.md pour les détails.


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

3. COMMANDES CLI

   osk init          Initialiser + installer slash commands
   osk ingest        Export contexte (CI/CD)

   Slash commands disponibles après init :
      /audit /spec /assess /domain /context /incident

   Voir cli/README.txt pour détails.


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

4. STRUCTURE

   OpenSecKit/
   ├── cli/                   CLI Rust
   ├── templates/             Templates par principe (40 templates)
   │   ├── 01-threat-modeling/
   │   ├── 02-risk-analysis/
   │   ├── 03-security-requirements/
   │   ├── 04-security-testing/
   │   ├── 05-secrets-management/
   │   ├── 06-audit-logging/
   │   └── 07-patch-management/
   ├── domaines/              RGPD, NIS2, RGS (13 templates)
   ├── prompts/               Prompts pour Claude Code
   └── docs/                  Documentation


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

5. MODES D'UTILISATION

   5.1. Avec Claude Code (recommandé)

      $ osk init
      $ claude
      >>> /audit

      Avantages : Modifications automatiques, pas de clés API

   5.2. Export contexte

      $ osk ingest -p . -o context.txt

      Usage : Copier-coller dans n'importe quel LLM

   5.3. CI/CD

      Intégration GitHub Actions / GitLab CI pour génération automatique.
      Voir cli/README.txt section CI/CD pour exemples.

   5.4. Templates manuels

      Utiliser directement les templates markdown sans CLI.


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

6. DOMAINES SECTORIELS

   domaines/rgpd/              Conformité RGPD (UE)
   domaines/nis2/              Directive NIS2
   domaines/gouvernement-rgs/  RGS (secteur public français)

   Utilisation :

      $ claude
      >>> /domain rgpd


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

7. DOCUMENTATION

   QUICK-START.txt       Guide 10 min avec exemple e-commerce
   constitution.md      Les 7 principes détaillés
   FAQ.txt               Questions fréquentes
   cli/README.txt        Documentation CLI
   CONTRIBUTING.txt      Guide contribution


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

8. SUPPORT

   Issues       https://github.com/Scttpr/OpenSecKit/issues
   Discussions  https://github.com/Scttpr/OpenSecKit/discussions


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

9. LICENCE

   MIT License


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

   OpenSecKit v2.0.0
   https://github.com/Scttpr/OpenSecKit

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
