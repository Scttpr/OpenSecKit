 ██████╗ ██████╗ ███████╗███╗   ██╗███████╗███████╗ ██████╗██╗  ██╗██╗████████╗
██╔═══██╗██╔══██╗██╔════╝████╗  ██║██╔════╝██╔════╝██╔════╝██║ ██╔╝██║╚══██╔══╝
██║   ██║██████╔╝█████╗  ██╔██╗ ██║███████╗█████╗  ██║     █████╔╝ ██║   ██║
██║   ██║██╔═══╝ ██╔══╝  ██║╚██╗██║╚════██║██╔══╝  ██║     ██╔═██╗ ██║   ██║
╚██████╔╝██║     ███████╗██║ ╚████║███████║███████╗╚██████╗██║  ██╗██║   ██║
 ╚═════╝ ╚═╝     ╚══════╝╚═╝  ╚═══╝╚══════╝╚══════╝ ╚═════╝╚═╝  ╚═╝╚═╝   ╚═╝

╔═══════════════════════════════════════════════════════════════════════════╗
║                                                                           ║
║  Framework de sécurité applicative                                       ║
║  Templates + CLI + Intégration Claude Code                               ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝

                          Security as Code, AI-Ready

═══════════════════════════════════════════════════════════════════════════════

[SECTION 1] C'EST QUOI ?

Framework complet pour intégrer la sécurité dans vos projets.

    [*] 40 templates markdown organisés selon 7 principes
    [*] CLI Rust pour automatiser la génération de contexte
    [*] Intégration Claude Code via slash commands
    [*] Agnostique du langage

═══════════════════════════════════════════════════════════════════════════════

[SECTION 2] DÉMARRAGE RAPIDE

  Installation :

    $ git clone https://github.com/Scttpr/OpenSecKit
    $ cd OpenSecKit/cli
    $ cargo install --path .

  Utilisation :

    $ cd /mon-projet
    $ osk init      # Installe automatiquement les slash commands
    $ claude
    >>> /audit

  Voir QUICK-START.md pour le guide complet.

═══════════════════════════════════════════════════════════════════════════════

[SECTION 3] LES 7 PRINCIPES

┌────┬────────────────────────────────────────────────────────────────┐
│ I  │ Modélisation des menaces                                       │
│ II │ Analyse de risques                                             │
│ III│ Sécurité dès la conception                                     │
│ IV │ Tests de sécurité                                              │
│ V  │ Gestion des secrets                                            │
│ VI │ Journalisation d'audit                                         │
│ VII│ Patch management                                               │
└────┴────────────────────────────────────────────────────────────────┘

  Voir constitution.md pour les détails.

═══════════════════════════════════════════════════════════════════════════════

[SECTION 4] COMMANDES CLI

    osk init          Initialiser + installer slash commands Claude Code
    osk ingest        Export contexte (CI/CD)

  Slash commands disponibles après init :
    /audit /spec /assess /domain /context /incident

  Voir cli/README.md pour détails.

═══════════════════════════════════════════════════════════════════════════════

[SECTION 5] STRUCTURE

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

═══════════════════════════════════════════════════════════════════════════════

[SECTION 6] MODES D'UTILISATION

  [1] Avec Claude Code (recommandé)

        osk init → claude → /audit

      AVANTAGES : Modifications automatiques, pas de clés API

  [2] Export contexte

        osk ingest -p . -o context.txt

      USAGE : Copier-coller dans n'importe quel LLM

  [3] CI/CD

      Intégration GitHub Actions / GitLab CI pour génération automatique.

      Voir cli/README.md section CI/CD pour exemples complets.

  [4] Templates manuels

      Utiliser directement les templates markdown sans CLI.

═══════════════════════════════════════════════════════════════════════════════

[SECTION 7] DOMAINES SECTORIELS

    domaines/rgpd/              Conformité RGPD (UE)
    domaines/nis2/              Directive NIS2
    domaines/gouvernement-rgs/  RGS (secteur public français)

  Utilisation :

    $ claude
    >>> /domain rgpd

═══════════════════════════════════════════════════════════════════════════════

[SECTION 8] DOCUMENTATION

┌──────────────────────┬────────────────────────────────────────────────┐
│ Fichier              │ Contenu                                        │
├──────────────────────┼────────────────────────────────────────────────┤
│ QUICK-START.md       │ Guide 10 min avec exemple e-commerce          │
│ constitution.md      │ Les 7 principes détaillés                     │
│ FAQ.md               │ Questions fréquentes                          │
│ cli/README.md        │ Documentation CLI                             │
│ CONTRIBUTING.md      │ Guide contribution                            │
└──────────────────────┴────────────────────────────────────────────────┘

═══════════════════════════════════════════════════════════════════════════════

[SECTION 9] SUPPORT

    [*] Issues      : https://github.com/Scttpr/OpenSecKit/issues
    [*] Discussions : https://github.com/Scttpr/OpenSecKit/discussions

═══════════════════════════════════════════════════════════════════════════════

[SECTION 10] LICENCE

MIT License

═══════════════════════════════════════════════════════════════════════════════

╔═══════════════════════════════════════════════════════════════════════════╗
║                                                                           ║
║  OpenSecKit v2.0.0                                                        ║
║  https://github.com/Scttpr/OpenSecKit                                    ║
║                                                                           ║
║  "Security as Code, AI-Ready"                                            ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝
