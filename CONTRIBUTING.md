OpenSecKit - Contributing                                     Version 2.0.0


NAME
    Contributing - Guide de contribution


1. SETUP

   $ git clone https://github.com/Scttpr/OpenSecKit
   $ cd OpenSecKit
   $ ./scripts/setup-dev.sh

   Installe :
      - Dépendances Node.js (local)
      - Environnement Python (local)
      - Hook pre-commit (exécute npm test)


2. WORKFLOW

   [1] Fork le repo
   [2] git checkout -b feature/ma-feature
   [3] ./scripts/setup-dev.sh
   [4] Modifications
   [5] npm test
   [6] git commit -m "feat: ma feature"
   [7] git push origin feature/ma-feature
   [8] Pull Request


3. TESTS

   Tout tester :
      $ npm test

   Tests individuels :
      $ npm run check:links:all
      $ source .venv/bin/activate && npm run check:frontmatter
      $ npm run check:rust


4. CONVENTIONS

   4.1. Templates

      ---
      title: "Titre"
      constitutional_principle: "I"  # I-VII
      ssdlc_phase: "planning"        # planning, design, implementation, all
      ---

   4.2. Commits

      feat:     Nouvelle fonctionnalité
      fix:      Correction bug
      docs:     Documentation
      refactor: Refactoring
      test:     Tests
      chore:    Maintenance

   4.3. Rust

      $ cargo fmt
      $ cargo clippy


5. CHECKLIST PR

   [ ] cargo build (compile sans erreur)
   [ ] npm test (tous les tests passent)
   [ ] cargo fmt (code formaté)
   [ ] cargo clippy (pas de warnings)
   [ ] Liens markdown valides
   [ ] Frontmatter YAML valide
   [ ] Commits au format conventional
   [ ] Documentation à jour
   [ ] Exemple fourni (si nouveau template)


6. SUPPORT

   Issues       https://github.com/Scttpr/OpenSecKit/issues
   Discussions  https://github.com/Scttpr/OpenSecKit/discussions


---
OpenSecKit Contributing Guide v2.0.0
https://github.com/Scttpr/OpenSecKit
