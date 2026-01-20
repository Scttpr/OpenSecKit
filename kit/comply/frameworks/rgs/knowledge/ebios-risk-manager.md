# EBIOS Risk Manager

**Version 1.5 - Septembre 2024 - ANSSI-PA-048**

Guide officiel de la méthode d'appréciation et de traitement des risques numériques publiée par l'ANSSI (Agence Nationale de la Sécurité des Systèmes d'Information).

---

## Table des matières

1. [Introduction](#introduction)
2. [Qu'est-ce que la méthode EBIOS Risk Manager ?](#quest-ce-que-la-méthode-ebios-risk-manager)
3. [Une démarche itérative en 5 ateliers](#une-démarche-itérative-en-5-ateliers)
4. [Les différents usages d'EBIOS RM](#les-différents-usages-debios-rm)
5. [Atelier 1 : Cadrage et Socle de Sécurité](#atelier-1--cadrage-et-socle-de-sécurité)
6. [Atelier 2 : Sources de Risque](#atelier-2--sources-de-risque)
7. [Atelier 3 : Scénarios Stratégiques](#atelier-3--scénarios-stratégiques)
8. [Atelier 4 : Scénarios Opérationnels](#atelier-4--scénarios-opérationnels)
9. [Atelier 5 : Traitement du Risque](#atelier-5--traitement-du-risque)
10. [Bibliographie](#bibliographie)
11. [Termes et Définitions](#termes-et-définitions)

---

## Introduction

EBIOS Risk Manager (EBIOS RM) est la méthode d'appréciation et de traitement des risques numériques publiée par l'Agence nationale de la sécurité des systèmes d'information (ANSSI) avec le soutien du Club EBIOS. Elle propose une boîte à outils adaptable, dont l'utilisation varie selon l'objectif du projet et target les risques numériques.

La méthode EBIOS Risk Manager permet d'apprécier les risques numériques et d'identifier les mesures de sécurité à mettre en œuvre pour les maîtriser. Elle permet aussi de valider le niveau de risque acceptable et de s'inscrire à plus long terme dans une démarche d'amélioration continue. Enfin, cette méthode permet de partager et communiquer au sein de l'organisation ainsi qu'auprès des partenaires une culture commune de la gestion du risque numérique.

### Conformité aux normes

La méthode EBIOS Risk Manager adopte une approche de management du risque numérique conforme à la norme ISO 31000, et qui satisfait aux exigences de la norme ISO/CEI 27001, tout en proposant des liens avec les référentiels internationaux ISO 27002, ISO 27005 et le cadre de cybersécurité du NIST.

---

## Qu'est-ce que la méthode EBIOS Risk Manager ?

### La pyramide du management du risque numérique

EBIOS Risk Manager adopte une approche de management du risque qui part du plus haut niveau (grandes missions de l'objet étudié) pour s'intéresser progressivement aux éléments métier et techniques en s'appuyant sur :

- **Un socle de sécurité** solide, construit normativement et sur les retours d'expérience
- **Une appréciation des risques** ciblant les menaces les plus dangereuses

Cette méthode combine deux approches complémentaires :

1. **L'approche par conformité** : Sélection de mesures de sécurité standards à partir de bonnes pratiques pour contrer les risques courants
2. **L'approche par scénarios** : Identification de scénarios de menaces avancées qui ciblent les missions et valeurs métier de l'organisation

### La notion de cycle

EBIOS Risk Manager introduit la notion de cycles permettant au management du risque numérique de s'adapter à deux stratégies complémentaires :

- **Le cycle stratégique** : Réalisé moins fréquemment, principalement lors de la conception ou de changements majeurs de l'objet étudié. Valide les orientations stratégiques de traitement du risque.

- **Le cycle opérationnel** : Réalisé plus fréquemment pour prendre en compte l'évolution des contextes métier, technique et de la menace. Cible les scénarios de risque les plus vraisemblables.

---

## Une démarche itérative en 5 ateliers

La méthode EBIOS Risk Manager est articulée autour de cinq ateliers :

```
┌─────────────────────────────────────────────────────────────────────┐
│                        CYCLE STRATÉGIQUE                            │
│  ┌──────────────┐    ┌──────────────┐    ┌──────────────┐          │
│  │  ATELIER 1   │───▶│  ATELIER 2   │───▶│  ATELIER 3   │          │
│  │   Cadrage    │    │  Sources de  │    │  Scénarios   │          │
│  │  et Socle    │    │   Risque     │    │ Stratégiques │          │
│  └──────────────┘    └──────────────┘    └──────────────┘          │
└─────────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────────┐
│                       CYCLE OPÉRATIONNEL                            │
│                      ┌──────────────┐    ┌──────────────┐          │
│                      │  ATELIER 4   │───▶│  ATELIER 5   │          │
│                      │  Scénarios   │    │  Traitement  │          │
│                      │Opérationnels │    │  du Risque   │          │
│                      └──────────────┘    └──────────────┘          │
└─────────────────────────────────────────────────────────────────────┘
```

### Vue d'ensemble des ateliers

| Atelier | Nom | Objectif principal |
|---------|-----|-------------------|
| 1 | Cadrage et Socle de Sécurité | Définir le périmètre, les missions, les valeurs métier et établir le socle de sécurité |
| 2 | Sources de Risque | Identifier les sources de risque et leurs objectifs visés |
| 3 | Scénarios Stratégiques | Construire les chemins d'attaque de haut niveau via l'écosystème |
| 4 | Scénarios Opérationnels | Détailler les modes opératoires techniques |
| 5 | Traitement du Risque | Définir la stratégie de traitement et les mesures de sécurité |

---

## Les différents usages d'EBIOS RM

EBIOS Risk Manager est une méthode qui s'adapte à de nombreux contextes :

### 1. Mise en place ou renforcement d'un processus de management du risque numérique
La méthode propose un cadre structurant qui permet de s'inscrire dans une démarche d'amélioration continue.

### 2. Appréciation des risques d'un système d'information
Cadre pour analyser les risques liés à un système existant ou nouveau.

### 3. Définition du niveau de sécurité à atteindre
Peut servir de socle pour un cahier des charges, un appel d'offres, ou des exigences contractuelles.

### 4. Homologation de sécurité
Constitue le cœur de l'analyse de risques nécessaire pour obtenir une homologation de sécurité d'un système d'information.

---

## Atelier 1 : Cadrage et Socle de Sécurité

### 1/ Les objectifs de l'atelier

L'atelier 1 a pour objectif de :
- Définir le cadre de l'étude
- Délimiter le périmètre métier et technique
- Identifier les événements redoutés associés aux valeurs métier
- Déterminer le socle de sécurité

### 2/ Les participants à l'atelier

- **Direction** : Validation du périmètre et des orientations
- **Métiers** : Connaissance des processus et des valeurs métier
- **RSSI** : Expertise sécurité et référentiels
- **DSI** : Connaissance technique du SI

### 3/ Les données de sortie

À l'issue de l'atelier, vous devez avoir défini :
- Les missions de l'objet de l'étude
- Les valeurs métier et leurs critères de sécurité
- Les biens supports
- Les événements redoutés et leur gravité
- Le socle de sécurité (référentiels applicables et état d'application)

### 4/ Les étapes de l'atelier

Cet atelier, d'une durée indicative d'une à deux demi-journées de travail, se décompose en :

**A. Définir le cadre de l'étude**
- Identifier l'objet de l'étude (organisation, SI, produit)
- Préciser les missions essentielles
- Identifier le périmètre métier et technique

**B. Définir les valeurs métier**
- Ce sont les composantes importantes pour l'organisation dans l'accomplissement de sa mission
- Exemples : service client, données R&D, informations clients, savoir-faire métier

**C. Identifier les biens supports**
- Composantes du SI sur lesquelles reposent les valeurs métier
- Nature : numérique, physique ou organisationnelle
- Exemples : serveurs, réseaux, locaux, équipes

**D. Identifier les événements redoutés**

Pour chaque valeur métier, identifier les événements redoutés selon les critères de sécurité :
- **Disponibilité** (D)
- **Intégrité** (I)
- **Confidentialité** (C)
- **Traçabilité** (T) - optionnel

**E. Évaluer la gravité des événements redoutés**

Échelle de gravité recommandée :

| Niveau | Valeur | Description |
|--------|--------|-------------|
| G1 | Mineur | L'organisation surmontera les impacts sans difficulté |
| G2 | Significatif | L'organisation surmontera les impacts avec quelques difficultés |
| G3 | Grave | L'organisation surmontera les impacts avec de sérieuses difficultés |
| G4 | Critique | L'organisation ne surmontera pas les impacts (mission compromise) |

**F. Déterminer le socle de sécurité**

Le socle de sécurité comprend :
1. Les référentiels applicables (ISO 27001/27002, guides ANSSI, réglementations sectorielles)
2. L'état d'application de ces référentiels
3. Les écarts identifiés et les justifications

### Exemple : Société de biotechnologie fabriquant des vaccins

**Missions** : Fabriquer et distribuer des vaccins

**Valeurs métier identifiées** :
| Valeur métier | D | I | C |
|--------------|---|---|---|
| Recherche et Développement | 2 | 3 | 4 |
| Production industrielle | 4 | 3 | 2 |
| Distribution des vaccins | 3 | 4 | 2 |

**Événements redoutés** :
- ER1 : Vol des résultats de R&D (confidentialité) - Gravité 4
- ER2 : Arrêt de la production (disponibilité) - Gravité 4
- ER3 : Modification de l'étiquetage des vaccins (intégrité) - Gravité 4

---

## Atelier 2 : Sources de Risque

### 1/ Les objectifs de l'atelier

L'atelier 2 vise à :
- Identifier les sources de risque (SR) susceptibles de porter atteinte aux missions de l'objet étudié
- Déterminer leurs objectifs visés (OV)
- Évaluer leur pertinence
- Sélectionner les couples SR/OV les plus pertinents pour la suite de l'étude

### 2/ Les participants à l'atelier

- **RSSI** : Animation et expertise
- **DSI** : Connaissance des menaces techniques
- **Métiers** : Connaissance du contexte sectoriel
- Experts externes si nécessaire (veille menace, renseignement)

### 3/ Les données de sortie

- Liste des sources de risque pertinentes
- Objectifs visés associés
- Couples SR/OV retenus pour l'analyse

### 4/ Les étapes de l'atelier

Cet atelier nécessite une à deux demi-journées de travail.

**A. Identifier les sources de risque**

Catégories typiques :
- **États** : Services de renseignement, agences gouvernementales
- **Crime organisé** : Cybercriminels, mafias
- **Terroristes** : Groupes idéologiques
- **Activistes** : Hacktivistes, militants
- **Concurrents** : Espionnage industriel
- **Fournisseurs/Prestataires** : Menace interne involontaire ou malveillante
- **Personnel interne** : Employés malveillants ou négligents
- **Amateurs** : Script kiddies

**B. Identifier les objectifs visés**

Pour chaque source de risque, identifier ses motivations :
- Espionnage (vol d'informations)
- Prépositionnement stratégique
- Influence/manipulation
- Déstabilisation
- Lucre (gain financier)
- Idéologie
- Vengeance

**C. Évaluer la pertinence des couples SR/OV**

Critères d'évaluation :
- **Motivation** : Niveau d'intérêt de la SR pour l'OV
- **Ressources** : Moyens techniques et humains de la SR
- **Activité** : Historique d'attaques connues

Échelle de pertinence :
| Niveau | Description |
|--------|-------------|
| 1 | Peu pertinent |
| 2 | Pertinent |
| 3 | Très pertinent |
| 4 | Prioritaire |

**D. Sélectionner les couples SR/OV**

Retenir les couples les plus pertinents (typiquement 3 à 5) pour poursuivre l'analyse dans les ateliers suivants.

### Exemple : Société de biotechnologie

**Couples SR/OV retenus** :

| Source de risque | Objectif visé | Pertinence |
|-----------------|---------------|------------|
| Concurrent | Voler les informations de R&D | 4 |
| Activiste anti-vaccination | Perturber la production | 3 |
| Activiste anti-vaccination | Perturber la distribution | 3 |
| Cybercriminels | Rançonner l'entreprise | 2 |

---

## Atelier 3 : Scénarios Stratégiques

### 1/ Les objectifs de l'atelier

L'atelier 3 vise à :
- Identifier l'écosystème et ses parties prenantes
- Évaluer la dangerosité des parties prenantes
- Construire les scénarios stratégiques (chemins d'attaque)
- Définir les mesures de sécurité sur l'écosystème

### 2/ Les participants à l'atelier

- **RSSI** : Animation et expertise sécurité
- **Métiers** : Connaissance des relations partenaires
- **Achats/Juridique** : Connaissance des contrats
- **DSI** : Connaissance des interconnexions techniques

### 3/ Les données de sortie

- Cartographie de l'écosystème
- Parties prenantes critiques identifiées
- Scénarios stratégiques construits
- Mesures de sécurité sur l'écosystème

### 4/ Les étapes de l'atelier

Cet atelier nécessite deux à quatre demi-journées de travail.

**A. Identifier l'écosystème**

Les parties prenantes sont les éléments en interaction avec l'objet de l'étude :
- **Partenaires** : Co-traitants, sous-traitants
- **Prestataires** : Infogérance, hébergement, maintenance
- **Fournisseurs** : Matériel, logiciel, services
- **Clients** : Utilisateurs finaux
- **Entités internes** : Autres directions, filiales

**B. Évaluer les parties prenantes**

Pour chaque partie prenante, évaluer :
- **Dépendance** : Niveau de dépendance de l'objet étudié vis-à-vis de la PP
- **Pénétration** : Niveau d'accès de la PP au SI de l'objet étudié
- **Maturité cyber** : Niveau de sécurité de la PP
- **Confiance** : Niveau de confiance dans la PP

Formule de calcul de la **dangerosité** :
```
Dangerosité = (Dépendance + Pénétration) × (5 - Maturité cyber) × (5 - Confiance)
```

**C. Construire les scénarios stratégiques**

Un scénario stratégique décrit le **chemin d'attaque** qu'une source de risque pourrait emprunter pour atteindre son objectif :

```
Source de Risque ──▶ Partie Prenante ──▶ Valeur Métier ──▶ Événement Redouté
```

Les chemins d'attaque peuvent être :
- **Directs** : La SR attaque directement l'objet étudié
- **Indirects** : La SR passe par une partie prenante de l'écosystème

**D. Définir les mesures sur l'écosystème**

Types de mesures :
- Clauses contractuelles de sécurité
- Audits de sécurité des prestataires
- Cloisonnement des accès
- Surveillance des flux
- Plans de réversibilité

### Exemple : Scénarios stratégiques - Biotechnologie

**Scénario 1** : Un concurrent vole des informations de R&D grâce à un canal d'exfiltration direct
- SR : Concurrent
- Chemin : Attaque directe → SI bureautique → Données R&D
- ER : Vol des résultats de recherche

**Scénario 2** : Un concurrent vole des informations de R&D via le prestataire informatique
- SR : Concurrent
- Chemin : Compromission du prestataire → Accès maintenance → Données R&D
- ER : Vol des résultats de recherche

**Scénario 3** : Un activiste provoque un arrêt de production via le fournisseur de matériel
- SR : Activiste
- Chemin : Compromission équipement maintenance → Système industriel → Arrêt production
- ER : Indisponibilité de la production

---

## Atelier 4 : Scénarios Opérationnels

### 1/ Les objectifs de l'atelier

L'atelier 4 vise à :
- Identifier les biens supports critiques
- Construire les scénarios opérationnels (modes opératoires techniques)
- Évaluer la vraisemblance des scénarios

### 2/ Les participants à l'atelier

- **RSSI** : Animation
- **Équipes techniques** : Administrateurs, architectes
- **SOC/CERT** : Expertise en attaques
- **Experts en tests d'intrusion** (optionnel)

### 3/ Les données de sortie

- Biens supports critiques identifiés
- Scénarios opérationnels détaillés
- Vraisemblance évaluée pour chaque scénario

### 4/ Les étapes de l'atelier

Cet atelier nécessite deux à quatre demi-journées de travail.

**A. Identifier les biens supports critiques**

Ce sont les biens supports particulièrement exposés ou vulnérables, identifiés à partir :
- Des chemins d'attaque des scénarios stratégiques
- De la criticité des valeurs métier supportées
- De leur exposition (accessibilité depuis Internet, partenaires, etc.)

**B. Élaborer les scénarios opérationnels**

Un scénario opérationnel est une **séquence d'actions élémentaires** que la source de risque réalise pour atteindre son objectif.

Structure type d'un scénario opérationnel (basé sur la Kill Chain) :

| Phase | Actions élémentaires typiques |
|-------|------------------------------|
| Reconnaissance | Scan de ports, OSINT, phishing de reconnaissance |
| Armement | Développement d'exploit, achat de malware |
| Livraison | Phishing, exploitation web, clé USB piégée |
| Exploitation | Exécution de code, élévation de privilèges |
| Installation | Implant, backdoor, persistance |
| Commande & Contrôle | Canal C2, exfiltration DNS |
| Actions sur objectif | Vol de données, sabotage, chiffrement |

**C. Évaluer la vraisemblance**

Pour chaque action élémentaire, évaluer sa vraisemblance selon :
- La difficulté technique
- L'état des mesures de sécurité existantes
- Les capacités présumées de la source de risque

Échelle de vraisemblance :

| Niveau | Valeur | Description |
|--------|--------|-------------|
| V1 | Minime | Peu vraisemblable, conditions très difficiles |
| V2 | Significatif | Vraisemblable sous certaines conditions |
| V3 | Fort | Très vraisemblable, conditions favorables |
| V4 | Maximal | Quasi certain, conditions très favorables |

**D. Calculer la vraisemblance globale**

La vraisemblance d'un scénario opérationnel est généralement celle de son maillon le plus faible (action la plus difficile) ou une combinaison pondérée.

### Exemple : Scénario opérationnel - Vol de R&D via prestataire

```
┌─────────────────────────────────────────────────────────────────┐
│ Objectif : Voler les données R&D via le prestataire informatique│
└─────────────────────────────────────────────────────────────────┘
                              │
    ┌─────────────────────────┼─────────────────────────┐
    ▼                         ▼                         ▼
┌─────────┐            ┌─────────────┐           ┌──────────┐
│ AE1:    │            │ AE2:        │           │ AE3:     │
│Phishing │───────────▶│Compromission│──────────▶│Pivot vers│
│prestat. │            │ poste admin │           │ SI cible │
│ V3      │            │    V2       │           │   V2     │
└─────────┘            └─────────────┘           └──────────┘
                                                      │
    ┌─────────────────────────┬───────────────────────┘
    ▼                         ▼
┌──────────┐           ┌──────────────┐
│ AE4:     │           │ AE5:         │
│Mouvement │──────────▶│ Exfiltration │
│ latéral  │           │   données    │
│   V3     │           │     V3       │
└──────────┘           └──────────────┘

Vraisemblance globale : V2 (le maillon le plus faible)
```

---

## Atelier 5 : Traitement du Risque

### 1/ Les objectifs de l'atelier

L'atelier 5 vise à :
- Synthétiser les scénarios de risque
- Définir la stratégie de traitement du risque
- Identifier les mesures de sécurité
- Évaluer les risques résiduels
- Mettre en place le suivi des risques

### 2/ Les participants à l'atelier

Les mêmes participants que l'atelier 1 :
- Direction
- Métiers
- RSSI
- DSI

### 3/ Les données de sortie

- Stratégie de traitement du risque
- Synthèse des risques résiduels
- Plan de traitement du risque
- Cadre de suivi des risques

### 4/ Les étapes de l'atelier

Cet atelier nécessite deux à quatre demi-journées de travail.

**A. Réaliser une synthèse des scénarios de risque**

Positionner les scénarios de risque sur une matrice gravité/vraisemblance :

```
     GRAVITÉ
        4 │  R5    R4  │         │         │
        3 │            │         │         │
        2 │       R2   │   R1    │   R3    │
        1 │            │         │         │
          └────────────┴─────────┴─────────┘
              1         2         3         4
                    VRAISEMBLANCE
```

**B. Décider de la stratégie de traitement**

Pour chaque risque, définir son acceptabilité :

| Niveau de risque | Acceptabilité | Actions |
|-----------------|---------------|---------|
| Faible | Acceptable en l'état | Aucune action |
| Moyen | Tolérable sous contrôle | Suivi et amélioration continue |
| Élevé | Inacceptable | Mesures de réduction impératives |

Zones de la matrice :
- **Zone verte** : Risques acceptables
- **Zone orange** : Risques à surveiller et traiter à moyen terme
- **Zone rouge** : Risques inacceptables nécessitant un traitement immédiat

**C. Définir les mesures de sécurité**

Pour chaque risque non acceptable, définir des mesures de traitement :

| Catégorie | Types de mesures |
|-----------|-----------------|
| **Gouvernance** | Politiques, sensibilisation, audits, clauses contractuelles |
| **Protection** | Chiffrement, contrôle d'accès, cloisonnement, durcissement |
| **Défense** | Détection d'intrusion, SOC, analyse de journaux, CTI |
| **Résilience** | PCA/PRA, sauvegardes, redondance, exercices de crise |

**D. Formaliser le plan de traitement du risque**

Pour chaque mesure, documenter :
- Scénarios de risque associés
- Responsable
- Freins et difficultés de mise en œuvre
- Coût/complexité estimé
- Échéance
- Priorité
- Statut

### Exemple : Plan de traitement - Biotechnologie

| Mesure de sécurité | Risques | Responsable | Coût | Échéance | Statut |
|-------------------|---------|-------------|------|----------|--------|
| Sensibilisation renforcée hameçonnage | R1 | RSSI | + | 6 mois | En cours |
| Audit sécurité SI bureautique | R1, R5 | RSSI | ++ | 9 mois | À lancer |
| Clauses sécurité contrats prestataires | R2, R3, R4 | Juridique | ++ | 18 mois | En cours |
| Protection renforcée données R&D | R1, R3 | DSI | +++ | 9 mois | En cours |
| Renforcement sécurité système industriel | R4, R5 | RSSI/DSI | +++ | 12 mois | À lancer |
| Surveillance flux (sonde IDS) | R1 | DSI | ++ | 9 mois | À lancer |
| Renforcement PCA | R4, R5 | Équipe continuité | ++ | 6 mois | En cours |

**E. Évaluer et documenter les risques résiduels**

Après application des mesures, réévaluer chaque risque :

```
AVANT TRAITEMENT                 APRÈS TRAITEMENT
     GRAVITÉ                          GRAVITÉ
        4 │  R5    R4  │              4 │R4,R5│         │
        3 │            │              3 │     │         │
        2 │       R2   │   R1    R3   2 │  R2 │   R1    │  R3
        1 │            │              1 │     │         │
          └────────────┴─────────       └─────┴─────────┴──
              1    2    3    4            1    2    3    4
```

**F. Mettre en place le cadre de suivi des risques**

Définir :
- **Indicateurs de pilotage** :
  - Pourcentage de mesures en cours/terminées
  - Budget consommé
  - Nombre d'écarts du socle réduits
  - Fréquence des comités de suivi

- **Gouvernance** :
  - Comité de pilotage SSI (tous les 6 mois en montée en puissance, tous les 12 mois en rythme de croisière)
  - Revue des risques résiduels
  - Mise à jour de l'étude selon les cycles stratégique et opérationnel

**G. Mettre en place des mécanismes de surveillance**

- Veille sur les menaces et vulnérabilités
- Surveillance des évolutions de l'écosystème
- Déclencheurs de mise à jour de l'étude :
  - Changement réglementaire
  - Nouvelle vulnérabilité critique
  - Incident de sécurité significatif
  - Évolution majeure du SI

---

## Bibliographie

### Normes ISO

- **ISO 31000:2018** - Management du risque - Principes et lignes directrices
- **ISO 27001:2022** - Systèmes de management de la sécurité de l'information - Exigences
- **ISO 27002:2022** - Mesures de sécurité de l'information
- **ISO 27005:2022** - Préconisation pour la gestion des risques liés à la sécurité de l'information

### Guides ANSSI

- **Guide d'hygiène informatique** - Renforcer la sécurité de son système d'information en 42 mesures (septembre 2017)
- **Cartographie du système d'information** - Guide d'élaboration en 5 étapes (2018)
- **Guides sur la cybersécurité des systèmes industriels** (janvier 2014 et octobre 2016)

---

## Termes et Définitions

### A

**ACTION ÉLÉMENTAIRE** (*Elementary action*)
Action unitaire exécutée par une source de risque sur un bien support dans le cadre d'un scénario opérationnel.
*Exemples : exploiter une vulnérabilité, envoyer un email piégé, effacer des traces, augmenter des privilèges.*

**APPRÉCIATION DES RISQUES** (*Risk assessment*)
Ensemble du processus d'identification, d'analyse et d'estimation des risques (ISO 31000:2018). Dans EBIOS RM, cela correspond aux ateliers 2, 3 et 4.

### B

**BESOIN DE SÉCURITÉ** (*Security need*)
Propriété de sécurité à garantir pour une valeur métier. Elle traduit un enjeu de sécurité.
*Exemples : disponibilité, intégrité, confidentialité, traçabilité.*

**BIEN SUPPORT** (*Supporting asset*)
Composante du système d'information sur laquelle repose une ou plusieurs valeurs métier. Peut être de nature numérique, physique ou organisationnelle.
*Exemples : serveur, réseau de téléphonie, passerelle d'interconnexion, local technique, administrateurs.*

**BIEN SUPPORT CRITIQUE** (*Critical supporting asset*)
Bien support jugé très susceptible d'être ciblé par une source de risque pour atteindre son objectif.

### C

**CARTOGRAPHIE DE MENACE NUMÉRIQUE DE L'ÉCOSYSTÈME** (*Ecosystem digital threat mapping*)
Représentation visuelle du niveau de dangerosité numérique des parties prenantes de l'écosystème vis-à-vis de l'objet étudié.

**CARTOGRAPHIE DU RISQUE** (*Risk mapping*)
Représentation visuelle des risques issus des activités d'appréciation du risque (matrice, radar, diagramme de Farmer).

**CHEMIN D'ATTAQUE** (*Attack path*)
Suite d'événements distincts que la source de risque devra probablement générer pour atteindre son objectif. Concerne les scénarios stratégiques.

### E

**ÉCOSYSTÈME** (*Ecosystem*)
Ensemble des parties prenantes en interaction avec l'objet de l'étude.

**ÉVÉNEMENT INTERMÉDIAIRE** (*Intermediate event*)
Dans un scénario stratégique, événement généré par la source de risque à l'égard d'une partie prenante pour faciliter l'atteinte de son objectif.

**ÉVÉNEMENT REDOUTÉ** (*Feared event*)
Événement associé à une valeur métier qui porte atteinte à un critère de sécurité. Chaque événement redouté est estimé selon son niveau de gravité.

### G

**GRAVITÉ** (*Severity*)
Estimation du niveau et de l'intensité des effets d'un risque. Mesure des impacts préjudiciables perçus.
*Exemples : négligeable, mineure, majeure, critique, maximale.*

### H

**HOMOLOGATION DE SÉCURITÉ** (*Security accreditation*)
Validation par une autorité d'homologation que le niveau de sécurité atteint est conforme aux attentes et que les risques résiduels sont acceptés.

### M

**MENACE** (*Threat*)
Terme générique désignant toute intention hostile de nuire dans le cyberespace. Peut être ciblée ou non.

**MESURE DE SÉCURITÉ** (*Security control*)
Moyen de traiter un risque prenant la forme de solutions ou d'exigences. Peut être fonctionnelle, technique ou organisationnelle.

**MISSION** (*Mission*)
Fonction, finalité, raison d'être de l'objet de l'étude.

**MODE OPÉRATOIRE** (*Operating mode*)
Suite d'actions élémentaires que la source de risque devra réaliser pour atteindre son objectif. Concerne les scénarios opérationnels.

### N

**NIVEAU DE DANGEROSITÉ D'UNE PARTIE PRENANTE** (*Threat level of a stakeholder*)
Mesure du potentiel de risque que fait peser une partie prenante sur l'objet de l'étude.

**NIVEAU DE RISQUE** (*Risk level*)
Mesure de l'importance du risque, exprimée par la combinaison de la gravité et de la vraisemblance.

### O

**OBJECTIF VISÉ (OV)** (*Target objective*)
Finalité visée par une source de risque, selon ses motivations.
*Exemples : voler des informations, diffuser un message idéologique, se venger, générer une crise.*

**OBJET DE L'ÉTUDE** (*Subject of study*)
Organisation, système d'information ou produit faisant l'objet de l'appréciation des risques.

### P

**PARTIE PRENANTE** (*Stakeholder*)
Élément en interaction directe ou indirecte avec l'objet de l'étude. Peut être interne ou externe.
*Exemples : partenaire, prestataire, client, fournisseur, filiale.*

**PARTIE PRENANTE CRITIQUE (PPC)** (*Critical stakeholder*)
Partie prenante susceptible de constituer un vecteur d'attaque privilégié.

**PLAN DE TRAITEMENT DU RISQUE** (*Risk management plan*)
Formalise l'ensemble des mesures de traitement du risque à mettre en œuvre.

### R

**RISQUE** (*Risk*)
Possibilité qu'un événement redouté survienne et que ses effets impactent les missions de l'objet de l'étude. Décrit sous forme de scénario de risque.

**RISQUE INITIAL** (*Initial risk*)
Scénario de risque évalué avant application de la stratégie de traitement.

**RISQUE RÉSIDUEL** (*Residual risk*)
Scénario de risque subsistant après application de la stratégie de traitement.

### S

**SCÉNARIO DE RISQUE** (*Risk scenario*)
Scénario complet décrivant un chemin d'attaque et le scénario opérationnel associé, allant de la source de risque à l'objectif visé.

**SCÉNARIO OPÉRATIONNEL** (*Operational scenario*)
Enchaînement d'actions élémentaires portées sur les biens supports. Estimé en termes de vraisemblance.

**SCÉNARIO STRATÉGIQUE** (*Strategic scenario*)
Chemin d'attaque allant d'une source de risque à un objectif visé en passant par l'écosystème. Estimé en termes de gravité.

**SOURCE DE RISQUE (SR)** (*Risk origin*)
Élément, personne, groupe de personnes ou organisation susceptible d'engendrer un risque. Caractérisée par sa motivation, ses ressources, ses compétences, ses modes opératoires.
*Exemples : organismes étatiques, activistes, concurrents, collaborateurs.*

### V

**VALEUR MÉTIER** (*Business asset*)
Composante importante pour l'organisation dans l'accomplissement de sa mission. Représente le patrimoine informationnel qu'une source de risque aurait intérêt à attaquer.
*Exemples : service client, informations clients, données R&D, savoir-faire métier.*

**VRAISEMBLANCE** (*Likelihood*)
Estimation de la faisabilité ou de la probabilité qu'un risque se réalise.
*Exemples : très faible, peu vraisemblable, vraisemblable, quasi certain.*

**VRAISEMBLANCE ÉLÉMENTAIRE** (*Elementary likelihood*)
Vraisemblance d'une action élémentaire identifiée dans un scénario opérationnel.

**VULNÉRABILITÉ** (*Vulnerability*)
Faute dans les spécifications, la conception, la réalisation, l'installation ou la configuration d'un système, ou dans la façon de l'utiliser. Peut être exploitée et conduire à une intrusion.

---

## EBIOS Risk Manager - Évolutions

Cette mise à jour d'EBIOS Risk Manager (version 1.5) intègre des évolutions issues des retours d'expérience de l'ANSSI et des praticiens de la méthode. Ces évolutions ont pour vocation de rendre la méthode pleinement conforme à la norme ISO/CEI 27005:2022.

### Évolutions de vocabulaire par rapport à l'édition 2018

| Ancienne terminologie | Nouvelle terminologie |
|----------------------|----------------------|
| Plan d'Amélioration Continue de la Sécurité | Plan de Traitement du Risque |
| Entité ou personne responsable | Propriétaire |
| Synthèse des scénarios de risque | Évaluations des risques |

---

*Document publié sous Licence Ouverte/Open Licence (Etalab - V1)*

**AGENCE NATIONALE DE LA SÉCURITÉ DES SYSTÈMES D'INFORMATION**
ANSSI - 51, boulevard de la Tour-Maubourg - 75 700 PARIS 07 SP
www.cyber.gouv.fr
