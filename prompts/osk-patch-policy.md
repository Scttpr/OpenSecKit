---
description: Établit la politique de SLA pour les correctifs de sécurité
argument: perimetre
---

# Contexte
1.  **Dépendances** : Analyse `context.txt` (package.json, go.mod, requirements.txt) pour évaluer la surface d'attaque.
2.  **Template** : Utilise les fichiers dans `.osk/templates/07-patch-management/`.

# Tâche
Tu es le CISO. Définis les SLA de correction des vulnérabilités pour :
**"{{argument}}"**

# Instructions
1.  Définis les délais de correction (SLA) pour les criticités (Critique, Élevée, Moyenne, Faible).
2.  Adapte ces délais si le projet est exposé publiquement (info visible dans `context.txt`).
3.  Décris le processus de "Dérogation" si un patch ne peut pas être appliqué.
4.  Remplis le template.
5.  Sauvegarde le résultat dans `docs/security/`.