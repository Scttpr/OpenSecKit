# Référentiel Général de Sécurité version 2.0

## Annexe B1

### Mécanismes cryptographiques

**Règles et recommandations concernant le choix et le dimensionnement des mécanismes cryptographiques**

**Version 2.03 du 21 février 2014**

*(Annule et remplace la version 1.20 du 26 janvier 2010)*

---

## Historique des versions

| Date | Version | Évolutions du document |
|------|---------|----------------------|
| 19 novembre 2004 | 1.02 N°2791/SGDN/DCSSI/SDS/Crypto | Première version applicable. |
| 19 décembre 2006 | 1.10 N°2741/SGDN/DCSSI/SDS/LCR | Mise à jour planifiée. Principales modifications : prise en compte de la création du référentiel « gestion de clés » ; indication du document décrivant les fournitures nécessaires à l'évaluation des mécanismes cryptographiques ; prise en compte des évolutions de l'état de l'art (algorithmes de chiffrement par flot, modes opératoires, problèmes mathématiques asymétriques, génération d'aléa) ; rajout d'une mention concernant le statut de SHA-1. |
| 26 janvier 2010 | 1.20 | Intégration dans le RGS (en tant qu'annexe B1). Ajout ou modification de règles sur la primalité des ordres ; modification des règles concernant l'architecture et le retraitement pour la génération d'aléa ; ajout de règles concernant les algorithmes symétriques à l'horizon 2020. |
| 18 juin 2012 | 2.00 | Harmonisation du chapitre 1 avec l'annexe B2 ; précisions concernant les mécanismes symétriques et mention de HMAC-SHA-2 ; modification des règles concernant la taille des clés en cryptographie asymétrique pour une utilisation à long terme ; mention de la courbe elliptique FRP256v1 et du protocole ECKCDSA ; ajouts relatifs à l'établissement de clé ; ajout d'une règle concernant la génération d'aléa. |
| 17 octobre 2012 | 2.01 | Précisions concernant l'utilisation d'une mémoire non volatile dans la génération d'aléa. |
| 29 mars 2013 | 2.02 | Modification des règles et recommandations concernant le problème du logarithme discret ; mise à jour des records concernant le problème du logarithme discret ; précisions concernant l'initialisation de l'état interne d'un générateur d'aléa. |
| 21 février 2014 | 2.03 | Précisions concernant l'architecture d'un générateur d'aléa et le retraitement algorithmique ; mise à jour des records concernant le problème du logarithme discret. |

Les commentaires sur le présent document sont à adresser à :

> Agence nationale de la sécurité des systèmes d'information
> Sous-direction Expertise
> ANSSI/SDE
> 51 boulevard de La Tour-Maubourg
> 75700 Paris 07 SP
> crypto@ssi.gouv.fr

---

## Table des matières

- [1. Introduction](#1-introduction)
  - [1.1. Objectif du document](#11-objectif-du-document)
  - [1.2. Positionnement du document](#12-positionnement-du-document)
  - [1.3. Limites du champ d'application du document](#13-limites-du-champ-dapplication-du-document)
  - [1.4. Définition des règles et recommandations](#14-définition-des-règles-et-recommandations)
  - [1.5. Organisation du document](#15-organisation-du-document)
  - [1.6. Mise à jour du document](#16-mise-à-jour-du-document)
- [2. Règles et recommandations](#2-règles-et-recommandations)
  - [2.1. Cryptographie symétrique](#21-cryptographie-symétrique)
  - [2.2. Cryptographie asymétrique](#22-cryptographie-asymétrique)
  - [2.3. Fonctions de hachage](#23-fonctions-de-hachage)
  - [2.4. Génération d'aléa cryptographique](#24-génération-daléa-cryptographique)
  - [2.5. Gestion de clés](#25-gestion-de-clés)
- [Annexe A. Définitions et concepts](#annexe-a-définitions-et-concepts)
- [Annexe B. Éléments académiques de dimensionnement cryptographique](#annexe-b-éléments-académiques-de-dimensionnement-cryptographique)
- [Annexe C. Bibliographie](#annexe-c-bibliographie)

---

## 1. Introduction

### 1.1. Objectif du document

La cryptographie moderne met à la disposition des concepteurs de systèmes d'information des outils permettant d'assurer, ou de contribuer à assurer, des fonctions de sécurité telles que la confidentialité, l'intégrité, l'authenticité et la non-répudiation. Ces outils sont souvent qualifiés d'algorithmes, de primitives ou encore de **mécanismes cryptographiques**.

Suite aux développements majeurs qui ont eu lieu au cours des trois dernières décennies, la science cryptographique, bien qu'encore jeune, semble avoir atteint un degré de maturité suffisant pour permettre de dégager des règles générales concernant le choix et le dimensionnement corrects des mécanismes. Ce document vise à expliciter ces règles ainsi que certaines recommandations.

### 1.2. Positionnement du document

Ce document traite des règles et recommandations concernant le choix et le dimensionnement de mécanismes cryptographiques. Il est complété par deux documents :

- Un premier document intitulé « **Gestion des clés cryptographiques** – Règles et recommandations concernant la gestion des clés utilisées dans des mécanismes cryptographiques » traite plus spécifiquement des aspects liés à la création, la distribution et la manipulation de clés.
- Un second document intitulé « **Authentification** – Règles et recommandations concernant les mécanismes d'authentification » traite plus spécifiquement des aspects liés à l'utilisation de mots de passe, de cartes mémoire, de clés de déverrouillage pour accéder à un système d'information.

Ce document constitue l'annexe B1 du RGS.

### 1.3. Limites du champ d'application du document

Sont explicitement exclus de ce document :

- La recommandation de mécanismes cryptographiques précis permettant d'atteindre les différents niveaux de robustesse cryptographique définis dans ce document, bien que certaines primitives très classiques soient mentionnées ;
- Les aspects liés à l'implantation des mécanismes et en particulier au choix du support ainsi qu'à la sécurité de l'implantation face aux attaques par canaux auxiliaires (timing attack, SPA, DPA, HO-DPA, EMA...) ou par injection de faute (DFA) ;
- Les méthodes d'évaluation des mécanismes cryptographiques ;
- Les méthodes d'analyse de menaces et de développement de produits cryptographiques ;
- Les liens entre niveau de robustesse d'un mécanisme cryptographique et niveau de robustesse d'un produit tel que défini dans les Critères Communs ;
- Les fournitures nécessaires à l'évaluation de mécanismes cryptographiques ;
- Les mécanismes non cryptographiques assurant des fonctions de sécurité (mots de passe, biométrie...).

### 1.4. Définition des règles et recommandations

Les **règles** définissent des principes qui doivent *a priori* être suivis par tout mécanisme. L'observation de ces règles est une condition généralement nécessaire à la reconnaissance du bon niveau de sécurité du mécanisme. Cependant, le fait de suivre l'ensemble des règles, qui sont par nature très génériques, n'est pas suffisant pour garantir la robustesse du mécanisme cryptographique ; seule une analyse spécifique permet de s'en assurer.

En plus des règles, le présent document définit également des **recommandations**. Elles ont pour but de guider le choix de certaines primitives et d'inciter à certains dimensionnements permettant un gain considérable en termes de sécurité, pour un coût souvent modique.

> **Note importante** : Les règles et recommandations contenues dans ce document ne constituent pas un dogme imposé aux concepteurs de produits utilisant des mécanismes cryptographiques. L'objectif est de contribuer à une amélioration constante de la qualité des produits de sécurité. Le suivi des règles énoncées dans ce document doit être considéré comme une démarche saine permettant de se prémunir contre de nombreuses erreurs de conception ainsi que contre d'éventuelles faiblesses non décelées lors de l'évaluation des mécanismes cryptographiques.

### 1.5. Organisation du document

Ce document est organisé de la manière suivante :

- L'ensemble des règles et recommandations sont regroupées dans le **chapitre 2** ; elles sont repérées selon la codification suivante : les premières lettres (Règle ou Recom) indiquent si l'on a affaire à une règle ou une recommandation, le domaine d'application est ensuite précisé et, finalement, un chiffre permet de distinguer les règles d'une même catégorie. Par exemple, **RègleFact-3** désigne la règle numéro 3 concernant le problème de la factorisation ;
- Un rappel non mathématique des principaux concepts cryptographiques est proposé dans l'**annexe A** ;
- Des informations issues de publications académiques sur le dimensionnement des mécanismes cryptographiques sont regroupées dans l'**annexe B** ;
- Les références bibliographiques apparaissent dans l'**annexe C**.

Ce document ne comporte volontairement aucun tableau récapitulatif des tailles minimales de paramètres requis. La concision a été privilégiée dans l'expression des règles et recommandations ; vouloir les résumer à une simple valeur numérique serait une grave source d'erreur et de confusion.

### 1.6. Mise à jour du document

Ce document ayant en particulier pour but de fixer des bornes numériques, par exemple en termes de tailles de clés, il convient de le maintenir à jour régulièrement. Une révision tous les deux ans semble à la fois réaliste et suffisante.

---

## 2. Règles et recommandations

### 2.1. Cryptographie symétrique

#### 2.1.1. Taille de clé symétrique

Dans cette section sont définies les propriétés attendues de clés utilisées par des mécanismes symétriques. Dans ce document, la taille d'une clé est le nombre de bits effectifs de cette clé, c'est-à-dire le nombre de bits réellement variables. Par exemple le DES utilise des clés de 64 bits mais seuls 56 de ces bits peuvent être choisis aléatoirement, les 8 bits restants servant de contrôle de parité. C'est pourquoi on considère que les clés DES ont une taille de 56 bits.

Les tailles minimales définies ci-dessous n'ont de valeur que sous l'hypothèse que la meilleure attaque pratique permettant de mettre en défaut le mécanisme symétrique employé consiste à effectuer une recherche exhaustive sur l'espace des clés.

**Règles :**

| Référence | Règle |
|-----------|-------|
| **RègleCléSym-1** | La taille minimale des clés symétriques utilisées jusqu'en 2020 est de **100 bits**. |
| **RègleCléSym-2** | La taille minimale des clés symétriques devant être utilisées au-delà de 2020 est de **128 bits**. |

**Recommandation :**

| Référence | Recommandation |
|-----------|----------------|
| **RecomCléSym-1** | La taille minimale recommandée des clés symétriques est de **128 bits**. |

**Justifications :**

- L'estimation de la capacité de calcul que peut rassembler une organisation motivée fait l'objet de beaucoup de controverses. De nombreux indices indiquent cependant que l'emploi de clé de moins de 100 bits semble risqué. Les clés de 56 bits sont clairement insuffisantes et la capacité actuelle à attaquer des clés de 64 bits est aujourd'hui admise.
- Une recherche exhaustive sur des clés de 100 bits demeure difficilement concevable avant plusieurs dizaines d'années.
- L'emploi de clés de 128 bits permet de s'assurer que les attaques génériques par recherche exhaustive seront inopérantes, y compris à assez long terme.

**Remarques :**

- L'emploi de clés de 112 bits, comme dans le cas du triple DES, ne pose pas de problème pratique de sécurité vis-à-vis d'attaques par recherche exhaustive. L'utilisation du triple DES peut cependant être déconseillée pour d'autres raisons, en particulier liées à la taille du bloc (64 bits) insuffisante pour assurer une sécurité pratique avec certains modes opératoires classiques.

#### 2.1.2. Chiffrement symétrique

##### 2.1.2.1. Chiffrement par bloc

Les deux caractéristiques les plus simples d'un mécanisme de chiffrement par bloc sont la taille effective de la clé ainsi que la taille des blocs traités.

**Taille de bloc - Règles :**

| Référence | Règle |
|-----------|-------|
| **RègleBlocSym-1** | La taille minimale des blocs de mécanismes de chiffrement par bloc utilisés jusqu'en 2020 est de **64 bits**. |
| **RègleBlocSym-2** | Pour une utilisation au-delà de 2020, la taille minimale des blocs de mécanismes de chiffrement par bloc est de **128 bits**. |

**Recommandation :**

| Référence | Recommandation |
|-----------|----------------|
| **RecomBlocSym-1** | La taille recommandée des blocs de mécanismes de chiffrement par bloc est de **128 bits**. |

**Justifications :**

- L'emploi de blocs de taille trop petite rend des attaques élémentaires comme la constitution de dictionnaires plus efficaces en pratique que la recherche de la clé secrète. Il est communément admis que la taille d'un bloc doit être d'au moins 64 bits.
- La principale menace est la découverte d'attaques dites « en racine carrée », fondées sur le paradoxe des anniversaires ; certaines attaques deviennent opérationnelles dès que plus de 2^(n/2) blocs de message sont traités, où n désigne la taille en bits du bloc.

**Choix de l'algorithme - Règles :**

| Référence | Règle |
|-----------|-------|
| **RègleAlgoBloc-1** | Pour un algorithme de chiffrement ne devant pas être utilisé après 2020, aucune attaque nécessitant moins de N_op = 2^100 opérations de calcul ne doit être connue. |
| **RègleAlgoBloc-2** | Pour un algorithme de chiffrement dont l'utilisation au-delà de 2020 est envisagée, aucune attaque nécessitant moins de N_op = 2^128 opérations de calcul ne doit être connue. |

**Recommandation :**

| Référence | Recommandation |
|-----------|----------------|
| **RecomAlgoBloc-1** | Il est recommandé d'employer des algorithmes de chiffrement par bloc largement éprouvés dans le milieu académique. |

**Mécanisme conforme au référentiel :**

- L'**AES**, tel qu'il est spécifié dans le FIPS 197, est un mécanisme de chiffrement par bloc conforme au référentiel.

**Remarque sur le triple DES :**

Le triple DES avec deux clés est vulnérable à une attaque à clair connu. La complexité de cette attaque est de 2^80 pour 2^40 couples clairs-chiffrés connus et de 2^100 pour 2^20 couples clairs-chiffrés connus. Selon le cadre d'emploi, l'utilisation du triple DES avec deux clés peut ne pas être conforme au référentiel. En particulier, le contexte d'emploi ne doit pas permettre le chiffrement avec une même clé de plus de 2^20 blocs de message connus d'un attaquant.

**Mode opératoire pour le chiffrement - Règles :**

| Référence | Règle |
|-----------|-------|
| **RègleModeChiff-1** | Au sein du modèle de sécurité correspondant à l'usage du mode de chiffrement, il ne doit exister aucune attaque de complexité inférieure à 2^(n/2) appels de la primitive où n est la taille en bits du bloc. |

**Recommandations :**

| Référence | Recommandation |
|-----------|----------------|
| **RecomModeChiff-1** | L'emploi d'un mode opératoire de chiffrement non déterministe est recommandé. |
| **RecomModeChiff-2** | L'utilisation d'un mode opératoire de chiffrement se fera de préférence conjointement à l'utilisation d'un mécanisme d'intégrité. |
| **RecomModeChiff-3** | On utilisera de préférence des modes opératoires disposant d'une preuve de sécurité. |

**Mécanisme conforme au référentiel :**

- Le mode de chiffrement **CBC** utilisant une primitive de chiffrement conforme au référentiel comme l'AES et des valeurs initiales aléatoirement choisies pour chaque message et transmises en clair est un mécanisme de chiffrement symétrique conforme au référentiel. Il est particulièrement important de garantir que les valeurs initiales sont générées dans le périmètre de sécurité du chiffrement et avec un générateur d'aléa sûr. Elles ne doivent en aucun cas pouvoir être contrôlées ou prédites par un attaquant.

##### 2.1.2.2. Chiffrement par flot

**Règles :**

| Référence | Règle |
|-----------|-------|
| **RègleChiffFlot-1** | Pour un algorithme de chiffrement par flot ne devant pas être utilisé après 2020, aucune attaque nécessitant moins de 2^100 opérations de calcul ne doit être connue. |
| **RègleChiffFlot-2** | Pour un algorithme de chiffrement par flot devant être utilisé après 2020, aucune attaque nécessitant moins de 2^128 opérations de calcul ne doit être connue. |

**Recommandations :**

| Référence | Recommandation |
|-----------|----------------|
| **RecomChiffFlot-1** | Il est recommandé d'employer des primitives de chiffrement par bloc et non des algorithmes de chiffrement par flot dédiés. Il est ainsi possible, si les propriétés du chiffrement par flot sont requises, d'utiliser un mode opératoire par flot de chiffrement par bloc conforme au référentiel et simulant un chiffrement par flot. |
| **RecomChiffFlot-2** | En cas d'utilisation d'un algorithme de chiffrement par flot, il est recommandé d'employer des algorithmes de chiffrement par flot largement éprouvés dans le milieu académique. |

> **Note importante** : Les règles et recommandations en termes de chiffrement par flot se fondent sur le constat qu'à ce jour la recherche académique dans ce domaine ne semble pas posséder la même maturité que dans le domaine des primitives de chiffrement par bloc. Ces règles sont cependant susceptibles d'être revues si des avancées théoriques importantes sont effectuées.

#### 2.1.3. Authentification et intégrité de messages

**Règles :**

| Référence | Règle |
|-----------|-------|
| **RègleIntegSym-1** | Les méthodes symétriques d'intégrité les plus classiques se basent sur des mécanismes de chiffrement par bloc ou de hachage. De telles primitives doivent être conformes au référentiel. |
| **RègleIntegSym-2** | Il ne doit pas exister d'attaque sur le mécanisme d'intégrité utilisant moins de 2^(n/2) appels à la primitive sous-jacente où n est la taille de sortie de cette primitive. |

**Recommandation :**

| Référence | Recommandation |
|-----------|----------------|
| **RecomIntegSym-1** | On utilisera de préférence des mécanismes disposant d'une preuve de sécurité. |

**Remarques importantes :**

- Par confusion avec les modes opératoires de chiffrement, l'emploi de « valeurs initiales » est parfois constaté pour des mécanismes d'intégrité tels que le CBC-MAC ; de graves failles de sécurité peuvent en découler.
- **L'emploi de clés de taille importante ne garantit pas nécessairement une sécurité en rapport avec cette taille.** La plupart des variantes du CBC-MAC construites afin d'obtenir une sécurité comparable à celle du triple DES ont ainsi été cryptanalysées.
- Un mécanisme d'intégrité vient souvent en complément d'un mécanisme assurant la confidentialité. La composition de deux mécanismes cryptographiques n'est jamais simple et doit être réalisée avec soin.

**Mécanismes conformes au référentiel :**

- Le mode d'intégrité **CBC-MAC « retail »** utilisant l'AES comme mécanisme de chiffrement par bloc et deux clés distinctes (une pour la chaîne CBC et l'autre pour le surchiffrement dit « retail ») est conforme au référentiel (à condition de ne pas utiliser de valeur initiale). Le mode CBC-MAC sans surchiffrement n'est sûr que lorsqu'il est utilisé pour des messages de taille fixe.
- Le mode d'intégrité **HMAC** utilisant SHA-2 comme fonction de hachage est conforme au référentiel.

**Mécanisme non conforme au référentiel :**

- Le mode d'intégrité CBC-MAC « retail » n'est pas conforme au référentiel s'il est utilisé avec le **DES** comme mécanisme de chiffrement par bloc, et ce même s'il emploie deux clés distinctes. En effet, bien qu'utilisant alors 112 bits de clé, l'observation de 2^32 MACs valides permet ensuite de retrouver ces 112 bits de clé en effectuant « seulement » de l'ordre de 2^56 calculs de DES.

### 2.2. Cryptographie asymétrique

Les mécanismes de cryptographie asymétrique reposent tous sur des problèmes mathématiques difficiles, généralement issus de la théorie des nombres.

#### 2.2.1. Problèmes mathématiques asymétriques

##### 2.2.1.1. Factorisation

Le problème de la factorisation consiste à retrouver la décomposition en facteurs premiers d'un entier donné, obtenu de manière secrète par multiplication de deux nombres premiers, généralement de taille comparable. Un tel nombre composé est classiquement appelé « module ».

**Règles :**

| Référence | Règle |
|-----------|-------|
| **RègleFact-1** | La taille minimale du module est de **2048 bits**, pour une utilisation ne devant pas dépasser l'année 2030. |
| **RègleFact-2** | Pour une utilisation au-delà de 2030, la taille minimale du module est de **3072 bits**. |
| **RègleFact-3** | Les exposants secrets doivent être de même taille que le module. |
| **RègleFact-4** | Pour les applications de chiffrement, les exposants publics doivent être strictement supérieurs à 2^16 = 65536. |

**Recommandations :**

| Référence | Recommandation |
|-----------|----------------|
| **RecomFact-1** | Il est recommandé d'employer des modules d'au moins **3072 bits**, même pour une utilisation ne devant pas dépasser 2030. |
| **RecomFact-2** | Il est recommandé, pour toute application, d'employer des exposants publics strictement supérieurs à 2^16 = 65536. |
| **RecomFact-3** | Il est recommandé que les deux nombres premiers p et q constitutifs du module soient de même taille et choisis aléatoirement uniformément. |

**Justifications :**

- L'emploi de modules de 1024 bits est considéré comme une prise de risque incompatible avec des critères de sécurité raisonnables.
- L'emploi d'exposants secrets particuliers (comme des exposants secrets petits) afin d'améliorer les performances est à proscrire étant donné les attaques pratiques publiées.
- L'emploi d'exposants publics très petits, tels que l'exposant 3, est à proscrire dans le cas du chiffrement à cause des attaques existantes.
- L'emploi de nombres premiers p et q trop proches ou de tailles trop différentes peut compromettre la sécurité du système.

##### 2.2.1.2. Logarithme discret

Le problème dit « du logarithme discret » est fondé sur la difficulté d'inverser l'opération d'exponentiation dans un groupe.

**Logarithme discret dans GF(p) - Règles :**

| Référence | Règle |
|-----------|-------|
| **RègleLogp-1** | La taille minimale de modules premiers est de **2048 bits** pour une utilisation ne devant pas dépasser l'année 2030. |
| **RègleLogp-2** | Pour une utilisation au-delà de 2030, la taille minimale de modules premiers est de **3072 bits**. |
| **RègleLogp-3** | On emploiera des sous-groupes dont l'ordre est multiple d'un nombre premier d'au moins **200 bits**. |

**Recommandations :**

| Référence | Recommandation |
|-----------|----------------|
| **RecomLogp-1** | Il est recommandé d'employer des modules premiers d'au moins **3072 bits**, même pour une utilisation ne devant pas dépasser 2030. |
| **RecomLogp-2** | Il est recommandé d'employer des sous-groupes dont l'ordre est premier (au lieu d'être multiple d'un nombre premier). |

**Logarithme discret dans les courbes elliptiques définies sur GF(p) - Règles :**

| Référence | Règle |
|-----------|-------|
| **RègleECp-1** | Pour une utilisation ne devant pas dépasser 2020, on emploiera des sous-groupes dont l'ordre est multiple d'un nombre premier d'au moins **200 bits**. |
| **RègleECp-2** | Pour une utilisation au-delà de 2020, on emploiera des sous-groupes dont l'ordre est multiple d'un nombre premier d'au moins **256 bits**. |
| **RègleECp-3** | En cas d'utilisation de courbes particulières faisant reposer la sécurité sur un problème mathématique plus facile que le problème générique de calcul de logarithme discret sur courbe elliptique, ce problème devra vérifier les règles correspondantes. |

**Recommandation :**

| Référence | Recommandation |
|-----------|----------------|
| **RecomECp-1** | Il est recommandé d'employer des sous-groupes dont l'ordre est premier (au lieu d'être multiple d'un nombre premier). |

**Mécanisme conforme au référentiel :**

- L'emploi de la courbe **FRP256v1** – définie dans le journal officiel n°241 du 16/10/2011 et dont les paramètres, validés par l'ANSSI, peuvent librement être intégrés dans tous les produits de sécurité – est conforme au référentiel.
- L'emploi des courbes **P-256, P-384 et P-521** définies dans le FIPS 186-2 est également conforme.

**Logarithme discret dans les courbes elliptiques définies sur GF(2^n) - Règles :**

| Référence | Règle |
|-----------|-------|
| **RègleEC2-1** | Pour une utilisation ne devant pas dépasser 2020, on emploiera des sous-groupes dont l'ordre est multiple d'un nombre premier d'au moins **200 bits**. |
| **RègleEC2-2** | Pour une utilisation au-delà de 2020, on emploiera des sous-groupes dont l'ordre est multiple d'un nombre premier d'au moins **256 bits**. |
| **RègleEC2-3** | Le paramètre n doit être un nombre premier. |
| **RègleEC2-4** | En cas d'utilisation de courbes particulières, ce problème devra vérifier les règles correspondantes. |

**Recommandation :**

| Référence | Recommandation |
|-----------|----------------|
| **RecomEC2-1** | Il est recommandé d'employer des sous-groupes dont l'ordre est premier. |

**Mécanisme conforme au référentiel :**

- L'emploi des courbes **B-283, B-409 et B-571** définies dans le FIPS 186-2 est conforme au référentiel.

##### 2.2.1.3. Autres problèmes

Plusieurs autres problèmes ont été proposés dans le milieu académique (résolution de systèmes d'équations multivariées, recherche de plus courts ou plus proches vecteurs dans des réseaux euclidiens, décodage de codes correcteurs d'erreurs). Ces problèmes n'étant que peu utilisés aujourd'hui en pratique, leur sécurité est à traiter au cas par cas.

#### 2.2.2. Chiffrement asymétrique

**Recommandation :**

| Référence | Recommandation |
|-----------|----------------|
| **RecomChiffAsym-1** | Il est recommandé d'employer des mécanismes de chiffrement asymétrique disposant d'une preuve de sécurité. |

**Mécanisme conforme au référentiel :**

- Le mécanisme de chiffrement asymétrique **RSAES-OAEP** défini dans le document PKCS#1 v2.1 est conforme au référentiel à condition de respecter les règles RègleFact-1, RègleFact-2, RègleFact-3 et RègleFact-4.

**Mécanisme non conforme au référentiel :**

- Le mécanisme de chiffrement asymétrique **RSAES**, mis en œuvre selon le document PKCS#1 v1.5 n'est pas conforme au référentiel dans un contexte où il est possible d'invoquer un oracle de vérification de padding. Bleichenbacher a mis en évidence en 1998 une attaque exploitant judicieusement un tel oracle.

#### 2.2.3. Signature asymétrique

**Recommandation :**

| Référence | Recommandation |
|-----------|----------------|
| **RecomSignAsym-1** | Il est recommandé d'employer des mécanismes de signature asymétrique disposant d'une preuve de sécurité. |

**Mécanismes conformes au référentiel :**

- Le mécanisme de signature asymétrique **RSA-SSA-PSS** défini dans le document PKCS#1 v2.1 est conforme au référentiel à condition de respecter les règles RègleFact-1, RègleFact-2, RègleFact-3 et RègleFact-4.
- Le mécanisme de signature asymétrique **ECDSA** défini dans le FIPS 186-2, ainsi que le mécanisme de signature asymétrique **ECKCDSA**, sont conformes au référentiel lorsqu'ils utilisent la courbe FRP256v1 ou l'une des courbes P-256, P-384, P-521, B-283, B-409 et B-571 définies dans le FIPS 186-2.

**Mécanisme non conforme au référentiel :**

- Le mécanisme de signature asymétrique **RSASSA**, mis en œuvre selon le document PKCS#1 v1.5 n'est pas conforme au référentiel lorsque l'exposant public e est petit et pour un mauvais choix d'implantation des vérifications liées au padding. Bleichenbacher a mis en évidence en 2006 une attaque permettant de forger des signatures dans ce cas.

#### 2.2.4. Authentification d'entités et établissement de clé

Les mécanismes interactifs d'authentification d'entités et d'établissement de clé reposent en général au moins partiellement sur des mécanismes de génération d'aléa, de hachage et de chiffrement ou de signature à clé publique. Les règles et recommandations énoncées dans les sections précédentes s'appliquent alors directement.

**Points importants :**

- Une attention particulière devra être portée à la résistance des mécanismes d'établissement de clé aux **attaques par le milieu**.
- L'utilisation conjointe de mécanismes d'établissement de clé et d'authentification d'entité convenablement liés l'un à l'autre peut permettre de se prémunir contre des attaques de ce type.
- Il est souhaitable qu'un mécanisme d'établissement de clé assure la **confidentialité dans le futur** ou **PFS** (Perfect Forward Secrecy) des clés symétriques temporaires.

### 2.3. Fonctions de hachage

Les fonctions de hachage cryptographiques doivent avoir plusieurs propriétés telles que la résistance à la recherche de « collisions ». En pratique, afin de contrer les attaques fondées sur le paradoxe des anniversaires, une empreinte doit être deux fois plus longue qu'une clé symétrique pour atteindre le même niveau de robustesse.

**Règles :**

| Référence | Règle |
|-----------|-------|
| **RègleHash-1** | Pour une utilisation ne devant pas dépasser 2020, la taille minimale des empreintes générées par une fonction de hachage est de **200 bits**. |
| **RègleHash-2** | Pour une utilisation au-delà de 2020, la taille minimale des empreintes générées par une fonction de hachage est de **256 bits**. |
| **RègleHash-3** | La meilleure attaque connue permettant de trouver des collisions doit nécessiter de l'ordre de 2^(h/2) calculs d'empreintes, où h désigne la taille en bits des empreintes. |

**Recommandation :**

| Référence | Recommandation |
|-----------|----------------|
| **RecomHash-1** | L'emploi de fonctions de hachage pour lesquelles des « attaques partielles » sont connues est déconseillé. |

**Mécanisme conforme au référentiel :**

- Le mécanisme de hachage **SHA-256** défini dans le FIPS 180-2 est conforme au référentiel.

**Mécanisme non conforme au référentiel :**

- Le mécanisme de hachage **SHA-1** défini dans le FIPS 180-2 a récemment fait l'objet d'une attaque en recherche de collision. La complexité de cette attaque est estimée à 2^63, c'est-à-dire inférieure à 2^80. Le mécanisme de hachage SHA-1 n'est donc pas conforme au référentiel. Il ne respecte ni RègleHash-1, ni RègleHash-3.

### 2.4. Génération d'aléa cryptographique

La qualité de l'aléa est un élément crucial pour la sécurité d'un système, que ce soit pour la génération des clés ou pour le bon fonctionnement des primitives cryptographiques.

**Terminologie :**

- **Source d'aléa** : dispositif susceptible de fournir des éléments au moins partiellement aléatoires :
  - *Physique* : générateur physique d'aléa
  - *Systémique* : accumulation d'événements imprévisibles du système
  - *Importée* : données secrètes parfaitement aléatoires fournies par le système d'information
  - *Manuelle* : données aléatoires obtenues par action intentionnelle d'un utilisateur

- **Retraitement algorithmique** : mécanisme de nature cryptographique destiné à combiner différentes sources d'aléa et à garantir dans la durée la qualité de l'aléa produit.

- **État interne** : donnée secrète dédiée au retraitement, destinée généralement à accumuler l'entropie des sources d'aléa.

- **Mémoire non volatile** : élément de sécurité important pour stocker des données qui seront utilisées lors de l'initialisation suivante.

> **Note importante** : Les règles et recommandations applicables aux générateurs d'aléa se fondent sur le constat qu'il est aujourd'hui très difficile de fournir une preuve convaincante concernant la qualité de l'aléa issu d'un générateur physique, alors qu'il est relativement aisé de se convaincre de la qualité d'un bon retraitement.

#### 2.4.1. Architecture d'un générateur d'aléa

**Règles :**

| Référence | Règle |
|-----------|-------|
| **RègleArchiGDA-1** | Un retraitement algorithmique disposant d'un état interne doit être employé. |
| **RègleArchiGDA-2** | En l'absence de générateur physique d'aléa, le retraitement algorithmique doit disposer d'une mémoire non volatile. |
| **RègleArchiGDA-3** | L'état interne doit être au minimum de **128 bits**. En l'absence d'un rafraîchissement suffisamment fréquent par un générateur physique d'aléa, cette limite inférieure est portée à **160 bits**. |
| **RègleArchiGDA-4** | La qualité des sources d'aléa ponctuelles ou régulières utilisées pour initialiser l'état interne doit être suffisante pour assurer à la valeur initiale de cet état une entropie voisine de sa longueur, ou tout au moins supérieure au seuil défini dans la règle RègleArchiGDA-3. |

**Recommandation :**

| Référence | Recommandation |
|-----------|----------------|
| **RecomArchiGDA-1** | Il est recommandé d'utiliser un retraitement avec un état interne d'au moins **256 bits**, une mémoire non volatile et une source d'aléa rafraîchissant régulièrement l'état interne du générateur. |

**Justifications :**

- L'emploi d'un générateur physique non retraité est exclu.
- Le bon fonctionnement du retraitement algorithmique peut facilement être contrôlé, contrairement aux générateurs physiques pour lesquels il est tout juste possible de contrôler qu'ils ne sont pas dans un état de panne évident au moyen de tests statistiques.
- Utiliser des tests statistiques de panne à la sortie du retraitement algorithmique est non seulement inutile mais peut même s'avérer dangereux pour la sécurité.

#### 2.4.2. Générateur physique d'aléa

**Règles :**

| Référence | Règle |
|-----------|-------|
| **RègleArchiGVA-1** | Le générateur physique d'aléa doit disposer d'une description fonctionnelle. Celle-ci doit notamment indiquer les principes concourant à la génération de vrai aléa. |
| **RègleArchiGVA-2** | Des tests statistiques en sortie du générateur physique ne doivent pas faire apparaître de défauts significatifs dans l'aléa généré. |

**Recommandation :**

| Référence | Recommandation |
|-----------|----------------|
| **RecomArchiGVA-1** | Il est souhaitable qu'un raisonnement permette de justifier la qualité de l'aléa produit par le générateur physique. |

#### 2.4.3. Retraitement algorithmique

**Règles :**

| Référence | Règle |
|-----------|-------|
| **RègleAlgoGDA-1** | Les primitives cryptographiques employées par le retraitement algorithmique doivent être conformes au référentiel. |
| **RègleAlgoGDA-2** | Dans l'hypothèse où l'état interne est fiable, même en cas de défaillance des sources d'aléa présentes, les sorties successives du retraitement doivent être parfaitement aléatoires du point de vue de l'attaquant. De plus, la connaissance de ces sorties ne doit pas mettre en danger la confidentialité des états internes ni des sources d'aléa (fiables). |
| **RègleAlgoGDA-3** | En cas de compromission « simple » affectant ou bien l'état interne ou bien les sources d'aléa éventuellement présentes mais n'affectant pas simultanément ces deux types d'éléments, la sortie courante ne doit donner à l'attaquant aucune information exploitable sur les sorties passées. |

### 2.5. Gestion de clés

La gestion des clés est un problème à part entière, traité dans un document spécifique intitulé « Gestion des clés cryptographiques – Règles et recommandations concernant la gestion des clés utilisées dans des mécanismes cryptographiques ».

#### 2.5.1. Clés secrètes symétriques

**Règles :**

| Référence | Règle |
|-----------|-------|
| **RègleGestSym-1** | L'emploi d'une même clé pour plus d'un usage est exclu. |
| **RègleGestSym-2** | Les éventuelles clés différenciées utilisées avec un mécanisme conforme au référentiel doivent être générées en utilisant un mécanisme de diversification conforme au référentiel. |
| **RègleGestSym-3** | Les éventuelles clés dérivées doivent être générées en utilisant un mécanisme de diversification conforme au référentiel. |

**Recommandation :**

| Référence | Recommandation |
|-----------|----------------|
| **RecomGestSym-1** | L'emploi de clés communes est déconseillé. |

#### 2.5.2. Bi-clés asymétriques

**Règles :**

| Référence | Règle |
|-----------|-------|
| **RègleGestAsym-1** | L'emploi d'une même bi-clé à plus d'un usage est exclu. |
| **RègleGestAsym-2** | Les clés hiérarchiquement importantes, telles que les clés racine, doivent être générées et utilisées par des mécanismes conformes au référentiel. |

---

## Annexe A. Définitions et concepts

### A.1. Cryptographie symétrique

Les primitives cryptographiques symétriques se caractérisent par le fait qu'elles utilisent des clés cryptographiques partagées par plusieurs personnes ou entités. Une **clé secrète symétrique** est donc simplement un élément secret.

Une clé secrète est généralement une suite quelconque de bits de taille fixée. Cette taille de clé, notée n, est déterminante pour la sécurité du système car on peut former exactement 2^n clés de longueur n. Les systèmes symétriques sont en général susceptibles d'être attaqués de manière générique au moyen d'une énumération de toutes les clés possibles (attaque par **recherche exhaustive**).

**Ordre de grandeur de la valeur de 2^n :**

| n | 2^n | Ordre de grandeur |
|---|-----|-------------------|
| 32 | 2^32 | Nombre d'hommes sur Terre |
| 46 | 2^46 | Distance Terre–Soleil en millimètres |
| 55 | 2^55 | Nombre d'opérations en une année à 1GHz |
| 82 | 2^82 | Masse de la Terre en kilogrammes |
| 90 | 2^90 | Nombre d'opérations en 15 milliards d'années à 1GHz |
| 155 | 2^155 | Nombre de molécules d'eau sur Terre |
| 256 | 2^256 | Nombre d'électrons dans l'univers |

#### A.1.1. Chiffrement symétrique

##### A.1.1.1. Chiffrement par bloc

Une primitive de chiffrement par bloc est un algorithme traitant les données à chiffrer par blocs de taille fixée (typiquement 64 ou 128 bits).

**Exemples :**
- **DES** (Data Encryption Standard) : blocs de 64 bits, clés de 56 bits. La taille de clé est sous-dimensionnée, rendant aujourd'hui l'attaque par recherche exhaustive réalisable en quelques heures avec une machine dédiée.
- **Triple DES** : variante utilisant des clés de 112 bits, inattaquable par recherche exhaustive.
- **AES** (Advanced Encryption Standard) : blocs de 128 bits, clés de 128, 192 ou 256 bits.

**Mode opératoire CBC** : Le mode « cipher-block chaining » permet de chiffrer des messages de taille quelconque. Une valeur initiale (IV) aléatoire indépendante du message est utilisée pour randomiser le processus. Le chiffrement se fait par : c_i = E_K(c_{i-1} ⊕ x_i).

##### A.1.1.2. Chiffrement par flot

Les primitives de chiffrement par flot considèrent le message à chiffrer comme une suite de bits combinés (généralement par ou-exclusif) avec une séquence de bits dérivée de la clé secrète.

L'algorithme de **Vernam** (one-time pad) est parfaitement sûr si la clé est au moins de même taille que le message et n'est utilisée qu'une seule fois. Cependant, il ne protège pas l'intégrité : un attaquant peut inverser un bit du chiffré sans connaître la clé, modifiant ainsi le message clair.

#### A.1.3. Authentification et intégrité de messages

La principale technique pour assurer l'intégrité consiste à calculer un **code d'authentification de message (MAC)** à partir des données et d'une clé secrète partagée. Le **CBC-MAC** est l'algorithme le plus connu.

#### A.1.4. Authentification d'entités

La méthode symétrique la plus utilisée est le **défi-réponse** (challenge-response) : envoyer un message quelconque à authentifier, lui demander de le chiffrer avec la clé secrète partagée, puis vérifier le résultat.

### A.2. Cryptographie asymétrique

L'idée majeure est que les opérations publiques (chiffrement, vérification de signature) n'ont pas besoin d'utiliser les mêmes clés que les opérations privées (déchiffrement, signature).

Les problèmes mathématiques asymétriques utilisables en cryptographie sont principalement :
- **Factorisation** : retrouver les facteurs premiers d'un grand nombre
- **Logarithme discret** : dans GF(p), sur courbes elliptiques, etc.

#### A.2.1. Chiffrement asymétrique

L'utilisation de paires de clés (bi-clés) composées d'une clé privée et d'une clé publique associée permet de chiffrer un message à l'attention d'un correspondant en utilisant sa clé publique. Le déchiffrement utilise la clé privée.

#### A.2.2. Signature cryptographique asymétrique

La **signature cryptographique** permet de garantir l'intégrité d'un message sans clé secrète partagée, et d'assurer une authentification forte empêchant la répudiation.

#### A.2.3. Authentification asymétrique d'entités et établissement de clé

L'**échange de clé de Diffie-Hellman** permet à deux entités ne partageant initialement aucun secret commun de se mettre d'accord sur une valeur de clé secrète.

### A.3. Fonctions de hachage

Une **fonction de hachage** transforme, de manière déterministe, une suite de bits de longueur quelconque en un condensat (empreinte, haché) de taille fixée. Une fonction de hachage cryptographique doit être non inversible et sans collision.

Le **paradoxe des anniversaires** indique qu'il suffit de calculer environ 2^(n/2) hachés pour trouver une collision, si n est le nombre de bits du condensat.

### A.4. Génération d'aléa cryptographique

La cryptographie fait un usage intensif de données aléatoires. La qualité de ces données est parfois critique en termes de sécurité.

**Exemple DSA** : La signature d'un message nécessite un nombre aléatoire de 160 bits gardé secret. Une attaque permet de retrouver la clé privée si seulement 2 bits du nombre aléatoire sont connus, même si les 158 bits restants sont parfaitement aléatoires.

### A.5. Gestion de clés

#### A.5.1. Clés secrètes symétriques

- La **crypto-période** est la durée de vie maximale associée à chaque clé.
- Les **clés noires** sont les clés chiffrées, par opposition aux **clés rouges** qui sont en clair.
- L'emploi de **clés communes** (partagées par un grand nombre d'utilisateurs) est déconseillé.

#### A.5.2. Bi-clés asymétriques

Le problème majeur est la nécessité d'associer une clé publique à l'identité de son détenteur légitime. C'est le rôle des **certificats** et des **infrastructures à clé publique (ICP/PKI)**.

---

## Annexe B. Éléments académiques de dimensionnement cryptographique

### B.1. Records de calculs cryptographiques

#### B.1.1. Records de calculs en cryptographie symétrique

**Cassage de clés DES (56 bits) :**
- 17 juin 1997 : 96 jours de calcul distribué sur Internet
- 23 février 1998 : 41 jours de calcul distribué sur Internet
- 17 juillet 1998 : **56 heures** sur une machine spécifique (~250 000 dollars)
- 19 janvier 1999 : **22 heures** en combinant la machine et Internet

**Cassage de clés RC5 :**
- 19 octobre 1997 : version 56 bits cassée après 250 jours
- 14 juillet 2002 : version 64 bits cassée après 1757 jours

#### B.1.2. Records de calculs de factorisation

| Date | Auteurs | Taille |
|------|---------|--------|
| 9 juin 1993 | Denny, Dodson, Lenstra et Manasse | 397 bits (120 chiffres) |
| 2 avril 1994 | Atkins, Graff, Lenstra et Leyland | 426 bits (129 chiffres) |
| 22 août 1999 | te Riele et al. | 512 bits (155 chiffres) |
| 3 décembre 2003 | Franke et Kleinjung | 576 bits (174 chiffres) |
| 9 mai 2005 | Bahr, Boehm, Franke et Kleinjung | 663 bits (200 chiffres) |
| 12 décembre 2009 | Kleinjung et al. | **768 bits** (232 chiffres) |

#### B.1.3. Records de calcul de logarithme discret dans GF(p)

| Date | Auteurs | Taille |
|------|---------|--------|
| 25 novembre 1996 | Weber, Denny et Zayer | 281 bits (85 chiffres) |
| 18 juin 2005 | Joux et Lercier | 431 bits (130 chiffres) |
| 5 février 2007 | Bahr, Franke et Kleinjung | **530 bits** (160 chiffres) |

#### B.1.4. Records de calcul de logarithme discret dans GF(2^n)

Pour n premier :
| Date | Auteurs | Résultat |
|------|---------|----------|
| 10 avril 2013 | Barbulescu et al. | GF(2^809) - 809 bits |

Pour n composé :
| Date | Auteurs | Résultat |
|------|---------|----------|
| 31 janvier 2014 | Granger, Kleinjung et Zumbragel | GF(2^9234) - **9234 bits** |

> **Note** : À taille de corps équivalente, le calcul de logarithme discret est bien plus facile dans GF(2^n) que dans GF(p).

#### B.1.5. Records de calcul de logarithme discret dans GF(p^n)

Pour p = 3 :
- 28 janvier 2014 : GF(3^822) soit **1303 bits**

#### B.1.6. Calcul de logarithme discret sur courbe elliptique

**Challenges Certicom :**

| Challenge | Date d'annonce | Nombre d'opérations |
|-----------|----------------|---------------------|
| ECCp-79 | 06/12/1997 | 2^40 |
| ECCp-89 | 12/01/1998 | 2^44 |
| ECCp-97 | 18/03/1998 | 2^47 |
| ECCp-109 | 06/11/2002 | 2^54 |
| ECCp-131 | Non résolu | - |
| ECC2-109 | 15/04/2004 | 2^54 |
| ECC2K-108 | 04/04/2000 | 2^51 |

Le 8 juillet 2009 : résolution d'une instance sur courbe elliptique définie sur GF(p) avec p de **112 bits**.

### B.2. Étude de la taille des clés d'après l'article de Lenstra

> **Mise en garde** : Ce travail est mentionné car il est le plus complet réalisé dans ce domaine par des chercheurs du milieu académique. Le fait de le citer ne constitue cependant pas une caution de l'article pris dans son intégralité.

#### B.2.1. Évolution des tailles de clés symétriques

Sous l'hypothèse d'une loi de Moore (doublement de la puissance tous les 18 mois), la longueur de clé nécessaire augmente d'un bit tous les 18 mois.

**Remarques :**
- Des clés de **128 bits** apportent un très haut niveau de sécurité face à une attaque par recherche exhaustive.
- Des clés de **256 bits** offrent une sécurité démesurée face aux attaques par recherche exhaustive.

#### B.2.2. Évolution des tailles de modules en cryptographie asymétrique

- La croissance est nettement non linéaire à cause des algorithmes « sous-exponentiels » (cribles quadratiques et algébriques).
- À moyen terme, l'emploi de modules de grande taille s'impose.
- Le problème du logarithme discret est légèrement plus difficile que celui de la factorisation (100 à 200 bits de différence en pratique).

#### B.2.4. Équivalence de sécurité entre tailles de module asymétrique et de clé symétrique

- Rechercher une sécurité équivalente à celle d'une clé symétrique de plus de 128 bits mène à des tailles de module très importantes, peu utilisables en pratique.
- Il est absurde de chercher à utiliser des modules d'une sécurité comparable à celle d'une clé symétrique de 256 bits.
- **Pour des clés d'environ 1024 bits** : un bit symétrique équivaut à 51 bits asymétriques.
- **Pour des clés d'environ 2048 bits** : un bit symétrique équivaut à 77 bits asymétriques.

---

## Annexe C. Bibliographie

- **[Ble98]** Daniel Bleichenbacher. *Chosen ciphertext attacks against protocols based on the RSA encryption standard PKCS #1*. In Proceedings of Crypto 1998, volume 1462 of LNCS, pages 1–12, 1998.
- **[Ble06]** Daniel Bleichenbacher. *Crypto Rump Session*, 2006.
- **[BR94]** Mihir Bellare and Philipp Rogaway. *Optimal asymmetric encryption*. In Proceedings of Eurocrypt 1994, volume 839 of LNCS, pages 92–111. Springer-Verlag, 1994.
- **[CFA+06]** Henri Cohen, Gerhard Frey, Roberto Avanzi, et al., editors. *Handbook of elliptic and hyperelliptic curve cryptography*. Chapman & Hall/CRC, 2006.
- **[DH76]** Whitfield Diffie and Martin Hellman. *New directions in cryptography*. IEEE Transactions on Information Theory, 22(6):644–654, 1976.
- **[Len04]** Arjen Lenstra. *Handbook of Information Security*, volume 2, chapter Key Lengths. Wiley, 2004.
- **[MvV97]** Alfred J. Menezes, Paul C. van Oorschot, and Scott A. Vanstone. *Handbook of Applied Cryptography*. CRC Press, 1997.
- **[Sch01]** Bruce Schneier. *Cryptographie appliquée*. Vuibert, 2001.
- **[Sin99]** Simon Singh. *Histoire des codes secrets*. JC Lattès, 1999.
- **[Ste98]** Jacques Stern. *La science du secret*. Éditions Odile Jacob, 1998.
- **[Sti01]** Douglas Stinson. *Cryptographie, théorie et pratique*. Vuibert, 2001.
- **[Vau06]** Serge Vaudenay. *A Classical Introduction to Cryptography: Applications for Communications Security*. Springer, 2006.

---

## Annexe D. Liste des tableaux

1. Primitives cryptographiques offrant un service donné
2. Ordre de grandeur de la valeur de 2^n
3. Challenges Certicom

---

## Annexe E. Table des figures

1. Architecture générique pour la génération d'aléa cryptographique
2. Architecture minimale pour la génération d'aléa
3. Mode opératoire CBC
4. Mode opératoire CBC-MAC
5. Records de factorisation
6. Taille de clé symétrique offrant une sécurité équivalente à celle du DES en 1982
7. Taille de module offrant une sécurité équivalente à celle du DES en 1982
8. Comparaison entre les records de factorisation et les tailles théoriques nécessaires
9. Taille de courbes elliptiques offrant une sécurité équivalente à celle du DES en 1982
10. Équivalence entre taille de module asymétrique et taille de clé symétrique
11. Équivalence entre dimensionnement de cryptosystème à base de courbe elliptique et taille de clé symétrique

---

## Annexe F. Index des termes et des acronymes utilisés

- **AES** : Advanced Encryption Standard
- **CBC** : Cipher Block Chaining
- **CBC-MAC** : Cipher Block Chaining Message Authentication Code
- **CFB** : Cipher Feedback
- **CTR** : Counter mode
- **DES** : Data Encryption Standard
- **DFA** : Differential Fault Analysis
- **DPA** : Differential Power Analysis
- **ECDSA** : Elliptic Curve Digital Signature Algorithm
- **ECKCDSA** : Elliptic Curve Korean Certificate-based Digital Signature Algorithm
- **GF(p)** : Corps fini à p éléments
- **GF(2^n)** : Corps fini binaire
- **HMAC** : Hash-based Message Authentication Code
- **MAC** : Message Authentication Code
- **NIST** : National Institute of Standards and Technology
- **OAEP** : Optimal Asymmetric Encryption Padding
- **OFB** : Output Feedback
- **PIN** : Personal Identification Number
- **PKCS** : Public-Key Cryptography Standards
- **PSS** : Probabilistic Signature Scheme
- **RSA** : Rivest–Shamir–Adleman
- **SHA** : Secure Hash Algorithm
- **SPA** : Simple Power Analysis

---

## Annexe G. Index des règles et des recommandations

### Règles

| Domaine | Règles |
|---------|--------|
| Algorithmes de bloc | RègleAlgoBloc-1, RègleAlgoBloc-2 |
| Algorithmes de génération d'aléa | RègleAlgoGDA-1, RègleAlgoGDA-2, RègleAlgoGDA-3 |
| Architecture générateur d'aléa | RègleArchiGDA-1 à 4, RègleArchiGVA-1 à 2 |
| Taille de bloc symétrique | RègleBlocSym-1, RègleBlocSym-2 |
| Chiffrement par flot | RègleChiffFlot-1, RègleChiffFlot-2 |
| Taille de clé symétrique | RègleCléSym-1, RègleCléSym-2 |
| Courbes elliptiques sur GF(2^n) | RègleEC2-1 à 4 |
| Courbes elliptiques sur GF(p) | RègleECp-1 à 3 |
| Factorisation | RègleFact-1 à 4 |
| Gestion de clés asymétriques | RègleGestAsym-1, RègleGestAsym-2 |
| Gestion de clés symétriques | RègleGestSym-1 à 3 |
| Fonctions de hachage | RègleHash-1 à 3 |
| Intégrité symétrique | RègleIntegSym-1, RègleIntegSym-2 |
| Logarithme discret dans GF(p) | RègleLogp-1 à 3 |
| Mode de chiffrement | RègleModeChiff-1 |

### Recommandations

| Domaine | Recommandations |
|---------|-----------------|
| Algorithmes de bloc | RecomAlgoBloc-1 |
| Architecture générateur d'aléa | RecomArchiGDA-1, RecomArchiGVA-1 |
| Taille de bloc symétrique | RecomBlocSym-1 |
| Chiffrement asymétrique | RecomChiffAsym-1 |
| Chiffrement par flot | RecomChiffFlot-1, RecomChiffFlot-2 |
| Taille de clé symétrique | RecomCléSym-1 |
| Courbes elliptiques | RecomEC2-1, RecomECp-1 |
| Factorisation | RecomFact-1 à 3 |
| Gestion de clés symétriques | RecomGestSym-1 |
| Fonctions de hachage | RecomHash-1 |
| Intégrité symétrique | RecomIntegSym-1 |
| Logarithme discret | RecomLogp-1, RecomLogp-2 |
| Mode de chiffrement | RecomModeChiff-1 à 3 |
| Signature asymétrique | RecomSignAsym-1 |
