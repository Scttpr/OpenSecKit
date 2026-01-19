# Recommandation cookies et autres traceurs - Modalités pratiques de mise en conformité

**Source:** CNIL - Délibération n° 2020-092 du 17 septembre 2020
**URL:** https://www.cnil.fr/sites/cnil/files/atoms/files/recommandation-cookies-et-autres-traceurs.pdf
**Base légale:** Article 82 de la loi Informatique et Libertés

---

## Introduction

Cette recommandation complète les lignes directrices (Délibération n° 2020-091) et propose des **modalités pratiques de recueil d'un consentement conforme** aux règles applicables.

**Important :** Cette recommandation n'est ni prescriptive ni exhaustive. D'autres méthodes de recueil du consentement peuvent être utilisées, dès lors qu'elles permettent d'obtenir un consentement conforme aux textes en vigueur.

---

## Article 1 - Périmètre de la recommandation

### 1.1 Acteurs concernés

Tous les organismes qui recourent à des traceurs, tels que définis dans les lignes directrices du 17 septembre 2020.

### 1.2 Environnements concernés

| Environnement | Couvert |
|---------------|---------|
| Sites web | Oui |
| Applications mobiles | Oui |
| Télévision connectée | Applicable par analogie |
| Consoles de jeux vidéo | Applicable par analogie |
| Assistants vocaux | Applicable par analogie |
| Objets connectés | Applicable par analogie |
| Véhicules connectés | Applicable par analogie |

**Univers logués et non logués :** La recommandation concerne les deux. Le fait que les utilisateurs soient authentifiés ne dispense pas de recueillir leur consentement.

---

## Article 2 - Information, consentement et refus

### Principe général

Lorsqu'aucune exception de l'article 82 n'est applicable, les utilisateurs doivent :
1. Recevoir une information conforme à l'article 82 (complétée par le RGPD le cas échéant)
2. Se voir indiquer les conséquences de leur choix

**Définition du refus :** Toute inaction ou action autre qu'un acte positif signifiant le consentement doit être interprétée comme un refus. Aucune opération de lecture ou d'écriture soumise au consentement ne peut alors avoir lieu.

### Recommandations générales

- S'assurer que les utilisateurs prennent la pleine mesure des options via le design et l'information
- Encourager le développement d'interfaces standardisées avec un vocabulaire uniformisé

---

### 2.1 Information sur les finalités des traceurs

Les finalités doivent être :
- Présentées **avant** que les utilisateurs puissent consentir ou refuser
- Formulées de manière **intelligible**, dans un langage adapté et suffisamment clair
- Mises en exergue dans un **intitulé court** et mis en évidence, accompagné d'un bref descriptif

#### Exemples de formulations par finalité

| Finalité | Exemple de formulation |
|----------|------------------------|
| **Publicité personnalisée** | « [nom du site/app] [et nos partenaires] utilise(nt) des traceurs afin d'afficher de la publicité personnalisée en fonction de votre navigation et de votre profil » |
| **Publicité non personnalisée** | « [nom du site/app] [et nos partenaires] utilise(nt) des traceurs dans le but de mesurer l'audience de la publicité, sans vous profiler » |
| **Publicité géolocalisée** | « [nom du site/app] [et nos partenaires] utilise(nt) des traceurs pour vous adresser de la publicité en fonction de votre localisation » |
| **Personnalisation de contenu** | « Notre site/app [et des sociétés tierces] utilise(nt) des traceurs pour personnaliser le contenu éditorial en fonction de votre utilisation » |
| **Partage sur les réseaux sociaux** | « Notre site/app utilise des traceurs pour vous permettre de partager du contenu sur les réseaux sociaux ou plateformes présents » |

#### Description détaillée des finalités

La Commission recommande de faire figurer, en complément du premier écran, une **description plus détaillée** des finalités, accessible via :
- Un bouton de déroulement activable directement
- Un lien hypertexte présent au premier niveau d'information

**Exemple de détail pour la publicité :**
- Sélection d'une publicité en fonction du profil
- Plafonnement de l'affichage (« capping publicitaire »)
- Lutte contre la « fraude au clic »
- Facturation de la prestation d'affichage
- Mesure des cibles ayant plus d'appétences à la publicité

---

### 2.2 Information sur la portée du consentement

Lorsque des traceurs tiers permettent un suivi de la navigation **au-delà du site ou de l'application** où ils sont initialement déposés, la Commission recommande fortement que le consentement soit recueilli sur **chacun des sites ou applications** concernés.

---

### 2.3 Information sur l'identité des responsables de traitement

**Obligation :** Les utilisateurs doivent pouvoir prendre connaissance de l'identité de **l'ensemble des responsables** du ou des traitements (y compris responsables conjoints) **avant** de donner leur consentement ou de refuser.

**Mise en pratique :**

