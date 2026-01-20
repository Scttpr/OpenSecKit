# Guide de l'Homologation de Sécurité des Systèmes d'Information

**ANSSI - Agence Nationale de la Sécurité des Systèmes d'Information**

Version 2.1 - Avril 2025

ISBN 978-2-11-167188-1 (imprimé) - ISBN 978-2-11-167189-8 (en ligne)

Licence Ouverte/Open Licence (Etalab - V2)

---

## Préambule

Pour répondre aux enjeux croissants de la sécurité du numérique, l'ANSSI a décidé de faire évoluer ses différentes publications.

Ce document remplace le Guide d'homologation publié en 2014. Il intègre les retours d'expériences et les évolutions méthodologiques de ces dix dernières années. Il prend également en compte les évolutions réglementaires et les travaux de simplification de l'ANSSI.

L'objectif du guide est d'accompagner les organisations dans leur démarche d'homologation de sécurité des systèmes d'information. Il présente les principes fondamentaux de l'homologation et propose une méthodologie adaptée aux différents contextes.

Ce guide s'adresse principalement :
- Aux autorités d'homologation
- Aux responsables de la sécurité des systèmes d'information (RSSI)
- Aux chefs de projet
- Aux équipes techniques et métiers impliquées dans la sécurisation des systèmes d'information

---

## Table des matières

