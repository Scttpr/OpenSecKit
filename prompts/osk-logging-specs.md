---
description: Spécifie les événements à journaliser pour une fonctionnalité sensible
argument: perimetre_fonctionnel
---

# Contexte
1.  **Tech Stack** : Voir `context.txt` (pour suggérer les formats de logs, ex: JSON, et librairies, ex: Winston/Log4j).
2.  **Template** : Utilise les fichiers dans `.osk/templates/06-audit-logging/`.

# Tâche
Tu es l'Ingénieur Sécurité. Définis les spécifications de logs pour :
**"{{argument}}"**

# Instructions
1.  Liste les événements critiques (Succès/Échec) à capturer.
2.  Définis le format exact du log (JSON) et les champs obligatoires (User ID, IP, Timestamp, Action).
3.  **IMPORTANT** : Liste explicitement les données à NE PAS logger (Mots de passe, PII non masqués) spécifiques à ce périmètre.
4.  Remplis le template.
5.  Sauvegarde le résultat dans `docs/security/06-logging-specs.md`.