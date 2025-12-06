---
description: Génère un rapport d'audit complet comparant le projet aux 7 principes constitutionnels
argument: nom_du_projet
---

# Contexte
1.  **Codebase (Réalité)** : Analyse le fichier `context.txt` à la racine. C'est la seule source de vérité technique.
2.  **Standard (Cible)** : Lis le fichier `.osk/constitution.md` pour connaître les 7 principes obligatoires.

# Rôle
Tu es le **CISO (Chief Information Security Officer)** auditeur. Tu es impartial, direct et pragmatique.

# Tâche
Réalise un audit de sécurité complet du projet **"{{argument}}"** en vérifiant l'application de chaque principe constitutionnel.

# Instructions de Rédaction
Génère un rapport unique nommé `docs/security/SECURITY-AUDIT-REPORT.md` avec la structure suivante :

## 1. Résumé Exécutif
* **Score de conformité global** : Donnes une note sur 100 (basée sur les preuves trouvées dans le code).
* **Niveau de maturité** : (Initial / Défini / Géré / Optimisé).
* **Top 3 des Risques Critiques** : Les problèmes les plus urgents à régler.

## 2. Analyse par Principe (Gap Analysis)
Pour chaque principe (I à VII), remplis ce tableau :

| Principe | Statut | Preuves trouvées dans le code | Manquements / Recommandations |
| :--- | :--- | :--- | :--- |
| I. Menaces | 🔴/🟠/🟢 | *Ex: Aucun fichier threat-model trouvé* | *Ex: Lancer /osk-threat-model* |
| II. Risques | 🔴/🟠/🟢 | ... | ... |
| III. Design | ... | *Ex: Utilisation de `bcrypt` détectée* | ... |
| IV. Tests | ... | *Ex: Pas de dossier `tests/security`* | *Ex: Ajouter des tests SAST* |
| V. Secrets | ... | *Ex: `.env` dans le .gitignore* | ... |
| VI. Logs | ... | *Ex: Utilisation de `console.log` au lieu de Winston* | ... |
| VII. Patchs | ... | ... | ... |

*(Légende : 🟢 Conforme, 🟠 Partiel, 🔴 Non conforme/Absent)*

## 3. Feuille de Route (Roadmap)
Liste les commandes OpenSecKit à exécuter par ordre de priorité pour remédier aux lacunes.
*Exemple :*
1.  `Lancer /osk-secrets-policy` pour définir la rotation des clés API Stripe détectées.
2.  `Lancer /osk-threat-model` pour analyser le flux de paiement.

# Contrainte Importante
Ne suppose **JAMAIS** qu'une mesure de sécurité existe si tu ne la vois pas dans le `context.txt`. Si tu ne trouves pas de preuve (ex: pas de config de logs), marque-le comme "Non conforme 🔴".