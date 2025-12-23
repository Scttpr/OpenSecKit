╔══════════════════════════════════════════════════════════════════════════╗
║                       OPENSECKIT - DEV GUIDE                             ║
║                       Guide développeur                                  ║
║                          Version 3.0.0                                   ║
╚══════════════════════════════════════════════════════════════════════════╝


NAME
    Dev Guide - Developer Guide


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

1. INSTALLATION

   $ ./scripts/setup-dev.sh

   Installe :
      - Node.js (local)
      - Python virtualenv (local)
      - Hook pre-commit


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

2. COMMANDES

   $ npm test                           # Tout tester
   $ npm run check:links:all            # Liens morts
   $ npm run check:frontmatter          # Frontmatter YAML
   $ npm run check:rust                 # Rust linting


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

3. TEST LOCAL

   $ ./scripts/test-local.sh            # Crée projet test + osk init
   $ ./scripts/test-local.sh /path      # Teste sur projet existant
   $ ./scripts/test-local.sh --clean    # Nettoie projets test


NOTE
   Tout est installé localement. Rien de global.


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
OpenSecKit Dev Guide v3.0.0
https://github.com/Scttpr/OpenSecKit

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
