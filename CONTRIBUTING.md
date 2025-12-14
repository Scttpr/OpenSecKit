 ██████╗ ██████╗ ███╗   ██╗████████╗██████╗ ██╗██████╗ ██╗   ██╗████████╗██╗███╗   ██╗ ██████╗
██╔════╝██╔═══██╗████╗  ██║╚══██╔══╝██╔══██╗██║██╔══██╗██║   ██║╚══██╔══╝██║████╗  ██║██╔════╝
██║     ██║   ██║██╔██╗ ██║   ██║   ██████╔╝██║██████╔╝██║   ██║   ██║   ██║██╔██╗ ██║██║  ███╗
██║     ██║   ██║██║╚██╗██║   ██║   ██╔══██╗██║██╔══██╗██║   ██║   ██║   ██║██║╚██╗██║██║   ██║
╚██████╗╚██████╔╝██║ ╚████║   ██║   ██║  ██║██║██████╔╝╚██████╔╝   ██║   ██║██║ ╚████║╚██████╔╝
 ╚═════╝ ╚═════╝ ╚═╝  ╚═══╝   ╚═╝   ╚═╝  ╚═╝╚═╝╚═════╝  ╚═════╝    ╚═╝   ╚═╝╚═╝  ╚═══╝ ╚═════╝

╔═══════════════════════════════════════════════════════════════════════════╗
║                                                                           ║
║  CONTRIBUTING - Guide de contribution                                    ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝

═══════════════════════════════════════════════════════════════════════════════

[SECTION 1] SETUP

    $ git clone https://github.com/Scttpr/OpenSecKit
    $ cd OpenSecKit
    $ ./scripts/setup-dev.sh

  Installe :
    - Dépendances Node.js (local)
    - Environnement Python (local)
    - Hook pre-commit (exécute npm test)

═══════════════════════════════════════════════════════════════════════════════

[SECTION 2] WORKFLOW

    [1] Fork le repo
    [2] git checkout -b feature/ma-feature
    [3] ./scripts/setup-dev.sh
    [4] Modifications
    [5] npm test
    [6] git commit -m "feat: ma feature"
    [7] git push origin feature/ma-feature
    [8] Pull Request

═══════════════════════════════════════════════════════════════════════════════

[SECTION 3] TESTS

  TOUT TESTER :

    $ npm test

  TESTS INDIVIDUELS :

    $ npm run check:links:all           # Liens morts
    $ source .venv/bin/activate && npm run check:frontmatter  # Frontmatter YAML
    $ npm run check:rust                  # Linting Rust

═══════════════════════════════════════════════════════════════════════════════

[SECTION 4] CONVENTIONS

  TEMPLATES :

    ---
    title: "Titre"
    constitutional_principle: "I"  # I-VII
    ssdlc_phase: "planning"        # planning, design, implementation, all
    ---

  COMMITS :

    feat:     Nouvelle fonctionnalité
    fix:      Correction bug
    docs:     Documentation
    refactor: Refactoring
    test:     Tests
    chore:    Maintenance

  RUST :

    $ cargo fmt
    $ cargo clippy

═══════════════════════════════════════════════════════════════════════════════

[SECTION 5] CHECKLIST PR

    [ ] cargo build (compile sans erreur)
    [ ] npm test (tous les tests passent)
    [ ] cargo fmt (code formaté)
    [ ] cargo clippy (pas de warnings)
    [ ] Liens markdown valides
    [ ] Frontmatter YAML valide
    [ ] Commits au format conventional
    [ ] Documentation à jour
    [ ] Exemple fourni (si nouveau template)

═══════════════════════════════════════════════════════════════════════════════

[SECTION 6] SUPPORT

    [*] Issues      : https://github.com/Scttpr/OpenSecKit/issues
    [*] Discussions : https://github.com/Scttpr/OpenSecKit/discussions

═══════════════════════════════════════════════════════════════════════════════

╔═══════════════════════════════════════════════════════════════════════════╗
║                                                                           ║
║  OpenSecKit Contributing Guide v2.0.0                                     ║
║  https://github.com/Scttpr/OpenSecKit                                    ║
║                                                                           ║
║  "Security as Code, AI-Ready"                                            ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝
