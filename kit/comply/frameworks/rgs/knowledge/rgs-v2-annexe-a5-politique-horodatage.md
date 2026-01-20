# Référentiel Général de Sécurité version 2.0

## Annexe A5 - Politique d'Horodatage Type

**Version 3.0 du 27 février 2014**

---

## Historique des versions

| Date | Version | Évolution du document | Rédacteur |
|------|---------|----------------------|-----------|
| 06/11/2006 | 2.1 | Document constitutif de la Politique de Référencement Intersectorielle de Sécurité – PRISv2.1 | DCSSI / SDAE |
| 12/12/2008 | 2.2 | Document constitutif du RGSv0.98, annexe A12. L'extension extendedKeyUsage est marquée comme critique conformément au RFC 3161 | DCSSI / DGME |
| 18/02/2010 | 2.3 | Document constitutif du RGSv1.0, annexe A12. Réécriture des chapitres VI.1.2, VI.2.1 et IX.2 ; Modifications des obligations pour les AC fournissant des certificats d'horodatage ; Modification de la durée de validité des certificats d'horodatage | ANSSI / DGME |
| 27/02/2014 | 3.0 | Modification des exigences relatives à la sauvegarde des clés d'UH ; Modification des exigences sur la qualification du module d'horodatage | ANSSI |

---

## Sommaire

