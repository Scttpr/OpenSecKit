OpenSecKit - FAQ                                              Version 2.0.0


NAME
    FAQ - Questions fréquentes


1. DÉMARRAGE

   Q: Par où commencer ?
   A: Suivez QUICK-START.md (10 minutes).

   Q: Dois-je remplir TOUS les templates ?
   A: Non. Minimum vital :
      - Modélisation menaces
      - Analyse risques
      - Détection secrets

   Q: Combien de temps ?
   A: Version rapide : 10-15 min
      Version complète : 1-2 jours

   Q: Quel langage ?
   A: Tous. OpenSecKit est agnostique du langage.


2. CLAUDE CODE

   Q: Quels agents IA supportés ?
   A: Claude Code uniquement pour l'exécution automatisée.
      Autres LLMs via copier-coller manuel.

   Q: Comment ça marche ?
   A: $ osk init
      Installe slash commands dans .claude/commands/
      Puis /audit dans Claude Code

   Q: Mon code est envoyé où ?
   A: Nulle part. Tout reste local. Claude Code utilise votre session.

   Q: Comment mettre à jour les slash commands ?
   A: $ osk init --force
      Télécharge les dernières versions depuis le repo OpenSecKit.


3. CONFORMITÉ

   Q: Comment utiliser les templates RGPD/NIS2/RGS ?
   A: $ osk init
      $ claude
      >>> /domain rgpd
      >>> /domain nis2
      >>> /domain rgs

      Puis suivez les templates dans domaines/{secteur}/

   Q: Puis-je combiner plusieurs domaines ?
   A: Oui. Les templates sont compatibles entre eux.


4. OUTILS

   Q: Quels outils recommandés ?
   A: Détection secrets    gitleaks, truffleHog
      SAST                 Semgrep, SonarQube
      DAST                 OWASP ZAP, Burp Suite
      SCA                  Dependabot, Snyk

   Q: Intégration CI/CD ?
   A: Oui, via osk ingest. Voir cli/README.md section CI/CD.


5. DÉPANNAGE

   Problème : 'osk' n'est pas reconnu
   Solution :
      $ cd OpenSecKit/cli
      $ cargo install --path .
      $ osk --version

   Problème : Config introuvable
   Solution :
      $ osk init

   Problème : Templates obsolètes
   Solution :
      $ osk init --force

   Problème : Secret trouvé dans l'historique git
   Solution :
      [1] Révoquer le secret immédiatement
      [2] Nettoyer l'historique (git-filter-repo, BFG)
      [3] Force push (coordonner avec l'équipe)
      [4] Installer gitleaks pre-commit hook


6. CONTRIBUTION

   Q: Puis-je contribuer ?
   A: Oui. Nouveaux templates, exemples, traductions, corrections.
      Voir CONTRIBUTING.md.

   Q: Comment proposer un template ?
   A: [1] Fork le repo
      [2] Créer template dans templates/{principe}/
      [3] Ajouter exemple _example-*.md
      [4] Tester avec projet réel
      [5] Pull request


7. SUPPORT

   Issues       https://github.com/Scttpr/OpenSecKit/issues
   Discussions  https://github.com/Scttpr/OpenSecKit/discussions


---
OpenSecKit FAQ v2.0.0
https://github.com/Scttpr/OpenSecKit
