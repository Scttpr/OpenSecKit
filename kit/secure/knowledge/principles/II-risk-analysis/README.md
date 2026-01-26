# Modèles d'analyse de risques

## Aperçu

Ce répertoire contient des modèles pour le **Principe Constitutionnel II : analyse de risques** - l'évaluation et la priorisation systématiques des risques de sécurité pour guider les efforts d'atténuation et l'allocation des ressources.

## Pourquoi l'analyse de risques ?

L'analyse de risques vous aide à :

- **Quantifier les menaces de sécurité** avec une notation objective
- **Prioriser les efforts d'atténuation** en fonction du niveau de risque
- **Prendre des décisions éclairées** sur l'acceptation des risques
- **Démontrer la diligence raisonnable** aux parties prenantes et auditeurs
- **Suivre les risques dans le temps** et mesurer les améliorations de sécurité

## Modèles disponibles

| Modèle | Phase | Difficulté | Temps | Description |
|--------|-------|------------|-------|-------------|
| [`scoring-methodology.md`](scoring-methodology.md) | Planification | Intermédiaire | 30-60 min | Quantifier les risques avec la méthodologie Criticité × Probabilité × Exposition |
| [`register-guide.md`](register-guide.md) | Toutes Phases | Débutant | 15 min/semaine | Document évolutif suivant tous les risques de sécurité identifiés |

## Quand utiliser ces modèles

### Phase de planification

- Utiliser la **Notation des Risques** immédiatement après la modélisation des menaces
- Initialiser le **registre des risques** pour suivre tous les risques identifiés

### Toutes Phases (En Continu)

- Mettre à jour le **registre des risques** chaque semaine avec les nouveaux risques et changements de statut
- Ré-exécuter la **Notation des Risques** quand les risques changent ou que de nouvelles informations sont disponibles

## Méthodologie de Notation des Risques

Cette boîte à outils utilise un **modèle de notation à trois dimensions** :

```
Score de Risque = Criticité × Probabilité × Exposition

Où :
- Criticité (1-5) : Gravité de l'impact si la menace se matérialise
- Probabilité (1-5) : Probabilité que la menace se produise
- Exposition (1-5) : Degré d'exposition de la surface d'attaque

Plage de Score Total : 1 (minimal) à 125 (critique)
```

### Priorisation des Risques

| Score | Priorité | Action Requise | Approbation |
|-------|----------|----------------|-------------|
| 80-125 | P0 - Critique | Atténuation immédiate, bloquer la mise en production | Direction/Exécutif |
| 49-79 | P1 - Élevé | Atténuation avant mise en production | Champion de sécurité + Manager |
| 25-48 | P2 - Moyen | Backlog de sprint | Champion de sécurité |
| 11-24 | P3 - Faible | Planifié, non bloquant | Chef d'Équipe |
| 1-10 | P4 - Minimal | Surveillance | Aucune |

## Workflow Recommandé

```
1. Compléter la modélisation des menaces
   ↓
2. Noter Chaque Menace (Modèle de Notation des Risques)
   ↓
3. Ajouter au registre des risques
   ↓
4. Obtenir les Approbations Requises (P0, P1)
   ↓
5. Planifier les Atténuations (voir exigences de sécurité)
   ↓
6. Mettre à Jour le registre des risques Chaque Semaine
   ↓
7. Clôturer les Risques Quand Atténués (vérifier d'abord !)
```

## Démarrage rapide

### Première analyse de risques ?

**Commencez ici** : [`scoring-methodology.md`](scoring-methodology.md)

1. **Rassembler le modèle de menaces** : Avoir votre analyse STRIDE ou Arbre d'Attaque prête
2. **Noter chaque menace** : Utiliser Criticité × Probabilité × Exposition
3. **Documenter dans le registre des risques** : Ajouter tous les risques au document de suivi
4. **Obtenir les approbations** : Obtenir les signatures requises pour les risques P0-P1
5. **Planifier les atténuations** : Définir les contre-mesures pour les risques prioritaires

### Vous Avez Déjà des Risques Identifiés ?

**Commencez ici** : [`register-guide.md`](register-guide.md)

1. **Créer votre registre** : Initialiser le document de suivi des risques
2. **Ajouter tous les risques** : Copier depuis le modèle de menaces avec les scores
3. **Assigner les responsables** : Désigner les équipes/individus responsables
4. **Définir les échéances** : Établir les dates cibles d'atténuation
5. **Réviser chaque semaine** : Mettre à jour le statut et clôturer les risques atténués

## Intégration avec les Principes Constitutionnels

L'analyse de risques fait le lien entre la modélisation des menaces et l'implémentation de la sécurité :

**Entrées** :

