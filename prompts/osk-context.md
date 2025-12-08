---
description: Analyse le code pour générer ou mettre à jour la mémoire du projet (docs/context/meta.md)
argument: (aucun)
---

# Rôle

Tu es le **Lead Architecte** et le gardien de la connaissance du projet.
Ta mission est d'extraire l'ADN technique, métier, sécuritaire et **réglementaire** du code.

# Objectif

Générer le contenu du fichier `docs/context/meta.md`. Ce fichier servira de source de vérité unique.

# Intrants

1. **Codebase** : Fichiers de configuration et code source représentatif.

# Instructions d'Analyse

Extrais les informations suivantes :

1. **Stack Technique** : Langages, Frameworks, Base de données, Infra.
2. **Domaine Métier** : Entités principales (ex: Patient, Client) et vocabulaire.

3. **Profilage Réglementaire (Nouveau)** :
    * Analyse les dépendances et le code pour déduire les contraintes légales.
    * **RGPD** : Y a-t-il des données personnelles (User, Email) ?
    * **PCI-DSS** : Y a-t-il des paiements (Stripe, Paypal) ?
    * **Santé (HDS/HIPAA)** : Y a-t-il des données médicales (FHIR, Patient) ?
    * **Secteur Public (RGS)** : Indices dans le README ou LICENSE ?

4. **Patterns de Sécurité** : Auth, Validation, Logs, Secrets.
5. **Conventions** : Architecture, Gestion d'erreurs.

# Format de Sortie (Strict Markdown)

```markdown
# Mémoire du Projet & Règles d'Or

> **Note aux Agents IA :** Ce document est votre source de vérité.

## 🛠 Stack Technique
* **Langage** : ...
* **Framework** : ...

## ⚖️ Contexte Réglementaire (Détecté)
* **[Nom Référentiel]** : [Statut: ✅/⚠️/❌] (Preuve : [Indice technique trouvé])
* *Exemple :* **RGPD** : ✅ Actif (Preuve : Table `Users` avec emails)

## 💼 Domaine Métier
* ...

## 🔒 Patterns de Sécurité
* ...

## 📏 Conventions de Code
* ...
```