- [I. Introduction](#i-introduction)
- [II. Généralités](#ii-généralités)
- [III. Politique d'horodatage](#iii-politique-dhorodatage)
- [IV. Déclaration des Pratiques d'Horodatage](#iv-déclaration-des-pratiques-dhorodatage)
- [V. Conditions générales d'utilisation](#v-conditions-générales-dutilisation)
- [VI. Contenu de la politique d'horodatage](#vi-contenu-de-la-politique-dhorodatage)
- [VII. Annexe 1 : Documents cités en référence](#vii-annexe-1--documents-cités-en-référence)
- [VIII. Annexe 2 : Exigences sur les formats](#viii-annexe-2--exigences-sur-les-formats-des-contremarques-de-temps-des-certificats-et-des-lcr-et-sur-les-algorithmes-cryptographiques)
- [IX. Annexe 3 : Exigences de sécurité du module d'horodatage](#ix-annexe-3--exigences-de-sécurité-du-module-dhorodatage-des-uh)
- [X. Annexe 4 : Vérification ou utilisation](#x-annexe-4---vérification-ou-utilisation-informative)
- [XI. Annexe 5 : Précision de la synchronisation de l'horloge](#xi-annexe-5---précision-de-la-synchronisation-de-lhorloge)
- [XII. Annexe 6 : Protocole d'horodatage](#xii-annexe-6---protocole-dhorodatage)
- [XIII. Annexe 7 : Compatibilité avec la politique d'horodatage de l'ETSI](#xiii-annexe-7---compatibilité-avec-la-politique-dhorodatage-de-letsi)

---

## I. Introduction

### I.1. Présentation générale

Le présent document fait partie des documents constitutifs du [RGS]. Il concerne la fonction de sécurité « horodatage » et constitue la **politique d'horodatage type (PH Type)** destinée aux prestataires de services d'horodatage électronique (PSHE) souhaitant fournir des contremarques de temps à des usagers, des agents ou des applications.

Le service d'horodatage est composé d'un seul niveau de sécurité.

Afin de sécuriser les systèmes d'informations sous la responsabilité d'une autorité administrative (AA), celle-ci peut recourir à la fonction de sécurité « horodatage ». Dès lors, l'autorité administrative doit utiliser des jetons d'horodatage délivrés par des PSHE conformes à la présente PH Type.

Un PSHE peut demander la **qualification** de son offre de service selon les modalités précisées dans le [DécretRGS]. Ce label permet d'attester de la conformité de l'offre du PSHE au présent référentiel.

Cette PH Type n'est pas une PH à part entière : elle ne peut pas être utilisée telle quelle par un PSHE. Un PSHE souhaitant être qualifié par rapport à la présente PH Type doit en reprendre, dans sa propre PH, l'ensemble des engagements.

### I.2. Identification du document

La présente PH Type est dénommée "RGS - Politique d'Horodatage Type". Elle peut être identifiée par son nom, numéro de version, et sa date de mise à jour.

### I.3. Qu'est-ce que l'horodatage ?

L'horodatage permet d'attester qu'une donnée existe à un instant donné. Pour cela, il convient d'associer une représentation sans équivoque d'une donnée (par exemple une valeur de hachage associée à un identifiant d'algorithme de hachage) à un instant dans le temps.

La garantie de cette association est fournie au moyen d'une **contremarque de temps** qui est une structure signée contenant :
- L'identifiant de la politique d'horodatage sous laquelle la contremarque de temps a été générée
- La valeur de hachage et l'algorithme de hachage de la donnée qui a été horodatée
- La date et le temps UTC
- L'identifiant du certificat de l'Unité d'horodatage (UH) qui a généré la contremarque de temps

L'horodatage ne nécessite pas le déploiement d'une infrastructure étendue pour que la validité des certificats des unités d'horodatage puisse être vérifiée. Les utilisateurs finaux ne doivent pas nécessairement avoir des certificats eux-mêmes, mais doivent avoir accès aux informations de validité des certificats d'horodatage (chaîne de certification, LCR…) pour vérifier les contremarques de temps.

Une Autorité d'horodatage peut faire fonctionner plusieurs unités d'horodatage (UH). Chaque unité d'horodatage dispose de sa propre bi-clé.

### I.4. Comment établir la confiance en l'horodatage

La garantie apportée par l'autorité d'horodatage s'appuie sur des éléments techniques et des règles de gestion présentées dans la politique d'horodatage. La politique d'horodatage présente aux utilisateurs les engagements que prend l'autorité d'horodatage, notamment ceux pris en matière de sécurité, et décrit de façon macroscopique les moyens mis en œuvre pour tenir ces engagements.

L'Autorité d'horodatage a la responsabilité d'assurer que ces exigences sont remplies (par exemple par des obligations contractuelles). Elle peut sous-traiter à d'autres parties un sous-ensemble des services d'horodatage.

### I.5. Présentation des rôles et relations

L'AH exploite l'ensemble des services d'horodatage qui regroupe les diverses prestations organisationnelles et techniques nécessaires à la génération et à la gestion des contremarques de temps. Chaque UH signe ses contremarques de temps à l'aide d'une clé privée dont la clé publique correspondante a été certifiée au préalable par une autorité de certification (AC). Les clés privées sont conservées et mises en œuvre dans des modules d'horodatage.

### I.6. Autres aspects

D'un point de vue technique, cette politique s'appuie sur un module d'horodatage pour la protection des clés de signature et de l'horloge.

---

## II. Généralités

### II.1. Définitions

| Terme | Définition |
|-------|------------|
| **Abonné** | Entité ayant besoin de faire horodater des données par une Autorité d'horodatage et qui a accepté les conditions d'utilisation de ses services |
| **Autorité de Certification (AC)** | Cf. les Politiques de Certification Types du [RGS] |
| **Autorité d'horodatage (AH)** | Au sein d'un PSHE, une Autorité d'Horodatage a en charge, au nom et sous la responsabilité de ce PSHE, l'application d'au moins une politique d'horodatage en s'appuyant sur une ou plusieurs Unités d'Horodatage |
| **Contremarque de temps** | Donnée signée qui lie une représentation d'une donnée à un temps particulier, exprimé en heure UTC, établissant ainsi la preuve que la donnée existait à cet instant-là |
| **Coordinated Universal Time (UTC)** | Échelle de temps liée à la seconde, telle que définie dans la recommandation ITU-R TF.460-5 |
| **Déclaration des pratiques d'horodatage (DPH)** | Document identifiant les pratiques (organisation, procédures opérationnelles, moyens techniques et humains) que l'AH applique dans le cadre de la fourniture de ses services d'horodatage |
| **Module d'horodatage** | Produit de sécurité comportant une ressource cryptographique et dédié à la mise en œuvre des fonctions d'horodatage de l'UH |
| **Politique d'horodatage (PH)** | Ensemble de règles, identifié par un nom (OID), définissant les exigences auxquelles une AH se conforme dans la mise en place et la fourniture de ses prestations |
| **Prestataire de services d'horodatage (PSHE)** | Personne ou entité responsable de la génération et de la gestion de contremarques de temps |
| **Unité d'Horodatage (UH)** | Ensemble de matériel et de logiciel en charge de la création de contremarques de temps, caractérisé par un identifiant et une clé unique de signature |
| **UTC(k)** | Temps de référence réalisé par le laboratoire "k" et synchronisé avec précision avec le temps UTC (±100 ns) |
| **Utilisateur de contremarque de temps** | Entité (personne ou système) qui fait confiance à une contremarque de temps émise sous une politique d'horodatage donnée |

### II.2. Abréviations

| Abréviation | Signification |
|-------------|---------------|
| AC | Autorité de Certification |
| AH | Autorité d'horodatage |
| ANSSI | Agence Nationale de la Sécurité des Systèmes d'Information |
| CG | Conditions Générales d'utilisation du service d'horodatage |
| DPH | Déclaration des Pratiques d'Horodatage |
| ETSI | European Telecommunications Standards Institute |
| LCR | Liste des Certificats Révoqués |
| IGC | Infrastructure de Gestion de Clés |
| OID | Object Identifier |
| PH | Politique d'Horodatage |
| PP | Profil de Protection |
| PSHE | Prestataire de Services d'Horodatage |
| UH | Unité d'Horodatage |
| UTC | Coordinated Universal Time |

---

## III. Politique d'horodatage

Pour cette politique, la date et le temps de chaque contremarque de temps doivent être synchronisés avec le temps UTC avec une exactitude précisée dans la déclaration des pratiques d'horodatage.

> **Note** : Il est recommandé que l'exactitude soit d'une seconde. Elle ne doit en tout cas pas excéder la minute.

La présente PH Type impose un format de contremarque de temps spécifique, qui doit répondre aux exigences du chapitre VIII.

Cette politique n'impose pas l'usage d'un protocole d'horodatage spécifique. Cependant, un protocole a été défini dans le [RFC3161] et profilé dans le document [ETSI_TSP] et son usage est recommandé.

**Paramètres utilisés pour la politique d'horodatage** :
- La protection des clés et de l'horloge doit respecter les exigences spécifiées au chapitre IX
- L'AC générant les certificats de clé publique pour les unités d'horodatage doit gérer le service de révocation pour chaque certificat publié

---

## IV. Déclaration des Pratiques d'Horodatage

La déclaration des pratiques d'horodatage expose les mécanismes et les procédures mis en œuvre pour atteindre les objectifs de sécurité de la politique d'horodatage, en particulier les processus qu'une Autorité d'horodatage emploiera pour la création des contremarques de temps et le maintien de l'exactitude de ses horloges.

La DPH doit définir comment l'Autorité d'horodatage se conforme aux exigences physiques, environnementales, procédurales, organisationnelles et techniques identifiées dans une politique d'horodatage.

La déclaration des pratiques d'horodatage est toujours approuvée par le PSHE.

**Éléments publics de la DPH à publier** :
- Le cadre d'application de la DPH
- Les coordonnées de l'AH
- La PH appliquée
- Les algorithmes de hachage autorisés pour constituer l'objet horodaté
- La durée minimum pendant laquelle il est possible de vérifier les contremarques de temps
- La précision de la date des contremarques de temps par rapport à l'échelle de temps UTC
- Les obligations des abonnés
- Les obligations des utilisateurs de contremarque de temps
- Les informations permettant de vérifier la contremarque de temps
- Les limitations de responsabilité

---

## V. Conditions générales d'utilisation

Compte tenu de la complexité de lecture d'une PH et d'une DPH pour des utilisateurs non-spécialistes, l'AH doit définir des conditions générales d'utilisation (correspondant aux "TSA Disclosure Statement" définis par [ETSI_PH]).

Ces conditions générales d'utilisation ne remplacent pas la politique d'horodatage ou la déclaration des pratiques d'horodatage mais sont destinées à des abonnés et utilisateurs non-techniciens.

L'AH doit spécifier dans ses conditions générales d'utilisation les identifiants des politiques d'horodatage supportées.

Il est recommandé que ces conditions générales aient une structure conforme à celle décrite en annexe B de [ETSI_PH].

---

## VI. Contenu de la politique d'horodatage

Ce chapitre décrit les dispositions générales ainsi que les exigences opérationnelles, physiques et environnementales, procédurales et organisationnelles et de sécurité technique auxquelles l'AH doit se conformer.

### VI.1. Dispositions générales

#### VI.1.1. Obligations de l'Autorité d'horodatage

L'Autorité d'horodatage doit :
- Garantir la conformité des exigences et des procédures prescrites dans cette politique, même quand les fonctionnalités d'horodatage sont remplies par des sous-traitants
- Garantir l'adhésion aux obligations complémentaires indiquées dans la contremarque de temps
- Fournir des services d'horodatage conformément à sa déclaration des pratiques d'horodatage
- Remplir tous ses engagements tels que stipulés dans ses conditions générales d'utilisation

#### VI.1.2. Obligations de l'abonné

Au-delà des exigences spécifiques incluses dans les conditions générales d'utilisation, il est recommandé que l'abonné, au moment de l'obtention d'une contremarque de temps, vérifie que le certificat de l'unité d'horodatage n'est pas révoqué (selon le contexte d'utilisation).

#### VI.1.3. Obligations de l'utilisateur de contremarques de temps

Pour faire confiance à une contremarque de temps, l'utilisateur devra :
- Vérifier que la contremarque de temps a été correctement signée et que le certificat de l'unité d'horodatage est valide à l'instant de la vérification
- Tenir compte des limitations sur l'utilisation de la contremarque de temps indiquées dans la politique d'horodatage, la DPH et les conditions générales d'utilisation

#### VI.1.4. Obligations pour les AC fournissant les certificats des unités d'horodatage

Les certificats des clés publiques délivrés aux unités d'horodatage doivent être délivrés par des prestataires de service de certification électronique (PSCE) conformes au RGS, respectant au minimum les exigences du niveau de sécurité une étoile (*) de la Politique de Certification Type "cachet serveur".

Un prestataire de service d'horodatage souhaitant faire qualifier son service d'horodatage devra recourir à un service de certification électronique d'un PSCE lui-même qualifié.

#### VI.1.5. Déclarations des pratiques d'horodatage

L'Autorité d'horodatage doit garantir qu'elle possède la fiabilité nécessaire pour fournir des services d'horodatage :

a) L'AH doit faire effectuer une **évaluation de risques** pour évaluer les actifs et les menaces

b) L'AH doit avoir une déclaration des pratiques et des procédures pour adresser toutes les exigences identifiées dans chaque politique d'horodatage supportée

c) La DPH doit identifier les obligations de toutes les organisations externes participant à la fourniture des services d'horodatage

d) L'AH doit mettre à la disposition des abonnés et des utilisateurs les éléments publics de sa DPH

e) L'AH devra disposer d'une organisation adéquate pour l'approbation de la DPH

f) Le responsable opérationnel doit garantir que les pratiques sont correctement mises en œuvre

g) L'AH doit définir une procédure de contrôle périodique de la conformité des pratiques

h) L'AH doit informer au préalable les abonnés pour tout changement dans la partie publique de sa DPH

i) Si l'AH a été évaluée pour être en conformité et si une modification envisagée pourrait entraîner une non-conformité, alors l'AH doit soumettre cette modification à l'organisme évaluateur indépendant

#### VI.1.6. Conditions générales d'utilisation

L'Autorité d'horodatage doit mettre à disposition pour chaque politique d'horodatage supportée :
- Information sur un point de contact
- Description ou référence de la politique d'horodatage appliquée
- Au moins un algorithme de hachage autorisé
- La période de temps minimum pendant laquelle les contremarques de temps seront vérifiables
- L'exactitude du temps par rapport au temps UTC
- Limitations sur l'utilisation du service
- Obligations de l'abonné et des utilisateurs
- Information sur la manière de vérifier les contremarques de temps
- Période de conservation des fichiers d'audit
- Système légal applicable et limitations de responsabilité
- Procédures pour les plaintes et le règlement des conflits
- Nom de l'organisme de qualification indépendant
- Éléments permettant de valider la chaîne de certificats
- Nom du pays et identifiant de l'AH

#### VI.1.7. Conformité avec les exigences légales

L'Autorité d'horodatage doit garantir la conformité avec les exigences légales :
- Mesures techniques et organisationnelles contre le traitement non autorisé des données personnelles (cf. [CNIL])
- Les informations fournies par les abonnés ne doivent pas être divulguées sans accord, décision judiciaire ou exigence légale

### VI.2. Exigences opérationnelles

#### VI.2.1. Gestion des requêtes de contremarques de temps

La fourniture d'une contremarque de temps en réponse à une demande est à la discrétion de l'AH selon les conditions générales d'utilisation. Il est recommandé que la réponse n'excède pas quelques secondes (délai entre la réception de la requête et la signature de la contremarque de temps).

#### VI.2.2. Fichiers d'audit

L'Autorité d'horodatage doit garantir que toutes les informations appropriées concernant le fonctionnement du service d'horodatage sont enregistrées pendant une période de temps suffisante.

**Général** :
- Les événements spécifiques et les données enregistrées doivent être documentés
- La confidentialité et l'intégrité des enregistrements d'audit doivent être assurées
- Les enregistrements doivent être archivés de manière adaptée à la sensibilité des informations
- L'instant précis d'évènements significatifs doit être enregistré
- Les enregistrements doivent être gardés après la date d'expiration de la clé de signature pendant une période appropriée
- Les événements doivent être enregistrés de façon à ne pas pouvoir être facilement supprimés

**Gestion des clés** :
- Enregistrement de tous les événements touchant au cycle de vie des clés
- Enregistrement de tous les événements touchant au cycle de vie des certificats des UH

**Synchronisation de l'horloge** :
- Enregistrement de tous les événements de synchronisation de l'horloge
- Enregistrement de tous les événements de détection de perte de synchronisation

#### VI.2.3. Gestion de la durée de vie de la clé privée

L'Autorité d'horodatage doit garantir que les clés privées de signature des UH ne sont pas employées au-delà de la fin de leur cycle de vie :
- Des procédures opérationnelles ou techniques doivent assurer qu'une nouvelle paire de clés est mise en place quand la fin de la période d'utilisation est atteinte
- Le système d'horodatage doit détruire la clé privée si la fin de la période d'utilisation est atteinte

#### VI.2.4. Synchronisation de l'horloge

L'Autorité d'horodatage doit garantir que son horloge est synchronisée avec le temps UTC selon l'exactitude déclarée :
- Le calibrage de chaque horloge d'UH doit être maintenu de manière à éviter la dérive
- Les horloges doivent être protégées contre les menaces (modifications non autorisées, ondes radio, chocs électriques)
- L'AH doit garantir que si son horloge interne ne respecte plus l'exactitude déclarée, cela sera détecté
- Si l'horloge est détectée hors de l'exactitude annoncée, les contremarques de temps ne doivent plus être générées
- L'AH doit garantir que la synchronisation est maintenue lors d'un saut de seconde

#### VI.2.5. Exigences du contenu d'une contremarque de temps

L'Autorité d'horodatage doit garantir que les contremarques de temps sont générées en toute sécurité et incluent le temps correct :

a) La contremarque de temps doit inclure l'identifiant du certificat de l'UH contenant :
   - Un identifiant du pays où l'AH est établie
   - Un identifiant de l'AH
   - Une identification de l'UH

b) La contremarque de temps doit inclure un identifiant de la politique d'horodatage

