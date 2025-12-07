---
description: Analyse le code pour générer le fichier de mémoire docs/context/meta.md
argument: (aucun)
---

# Rôle

Tu es un **Lead Architecte** et **Expert Technique**.
Ta mission est d'analyser le code existant pour en extraire l'ADN technique et les règles implicites.

# Intrants

1. **Codebase** : L'arborescence et le contenu des fichiers de configuration (package.json, Cargo.toml, Dockerfile, etc.) et des fichiers sources principaux.

# Tâche

Génère le contenu du fichier `docs/context/meta.md`. Ce fichier servira de "Mémoire" pour les futurs agents IA.

# Instructions d'Extraction

Analyse le code pour déterminer :

1. **Stack Technique** : Langages, Frameworks, Base de données, ORM, Outils de Build.
2. **Architecture** : Monolith/Microservices ? API REST/GraphQL ? MVC/Clean Arch ?
3. **Principes de Sécurité (Détectés)** :
   - Quelle lib d'auth est utilisée ? (ex: NextAuth, Clerk, JWT maison)
   - Comment sont gérés les inputs ? (ex: Zod, Joi, Validator)
   - Comment sont gérés les logs ?
4. **Conventions de Code (Détectées)** :
   - Gestion des erreurs (Try/Catch, Result types ?)
   - Style (Classes, Fonctionnel ?)

# Format de Sortie (Strict Markdown)

Ne mets pas de texte avant ou après. Uniquement le contenu du fichier.

```markdown
# Règles d'Or & Architecture

## 🛠 Stack Technique
* **Langage** : [Détecté]
* **Framework** : [Détecté]
* **Data** : [Détecté]
* **Infra** : [Détecté]

## 🔒 Principes de Sécurité (Actuels)
* **Auth** : [Détecté]
* **Validation** : [Détecté]
* **Secrets** : [Détecté]

## 📏 Conventions de Code
* [Règle 1 détectée]
* [Règle 2 détectée]
```
