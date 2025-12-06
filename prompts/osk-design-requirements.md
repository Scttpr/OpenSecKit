---
description: Définit les exigences de sécurité (ASVS) pour une fonctionnalité donnée
argument: description_fonctionnalite
---

# Contexte
1.  **Architecture** : Analyse `context.txt` pour identifier les technologies (Langage, Framework, BDD).
2.  **Référence** : Utilise les templates dans `.osk/templates/03-security-requirements`.\
3. **Référence** : Utilise les fichiers de modelisation de menace et d'analyse de risque dans `docs/security/` qui font reference a cette fonctionnalite

# Tâche
Tu es l'Architecte de Sécurité. Ton objectif est de définir les contrôles de sécurité requis pour :
**"{{argument}}"**

# Instructions
1.  Sélectionne les exigences ASVS pertinentes dans le template (ex: V2 pour l'auth, V5 pour les inputs).
2.  Précise comment implémenter techniquement ces contrôles avec la stack détectée dans `context.txt` (ex: "Utiliser la librairie `bcrypt` pour Node.js").
3.  Remplis le fichier Markdown en ne gardant que les sections pertinentes.
4.  Sauvegarde le résultat dans `docs/security/`.