c) Chaque contremarque de temps doit comporter un identifiant unique

d) Les informations de temps doivent pouvoir être reliées à au moins un temps fourni par un laboratoire UTC(k)

e) Le temps inclus doit être synchronisé avec le temps UTC au moins avec l'exactitude définie dans la DPH

f) Si l'exactitude est différente de la seconde, elle doit être indiquée dans la contremarque de temps

g) La contremarque de temps doit inclure une représentation de la donnée à horodater (valeur de hachage et identifiant d'algorithme)

h) La contremarque de temps doit être signée en employant une clé produite exclusivement à cette fin

#### VI.2.6. Compromission de l'AH

En cas d'événements affectant la sécurité (compromission de clé privée ou perte de calibrage), l'AH doit :
- Traiter le cas dans son plan de secours
- Mettre à disposition une description de la compromission survenue
- Prendre les mesures pour que les contremarques de temps ne soient plus générées jusqu'à restauration
- Mettre à disposition toute information permettant d'identifier les contremarques de temps affectées
- Prévenir directement et sans délai le point de contact de la DGME

#### VI.2.7. Fin d'activité

L'Autorité d'horodatage doit garantir que les dérangements sont réduits au minimum suite à la cessation d'activité :

- Informer tous les abonnés et utilisateurs de la fin d'activité
- Abroger les autorisations données aux sous-traitants
- Transférer à un organisme fiable les obligations de maintien des fichiers d'audit
- Maintenir ou transférer les obligations de rendre disponibles les clés publiques et certificats
- Détruire les clés privées des UH de façon irréversible
- Prendre les mesures pour couvrir les dépenses en cas de faillite
- Prévenir le point de contact de la DGME