| Niveau | Contenu |
|--------|---------|
| **Premier niveau** | Nombre de responsables impliqués (optionnel mais recommandé) |
| **Second niveau** | Liste exhaustive et régulièrement mise à jour (identité, lien vers politique de confidentialité) |

**Recommandations :**
- Utiliser une dénomination descriptive : « liste des sociétés utilisant des traceurs sur notre site/application »
- Mettre la liste à disposition de manière permanente (icône statique « cookie », lien en bas/haut de page)
- Regrouper les responsables par catégories selon leur activité et la finalité des traceurs

---

### 2.4 L'expression du consentement ou du refus

#### Caractère univoque du consentement

Le consentement doit se manifester par un **acte positif clair**.

**Moyens recommandés :**
- Cases à cocher, **décochées par défaut**
- Interrupteurs (« sliders »), **désactivés par défaut**, si le choix exprimé est aisément identifiable

**Attention :** L'information accompagnant chaque élément actionnable doit être facilement compréhensible et ne pas nécessiter d'efforts de concentration ou d'interprétation. Elle ne doit pas être rédigée de manière qu'une lecture rapide laisse croire que l'option produit l'inverse de ce que l'utilisateur pensait choisir.

#### Caractère libre du consentement

**Principe :** Demander le consentement de façon indépendante et spécifique pour chaque finalité distincte.

**Toutefois :** Il est possible de proposer un consentement global à un ensemble de finalités, **sous réserve de présenter au préalable l'ensemble des finalités**.

**Boutons acceptables au premier niveau :**
- « Tout accepter » et « Tout refuser »
- « J'autorise » et « Je n'autorise pas »
- « J'accepte tout » et « Je n'accepte rien »

**Choix granulaire :** Inclure un bouton sur le même niveau permettant d'accéder au choix finalité par finalité :
- « Personnaliser mes choix »
- « Décider par finalité »

#### Modalités du refus

**Principe fondamental :** Le responsable de traitement doit offrir la possibilité d'accepter **et** de refuser avec le **même degré de simplicité**.

**Recommandation forte :** Le mécanisme permettant d'exprimer un refus doit être accessible sur le **même écran** et avec la **même facilité** que le mécanisme permettant d'exprimer un consentement.

**Risque identifié :** Les interfaces nécessitant un seul clic pour consentir tandis que plusieurs actions sont nécessaires pour « paramétrer » un refus présentent, dans la plupart des cas, le risque de biaiser le choix de l'utilisateur.

**Exemple conforme :** Deux boutons présentés au même niveau et sur le même format :
- « Tout accepter » / « Tout refuser »
- « Autoriser » / « Interdire »
- « Consentir » / « Ne pas consentir »

**Autres modalités de refus :**
- Fermeture de la fenêtre de recueil du consentement
- Absence d'interaction pendant un certain laps de temps
- Lien « Continuer sans accepter »

**Important :** Ces possibilités doivent être clairement indiquées aux utilisateurs sur la fenêtre.

#### Pratiques de design à éviter

La Commission recommande de s'assurer que les interfaces :
- N'intègrent pas de pratiques de design potentiellement trompeuses
- Ne laissent pas penser que le consentement est obligatoire
- Ne mettent pas visuellement plus en valeur un choix plutôt qu'un autre

**Recommandation :** Utiliser des boutons et une police d'écriture de **même taille**, offrant la **même facilité de lecture**, et mis en évidence de **manière identique**.

#### Conservation des choix

| Recommandation | Détail |
|----------------|--------|
| **Pendant la navigation** | Conserver les choix pour éviter une nouvelle demande à chaque page |
| **Disparition du bandeau** | Si le refus peut être manifesté par la poursuite de la navigation, le bandeau doit disparaître rapidement |
| **Durée de conservation recommandée** | **6 mois** (tant le consentement que le refus) |
| **Renouvellement** | Renouveler le recueil du consentement à des intervalles appropriés |

---

## Article 3 - Retrait et gestion du consentement

### Principe

Les utilisateurs ayant donné leur consentement doivent être en mesure de le **retirer à tout moment**.

**Il doit être aussi simple de retirer son consentement que de le donner.**

### Information préalable

Les utilisateurs doivent être informés de manière simple et intelligible, **avant même de donner leur consentement**, des solutions mises à leur disposition pour le retirer.

### Accessibilité du mécanisme de retrait

Les solutions permettant de retirer le consentement doivent être **aisément accessibles à tout moment**.

**Critère de simplicité :** Temps passé et nombre d'actions nécessaires pour effectivement retirer le consentement.

### Exemples de mise en oeuvre

- Lien accessible à tout moment depuis le service concerné
- Dénominations recommandées :
  - « Module de gestion des cookies »
  - « Gérer mes cookies »
  - « Cookies »
- Icône « cookie » située en bas à gauche de l'écran, accessible sur toutes les pages

### Placement du mécanisme

