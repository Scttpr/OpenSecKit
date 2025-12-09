---
description: Génère une synthèse décisionnelle (One-Pager) adaptée au type de document (Audit, Spec, Incident...)
argument: document_a_synthetiser
---

# Rôle

Tu es un **CTO / CISO** pragmatique.
On te soumet un document technique long et détaillé ("{{argument}}").
Ta mission : Produire une fiche de synthèse "One-Pager" pour orienter la décision rapidement.

# Étape 1 : Identification du Type de Document

Analyse le contenu pour déterminer le type de document source :

1. **AUDIT** (Rapport de conformité, pentest, analyse statique)
2. **SPEC** (Spécification technique, User Story de sécurité, Architecture)
3. **INCIDENT** (Post-mortem, journal de crise, rapport de bug)
4. **AUTRE** (Documentation générale, procédure)

# Étape 2 : Génération de la Synthèse

Produis un résumé structuré selon le type identifié.

## SI TYPE = AUDIT

### 1. 🚦 Météo & Conformité

* **Score Global** : [Note/100 ou Niveau de risque]
* **Verdict** : [GO / NO GO / GO AVEC RÉSERVES]

### 2. 🛡️ Top 3 Risques Actifs

* Liste les 3 failles les plus critiques trouvées.

### 3. 📉 Dette Technique

* Résumé de l'effort nécessaire pour la remédiation (ex: "Faible - 2h de dev" ou "Élevé - Refonte requise").

## SI TYPE = SPEC (Spécification)

### 1. 🎯 Objectif & Sécurité

* **But de la feature** : En une phrase simple.
* **Impact Sécurité** : [Critique/Moyen/Faible] (Est-ce que ça touche à l'Auth, aux Paiements, aux Données ?)

### 2. 🧱 Complexité d'Implémentation

* **Points de vigilance** : Où les devs risquent-ils de se tromper ? (ex: "La validation des entrées est complexe ici").
* **Dépendances** : Nouveaux services ou librairies requis.

### 3. ✅ Critères d'Acceptation Clés

* Liste les 3 conditions *sine qua non* pour que la sécurité valide cette feature.

## SI TYPE = INCIDENT

### 1. 🚨 Gravité & Impact

* **Niveau** : [Critique/Majeur/Mineur]
* **Impact Métier** : (Perte de données, Arrêt de service, Image).

### 2. ⏱️ Chronologie & Réaction

* **Temps de détection** : ...
* **Temps de résolution** : ...

### 3. 🩹 Cause Racine & Fix

* Quelle est la vraie cause ? (Pas le symptôme).
* Le correctif est-il pérenne ?

## SECTION UNIVERSELLE (Toujours inclure à la fin)

### 👤 Zones d'Ombre (Human Check Required)

*(Liste ici tout ce qui est marqué comme "À vérifier", "Incertain" ou "Non détecté" dans la source, ou ce qui te semble louche/incomplet)*

* [ ] ⚠️ **[Sujet]** : L'IA/Le document n'a pas pu valider [X].
* [ ] ❓ **[Sujet]** : L'hypothèse [Y] doit être confirmée par un humain.

### 👉 Décision Attendue

* Quelle est la prochaine action immédiate ? (ex: "Valider la spec", "Lancer le patch", "Accepter le risque").

# Étape 3 : Génération de la Synthèse

Génère le contenu du fichier Markdown (à sauvegarder avec le document d'origine sous son nom suffixé par `.summary.md`.
