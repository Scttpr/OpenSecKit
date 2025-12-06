---
description: Génère un modèle de menaces (STRIDE) pour une nouvelle fonctionnalité
argument: description_feature
---

# Contexte
1.  **Architecture existante** : Lis le fichier `context.txt` à la racine pour comprendre la stack technique et les flux actuels.
2.  **Template** : Lis les fichiers dans le dossier `.osk/templates/01-threat-modeling`.

# Tâche
Tu es l'Architecte de Sécurité. Ton objectif est de remplir le template de modélisation des menaces pour la fonctionnalité suivante :
**"{{argument}}"**

# Instructions
1.  Analyse l'impact de cette nouvelle feature sur l'architecture existante (décrite dans `context.txt`).
2.  Identifie les flux de données (DFD) impactés.
3.  Applique la méthode STRIDE pour trouver les menaces spécifiques à cette feature.
4.  Remplis le template Markdown. Ne laisse aucun champ vide (utilise des hypothèses réalistes basées sur `context.txt` si nécessaire).
5. Sauvegarde le résultat dans le dossier `docs/security/` avec un nom de fichier explicite vis a vis de l'argument.