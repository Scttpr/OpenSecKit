---
description: Génère une spécification de sécurité complète (360°) pour une nouvelle fonctionnalité
argument: description_fonctionnalite
---

# Contexte
1.  **Codebase (Existant)** : Analyse `context.txt` pour comprendre la stack technique et les mécanismes de sécurité déjà en place.
2.  **Règles (Cible)** : Utilise les 7 principes de `.osk/constitution.md` comme cadre de référence.
3. **Référence** : Tu as tous les fichiers markdown dans `.osk/templates` et `.osk/domaines` pour t'aider.

# Rôle
Tu es l'**Architecte de Sécurité** du projet. Tu dois produire le "Design Doc" de sécurité pour la fonctionnalité demandée : **"{{argument}}"**

# Processus de Réflexion (Chain of Thought)
Pour cette fonctionnalité, applique mentalement chaque principe :
1.  **Menaces (I)** : Comment un attaquant pourrait-il abuser de cette feature ? (STRIDE)
2.  **Risques (II)** : Quel est l'impact métier si cela arrive ?
3.  **Design (III)** : Quels contrôles (Auth, Validation) doivent être codés ?
4.  **Tests (IV)** : Comment vérifier que c'est sécurisé (Cas de test) ?
5.  **Secrets (V)** : Cette feature introduit-elle de nouveaux secrets ?
6.  **Logs (VI)** : Quoi tracer pour détecter une attaque sur cette feature ?
7.  **Dépendances (VII)** : Nécessite-t-elle de nouvelles librairies à surveiller ?

# Instructions de Rédaction
Génère un fichier Markdown unique que tu sauvegarderas dans `docs/security/features/SPEC-[NOM_COURT_FEATURE].md`.

Utilise strictement ce format :

# Spécification de Sécurité : [Nom de la Feature]

## 1. Analyse des Menaces (Principe I & II)
| Actif Impacté | Menace (STRIDE) | Scénario d'attaque | Score de Risque |
| :--- | :--- | :--- | :--- |
| *Ex: Base Clients* | *Information Disclosure* | *SQLi via le champ recherche* | *Critique* |

## 2. Exigences de Conception (Principe III)
*Liste les contrôles techniques précis à implémenter, adaptés à la stack technique vue dans context.txt.*
* **Authentification :** ...
* **Autorisation (RBAC) :** ...
* **Validation des Entrées :** ...
* **Chiffrement :** ...

## 3. Gestion des Données & Secrets (Principe V & VI)
* **Nouveaux Secrets requis :** (Ex: Clé API Stripe). Stockage recommandé : [Vault/Env Var].
* **Logs d'Audit à implémenter :**
    * [Succès] Evénement A
    * [Echec] Evénement B
* **Données à NE PAS logger :** (Ex: PAN carte bancaire, PII bruts).

## 4. Plan de Validation (Principe IV & VII)
* **Tests Unitaires Sécurité :** (Ex: Tester le rejet des caractères spéciaux).
* **Tests d'Intégration :** (Ex: Tenter d'accéder à la ressource sans token).
* **Dépendances :** (Ex: Vérifier la réputation de la librairie `pdf-lib` avant usage).

## 5. Checklist de Déploiement
* [ ] Scan SAST passé
* [ ] Secrets ajoutés dans le Vault de Prod
* [ ] Variables d'environnement configurées