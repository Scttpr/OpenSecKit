 ███████╗ █████╗  ██████╗
 ██╔════╝██╔══██╗██╔═══██╗
 █████╗  ███████║██║   ██║
 ██╔══╝  ██╔══██║██║▄▄ ██║
 ██║     ██║  ██║╚██████╔╝
 ╚═╝     ╚═╝  ╚═╝ ╚══▀▀═╝

╔═══════════════════════════════════════════════════════════════════════════╗
║                                                                           ║
║  FAQ - Questions fréquentes                                               ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝

═══════════════════════════════════════════════════════════════════════════════

[SECTION 1] DÉMARRAGE

  Q: Par où commencer ?
  R: Suivez QUICK-START.md (10 minutes).

  Q: Dois-je remplir TOUS les templates ?
  R: Non. Minimum vital :
     - Modélisation menaces
     - Analyse risques
     - Détection secrets

  Q: Combien de temps ?
  R: Version rapide : 10-15 min
     Version complète : 1-2 jours

  Q: Quel langage ?
  R: Tous. OpenSecKit est agnostique du langage.

═══════════════════════════════════════════════════════════════════════════════

[SECTION 2] CLAUDE CODE

  Q: Quels agents IA supportés ?
  R: Claude Code uniquement pour l'exécution automatisée.
     Autres LLMs via copier-coller manuel.

  Q: Comment ça marche ?
  R: osk init → Installe slash commands dans .claude/commands/
              → /audit dans Claude Code

  Q: Mon code est envoyé où ?
  R: Nulle part. Tout reste local. Claude Code utilise votre session.

  Q: Comment mettre à jour les slash commands ?
  R: $ osk init --force

     Cela télécharge et installe les dernières versions des prompts
     depuis le repo OpenSecKit.

═══════════════════════════════════════════════════════════════════════════════

[SECTION 3] CONFORMITÉ

  Q: Comment utiliser les templates RGPD/NIS2/RGS ?
  R: $ osk init              # Installe les slash commands
     $ claude
     >>> /domain rgpd
     >>> /domain nis2
     >>> /domain rgs

     Puis suivez les templates dans domaines/{secteur}/

  Q: Puis-je combiner plusieurs domaines ?
  R: Oui. Les templates sont compatibles entre eux.

═══════════════════════════════════════════════════════════════════════════════

[SECTION 4] OUTILS

  Q: Quels outils recommandés ?
  R:
     Détection secrets    gitleaks, truffleHog
     SAST                 Semgrep, SonarQube
     DAST                 OWASP ZAP, Burp Suite
     SCA                  Dependabot, Snyk

  Q: Intégration CI/CD ?
  R: Oui, via osk ingest. Voir cli/README.md section CI/CD.

═══════════════════════════════════════════════════════════════════════════════

[SECTION 5] DÉPANNAGE

  PROBLÈME : 'osk' n'est pas reconnu
  SOLUTION :
    $ cd OpenSecKit/cli
    $ cargo install --path .
    $ osk --version

  PROBLÈME : Config introuvable
  SOLUTION :
    $ osk init

  PROBLÈME : Templates obsolètes
  SOLUTION :
    $ osk init --force

  PROBLÈME : Secret trouvé dans l'historique git
  SOLUTION :
    [1] Révoquer le secret immédiatement
    [2] Nettoyer l'historique (git-filter-repo, BFG)
    [3] Force push (coordonner avec l'équipe)
    [4] Installer gitleaks pre-commit hook

═══════════════════════════════════════════════════════════════════════════════

[SECTION 6] CONTRIBUTION

  Q: Puis-je contribuer ?
  R: Oui. Nouveaux templates, exemples, traductions, corrections.
     Voir CONTRIBUTING.md.

  Q: Comment proposer un template ?
  R:
    [1] Fork le repo
    [2] Créer template dans templates/{principe}/
    [3] Ajouter exemple _example-*.md
    [4] Tester avec projet réel
    [5] Pull request

═══════════════════════════════════════════════════════════════════════════════

[SECTION 7] SUPPORT

    [*] Issues      : https://github.com/Scttpr/OpenSecKit/issues
    [*] Discussions : https://github.com/Scttpr/OpenSecKit/discussions

═══════════════════════════════════════════════════════════════════════════════

╔═══════════════════════════════════════════════════════════════════════════╗
║                                                                           ║
║  OpenSecKit FAQ v2.0.0                                                    ║
║  https://github.com/Scttpr/OpenSecKit                                    ║
║                                                                           ║
║  "Security as Code, AI-Ready"                                            ║
║                                                                           ║
╚═══════════════════════════════════════════════════════════════════════════╝