1. [Avant de commencer](#1-avant-de-commencer)
   - 1.1 [Qu'est-ce que l'homologation de sécurité ?](#11-quest-ce-que-lhomologation-de-sécurité)
   - 1.2 [Pourquoi homologuer ?](#12-pourquoi-homologuer)
   - 1.3 [Quand homologuer ?](#13-quand-homologuer)
   - 1.4 [Qui est concerné par l'homologation ?](#14-qui-est-concerné-par-lhomologation)
2. [Sécuriser le système d'information](#2-sécuriser-le-système-dinformation)
   - 2.1 [Organiser la gouvernance de la sécurité](#21-organiser-la-gouvernance-de-la-sécurité)
   - 2.2 [Définir le périmètre du système d'information](#22-définir-le-périmètre-du-système-dinformation)
   - 2.3 [Identifier l'écosystème du système d'information](#23-identifier-lécosystème-du-système-dinformation)
   - 2.4 [Identifier les réglementations applicables](#24-identifier-les-réglementations-applicables)
   - 2.5 [Identifier et appliquer les mesures de sécurité](#25-identifier-et-appliquer-les-mesures-de-sécurité)
   - 2.6 [Cartographier le système d'information](#26-cartographier-le-système-dinformation)
   - 2.7 [Planifier l'exploitation du système d'information](#27-planifier-lexploitation-du-système-dinformation)
   - 2.8 [Analyser les risques](#28-analyser-les-risques)
   - 2.9 [Auditer le système d'information](#29-auditer-le-système-dinformation)
   - 2.10 [Définir et suivre un plan d'action](#210-définir-et-suivre-un-plan-daction)
3. [Homologuer en quatre étapes](#3-homologuer-en-quatre-étapes)
   - 3.1 [Première étape : Constituer le comité d'homologation](#31-première-étape--constituer-le-comité-dhomologation)
   - 3.2 [Deuxième étape : Identifier le niveau de la démarche](#32-deuxième-étape--identifier-le-niveau-de-la-démarche)
   - 3.3 [Troisième étape : Constituer le dossier d'homologation](#33-troisième-étape--constituer-le-dossier-dhomologation)
   - 3.4 [Quatrième étape : Organiser la commission d'homologation](#34-quatrième-étape--organiser-la-commission-dhomologation)
4. [Améliorer la sécurité du système d'information](#4-améliorer-la-sécurité-du-système-dinformation)
   - 4.1 [Revoir régulièrement la sécurité des systèmes d'information](#41-revoir-régulièrement-la-sécurité-des-systèmes-dinformation)
   - 4.2 [Renouveler une homologation de sécurité](#42-renouveler-une-homologation-de-sécurité)
   - 4.3 [Arrêter un système d'information](#43-arrêter-un-système-dinformation)
5. [Vocabulaire](#vocabulaire)
6. [Références](#références)

---

## 1. Avant de commencer

### 1.1 Qu'est-ce que l'homologation de sécurité ?

L'homologation de sécurité est une décision formelle prise par une autorité responsable, attestant que les risques liés à l'emploi d'un système d'information sont acceptables au regard des enjeux de l'organisation.

Cette décision est prononcée après évaluation des risques et des mesures de sécurité mises en place pour les traiter. Elle autorise la mise en service ou le maintien en service du système d'information.

L'homologation n'est pas une garantie absolue de sécurité. Elle atteste que :
- Les risques ont été identifiés et évalués
- Les mesures de sécurité appropriées ont été mises en place
- Les risques résiduels sont acceptés en connaissance de cause par l'autorité responsable

> **NOTE** : L'homologation de sécurité est une démarche continue qui doit être renouvelée régulièrement et à chaque changement significatif du système d'information ou de son contexte.

### 1.2 Pourquoi homologuer ?

L'homologation de sécurité répond à plusieurs objectifs :

**Objectifs réglementaires :**
- Respecter les obligations légales (RGS, PSSIE, IGI 1300, II 901, LPM/SIIV)
- Démontrer la conformité aux exigences de sécurité

**Objectifs de gouvernance :**
- Responsabiliser les acteurs de la sécurité
- Formaliser l'acceptation des risques par la direction
- Documenter les décisions de sécurité

**Objectifs opérationnels :**
- Identifier et traiter les risques de sécurité
- Améliorer la sécurité du système d'information
- Maintenir un niveau de sécurité adapté aux enjeux

> **NOTE** : Un système d'information non homologué présente un risque juridique pour l'organisation et ses responsables en cas d'incident de sécurité.

### 1.3 Quand homologuer ?

L'homologation doit être prononcée :
- **Avant la mise en service** d'un nouveau système d'information
- **Avant tout changement significatif** du système ou de son contexte
- **À l'expiration** de la période d'homologation précédente

Le processus d'homologation doit être initié dès les phases amont du projet, idéalement dès la phase de conception. Cette anticipation permet de :
- Intégrer la sécurité dès la conception (security by design)
- Éviter les surcoûts liés à des mesures correctives tardives
- Respecter les délais de mise en service

> **NOTE** : Un système d'information en situation de test, en cours de conception, n'utilisant pas de véritables informations et déconnecté de tout système en production ne nécessite pas d'homologation. Elle devient obligatoire avant son passage en condition réelle de fonctionnement.

### 1.4 Qui est concerné par l'homologation ?

L'homologation de sécurité est obligatoire pour :
- Les systèmes d'information de l'État et des organismes placés sous sa tutelle
- Les systèmes d'information traitant des informations classifiées
- Les systèmes d'information d'importance vitale (SIIV)
- Les systèmes d'information soumis au Référentiel Général de Sécurité (RGS)

Elle est fortement recommandée pour :
- Tous les systèmes d'information critiques des organisations
- Les systèmes traitant des données sensibles
- Les systèmes exposés à des menaces significatives

**Acteurs de l'homologation :**

| Rôle | Responsabilités |
|------|-----------------|
| Autorité d'homologation | Prononce la décision d'homologation, assume la responsabilité des risques acceptés |
| Comité d'homologation | Évalue les risques et émet un avis à l'attention de l'autorité |
| RSSI / CSN | Anime la démarche d'homologation, coordonne les travaux |
| Responsable métier | Exprime les besoins de sécurité, valide l'adéquation des mesures |
| Expert technique | Contribue à l'évaluation technique de la sécurité |

---

## 2. Sécuriser le système d'information

La sécurisation du système d'information est le préalable à l'homologation. Elle consiste à mettre en place les mesures nécessaires pour protéger le système contre les menaces et réduire les risques à un niveau acceptable.

### 2.1 Organiser la gouvernance de la sécurité

La gouvernance de la sécurité doit être clairement définie et documentée. Elle repose sur le modèle des trois lignes de défense :

**Première ligne de défense : Contrôle opérationnel**
- Les équipes opérationnelles sont responsables de l'application des mesures de sécurité
- Elles identifient et gèrent les risques dans leur périmètre quotidien

**Deuxième ligne de défense : Surveillance et pilotage**
- Le RSSI et les fonctions de contrôle supervisent l'application des politiques
- Ils assurent le pilotage de la sécurité et la gestion des risques

**Troisième ligne de défense : Audit indépendant**
- L'audit interne vérifie l'efficacité des contrôles
- Il fournit une assurance indépendante à la direction

> **NOTE** : Une gouvernance claire permet d'identifier les responsabilités de chacun et facilite la prise de décision lors de l'homologation.

### 2.2 Définir le périmètre du système d'information

Le périmètre du système d'information doit être clairement défini et documenté. Il comprend :

**Éléments à inclure :**
- Les composants matériels (serveurs, postes de travail, équipements réseau)
- Les composants logiciels (applications, systèmes d'exploitation, middleware)
- Les données traitées et stockées
- Les locaux et infrastructures physiques
- Les processus et procédures associés
- Les personnes et les rôles

**Points d'attention :**
- Les frontières avec les autres systèmes doivent être clairement identifiées
- Les interfaces et interconnexions doivent être documentées
- Les zones de responsabilité doivent être définies

> **NOTE** : Un périmètre mal défini rend l'homologation difficile et peut conduire à des zones d'ombre en termes de sécurité.

### 2.3 Identifier l'écosystème du système d'information

L'écosystème comprend l'ensemble des acteurs et systèmes en interaction avec le système d'information :

**Acteurs externes :**
- Utilisateurs (internes, externes, partenaires)
- Fournisseurs et prestataires
- Autorités de régulation
- Sources de menaces potentielles

**Systèmes interconnectés :**
- Systèmes d'information partenaires
- Services cloud et externalisés
- Réseaux de communication
- Systèmes d'information de l'écosystème métier

**Flux d'information :**
- Nature des données échangées
- Sens des flux (entrant, sortant, bidirectionnel)
- Protocoles et mécanismes de sécurité associés

### 2.4 Identifier les réglementations applicables

Les réglementations applicables déterminent les exigences de sécurité minimales :

**Réglementations générales :**
- Référentiel Général de Sécurité (RGS)
- Règlement Général sur la Protection des Données (RGPD)
- Directive NIS 2

**Réglementations sectorielles :**
- PSSIE (administrations)
- IGI 1300 (informations classifiées)
- II 901 (systèmes traitant des informations Diffusion Restreinte)
- LPM / SIIV (opérateurs d'importance vitale)
- Réglementations sectorielles spécifiques (santé, finance, etc.)

> **NOTE** : L'identification des réglementations applicables est un préalable indispensable pour définir le niveau de sécurité attendu.

### 2.5 Identifier et appliquer les mesures de sécurité

Les mesures de sécurité doivent couvrir les quatre piliers de la sécurité :

**Gouvernance :**
- Politique de sécurité
- Organisation et responsabilités
- Gestion des risques
- Conformité et contrôle

**Protection :**
- Contrôle d'accès (authentification, autorisation)
- Protection des données (chiffrement, sauvegarde)
- Sécurité des développements
- Sécurité physique et environnementale

**Défense :**
- Détection des incidents
- Journalisation et supervision
- Réponse aux incidents
- Gestion des vulnérabilités

**Résilience :**
- Continuité d'activité
- Reprise après sinistre
- Gestion de crise
- Tests et exercices

> **NOTE** : Les mesures de sécurité doivent être proportionnées aux risques identifiés et aux enjeux de l'organisation.

### 2.6 Cartographier le système d'information

La cartographie du système d'information est un élément essentiel du dossier d'homologation. Elle permet de :
- Visualiser l'architecture du système
- Identifier les composants critiques
- Comprendre les flux d'information
- Évaluer la surface d'attaque

**Types de cartographies :**

| Type | Description |
|------|-------------|
| Vue métier | Processus et fonctions supportés par le SI |
| Vue applicative | Applications et leurs interactions |
| Vue infrastructure | Composants techniques et réseau |
| Vue données | Données traitées et leur classification |

> **NOTE** : La cartographie doit être maintenue à jour tout au long du cycle de vie du système d'information.

### 2.7 Planifier l'exploitation du système d'information

L'exploitation du système d'information doit être planifiée et documentée :

**Maintien en Condition Opérationnelle (MCO) :**
- Supervision et surveillance
- Gestion des incidents
- Gestion des changements
- Support utilisateur

**Maintien en Condition de Sécurité (MCS) :**
- Gestion des vulnérabilités
- Application des correctifs
- Mise à jour des mesures de sécurité
- Veille sécurité

**Plan de résilience :**
- Plan de continuité d'activité (PCA)
- Plan de reprise d'activité (PRA)
- Procédures de gestion de crise
- Exercices et tests réguliers

### 2.8 Analyser les risques

L'analyse de risques est au cœur de la démarche d'homologation. Elle permet d'identifier, d'évaluer et de traiter les risques liés à l'emploi du système d'information.

**Méthode recommandée : EBIOS Risk Manager**

La méthode EBIOS RM, portée par l'ANSSI et le Club EBIOS, est structurée en cinq ateliers :

1. **Cadrage et socle de sécurité** : Définir le périmètre et identifier les mesures de sécurité existantes
2. **Sources de risque** : Identifier les sources de risque et leurs objectifs
3. **Scénarios stratégiques** : Élaborer les scénarios d'attaque de haut niveau
4. **Scénarios opérationnels** : Détailler les modes opératoires des attaquants
5. **Traitement du risque** : Définir les mesures de sécurité pour traiter les risques

> **NOTE** : L'analyse de risques doit être proportionnée à la criticité du système. Une approche simplifiée peut être utilisée pour les systèmes peu critiques.

**Traitement des risques :**

| Option | Description |
|--------|-------------|
| Réduction | Mettre en place des mesures de sécurité pour réduire le risque |
| Transfert | Transférer le risque (assurance, externalisation) |
| Évitement | Supprimer l'activité ou la fonctionnalité à risque |
| Acceptation | Accepter le risque en connaissance de cause |

### 2.9 Auditer le système d'information

L'audit permet de vérifier l'application effective des mesures de sécurité et d'identifier les vulnérabilités résiduelles.

**Types d'audits :**

| Type | Objectif |
|------|----------|
| Audit organisationnel | Vérifier la conformité aux politiques et procédures |
| Audit de configuration | Contrôler les paramètres de sécurité des composants |
| Audit de code | Analyser la sécurité du code source des applications |
| Test d'intrusion | Évaluer la résistance face à des attaques simulées |
| Audit d'architecture | Vérifier la cohérence de l'architecture de sécurité |

**Prestataires qualifiés :**

Pour les systèmes d'information les plus sensibles, il est recommandé de faire appel à des prestataires qualifiés par l'ANSSI (PASSI - Prestataires d'Audit de la Sécurité des Systèmes d'Information).

> **NOTE** : Les audits doivent être réalisés régulièrement et à chaque changement significatif du système.

### 2.10 Définir et suivre un plan d'action

Le plan d'action formalise les mesures à mettre en œuvre pour traiter les risques identifiés et atteindre le niveau de sécurité attendu.

**Contenu du plan d'action :**
- Description des actions à réaliser
- Responsable de chaque action
- Échéances prévisionnelles
- Ressources nécessaires
- Critères de succès

**Suivi du plan d'action :**
- Indicateurs d'avancement
- Revues périodiques
- Gestion des écarts
- Communication à l'autorité d'homologation

> **NOTE** : Le plan d'action doit être réaliste et priorisé en fonction des risques et des ressources disponibles.

---

## 3. Homologuer en quatre étapes

La démarche d'homologation se déroule en quatre étapes successives :

### 3.1 Première étape : Constituer le comité d'homologation

Le comité d'homologation est l'instance qui évalue la sécurité du système d'information et émet un avis à l'attention de l'autorité d'homologation.

**Composition du comité :**

| Membre | Rôle |
|--------|------|
| RSSI / CSN | Anime le comité, coordonne les travaux |
| Responsable métier | Représente les enjeux métier |
| Chef de projet | Apporte la connaissance du système |
| Expert technique | Évalue les aspects techniques |
| Juriste (optionnel) | Vérifie la conformité réglementaire |
| Représentant DSI | Assure la cohérence avec le SI global |

**Missions du comité :**
- Collecter les informations nécessaires à l'homologation
- Évaluer l'application des mesures de sécurité
- Analyser les risques résiduels
- Émettre un avis motivé à l'autorité

> **NOTE** : La composition du comité doit être adaptée à la criticité du système. Pour les systèmes peu critiques, un comité restreint peut suffire.

### 3.2 Deuxième étape : Identifier le niveau de la démarche

Le niveau de la démarche d'homologation dépend de la criticité du système et de son exposition aux menaces. Trois niveaux sont proposés :

**Niveau simplifié :**
- Systèmes peu critiques, faiblement exposés
- Démarche allégée, documentation minimale
- Décision possible par échange électronique

**Niveau intermédiaire :**
- Systèmes de criticité moyenne
- Démarche standard, documentation proportionnée
- Commission d'homologation recommandée

**Niveau renforcé :**
- Systèmes critiques, fortement exposés
- Démarche complète, documentation exhaustive
- Commission d'homologation obligatoire

**Critères de détermination du niveau :**

| Critère | Simplifié | Intermédiaire | Renforcé |
|---------|-----------|---------------|----------|
| Impact en cas d'incident | Limité | Significatif | Grave |
| Exposition aux menaces | Faible | Moyenne | Forte |
| Données traitées | Non sensibles | Sensibles | Très sensibles |
| Interconnexions | Isolé ou peu connecté | Connecté | Fortement interconnecté |

### 3.3 Troisième étape : Constituer le dossier d'homologation

Le dossier d'homologation rassemble l'ensemble des documents nécessaires à la prise de décision.

**Contenu minimal (niveau simplifié) :**
- Fiche d'identité du système d'information
- Déclaration d'application des mesures de sécurité
- Avis du comité d'homologation

**Contenu standard (niveau intermédiaire) :**
- Tout le contenu du niveau simplifié
- Cartographie du système d'information
- Analyse de risques simplifiée
- Plan d'action
- Résultats d'audit

**Contenu complet (niveau renforcé) :**
- Tout le contenu du niveau intermédiaire
- Analyse de risques complète (EBIOS RM)
- Politique de sécurité du système d'information
- Plans de continuité et de reprise d'activité
- Rapports d'audit détaillés

#### 3.3.1 Fiche d'identité du système d'information

La fiche d'identité présente les caractéristiques essentielles du système :
- Nom et description du système
- Périmètre et composants principaux
- Données traitées et leur sensibilité
- Utilisateurs et volumétrie
- Interconnexions avec d'autres systèmes
- Responsables (métier, technique, sécurité)

#### 3.3.2 Avis du comité d'homologation

L'avis du comité est un document essentiel qui :
- Synthétise l'évaluation de la sécurité
- Présente les risques résiduels
- Recommande ou non l'homologation
- Propose une durée d'homologation
- Identifie les réserves éventuelles

### 3.4 Quatrième étape : Organiser la commission d'homologation

La commission d'homologation est la réunion de présentation du système d'information à l'autorité d'homologation. Elle est nécessaire pour tous les systèmes d'information de démarche renforcée.

#### 3.4.1 Préparer la commission d'homologation

**L'autorité d'homologation peut être :**
- L'autorité qualifiée de la sécurité du système d'information (AQSSI), directement responsable de la sécurité des systèmes d'information de l'organisation
- Par délégation de pouvoir, une autorité d'homologation (AH) désignée par l'AQSSI
- Un dirigeant de l'organisation ou un responsable ayant reçu une délégation de signature

> **NOTE** : Pour assurer une compréhension du contexte et une vraie prise en compte des risques, l'autorité d'homologation doit être au plus proche du métier du système d'information.

**Préparation :**
- Planifier la commission suffisamment à l'avance
- S'assurer de la disponibilité de l'autorité
- Réaliser une "pré-commission" pour préparer les réponses aux questions
- Préparer des supports de présentation synthétiques

#### 3.4.2 Animer la commission d'homologation

> **NOTE** : Il n'est pas utile de rassembler tous les membres du comité d'homologation lors de la commission afin de ne pas surcharger la réunion. Néanmoins les membres clés (DSI, RSSI, Responsable métier) doivent y participer.

**Recommandations :**
- La tenue en présentiel est fortement recommandée
- La commission ne doit pas durer plus d'une heure
- Un "gardien du temps" est préconisé
- Les supports doivent être synthétiques

> **NOTE** : Les risques liés à l'emploi du système d'information doivent être ouvertement et clairement exposés à l'autorité. Ils ne doivent pas être sous-évalués ou cachés.

**Durée de l'homologation :**

La durée de l'homologation proposée dépend de :
- La maturité de la sécurité du système d'information
- Les évolutions prévues
- L'exposition aux sources de risques numériques

**L'ANSSI ne recommande pas de prononcer une homologation de plus de trois ans.**

- Un système avec un plan d'action conséquent ne peut pas être homologué sur une période trop longue
- Réduire la durée d'homologation revient à augmenter la surveillance du système
- Il n'y a pas de renouvellement automatique d'une homologation

> **NOTE** : Si aucune réserve n'a été émise lors de sa prononciation, l'ANSSI recommande de ne pas revenir sur une décision d'homologation avant la fin de la période.

**Formalisation de la décision :**

- Dans le cas où une décision ne peut être prise, ou prise avec réserves, un plan d'action doit être établi et une nouvelle commission reprogrammée
- La décision doit être formalisée de façon écrite en utilisant les supports officiels de l'organisation

> **NOTE** : Informer l'utilisateur du système d'information de l'obtention de l'homologation renforce sa confiance. Cela peut se faire par l'affichage d'une mention au démarrage du système ou une communication officielle.

---

## 4. Améliorer la sécurité du système d'information

La sécurité du système d'information ne doit pas s'arrêter une fois l'homologation obtenue. Elle doit être maintenue ou améliorée.

Cela se traduit notamment :
- Par la réalisation des actions définies dans le plan d'action
- Par la réalisation des actions permettant de lever les éventuelles réserves
- Par l'ajout d'actions suite à des changements impactant le périmètre ou l'écosystème
- Par le maintien en condition opérationnelle et de sécurité
- Par la correction de vulnérabilités identifiées
- Par l'application de mesures liées à d'éventuels incidents de sécurité
- Par la mise à jour de la documentation

### 4.1 Revoir régulièrement la sécurité des systèmes d'information

La sécurité de l'ensemble des systèmes d'information d'une organisation doit être revue **au moins une fois par an**, en dehors de toute commission d'homologation.

**Organisation de la revue :**
- Présidée par l'autorité d'homologation ou son délégué
- Durée et contenu adaptés au nombre de systèmes à étudier

**Points à présenter durant la revue :**
- L'état d'avancement des actions décidées lors de la précédente revue
- Les indicateurs de performance
- Les événements marquants internes ou externes pouvant impacter la sécurité
- Les événements majeurs et incidents de sécurité survenus
- Les actions d'amélioration proposées pour la période suivante

Il est recommandé de présenter l'ensemble des systèmes d'information homologués ou non homologués. Pour ces derniers, il est important d'en comprendre la raison.

> **NOTE** : Durant la période d'homologation, l'autorité d'homologation peut être amenée à changer. Sauf demande spécifique, cela n'oblige pas à la tenue d'une nouvelle commission.

### 4.2 Renouveler une homologation de sécurité

Avant la date de fin d'homologation de sécurité d'un système d'information, l'avis de l'autorité doit être renouvelé.

**Préparation du renouvellement :**
- Si la démarche a donné satisfaction, une approche similaire peut être réutilisée
- Un système correctement suivi et dont la sécurité a été améliorée ne nécessite pas un gros effort de documentation
- Le comité peut être composé des mêmes personnes que pour l'homologation initiale

**Points à examiner par le comité :**
- L'histoire du système depuis sa dernière homologation (incidents, changements, améliorations)
- Le suivi et l'avancée des actions présentées lors de la précédente commission
- L'évolution du contexte pour la prochaine période

L'avis et la durée proposés pour le renouvellement peuvent être différents des précédents, selon le contexte et les enjeux de sécurité au jour de la commission.

> **NOTE** : Il n'est pas recommandé de renouveler l'homologation d'un système d'information si aucune action n'a été réalisée pour combler ses faiblesses de sécurité.

### 4.3 Arrêter un système d'information

Le système d'information est considéré comme arrêté lorsque :
- Toutes les données qu'il traite ont été détruites, déclassifiées, transférées ou conservées par un service spécialisé
- Les éléments qui le composent ont été décommissionnés

**Exigences :**
- Des preuves doivent être apportées à ces actions
- Les processus de décommissionnement doivent être établis avant le retrait du service
- Ces processus doivent être accessibles et évalués par le comité d'homologation

L'homologation prend fin lors du retrait du service et la suppression des risques associés.

---

## L'essentiel du guide

Même si la démarche d'homologation de sécurité est une obligation réglementaire pour un certain nombre de systèmes d'information, elle est fortement recommandée pour les systèmes d'information les plus critiques.

**Principes fondamentaux :**

- L'homologation porte sur un système d'information dont le **périmètre doit être clairement défini**
- Toutes les briques sur lesquelles il repose, son écosystème, et les risques qu'ils entraînent doivent être identifiés
- La nature des travaux d'évaluation doit correspondre à la **criticité** et à l'**exposition** aux sources de risques
- Il s'agit d'une **décision prononcée par une autorité compétente** au regard des risques
- La décision s'appuie sur l'**avis du comité d'homologation**
- **Un système peut être homologué même s'il comporte des risques non traités mais acceptables** par l'autorité
- La durée dépend des enjeux de sécurité, de la maturité du système et des réglementations
- Une **revue de contrôle** doit être effectuée **au minimum une fois par an**
- La démarche doit rester un processus **simple, utile et adapté**
- **Le système d'information ne doit être employé qu'après avoir été homologué**

---

## Vocabulaire

### Analyse de risque
L'analyse de risque (AdR) est une démarche méthodologique visant à identifier, évaluer et traiter les risques. Elle doit mener à la création d'un plan de traitement des risques visant à réduire la vraisemblance de la survenance du risque.

### Comité d'homologation
Le comité d'homologation réunit les personnes aptes à vérifier la pertinence des mesures de sécurité appliquées à un système d'information pour que les risques auxquels il est exposé soient acceptables.

### Commission d'homologation
La commission d'homologation est l'acte durant lequel l'autorité d'homologation émet une décision d'homologation. Les échanges qui permettent cette décision peuvent être numériques (échanges par email) ou dans le cadre d'une réunion en présentiel réunissant les personnes impliquées par la décision.

### Décision d'homologation
Une décision d'homologation est prise par l'autorité d'homologation. Elle donne ou refuse l'autorisation d'employer un système d'information au regard des risques auxquels il est exposé. La décision d'homologation doit être renouvelée régulièrement.

### Démarche d'homologation de sécurité
La démarche d'homologation est le processus qui suit le cycle de vie d'un système d'information. Elle est jalonnée par des phases d'homologation permettant une acceptation des risques par une autorité d'homologation.

### EBIOS RM
EBIOS RM pour l'Expression des Besoins et Identification des Objectifs de Sécurité Risk Manager est la méthode d'évaluation des risques portée par l'ANSSI et le Club EBIOS. La méthode repose sur une approche en ateliers permettant de s'appuyer sur les socles de sécurité existants avant de s'intéresser aux scénarios d'attaque. Elle alterne les points de vue métier et technique et vise à être efficace plutôt qu'exhaustif.

### Revue des systèmes d'information
Point formel permettant d'identifier l'état de l'ensemble des systèmes d'information d'une organisation. La revue doit permettre à l'équipe de direction de prendre connaissance de l'état d'avancement des actions décidées lors des revues précédentes, des modifications pertinentes ayant un impact sur les risques et des opportunités d'amélioration.

### Sécurisation d'un système d'information
Processus permettant de ralentir ou d'empêcher une attaque cyber sur un système d'information. La sécurisation se caractérise par l'application de mesures de gouvernance, de protection, de défense et de résilience.

### Système d'information de l'État
D'après le Décret n° 2019-1088 du 25 octobre 2019 relatif au système d'information et de communication de l'État et à la direction interministérielle du numérique : Le système d'information et de communication de l'État est composé de l'ensemble des infrastructures et services logiciels informatiques permettant de collecter, traiter, transmettre et stocker sous forme numérique les données qui concourent aux missions des services de l'État et des organismes placés sous sa tutelle.

---

## Références

- **La méthode EBIOS Risk Manager** : https://cyber.gouv.fr/la-methode-ebios-risk-manager
- **Cartographie du système d'information** : https://cyber.gouv.fr/publications/cartographie-du-systeme-dinformation
- **Guide de l'hygiène informatique** : https://cyber.gouv.fr/publications/guide-dhygiene-informatique
- **Maîtrise du risque numérique - L'atout confiance** : https://cyber.gouv.fr/publications/maitrise-du-risque-numerique-latout-confiance
- **Anticiper et gérer une crise cyber** : https://cyber.gouv.fr/anticiper-et-gerer-une-crise-cyber
- **Mon Service Sécurisé** : https://monservicesecurise.cyber.gouv.fr
- **SI du Centre de Veille (CERT) de l'ANSSI** : https://www.cert.ssi.gouv.fr

---

*Source : ANSSI - Agence Nationale de la Sécurité des Systèmes d'Information*
*51, boulevard de la Tour-Maubourg - 75 700 PARIS 07 SP*
*www.cyber.gouv.fr*
