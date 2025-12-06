---
description: Effectue une analyse de risque (Scoring) pour un scénario donné
argument: scenario_a_analyser
---

# Contexte
1.  **Architecture** : Voir `context.txt`.
2.  **Matrice de référence** : Voir `.osk/templates/02-risk-analysis/risk-scoring-template-planning.md`.

# Tâche
Tu es l'Analyste de Risques. Évalue le risque pour le scénario suivant :
**"{{argument}}"**

# Instructions
1.  Détermine la Criticité, la Probabilité et l'Exposition en te basant sur la stack technique (ex: est-ce exposé sur internet ? est-ce une donnée critique ?).
2.  Calcule le score final selon la formule du template.
3.  Génère le rapport au format Markdown en utilisant le template fourni.
5. Sauvegarde le résultat dans le dossier `docs/security/`.