Le mécanisme doit être placé :
- Dans une zone qui attire l'attention des utilisateurs
- Dans des zones où ils s'attendent à le trouver
- Avec des visuels les plus explicites possibles

### Effectivité du retrait

Pour que le retrait soit effectif, il peut être nécessaire de mettre en place des solutions spécifiques pour garantir l'absence de lecture ou d'écriture des traceurs précédemment utilisés.

---

## Article 4 - Preuve du consentement

### Obligation

Les responsables du ou des traitements doivent être en mesure de **démontrer, à tout moment**, que les utilisateurs ont donné leur consentement.

### Cas des cookies tiers

Une simple clause contractuelle engageant une partie à recueillir le consentement pour le compte de l'autre **ne suffit pas** à garantir l'existence d'un consentement valide.

**Recommandation :** La clause doit préciser que l'organisme qui recueille le consentement doit également **mettre à disposition des autres parties la preuve du consentement**.

### Modalités de preuve recommandées

| Modalité | Description |
|----------|-------------|
| **Séquestre du code** | Mettre les différentes versions du code informatique sous séquestre auprès d'un tiers |
| **Hash horodaté** | Publier un condensat (hash) du code de façon horodatée sur une plate-forme publique |
| **Capture d'écran** | Conserver une capture d'écran horodatée du rendu visuel pour chaque version du site ou de l'application |
| **Audits réguliers** | Audits des mécanismes de recueil du consentement mis en oeuvre par des tiers mandatés |
| **Configuration CMP** | Conserver de façon horodatée les informations relatives aux outils et à leurs configurations successives (Consent Management Platform) |

---

## Article 5 - Traceurs exemptés du recueil du consentement

### Principe

L'article 82 n'impose pas d'informer les utilisateurs sur l'existence d'opérations de lecture et écriture **non soumises au consentement préalable**.

**Exemple :** Un cookie de préférence linguistique stockant uniquement la langue préférée de l'utilisateur.

### Recommandation de transparence

Afin d'assurer une transparence pleine et entière, la Commission recommande que les utilisateurs soient **également informés** de l'existence de ces traceurs et de leurs finalités (par exemple dans la politique de confidentialité).

### Traceurs de mesure d'audience exemptés

Pour les traceurs de mesure d'audience exemptés (décrits à l'article 5 des lignes directrices), la Commission recommande :

| Critère | Recommandation |
|---------|----------------|
| **Information** | Informer les utilisateurs via la politique de confidentialité |
| **Durée de vie des traceurs** | Limitée à **13 mois** maximum (sans prorogation automatique lors des nouvelles visites) |
| **Durée de conservation des données** | **25 mois** maximum |
| **Examen périodique** | Des durées de vie et de conservation |

---

## Article 6 - Mesures techniques pour accroître la transparence

### Cookies distincts par finalité

L'utilisation de cookies différents pour chaque finalité distincte permettrait aux utilisateurs de les distinguer et de s'assurer du respect de leur consentement.

**Recommandation :** Les traceurs exemptés du consentement ne doivent être utilisés que pour **une seule et même finalité**.

### Pratiques à éviter

La Commission incite à **ne pas avoir recours** à des techniques de masquage de l'identité de l'entité utilisant des traceurs, telles que la **délégation de sous-domaine**.

### Nommage des traceurs

**Recommandations :**
- Les noms des traceurs utilisés doivent être **explicites**
- Dans la mesure du possible, **uniformisés** quel que soit l'acteur à l'origine de leur émission

### Cookie de consentement

La Commission encourage les professionnels à nommer le traceur permettant de stocker le choix des utilisateurs **« eu-consent »**, en attachant à chaque finalité une valeur booléenne « vrai » ou « faux » mémorisant les choix effectués.

---

## Résumé des bonnes pratiques

### Premier niveau d'information (bandeau/pop-up)

| Élément | Recommandation |
|---------|----------------|
| Finalités | Intitulé court + bref descriptif pour chaque finalité |
| Boutons | « Tout accepter » et « Tout refuser » au même niveau et même format |
| Personnalisation | Bouton « Personnaliser mes choix » au même niveau |
| Responsables | Nombre ou lien vers la liste |
| Refus alternatif | Si applicable, indiquer clairement « Continuer sans accepter » ou fermer la fenêtre |

### Second niveau d'information

| Élément | Recommandation |
|---------|----------------|
| Finalités détaillées | Description complète de chaque finalité |
| Choix granulaire | Cases à cocher ou sliders décochés par défaut, finalité par finalité |
| Liste des responsables | Identité complète + lien vers politique de confidentialité |

### Gestion continue

| Élément | Recommandation |
|---------|----------------|
| Retrait du consentement | Accessible à tout moment (icône cookie, lien permanent) |
| Conservation des choix | 6 mois |
| Durée des traceurs d'audience exemptés | 13 mois |
| Conservation des données d'audience | 25 mois |
| Preuve du consentement | Hash horodaté, captures d'écran, audits |
