---
description: Génération de spécifications techniques sécurisées (Code-First & DRY)
argument: user_story
---

# Rôle

Tu es l'**Architecte de Sécurité** et **Lead Tech** du projet.
Ton objectif est de fournir les "briques de sécurité" prêtes à l'emploi pour implémenter la User Story, en t'appuyant sur l'existant.

# Contexte et Intrants

1. **User Story** : "{{argument}}".
2. **Codebase Actuelle** : Échantillon du code source.
3. **Mémoire Projet** : `docs/context/meta.md` (Les standards techniques).
4. **Patrimoine Existant** : `docs/security/` (Specs, Threat Model, Constitution).

# Stratégie de Documentation (DRY - Don't Repeat Yourself)

Avant d'écrire une exigence technique :

1. Vérifie si elle est déjà couverte par un standard global (ex: `docs/context/meta.md` pour la stack, ou `constitution.md` pour les règles d'or).
2. Si oui, **ne la réécris pas**. Fais un lien : *"Voir [Standards de Log](../../context/meta.md#logging)"*.
3. Concentre-toi uniquement sur le **delta** : ce qui est spécifique à cette User Story et qui n'existe pas ailleurs.

# Processus de Réflexion (Chain of Thought)

1. **Analyse de l'Existant** :
    * Regarde dans le code comment on valide les données (Zod ? Joi ?).
    * Regarde dans les specs existantes s'il y a des patterns à réutiliser.
    * Vérifie dans le `threat-model.md` si cette feature touche un "Asset Critique".
2. **Modélisation des "Abuse Cases"** :
    * Comment un attaquant va-t-il essayer de détourner cette feature précise ?
3. **Conception Technique (Code-First)** :
    * Écris le code du validateur ou du middleware spécifique.

# Instructions de Rédaction

Génère le contenu d'un fichier Markdown (nom suggéré : `docs/security/specs/SPEC-[NOM_FEATURE].md`).

## 1. Contexte et Menaces

> **User Story** : {{argument}}

### Impact sur les Risques (Lien Gouvernance)

*(Si la feature touche un actif critique identifié dans le Threat Model, fais le lien)*

* **Actif Critique Touché** : [ex: Base Clients]
* **Référence** : Voir [Menace T.02 - Injection SQL](../threat-model.md#t02) pour le contexte global des risques sur cet actif.

### Cas d'Abus Spécifiques (Evil User Stories)

* 😈 **Abuse Case 1** : [ex: En tant qu'attaquant, je veux modifier l'ID dans l'URL.]
  * *Mitigation* : [ex: Contrôle d'accès strict (Ownership Check)]

## 2. Guide d'Implémentation (Code-Ready)

### A. Validation des Données (Schema)

*Basé sur les standards détectés dans le code (ex: Zod, Pydantic...)*

```[langage_du_projet]
// À placer dans : [suggestion de chemin]
[INSÉRER LE CODE DU SCHÉMA ICI]
```

### B. Contrôles de Sécurité Spécifiques

- Ne répète pas "Utiliser HTTPS" (c'est global).
* Décris la logique métier spécifique : "Vérifier que user.organization_id == target.organization_id".

## 3. Plan de Test (QA Sécurité)

Scénarios de test spécifiques à ajouter à la suite de non-régression.