### VI.3. Exigences physiques et environnementales, procédurales et organisationnelles

#### VI.3.1. Exigences physiques et environnementales

L'Autorité d'horodatage doit garantir que l'accès physique aux services critiques est contrôlé et que les risques physiques sont réduits au minimum :

- L'accès physique aux équipements doit être limité aux individus autorisés
- Des contrôles doivent éviter la perte, les dégâts ou la compromission d'actifs
- Des contrôles d'accès doivent être appliqués aux modules d'horodatage
- Le système d'horodatage doit fonctionner dans un environnement protégeant physiquement les services
- La protection physique doit être réalisée par un périmètre de sécurité dédié (barrières physiques)
- La politique de sécurité physique et environnementale doit concerner : contrôle d'accès physique, protection catastrophes naturelles, sécurité incendie, défaillance services de base, protection vol/casse/pénétration, rétablissement après désastre

#### VI.3.2. Exigences procédurales

L'Autorité d'horodatage doit garantir que les composants du système d'Horodatage sont sûrs et correctement opérés :

- L'intégrité des composants doit être protégée contre les virus, logiciels malveillants et non autorisés
- Un rapport d'incident et des procédures de réponse aux incidents doivent être employés
- Les supports doivent être manipulés de manière sécuritaire
- Des procédures doivent être établies pour tous les rôles de confiance et administratifs
- Les charges doivent être contrôlées et des projections de charge effectuées
- L'AH agira de façon opportune et coordonnée pour répondre aux incidents
- Les opérations de sécurité doivent être séparées des autres opérations

