# Référentiel Général de Sécurité - Annexe B3

## Authentification

### Règles et recommandations concernant les mécanismes d'authentification

**Version 1.0 du 13 janvier 2010**

---

## Historique des versions

| Date | Version | Évolution du document |
|------|---------|----------------------|
| 13 janvier 2010 | 1.0 | Intégration dans le Référentiel général de sécurité (en tant qu'annexe B3) |

**Contact :**
- Agence nationale de la sécurité des systèmes d'information
- Sous-direction Assistance, Conseil et Expertise
- SGDSN/ANSSI/ACE
- 51 boulevard de La Tour-Maubourg
- 75700 Paris 07 SP

---

## Table des matières

- [A. Introduction](#a-introduction)
  - [A.1. Contexte](#a1-contexte)
  - [A.2. Modèle de la fonction d'authentification](#a2-modèle-de-la-fonction-dauthentification)
- [B. Règles et recommandations](#b-règles-et-recommandations)
  - [B.1. Authentification de machines](#b1-authentification-de-machines)
  - [B.2. Authentification de personnes](#b2-authentification-de-personnes)
- [C. Guide d'interprétation dans certains cas particuliers](#c-guide-dinterprétation-dans-certains-cas-particuliers)

---

## A. Introduction

### A.1. Contexte

#### A.1.a. Objectif du document

L'objectif de ce document est de présenter une modélisation permettant de décrire ou d'évaluer les mécanismes d'authentification et de conseiller sur les « meilleures pratiques » à suivre en matière d'authentification lors de l'élaboration d'un système d'information.

Ce document est principalement destiné aux développeurs de produits de sécurité utilisant des fonctions d'authentification pour les aider à réaliser ces fonctions de sécurité.

La lecture de ce document présuppose que le lecteur est familier avec les concepts utilisés en cryptographie et particulièrement, ceux exposés dans « règles et recommandations concernant le choix et le dimensionnement des mécanismes cryptographiques ».

#### A.1.b. Rôle de l'authentification

L'authentification a pour but de vérifier l'identité dont une entité (personne ou machine) se réclame. L'authentification est toujours précédée ou combinée avec une identification qui permet à cette entité de se faire reconnaître du système par un élément dont on l'a doté : un identifiant.

En résumé :
- **S'identifier** : communiquer un identifiant présumé
- **S'authentifier** : apporter la preuve que l'entité s'est vue attribuer cet identifiant

Ce document ne traite que la fonction d'authentification. L'identification est considérée comme acquise.

**L'authentification vise :**
- soit à contrôler l'accès à des informations, des locaux, plus généralement des biens d'un système d'information, en étant associée à une fonction d'attribution de privilèges particuliers liés à l'identité de l'entité ;
- soit à garantir une imputabilité avec vérification forte de l'identité affichée (journalisation d'actions, facturation, authentification de données, etc.) ;
- soit à assurer une combinaison de ces fonctions d'attribution de privilèges et d'imputation.

#### A.1.c. Typologie des fonctions d'authentification

On distingue deux grands types de solutions :
1. **L'acte signé** : le lien entre l'authentification et l'action est direct et intemporel
2. **La session authentifiée** : l'authentification intervient ponctuellement en début de session et nécessite une traçabilité entre l'ouverture et le déroulement de la session

> **Note :** La problématique liée à la signature électronique n'est pas l'objet de ce document.

#### A.1.d. Positionnement du document

Ce document complète le document « règles et recommandations concernant le choix et le dimensionnement des mécanismes cryptographiques » édité par l'ANSSI, en particulier son paragraphe « 2.2.4 authentification d'entités ».

**Définitions :**

> **Règles** : définissent des principes qui doivent a priori être suivis par tout mécanisme. L'observation de ces règles est une condition généralement nécessaire, mais non suffisante, à la reconnaissance du bon niveau de sécurité du mécanisme.

> **Recommandations** : ont pour but de guider dans le choix de certains mécanismes d'authentification permettant un gain important en terme de sécurité. Leur application peut être modulée en fonction d'autres impératifs (contraintes de performance, d'ergonomie ou de coût).

#### A.1.e. Organisation du document

- Les concepts généraux de modélisation de l'authentification sont présentés au paragraphe A.2
- L'ensemble des règles et recommandations sont regroupées dans le chapitre B
- Les règles et recommandations sont repérées selon la codification : **Règle** ou **Recom** suivi du domaine d'application et d'un chiffre

#### A.1.f. Mise à jour du document

Ce document a vocation à être révisé régulièrement pour tenir compte des évolutions constantes des menaces et des possibilités technologiques.

---

### A.2. Modèle de la fonction d'authentification

#### A.2.a. Préambule

Il est habituel de faire reposer l'authentification sur un ou plusieurs éléments parmi :
- **Ce que l'on sait** (par exemple, un mot de passe)
- **Ce que l'on a** (par exemple, une carte à puce)
- **Ce que l'on est** (par exemple, une empreinte digitale)
- **Ce que l'on sait faire** (par exemple, une signature manuscrite)

Les deux derniers éléments sont clairement anthropomorphes et ne s'appliquent pas à des dispositifs automatiques. On distingue donc deux modèles :
1. Authentification entre machines
2. Authentification d'une personne vis-à-vis d'une machine

#### A.2.b. Modèle général du processus d'authentification

##### A.2.b.1. Définition des notions

La réalisation des fonctions contrôle d'accès et imputation fait intervenir :
- Un **demandeur** : souhaite effectuer des actions et doit prouver son identité
- Un **receveur** : peut permettre les actions après vérification de l'identité

La suite des **actions** circule sur un **canal** reliant le demandeur au receveur. L'**authentification** permet de relier de façon fiable les actions circulant sur ce canal à l'**identité** du demandeur.

Le temps d'exploitation du canal constitue une **session authentifiée**. Cette session peut se terminer :
- à l'initiative du demandeur
- à l'initiative du receveur, s'il estime ne plus pouvoir garantir le lien entre les actions et l'identité

##### A.2.b.2. États constitutifs d'une authentification

On distingue :
1. **État initial** (non authentifié) : le receveur interdit les actions
2. **Phase de connexion** : contrôle de l'identité du demandeur par le receveur
3. **État authentifié** : session authentifiée pendant laquelle les actions sont autorisées
4. **Phase de déconnexion** : retour à l'état initial

Tous les états peuvent potentiellement engendrer une erreur qui peut générer une **alarme**.

#### A.2.c. Applications du modèle général

##### A.2.c.1. Authentification de machines

###### A.2.c.1.1. Modèle d'authentification de machines

Trois entités sont distinguées :
- **L'environnement de confiance local** (demandeur)
- **Le SI distant** (receveur)
- **Le SI d'authentification de confiance** (optionnel)

Si un SI d'authentification de confiance est présent :
1. Le SI distant doit lui faire confiance pour authentifier l'environnement de confiance local
2. L'environnement de confiance local lui fait confiance pour la protection des éléments d'authentification transmis

###### A.2.c.1.2. Règles et recommandations applicables à l'authentification de machines

Les règles et recommandations font l'objet du paragraphe B.1. Elles s'appliquent au canal de transmission entre l'environnement de confiance local et le SI distant qui est réputé non sûr (menaces d'interception, d'altération, d'écoute, de rejeu, etc.).

##### A.2.c.2. Authentification d'une personne vis-à-vis d'une machine

###### A.2.c.2.1. Modèle d'authentification d'une personne vis-à-vis d'une machine

L'authentification d'une personne vis-à-vis d'un système d'information est délicate à réaliser de façon directe. Du point de vue de la machine, seul un procédé de nature cryptographique s'avère sûr, tandis que la personne ne peut directement employer un tel mécanisme.

Les procédés « d'authentification » directe d'une personne se caractérisent tous par la possibilité de rejeu. Ces procédés sont qualifiés de **déverrouillage**.

**Exemples de procédés de déverrouillage :**
- La saisie d'un mot de passe (déverrouille un ordinateur)
- La présentation d'un badge personnel
- L'insertion d'un support amovible
- La saisie d'un PIN code (active les fonctionnalités d'une carte à puce)
- La reconnaissance d'une caractéristique biométrique

###### A.2.c.2.2. Règles et recommandations applicables

Les règles et recommandations font l'objet du paragraphe B.2. L'authentification de l'utilisateur vis-à-vis du système distant résulte de ces règles et des procédures d'enregistrement et de gestion des clés.

##### A.2.c.3. Authentification de personnes de bout-en-bout

L'authentification de bout-en-bout de deux personnes ne nécessite pas de règle supplémentaire. Elle peut être modélisée en symétrisant le modèle précédent.

---

## B. Règles et recommandations

### B.1. Authentification de machines

#### B.1.a. Mécanismes cryptographiques

L'utilisation de mécanismes cryptographiques robustes est indispensable pour :
- Éviter l'usurpation d'identité
- Éviter le rejeu d'une authentification

Ils mettent en œuvre une preuve de possession d'un élément secret (clé cryptographique) par l'intermédiaire d'un protocole d'authentification garantissant la confidentialité de l'élément secret.

Une propriété recherchée : il doit être impossible pour un attaquant, même s'il récupère les données secrètes d'authentification, de déchiffrer ou de modifier les communications d'une session qu'il n'a pas ouverte.

> **Note :** Les mécanismes utilisant des mots de passe ou des procédés biométriques ne sont pas de nature cryptographique. Ils sont qualifiés de mécanismes de déverrouillage.

> **Note :** Le simple chiffrement des données transmises n'est pas suffisant pour empêcher le rejeu.

---

**RègleProtocole**

> L'authentification entre deux machines doit faire intervenir un protocole cryptographique interactif d'authentification conforme au référentiel général de sécurité.

**Justification :**
- L'authentification de deux machines est un processus automatique qui doit s'appuyer sur un protocole interactif pour être sûr
- Entre deux machines, seul un procédé cryptographique permet d'éviter l'usurpation d'identité
- La simple présentation d'un élément, même signé, ne constitue pas un mécanisme d'authentification robuste du fait des possibilités de rejeu

---

#### B.1.b. Gestion de clés

Un mécanisme cryptographique nécessite une gestion des clés cryptographiques. En matière d'authentification, la gestion des clefs doit permettre d'interdire à un environnement de confiance local corrompu de se connecter sans perturber le fonctionnement du SI distant.

---

**RègleGestionClés**

> L'authentification entre deux machines doit faire intervenir une architecture de gestion des clés du protocole cryptographique utilisé conforme au référentiel général de sécurité.

**Justification :**
- Comme un procédé cryptographique est indispensable, la gestion de ses clés est nécessaire et doit viser un niveau de sécurité cohérent

---

#### B.1.c. États du processus d'authentification

##### B.1.c.1. Connexion

###### B.1.c.1.1. Authentification intrinsèque

---

**RecomAuthIntrinsèque**

> Il est recommandé que dans un système d'authentification entre deux machines, la défaillance du SI distant ou sa prise de contrôle par un adversaire malfaisant ne permette pas l'accès direct aux actions contrôlées.

**Justification :**
- Cette recommandation vise à réduire, voire supprimer le caractère potentiellement névralgique du SI distant

> **Exemple :** Si on cherche à protéger l'accès à des fichiers, on peut chiffrer ces derniers à l'aide d'une clé contenue dans le dispositif d'authentification.

---

###### B.1.c.1.2. Tiers de confiance

Il faut, dans la mesure du possible, éviter l'authentification « transitive » qui cumule les sources de vulnérabilités. Toutefois, ce type d'authentification présente des intérêts en termes d'administration de la sécurité.

---

**RègleTiersDeConfiance-1**

> Si un tiers de confiance est utilisé de façon directe pour une authentification entre deux machines, alors les mécanismes d'authentification du système local vis-à-vis de ce tiers de confiance doivent être conformes au référentiel général de sécurité.

---

**RègleTiersDeConfiance-2**

> Si un tiers de confiance est utilisé de façon directe pour une authentification entre deux machines, alors les mécanismes d'authentification du système distant vis-à-vis de ce tiers de confiance doivent être conformes au référentiel général de sécurité.

---

**RecomTiersDeConfiance**

> Si un tiers de confiance est utilisé de façon directe pour une authentification entre deux machines, alors il est recommandé que les mécanismes d'authentification du système distant vis-à-vis de ce tiers de confiance respectent de plus l'ensemble des recommandations du référentiel général de sécurité.

**Justification :**
- La réponse du tiers de confiance doit être authentifiée avec un niveau de sécurité supérieur car le tiers de confiance, s'il est usurpé, donne la possibilité d'usurper toute identité

---

##### B.1.c.2. Session authentifiée

Si l'authentification sert à établir un accès à des données confidentielles, le canal ouvert doit être protégé en intégrité et en confidentialité.

---

**RecomConfidentialité**

> Si une authentification est utilisée pour contrôler l'accès à des données confidentielles, alors il est recommandé que la session authentifiée permette la mise en place d'un mécanisme cryptographique conforme au référentiel général de sécurité, assurant la confidentialité et l'intégrité de ces données.

**Justification :**
- L'authentification nécessitant l'emploi d'un protocole cryptographique, il y a un réel avantage à en profiter pour chiffrer les données dont l'accès est contrôlé

---

##### B.1.c.3. Déconnexion

###### B.1.c.3.1. Effacement

---

**RègleEffacement**

> À la déconnexion d'une session authentifiée, si des éléments secrets ont été échangés lors de la phase d'authentification, ils doivent être effacés.

**Justification :**
- Il convient de bien insister sur la nécessité d'un effacement physique : les données correspondantes ne doivent plus être présentes

> **Note :** La suppression des moyens d'accéder à ces données (pointeurs) n'est pas suffisante. On prendra soin de mettre l'ensemble des mémoires correspondantes à zéro et de vider les éventuels tampons intermédiaires.

---

**RecomMémoireVolatile**

> Il est recommandé que les éléments secrets échangés lors de la connexion d'une session authentifiée soient uniquement stockés en mémoire volatile et jamais sur un support magnétique.

**Justification :**
- L'effacement de données stockées sur un support magnétique est très délicat à mettre en œuvre
- L'utilisation d'une mémoire volatile garantit que si le processus est interrompu par une panne, les éléments sensibles ne seront pas compromis
- Éviter que les pages mémoire utilisées puissent être stockées sur le disque (swap)

---

###### B.1.c.3.2. Inactivité

---

**RecomInactivité**

> Au cours d'une session authentifiée, il est recommandé d'incorporer un dispositif de déconnexion automatique en cas d'inactivité.

**Justification :**
- L'inactivité d'une session authentifiée est la première étape de scénarios d'attaque classique d'usurpation de session

---

#### B.1.d. Audit

---

**RègleAudit**

> Toute erreur survenant au cours d'une session authentifiée doit générer une trace d'alarme ne pouvant être modifiée ni effacée.

---

**RecomAudit**

> Il est recommandé que toute transition d'état survenant au cours d'une session authentifiée génère une trace d'alarme ne pouvant être modifiée ni effacée.

**Justification :**
- Le minimum requis est de pouvoir tracer des tentatives d'authentification ayant conduit à des anomalies
- En cas d'enquête, il peut être utile de disposer des traces d'authentification réussie

---

### B.2. Authentification de personnes

> **Note :** Les règles et recommandations ci-dessous ne s'appliquent pas à la problématique de la signature électronique.

#### B.2.a. Utilisation d'un environnement de confiance local

---

**RègleAuthentification**

> L'authentification d'un utilisateur auprès d'un SI distant doit faire intervenir un environnement de confiance local déverrouillé par l'utilisateur et réalisant, pour son compte, une authentification de machine à machine conforme au référentiel général de sécurité.

**Justification :**
- La simple utilisation à distance d'un mécanisme de déverrouillage ne saurait constituer un dispositif d'authentification
- Le simple chiffrement des données transmises n'est pas suffisant pour empêcher le rejeu
- La faiblesse intrinsèque d'un mécanisme de déverrouillage réside dans le fait que l'utilisateur ne peut que répéter une même opération à chaque occurrence (vulnérabilité au hameçonnage)

> **Note sur l'entropie des mots de passe :** Des mots de passe de 8 caractères alphanumériques ne permettent pas de générer des clés de plus de 47 bits, ce qui ne permet pas d'atteindre un niveau conforme au référentiel.

---

**RecomPérimètre**

> Dans une authentification distante d'un utilisateur, il est recommandé que le périmètre physique de l'environnement de confiance local utilisé pour réaliser l'authentification de machine avec le SI distant reste sous le contrôle de l'utilisateur.

**Justification :**
- L'environnement de confiance local accède aux informations de déverrouillage propres à l'utilisateur
- Les informations de déverrouillage doivent être considérées comme des données personnelles

---

**RecomCloisonnement**

> Dans une authentification distante d'un utilisateur, il est recommandé que les fonctions employées par l'environnement de confiance local pour réaliser l'authentification de machine avec le SI distant soient cloisonnées des autres fonctions de l'environnement de confiance local.

**Justification :**
- L'authentification de personnes repose sur des mécanismes de déverrouillage qui permettent le rejeu
- Les vulnérabilités éventuelles des autres fonctions ne doivent pas compromettre les informations sensibles de l'authentification

---

**RecomCloisonnementPhysique**

> Dans une authentification distante d'un utilisateur, il est recommandé que l'utilisation d'un support physique amovible soit indispensable à l'environnement de confiance local pour utiliser les clés cryptographiques nécessaires à l'authentification de machines avec le SI distant.

**Justification :**
- Il est raisonnable que l'utilisateur ait le moyen de conserver sous son contrôle physique les éléments déterminants de l'authentification

---

**RecomInterdictionAccèsClés**

> Dans une authentification distante d'un utilisateur, il est recommandé que les clés cryptographiques employées par l'environnement de confiance local pour réaliser l'authentification de machine avec le SI distant ne puissent être extraites par l'utilisateur.

**Justification :**
- La connaissance du secret de déverrouillage ne doit pas permettre d'activer des fonctions d'accès aux éléments secrets d'authentification
- L'utilisateur légitime ne doit pas pouvoir retrouver ses éléments secrets d'authentification

---

#### B.2.b. Mécanismes de déverrouillage

---

**RègleDéverrouillage**

> L'environnement de confiance local intervenant dans l'authentification d'un utilisateur auprès d'un SI distant doit nécessiter un déverrouillage par l'utilisateur avant de pouvoir réaliser, pour son compte, une authentification de machine à machine conforme au référentiel général de sécurité.

**Justification :**
- L'environnement de confiance local doit être protégé par un dispositif de verrouillage afin qu'il ne puisse pas être utilisé à l'insu de l'utilisateur

---

**RecomDéverrouillageLocal**

> Il est recommandé que l'environnement de confiance local intervenant dans l'authentification d'un utilisateur auprès d'un SI distant gère de façon autonome son mécanisme de déverrouillage.

**Justification :**
- L'autonomie de l'environnement de confiance local doit être recherchée car la mise en œuvre de tiers est très délicate s'agissant d'un mécanisme intrinsèquement vulnérable au rejeu

---

**RègleDéverrouillagePersonnel**

> L'activation de l'environnement de confiance local intervenant dans l'authentification d'un utilisateur auprès d'un SI distant nécessite la présentation d'un élément personnel à l'utilisateur légitime.

**Justification :**
- Le mécanisme de verrouillage doit mettre en œuvre un élément caractéristique de la personne authentifiée

---

**RecomDéverrouillagePersonnel**

> Il est recommandé que l'activation de l'environnement de confiance local intervenant dans une authentification d'un utilisateur auprès d'un SI distant nécessite la présentation de **deux** éléments personnels à l'utilisateur légitime.

**Justification :**
- Cette recommandation est en cohérence avec la notion traditionnelle « d'authentification forte », qui préconise de combiner deux mécanismes parmi ce que l'on sait, ce que l'on a, ce que l'on est ou ce que l'on sait faire

---

**RecomSecretDéverrouillage**

> Il est recommandé que l'activation de l'environnement de confiance local intervenant dans l'authentification d'un utilisateur auprès d'un SI distant nécessite la présentation d'un secret connu uniquement de l'utilisateur légitime.

---

**RecomTauxFausseAcceptation**

> Il est recommandé que le mécanisme de déverrouillage de l'environnement de confiance local intervenant dans l'authentification d'un utilisateur auprès d'un SI distant ne puisse pas être contourné par quiconque avec une probabilité de succès supérieure à une chance sur 2¹¹.

**Justification :**
- Cette probabilité correspond à environ une chance sur deux mille
- Cette recommandation s'applique aussi vis-à-vis des administrateurs

> **Exemple :** Un code porteur de 4 chiffres présente une entropie de 4.log₂(10) ≈ 13,29 bits. Si trois codes peuvent être présentés avant blocage, la probabilité de fausse acceptation est de 3 sur 2^13,29 soit environ 2^11,7. Un tel mécanisme est conforme à la recommandation.

---

**RecomCheminSûr**

> Il est recommandé que les informations dont dépend l'activation de l'environnement de confiance local intervenant dans l'authentification d'un utilisateur auprès d'un SI distant soient directement introduites au niveau des fonctions de l'environnement de confiance local qui les exploitent sans possibilité d'écoute ni de perturbation.

**Justification :**
- L'environnement de confiance local devrait disposer d'un accès direct au(x) dispositif(s) d'acquisition des éléments sensibles d'activation

> **Exemple :** La carte à puce avec mot de passe : le mot de déverrouillage peut soit être saisi sur le clavier du poste, soit être introduit directement au niveau du lecteur par l'utilisation d'un clavier intégré. La deuxième solution est plus sûre.

---

#### B.2.c. Audit

---

**RecomAuditPersonnel**

> Il est recommandé que dans une authentification distante d'un utilisateur, ce dernier soit en mesure de consulter de manière sécurisée les audits de ses authentifications, mais sans pouvoir ni les modifier, ni les effacer.

**Justification :**
- L'authentification distante peut mettre en œuvre un support physique amovible dont il peut être intéressant d'auditer l'exploitation
- Ce mécanisme relève du principe de précaution au cas où les mécanismes d'audit de l'authentification de machines seraient contournés

---

## C. Guide d'interprétation dans certains cas particuliers

### C.1. Mot de passe à usage unique

#### C.1.a. Préambule

L'utilisation d'un secret connu de l'utilisateur est le mécanisme de déverrouillage le plus répandu car :
- Facile à mettre en œuvre par l'utilisateur
- Ne nécessite pas de dispositif coûteux

Un mécanisme de déverrouillage souffre du grave défaut intrinsèque d'être rejouable. Les mécanismes de mot de passe à usage unique (OTP) permettent d'atteindre un niveau de sécurité supérieur à celui du mot de passe statique, sans toutefois atteindre un niveau comparable à un protocole d'authentification cryptographique.

**Entropie d'un mot de passe :**

| Caractéristiques | 10 symboles (chiffres) | 26 symboles (lettres) | 62 symboles | 90 symboles |
|-----------------|------------------------|----------------------|-------------|-------------|
| **Longueur** | 4 / 7 / 10 | 8 / 10 / 16 | 8 / 10 / 16 | 8 / 10 / 16 |
| **Taille de clé équivalente (bits)** | 13 / 23 / 33 | 38 / 47 / 75 | 48 / 60 / 95 | 52 / 65 / 104 |
| **Cassage exhaustif possible** | oui / oui / oui | oui / oui / ? | oui / oui / non | oui / oui / non |

#### C.1.b. Description du mécanisme

##### C.1.b.1. Adaptation du modèle d'authentification

La particularité d'un mécanisme d'authentification par mot de passe à usage unique est que l'utilisateur est amené à intervenir pour que son environnement de confiance local puisse réaliser l'authentification de machine.

Le corollaire est que l'environnement de confiance local n'a pas de moyen d'authentifier le SI distant. C'est l'interface de saisie qui doit authentifier le serveur distant par une authentification de machines.

> **Exemple :** Connexion SSL au SI distant avec vérification du certificat du serveur.

##### C.1.b.2. Description du protocole

**Éléments dont disposent les entités :**

Pour l'utilisateur :
- Un identifiant `id`
- Un éventuel mot de passe statique `smdp`
- Un environnement de confiance local (pouvant avoir son propre mécanisme de déverrouillage)

Pour l'environnement de confiance :
- Une clé `Kπ(id)` permettant de « prouver » l'identité
- Éventuellement un compteur interne `t`

Pour le SI distant :
- Éventuellement la valeur `H(id)(smdp)`
- Une clé `Kν(id)` permettant de « vérifier » l'identité
- Éventuellement un compteur interne `t'`
- Éventuellement un générateur de défi aléatoire `chall`

**Étapes du protocole :**

1. **Envoi de l'identifiant** au SI distant, éventuellement accompagné du mot de passe statique `smdp`
   - Intérêt du mot de passe statique : éviter l'énumération des identifiants valides

2. **Vérification** : Le SI distant compare `H(id)(smdp)` avec la valeur de référence
   - La fonction différenciée par identifiant évite les attaques par dictionnaire pré-calculé
   - Le mot de passe statique doit être transmis tel quel (pas H(id)(smdp))

3. **Génération d'un défi aléatoire** par le SI distant (optionnel)
   - Le défi doit être aléatoire (imprédictible)

4. **Retranscription du défi** par l'utilisateur sur son environnement de confiance

5. **Calcul de la réponse** par l'environnement de confiance :
   - À partir de la clé `Kπ(id)`
   - D'un éventuel compteur de synchronisation `t`
   - Du défi `chall` éventuellement reçu

6. **Retranscription de la réponse** par l'utilisateur, transmise au SI distant

7. **Vérification de la réponse** par le SI distant :
   - À partir de la clé `Kν(id)`
   - De son compteur de synchronisation `t'`
   - Du défi envoyé
   - De la réponse reçue

#### C.1.c. Conséquences sur les règles et recommandations à appliquer

##### C.1.c.1. Mécanismes cryptographiques

Les mécanismes cryptographiques utilisés doivent respecter les règles et recommandations concernant le choix et le dimensionnement des mécanismes cryptographiques :

1. Le mécanisme d'authentification de machines entre SI local et SI distant (doit aussi assurer la confidentialité)
2. Les fonctions de preuve `f` et de vérification `φ` (ne doivent pas divulguer d'information sur les clés)

##### C.1.c.2. Gestion des clés

Les clés utilisées doivent être gérées conformément aux règles et recommandations concernant la gestion des clés utilisées dans des mécanismes cryptographiques.

##### C.1.c.3. Cas particulier du défi / réponse

Les contraintes ergonomiques font qu'il n'est pas possible d'employer des données de défi / réponse d'entropie comparable à celle d'un protocole cryptographique. La session authentifiée doit donc être protégée en confidentialité.

###### C.1.c.3.1. Cas particulier de l'absence de synchronisation

Si on repose uniquement sur le défi (sans synchronisation), et qu'un attaquant peut observer des couples défi / réponse, il peut constituer un dictionnaire. La probabilité de réussite est soumise au paradoxe des anniversaires.

> **Exemple :** Si le défi peut être codé par 7 chiffres décimaux (entropie de 23 bits), la probabilité de contournement est de 2^11,5.

###### C.1.c.3.2. Cas général avec synchronisation anti-rejeu

Le compteur de synchronisation peut être considéré comme un compteur anti-rejeu. Sous réserve qu'il ne soit jamais réinitialisé et ne boucle pas, l'entropie de la réponse est le facteur discriminant de la sécurité.

Si le compteur est incrémenté à chaque authentification réussie, il faudra prévoir que l'environnement de confiance puisse être en avance. La fenêtre de synchronisation a un impact sur la sécurité (si on autorise `s` avances, un attaquant a `s` fois plus de chances de réussir).

---

*Annexe B3 au RGS v1.0 - Version 1.0 du 13/01/2010 - PUBLIC*
