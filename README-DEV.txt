╔══════════════════════════════════════════════════════════════════════════╗
║                       OPENSECKIT - DEV GUIDE                             ║
║                       Guide développeur                                  ║
║                          Version 3.0.1                                   ║
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


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

4. ARCHITECTURE DES PROMPTS

   Les prompts sont légers (~100 lignes) et référencent des templates :

   prompts/
   ├── osk-analyze.md       # 126 lignes
   ├── osk-configure.md     # 101 lignes
   ├── osk-dashboard.md     # 94 lignes
   └── ...

   templates/
   ├── schemas/             # Structures YAML
   ├── outputs/             # Templates fichiers générés (.tmpl)
   └── reports/             # Rapports terminaux (.txt)


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

5. AJOUTER UN NOUVEAU PROMPT

   1. Créer prompts/osk-xxx.md (court, ~100 lignes)
   2. Créer templates si besoin :
      - schemas/xxx-entry.yaml
      - outputs/xxx.md.tmpl
      - reports/xxx-report.txt
   3. Ajouter dans registry.toml
   4. Tester avec ./scripts/test-local.sh


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

NOTE
   Tout est installé localement. Rien de global.


━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
OpenSecKit Dev Guide v3.0.1
https://github.com/Scttpr/OpenSecKit

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
