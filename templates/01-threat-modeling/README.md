# Modélisation des menaces (Principe I)

## Aperçu
Ce répertoire contient les outils pour appliquer le **Principe Constitutionnel I : Modélisation des menaces**.
Nous utilisons une approche **Hybride** pour maximiser l'efficacité :
1.  **Checklist Standard** : Pour évacuer rapidement les vulnérabilités techniques connues (HTTPS, Logs, etc.).
2.  **Analyse Métier** : Pour se concentrer sur la logique propre à votre application.

## 📂 Contenu du kit

| Fichier | Usage | Pour qui ? |
| :--- | :--- | :--- |
| **[`00-threat-library-common.md`](00-threat-library-common.md)** | **Bibliothèque de référence.** Liste les vulnérabilités standards mappées aux menaces STRIDE. | Tout le monde (Référence) |
| **[`stride-threat-model-template-planning.md`](stride-threat-model-template-planning.md)** | **Le Template Principal (v2.0).** Contient la checklist technique ET la section d'analyse métier. | Devs, Architects, IA |
| **[`data-flow-diagram-template-design.md`](data-flow-diagram-template-design.md)** | **Modélisation visuelle.** Template pour cartographier les flux de données avec Mermaid JS. | Architects, Tech Leads |
| **[`attack-tree-template-planning.md`](attack-tree-template-planning.md)** | **Analyse avancée.** Pour visualiser les chemins d'attaque complexes sur des cibles critiques. | Security Champions |

---

## 🚀 Guide de démarrage (Workflow)

### Étape 1 : Cartographier (15 min)
Créez le Diagramme de Flux de Données (DFD) de votre fonctionnalité.
* 👉 Utilisez : [`data-flow-diagram-template-design.md`](data-flow-diagram-template-design.md)
* *Astuce : Copiez le code Mermaid dans le template principal ensuite.*

### Étape 2 : Analyser (30 min)
Remplissez le modèle de menaces STRIDE.
* 👉 Utilisez : [`stride-threat-model-template-planning.md`](stride-threat-model-template-planning.md)
* **Action A (Standard)** : Parcourez la section "Checklist" en vous aidant de la [Bibliothèque commune](00-threat-library-common.md). Cochez ce qui est traité.
* **Action B (Métier)** : Imaginez 2 ou 3 scénarios d'attaque spécifiques à votre logique business (ex: "Peut-on commander sans payer ?", "Peut-on voir les données du voisin ?").

### Étape 3 : Sécuriser
Transférez les contre-mesures identifiées (ex: "Ajouter un Rate Limiting", "Vérifier le owner_id") dans votre Backlog (Jira/GitHub Issues).

---

## 🤖 Utilisation avec une IA (Mode Agent)

Ce kit est optimisé pour être utilisé avec des assistants IA (ChatGPT, Claude, GitHub Copilot).

**Prompt suggéré pour l'IA :**
> "Agis comme un expert en sécurité. Voici le diagramme Mermaid de mon architecture [INSÉRER CODE MERMAID].
>
> 1. Vérifie la présence des vulnérabilités listées dans '00-threat-library-common.md'.
> 2. Utilise le template 'stride-threat-model-template-planning.md' pour générer un rapport.
> 3. Identifie 3 menaces spécifiques à la logique métier que je devrais investiguer."

---

## Ressources et Outils
* [Mermaid Live Editor](https://mermaid.live/) : Pour prévisualiser vos diagrammes.
* [OWASP Threat Modeling Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Threat_Modeling_Cheat_Sheet.html)
* [Microsoft STRIDE](https://learn.microsoft.com/en-us/azure/security/develop/threat-modeling-tool-threats)