- **Principe I (modélisation des menaces)** : Les menaces issues de STRIDE, Arbres d'Attaque deviennent des risques à noter
- **Décisions d'Architecture** : Les choix de conception impactant les scores d'Exposition

**Sorties** :

- **Principe III (exigences de sécurité)** : Les risques prioritaires génèrent les exigences de sécurité
- **Principe IV (tests de sécurité)** : Le registre des risques informe les priorités de couverture des tests
- **Communication aux Parties Prenantes** : Les scores de risques justifient les investissements de sécurité

## Conformité Constitutionnelle

### Exigences de vérification constitution

D'après la constitution SSDLC, le Principe II exige :

- **Notation des risques** complétée pour toutes les menaces identifiées
- **Risques > 48** (Élevé et Critique) validés par les parties prenantes appropriées
- **Registre des risques** maintenu tout au long du cycle de vie du projet
- **Approbation obtenue** avant d'accepter des risques ≥ 80 (Critique)

### Matrice d'Approbation

| Score de Risque | Approbation Requise De |
|-----------------|------------------------|
| 80-125 (Critique) | Direction/Équipe Executive |
| 49-79 (Élevé) | Champion de sécurité (2+) + Manager |
| 25-48 (Moyen) | Champion de sécurité |
| 11-24 (Faible) | Chef d'Équipe |
| 1-10 (Minimal) | Aucune approbation nécessaire |

## Cas d'usage Courants

### Développement de Nouvelle Fonctionnalité

1. Modéliser les menaces de la nouvelle fonctionnalité (Principe I)
2. Noter les menaces identifiées (Notation des Risques)
3. Ajouter au registre des risques
4. Si des risques P0-P1 existent, obtenir les approbations avant de procéder
5. Concevoir les atténuations dans l'architecture

### Préparation d'audit de sécurité

1. Réviser le registre des risques pour la complétude
2. Vérifier que tous les risques élevés sont atténués ou approuvés
3. Démontrer les tendances des risques dans le temps
4. Fournir des preuves de gestion continue des risques

### Réponse aux Incidents

1. Ajouter l'incident comme nouveau risque au registre
2. Noter en fonction de l'impact réel
3. Identifier les menaces de cause racine
4. Planifier les atténuations systémiques
5. Suivre jusqu'à clôture

### Revue de sécurité Trimestrielle

1. Re-noter tous les risques actifs (les circonstances changent)
2. Réviser les risques acceptés (toujours acceptables ?)
3. Analyser les tendances des risques
4. Rapporter à la direction

## Mapping de conformité

Ces modèles aident à satisfaire :

- **NIST RMF** : Évaluation des Risques (RMF Étape 2)
- **ISO 27001** : Annexe A.5.7 Veille sur les Menaces, A.8.2 Classification de l'Information
- **SOC 2** : CC2.1 Processus d'Évaluation des Risques
- **PCI-DSS** : Exigence 12.2 Méthodologie d'Évaluation des Risques
- **HIPAA** : § 164.308(a)(1)(ii)(A) analyse de risques

## Outils et ressources

### Outils d'analyse de risques

- **Excel/Google Sheets** : Suivi simple du registre des risques
- **Jira** : Suivi des risques avec intégration de tickets
- **ServiceNow GRC** : Gestion des risques d'entreprise
- **OWASP Risk Rating Calculator** : <https://owasp.org/www-project-risk-assessment-framework/>

### Ressources externes

- [NIST SP 800-30 - Guide d'Évaluation des Risques](https://csrc.nist.gov/publications/detail/sp/800-30/rev-1/final)
- [ISO 27005 - Gestion des Risques de sécurité de l'Information](https://www.iso.org/standard/75281.html)
- [OWASP Risk Rating Methodology](https://owasp.org/www-community/OWASP_Risk_Rating_Methodology)
- [FAIR Institute - Factor Analysis of Information Risk](https://www.fairinstitute.org/)

## Contribuer

Vous avez des améliorations pour les modèles d'analyse de risques ? Voir les [directives de contribution](../../CONTRIBUTING.txt).

Contributions précieuses :

- Méthodologies alternatives de notation des risques (FAIR, OCTAVE)
- Intégrations de registre des risques (Jira, Azure DevOps)
- Catalogues de risques spécifiques aux secteurs
- Tableaux de bord de visualisation des risques

---

**Besoin d'Aide ?** Ouvrir une [Discussion GitHub](https://github.com/Scttpr/OpenSecKit/issues) avec le tag `02-risk-analysis`.

**Prochaines étapes** : Après l'analyse de risques, procéder aux [exigences de sécurité](../III-security-design/) pour concevoir des contrôles pour les risques prioritaires.
