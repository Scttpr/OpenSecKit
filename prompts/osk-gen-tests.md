---
description: Génère un plan de tests de régression sécurité pour un risque spécifique
argument: risque_a_tester
---

# Contexte
1.  **Codebase** : Analyse `context.txt` pour comprendre comment écrire les tests (Jest, Pytest, Go test, etc.).
2.  **Modèle** : Utilise les fichiers dans `.osk/templates/04-security-testing/`.

# Tâche
Tu es l'Ingénieur QA Sécurité. Crée un plan de test automatisé pour vérifier :
**"{{argument}}"**

# Instructions
1.  Définis le scénario de test (Given/When/Then).
2.  Liste les payloads malveillants spécifiques à tester (ex: payloads SQLi adaptés à PostgreSQL si détecté).
3.  Écris un exemple de code de test adapté au langage du projet (trouvé dans `context.txt`).
4.  Remplis le template avec ces informations.
5.  Sauvegarde le résultat dans `docs/security/04-security-tests.md`.