**Gestion d'accès au système** :
- Des contrôles (pare-feux) doivent protéger le réseau interne d'accès non autorisés
- L'AH doit garantir une administration efficace des utilisateurs
- L'accès aux fonctions du système doit être limité conformément à la politique de contrôle d'accès
- Le personnel doit être correctement identifié et authentifié avant d'utiliser des applications critiques
- Le personnel sera tenu responsable de ses activités (fichiers d'audit)
- Une surveillance permanente et des équipements d'alarme doivent permettre de détecter, enregistrer et réagir aux tentatives d'accès non autorisé

**Déploiement et Maintenance** :
- L'AH emploiera des produits et systèmes de confiance
- Une analyse des exigences de sécurité doit être effectuée lors de la conception
- Des procédures de contrôle de changement doivent être appliquées pour les nouvelles versions et corrections

#### VI.3.3. Exigences organisationnelles

L'Autorité d'horodatage doit garantir que le personnel et les pratiques d'embauche améliorent la fiabilité des opérations :

- L'AH doit employer un personnel possédant l'expertise, l'expérience et les qualifications nécessaires
- Les rôles de sécurité et responsabilités doivent être documentés dans des descriptions de poste
- Les rôles de confiance doivent être clairement identifiés
- Des descriptions de fonctions doivent être définies du point de vue de la séparation des responsabilités et du privilège minimum

**Le personnel de gestion doit posséder** :
- La connaissance de la technologie de l'horodatage
- La connaissance de la technologie de la signature numérique
- La connaissance des mécanismes de calibrage/synchronisation des horloges avec le temps UTC
- Pour le personnel de sécurité : une bonne connaissance des procédures de sécurité
- L'expérience avec la sécurité de l'information et l'évaluation des risques

**Rôles de confiance** :
- **Officiers chargés de la sécurité** : responsabilité complète d'administrer les pratiques de sécurité
- **Administrateurs système** : autorisés à installer, configurer et maintenir les modules d'horodatage
- **Opérateurs système** : responsables du fonctionnement quotidien, autorisés pour les opérations de sauvegarde et secours
- **Auditeurs de système** : autorisés à consulter les archives et fichiers d'audit

L'AH ne doit pas nommer aux rôles de confiance toute personne connue pour avoir une condamnation pour un crime sérieux ou une infraction affectant son adéquation avec la position.

### VI.4. Exigences de sécurité techniques

#### VI.4.1. Exactitude temps

Si une unité d'horodatage fournit une exactitude différente de la seconde, alors cette exactitude doit être indiquée dans chaque contremarque de temps générée.

#### VI.4.2. Génération de clé

L'Autorité d'horodatage doit garantir que toutes les clés cryptographiques sont produites dans des circonstances contrôlées. La génération des clés de signature des UH doit être effectuée dans un module d'horodatage répondant aux exigences du chapitre IX.

#### VI.4.3. Certification des clés de l'unité d'horodatage

L'Autorité d'Horodatage doit :
- S'assurer que la valeur de la clé publique et l'identifiant de l'algorithme de signature contenus dans la demande de certificat sont égaux à ceux générés par l'UH
- S'assurer que la demande de certificat contient : le nom (DN) de l'UH, la valeur de la clé publique et l'identifiant de l'algorithme, la durée d'utilisation souhaitée pour la clé privée
- Vérifier, lors de l'import du certificat, qu'il provient bien de l'AC auprès de laquelle la demande a été effectuée
- S'assurer que l'UH ne peut être opérationnelle qu'une fois ces exigences remplies

#### VI.4.4. Protection des clés privées des unités d'horodatage

L'Autorité d'horodatage doit garantir que des clés privées des UH restent confidentielles et conservent leur intégrité. Les clés de signature des UH doivent être gardées et utilisées à l'intérieur d'un module d'horodatage répondant aux exigences du chapitre IX.

#### VI.4.5. Exigences de sauvegarde des clés des unités d'horodatage

Les clés privées des UH peuvent faire l'objet de copies de secours :
- Soit dans un module d'horodatage conforme aux exigences du chapitre IX
- Soit hors d'un module d'horodatage mais sous forme chiffrée et avec un mécanisme de contrôle d'intégrité

Le chiffrement correspondant doit offrir un niveau de sécurité équivalent ou supérieur au stockage au sein du module d'horodatage, s'appuyant sur un algorithme, une longueur de clé et un mode opératoire conformes au [RGS_B1].

Les opérations de chiffrement et de déchiffrement doivent être effectuées à l'intérieur du module d'horodatage de sorte que les clés privées ne soient à aucun moment en clair en dehors du module.

#### VI.4.6. Destruction des clés des unités d'horodatage

L'Autorité d'horodatage doit garantir que les clés de signature des UH sont détruites à la fin de leur cycle de vie.

#### VI.4.7. Algorithmes obligatoires

L'Autorité d'horodatage doit :
- Accepter des valeurs de hachage générées par des clients et employant les algorithmes de hachage conformes au chapitre VIII
- Générer des contremarques de temps signées selon les algorithmes et longueurs de clé conformes au chapitre VIII

#### VI.4.8. Vérification des contremarques de temps

L'Autorité d'horodatage doit garantir que les utilisateurs peuvent avoir accès à l'information utilisable pour vérifier la signature numérique :
- Les certificats des UH doivent être disponibles (joints à la contremarque de temps ou via un serveur)
- Un ou plusieurs certificats utilisables pour valider une chaîne de certificats doivent être disponibles

#### VI.4.9. Durée de validité des certificats de clé publique des unités d'horodatage

La durée de validité des certificats des UH ne doit pas être plus longue que :
- La durée de vie cryptographique de la clé privée associée (cf [RGS_B_1])
- La fin de validité du certificat d'AC qui l'a émis

Plus la durée de vie du certificat sera grande, plus la taille des enregistrements d'audit à conserver sera importante.

#### VI.4.10. Durée d'utilisation des clés privées des unités d'horodatage

La durée d'utilisation d'une clé privée sera au plus égale à la période de validité du certificat de clé publique correspondant. Toutefois, elle sera en pratique réduite afin que la validité des contremarques de temps générées puisse être effectuée durant un laps de temps suffisant.

La durée d'utilisation peut être définie soit au moment de l'initialisation du boîtier de l'UH, soit en définissant cette durée dans le certificat (PrivateKeyUsagePeriod).

---

## VII. Annexe 1 : Documents cités en référence

### VII.1. Réglementation

| Renvoi | Document |
|--------|----------|
| [CNIL] | Loi n° 78-17 du 6 janvier 1978 relative à l'informatique, aux fichiers et aux libertés, modifiée par la loi n° 2004-801 du 6 août 2004 |
| [ORDONNANCE] | Ordonnance n° 2005-1516 du 8 décembre 2005 relative aux échanges électroniques entre les usagers et les autorités administratives |
| [DécretRGS] | Décret pris pour l'application des articles 9, 10 et 12 de l'ordonnance n° 2005-1516 du 8 décembre 2005 |

### VII.2. Documents techniques

| Renvoi | Document |
|--------|----------|
| [RGS] | Référentiel Général de Sécurité – version 1.0 |
| [RGS_A_4] | RGS - Politiques de Certification Types - Profils de certificats, de LCR et OCSP et algorithmes cryptographiques – Version 2.3 |
| [RGS_B_1] | Règles et recommandations concernant le choix et le dimensionnement des mécanismes cryptographiques, ANSSI, Version 1.20 |
| [ETSI_PH] | ETSI TS 102 023 V1.2.2 (2008-10) Policy requirements for Time-Stamping Authority |
| [ETSI_TSP] | ETSI TS 101 861 V1.2.1 (2002-03) Time Stamping Profile |
| [PP_HORO] | DCSSI - Profil de Protection - Systèmes d'horodatage EAL3+ DCSSI PP 2008/07 |
| [RFC3161] | IETF - Internet X.509 Public Key Infrastructure - Time-Stamp Protocol - 08/2001 |
| [TF.460-5] | ITU-R Recommendation TF.460-5 (1997) "Standard-Frequency and Time-signal emissions" |
| [TF.536-1] | ITU-R Recommendation TF. TF.536-1(1998): "Time-Scale Notations" |

---

## VIII. Annexe 2 : Exigences sur les formats des contremarques de temps, des certificats et des LCR et sur les algorithmes cryptographiques

### VIII.1. Contremarques de temps

Les contremarques de temps fournies par les AH respectant la présente PH Type doivent être une structure **TimeStampToken** conforme au [RFC3161].

| Champ | Exigences |
|-------|-----------|
| messageImprint | Cf. chapitre VIII.3 sur les exigences concernant les fonctions de hachage |
| accuracy | Si la synchronisation avec le temps UTC est différente de 1 seconde, ce champ doit être présent et préciser l'exactitude. Si la synchronisation est de 1 seconde, il peut être omis |
| ordering | Ce champ doit être absent ou bien contenir la valeur false |
| tsa | Si ce champ est présent, il doit être identique au champ subject du certificat de l'UH ayant signé la contremarque de temps |
| extensions | Des extensions peuvent être incluses par l'AH, mais aucune ne doit être marquée comme critique |

### VIII.2. Certificats et LCR

Les gabarits des certificats d'UH doivent être conformes aux exigences des certificats de type « cachet » dont la clé privée associée est utilisée pour signer des jetons d'horodatage décrites dans le document [RGS_A_4].

**Rappels** :
- L'extension "Extended Key Usage" doit être présente, marquée critique, et ne contenir que l'identifiant **id-kp-timeStamping** à l'exclusion de toute autre
- Le champ "DN Subject" doit identifier l'AH suivant les mêmes règles que l'identification des AC (cf. chapitre VII.1 de [RGS_A_4]) et l'identifiant propre à l'UH concernée doit être porté dans l'attribut commonName du DN
- La durée de vie maximale est bornée selon le couple {durée de vie cryptographique de la clé ; fin de validité de l'AC émettrice}

Pour les LCR comportant des numéros de série correspondant à des certificats d'UH, il est obligatoire de supporter l'extension d'entrée LCR : **reasonCode**.

### VIII.3. Algorithmes cryptographiques

Les algorithmes et fonctions cryptographiques (hachage, signature) mis en œuvre pour la génération des différents certificats, pour la génération des contremarques de temps ainsi que la valeur du champ messageImprint doivent respecter les exigences correspondantes de [RGS_A_4].

---

## IX. Annexe 3 : Exigences de sécurité du module d'horodatage des UH

### IX.1. Exigences sur les objectifs de sécurité

Le module d'horodatage, utilisé par l'AH pour générer et mettre en œuvre les clés de signature des UH et pour générer les contremarques de temps, doit répondre aux exigences de sécurité suivantes :

- Garantir que la génération des bi-clés des UH est réalisée exclusivement par des utilisateurs autorisés et garantir la robustesse cryptographique des bi-clés générées
- Assurer la confidentialité et l'intégrité des clés privées de signature des UH durant tout leur cycle de vie, et permettre leur destruction sûre en fin de vie
- Garantir l'authenticité et l'intégrité des clés publiques lors de leur export hors du module
- Lors de son importation dans le module, vérifier la correspondance entre le certificat importé et la clé publique de l'UH contenue dans le module
- Être capable d'identifier et d'authentifier ses utilisateurs
- Limiter l'accès à ses services en fonction de l'utilisateur et du rôle qui lui a été assigné
- Être capable de mener une série de tests pour vérifier qu'il fonctionne correctement et entrer dans un état sûr s'il détecte une erreur
- Être capable de détecter les tentatives d'altérations physiques et d'entrer dans un état sûr
- Permettre de créer une signature numérique qui ne révèle pas les clés privées de l'UH et qui ne peut pas être falsifiée
- Créer des enregistrements d'audit pour chaque modification concernant la sécurité
- Garantir la synchronisation de son horloge avec le temps UTC suivant la précision définie dans la DPH
- Fournir des contremarques de temps conformes aux requêtes reçues

### IX.2. Exigences complémentaires

Il est recommandé que le module d'horodatage utilisé par l'AH soit **qualifié au niveau standard**, selon la procédure précisée dans le [DécretRGS], attestant de sa conformité aux exigences de sécurité du chapitre IX.1. Le profil de protection [PP_HORO] couvre ces exigences.

**Lorsque le PSHE souhaite faire qualifier son offre d'horodatage**, selon la procédure décrite dans le [DécretRGS], alors :
- Le module d'horodatage utilisé par l'AH **doit être qualifié au niveau standard**
- Les fonctions cryptographiques (génération des bi-clés des UH et signature des contremarques de temps) doivent être assurées par un module cryptographique évalué conformément aux exigences spécifiées aux alinéas 7.2.1.b et 7.2.2.a de la norme [ETSI_PH]

---

## X. Annexe 4 - Vérification ou utilisation (informative)

Cette politique d'horodatage prévoit la vérification d'une contremarque de temps pendant la période de validité du certificat de clé publique de l'unité d'horodatage qui l'a générée.

### X.1. Empilement des contremarques de temps

S'il s'avère nécessaire de prolonger la durée de vie d'une contremarque de temps ou d'en conforter la robustesse, il est possible d'ajouter une contremarque de temps supplémentaire fournie par une autre unité d'horodatage.

Pour cela, il convient de pouvoir prouver que le certificat de l'UH référencé dans la contremarque de temps d'origine n'était pas révoqué au moment où la contremarque de temps supplémentaire a été ajoutée.

Les LCR des AC en charge de l'UH devront être archivées afin de pouvoir démontrer que l'UH ayant généré la première contremarque de temps n'était pas révoquée à ce moment-là.

Lors d'une vérification ultérieure, un utilisateur devra vérifier les deux contremarques de temps et s'assurer que l'UH ayant généré la première contremarque n'était pas révoquée à la date de la seconde contremarque de temps.

### X.2. Gestion de la révocation par les Autorités de Certification

La gestion de la révocation des certificats des unités d'horodatage doit être assurée comme pour toute AC.

Un service OCSP ferait l'affaire mais l'usage des LCR est tout à fait adapté étant donné que le risque de compromission d'une clé est minime.

Il est recommandé de mettre en place une AC spécifiquement en charge de la gestion des unités d'horodatage.

---

## XI. Annexe 5 - Précision de la synchronisation de l'horloge

La précision de l'horloge a souvent été ressentie comme un paramètre essentiel. On peut facilement obtenir des synchronisations par rapport à une horloge de référence UTC avec une précision de 10 microsecondes, mais cette précision n'a pas de sens dans le cadre de l'horodatage pour deux raisons :
- Le temps de transit de la requête est largement supérieur à ce temps
- L'opération de signature de la contremarque de temps est largement supérieure et se situe au mieux dans l'échelle des 10 millisecondes

**Une précision d'une seconde est largement suffisante pour toutes les applications.**

Des précisions meilleures peuvent cependant être utiles dans des contextes de liaisons particulières "courtes" (réseaux locaux ou à l'intérieur d'un même système informatique). En l'état actuel de la technique, un horodatage bien meilleur que 10 millisecondes n'aurait pas grand sens.

Sans changer de politique, il est possible d'avoir une précision meilleure. Quand c'est le cas, cette précision est indiquée à l'intérieur de la contremarque de temps. La précision est une propriété intrinsèque de l'unité d'horodatage.

---

## XII. Annexe 6 - Protocole d'horodatage

Aucun protocole spécifique n'est requis, en dehors du format de la contremarque de temps (cf VIII.1.). Si la déclaration des pratiques d'horodatage déclare que le protocole d'horodatage est conforme au [RFC3161] ou au standard [ETSI_TSP], alors les sections suivantes s'appliquent.

### XII.1. Conformité au RFC 3161

Cette section s'appuie sur le contenu du RFC 2026 (section 4.1.2).

L'IETF n'a pas "des clauses de conformités". Au lieu de cela, l'IETF stipule des tests d'interopérabilité.

Il est requis de démontrer l'interopérabilité avec au moins une mise en œuvre indépendante réalisée à partir d'un code de base différent.

L'Autorité d'horodatage doit pouvoir fournir une documentation au sujet des tests d'interopérabilité :
1. Le test s'applique à toutes les options et particularités de la spécification
2. La documentation doit inclure l'information concernant le support de chacune des options individuelles et des particularités

### XII.2. Conformité au standard ETSI TS 101 861

La norme [ETSI_TSP] inclut dans sa section 5 un profil pour le format de la réponse et dans sa section 6 un profil pour les protocoles de transport à supporter.

---

## XIII. Annexe 7 - Compatibilité avec la politique d'horodatage de l'ETSI

La présente PH Type est compatible sur la plupart des points avec la politique d'horodatage définie dans le document [ETSI_PH].

**Points de la politique [ETSI_PH] non repris dans cette politique** :

| Référence ETSI | Texte | Justificatif |
|----------------|-------|--------------|
| 5.1 | Exigence d'une précision d'au moins une seconde | Non reprise pour permettre des précisions moindres |
| 7.1.2 d) 7 | Durée de vie estimée de la signature | Non reprise car il s'agit d'une information non garantie |
| 7.2.1 | Double contrôle de personnes pour actions sensibles sur les clés privées | Non reprise dans l'annexe A5 |
| 7.2.2 | Sauvegarde et import de clés privées sont des pratiques dépréciées | La sauvegarde et l'import des clés privées sont autorisés dans l'annexe A5 sans mention de dépréciation |
| 7.2.5.c | Rejet des tentatives de génération si la clé privée a expiré | L'obligation de destruction de la clé entraîne de facto l'impossibilité de générer de nouvelles contremarques de temps |

Cette politique d'horodatage a été réalisée dans l'optique de permettre à un opérateur de fournir un service compatible à la fois avec la présente PH et avec celle de l'ETSI. Il conviendra toutefois à l'AH de vérifier que les exigences des deux politiques sont respectées.

Si la conformité aux exigences de la PH de l'ETSI est réalisée, alors l'identifiant d'objet (OID) défini dans le document de l'ETSI peut aussi être utilisé :

```
{itu-t(0) identified-organization(4) etsi(0) time-stamp-policy(2023) policy-identifiers(1) baseline-ts-policy(1)}
```
