# Annexe A3 au RGS v2.0 : Politique de Certification Type - Certificats électroniques de services applicatifs

**Version 3.0 - 27 février 2014**

**Diffusion : PUBLIC**

---

## Table des matières

- [I. Introduction](#i-introduction)
- [II. Responsabilités concernant la mise à disposition des informations](#ii-responsabilités-concernant-la-mise-à-disposition-des-informations-devant-être-publiées)
- [III. Identification et authentification](#iii-identification-et-authentification)
- [IV. Exigences opérationnelles sur le cycle de vie des certificats](#iv-exigences-opérationnelles-sur-le-cycle-de-vie-des-certificats)
- [V. Mesures de sécurité non techniques](#v-mesures-de-sécurité-non-techniques)
- [VI. Mesures de sécurité techniques](#vi-mesures-de-sécurité-techniques)
- [VII. Profils des certificats, OCSP et des LCR](#vii-profils-des-certificats-ocsp-et-des-lcr)
- [VIII. Audit de conformité et autres évaluations](#viii-audit-de-conformité-et-autres-évaluations)
- [IX. Autres problématiques métiers et légales](#ix-autres-problématiques-métiers-et-légales)
- [X. Annexe 1 : Documents cités en référence](#x-annexe-1--documents-cités-en-référence)
- [XI. Annexe 2 : Exigences de sécurité du module cryptographique de l'AC](#xi-annexe-2--exigences-de-sécurité-du-module-cryptographique-de-lac)
- [XII. Annexe 3 : Exigences de sécurité du dispositif de protection](#xii-annexe-3--exigences-de-sécurité-du-dispositif-de-protection)

---

## I. Introduction

### I.1. Présentation générale

La présente Politique de Certification (PC) Type définit les exigences auxquelles une Autorité de Certification (AC) doit satisfaire pour la gestion des certificats électroniques de services applicatifs (serveurs) qu'elle délivre, ainsi que les exigences qui en découlent sur les autres composantes de l'Infrastructure de Gestion de Clés (IGC) et sur les autres intervenants.

Cette PC Type permet de délivrer des certificats de services applicatifs utilisés pour les fonctions de sécurité suivantes :
- **Cachet (seal)** : pour apposer un cachet électronique sur des données
- **Signature de codes (code signing)** : pour signer du code exécutable
- **Authentification serveur (server authentication)** : pour authentifier un serveur auprès de clients

### I.2. Identification du document

Cette PC Type est identifiée par un OID (Object Identifier) spécifique défini par l'ANSSI.

### I.3. Entités intervenant dans l'IGC

#### I.3.1. Autorité de Certification (AC)

L'AC est responsable de :
- L'émission, la délivrance et la gestion des certificats de services applicatifs
- La mise en œuvre de la Politique de Certification
- La publication des informations nécessaires

#### I.3.2. Autorité d'Enregistrement (AE)

L'AE est responsable de :
- La vérification de l'identité des demandeurs
- La validation des demandes de certificats
- La transmission des demandes à l'AC

#### I.3.3. Service de publication

Le service de publication assure la diffusion des :
- Certificats d'AC
- Listes de Certificats Révoqués (LCR)
- Politique de Certification

#### I.3.4. Responsable du Certificat (RC)

Le RC est la personne physique responsable du certificat de service applicatif. Le RC :
- Effectue la demande de certificat pour le compte du service applicatif
- S'engage au respect des conditions d'utilisation du certificat
- Est responsable de la protection de la clé privée du service applicatif

#### I.3.5. Mandataire de Certification (MC)

Le MC peut intervenir dans le processus d'enregistrement pour le compte d'une entité (entreprise, administration). Il peut :
- Valider les demandes de certificats pour son entité
- Effectuer les demandes de révocation

### I.4. Usages des certificats

#### I.4.1. Domaines d'utilisation applicables

Les certificats couverts par cette PC Type sont destinés à trois usages :

| Usage | Description |
|-------|-------------|
| **Cachet** | Apposer un cachet électronique sur des données pour garantir leur origine et intégrité |
| **Signature de codes** | Signer du code exécutable pour garantir son authenticité et son intégrité |
| **Authentification serveur** | Authentifier un serveur dans le cadre d'échanges sécurisés (TLS/SSL) |

#### I.4.2. Niveaux de sécurité

Cette PC Type définit trois niveaux de sécurité :

| Niveau | Description |
|--------|-------------|
| **(\*)** | Niveau élémentaire - Exigences de base |
| **(\*\*)** | Niveau standard - Exigences renforcées |
| **(\*\*\*)** | Niveau renforcé - Exigences maximales |

### I.5. Gestion de la PC

#### I.5.1. Entité responsable de la PC

L'AC est responsable de l'élaboration et de la mise à jour de sa PC en conformité avec la présente PC Type.

#### I.5.2. Point de contact

L'AC doit identifier un point de contact pour toute question relative à sa PC.

#### I.5.3. Conformité de la DPC à la PC

L'AC doit démontrer la conformité de sa Déclaration des Pratiques de Certification (DPC) avec sa PC.

---

## II. Responsabilités concernant la mise à disposition des informations devant être publiées

### II.1. Entités chargées de la mise à disposition des informations

L'AC est responsable de la mise à disposition des informations relatives aux certificats qu'elle émet.

### II.2. Informations publiées

L'AC doit publier au minimum :
- La PC applicable aux certificats de services applicatifs
- Les certificats d'AC (certificats racines et intermédiaires)
- Les Listes de Certificats Révoqués (LCR)
- Les conditions générales d'utilisation

### II.3. Délais et fréquences de publication

| Information | Délai de publication |
|-------------|---------------------|
| PC | Avant la première émission de certificat |
| Certificats d'AC | Immédiate |
| LCR | Selon la fréquence définie (cf. IV.9) |

### II.4. Contrôle d'accès aux informations publiées

Les informations publiées doivent être accessibles en lecture libre. L'AC doit garantir l'intégrité des informations publiées.

---

## III. Identification et authentification

### III.1. Nommage

#### III.1.1. Types de noms

Les certificats de services applicatifs doivent contenir un nom distinctif (DN - Distinguished Name) conforme à la norme X.500.

#### III.1.2. Nécessité d'utilisation de noms explicites

Les noms utilisés dans les certificats doivent être explicites et permettre l'identification sans ambiguïté du service applicatif.

#### III.1.3. Anonymat des services applicatifs

L'anonymat n'est pas permis pour les certificats de services applicatifs.

#### III.1.4. Règles d'interprétation des différentes formes de noms

L'AC doit documenter les règles d'interprétation des noms dans sa DPC.

#### III.1.5. Unicité des noms

L'AC doit garantir l'unicité des noms dans les certificats qu'elle émet. Un même DN ne peut être attribué qu'à un seul service applicatif.

#### III.1.6. Identification, authentification et rôle des marques déposées

L'AC doit vérifier que le demandeur est autorisé à utiliser le nom demandé et qu'il ne porte pas atteinte aux droits de propriété intellectuelle de tiers.

### III.2. Validation initiale de l'identité

#### III.2.1. Méthode pour prouver la possession de la clé privée

Si la bi-clé est générée par le service applicatif, la preuve de possession de la clé privée peut être apportée par :
- Une requête de certificat signée (format PKCS#10)
- Tout autre mécanisme cryptographique équivalent

#### III.2.2. Validation de l'identité de l'entité responsable du service applicatif

L'AC ou l'AE doit vérifier :
- L'existence légale de l'entité (entreprise, administration)
- L'identité du RC
- L'autorisation du RC à effectuer la demande

**Niveau (\*\*\*)**
- Vérification face-à-face de l'identité du RC
- Vérification des justificatifs d'identité originaux

**Niveau (\*\*)**
- Vérification de l'identité du RC par face-à-face ou par signature électronique de niveau (\*\*) minimum
- Vérification des justificatifs d'identité

**Niveau (\*)**
- Vérification de l'identité du RC par tout moyen approprié
- Vérification des justificatifs d'identité

#### III.2.3. Informations non vérifiées

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

#### III.2.4. Validation de l'autorité du demandeur

L'AC doit vérifier que le demandeur (RC) est autorisé à effectuer une demande de certificat pour le service applicatif concerné.

#### III.2.5. Critères d'interopérabilité

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

### III.3. Identification et validation lors du renouvellement

#### III.3.1. Renouvellement avec génération de nouvelles bi-clés

Le renouvellement avec nouvelle bi-clé suit les mêmes procédures que la demande initiale.

**Niveau (\*\*\*)**
L'identification et l'authentification du RC doivent être effectuées en face-à-face.

**Niveau (\*\*)**
L'identification peut être effectuée en face-à-face ou via une signature électronique de niveau (\*\*) minimum.

**Niveau (\*)**
L'identification peut être effectuée par tout moyen approprié.

#### III.3.2. Renouvellement sans génération de nouvelles bi-clés

Le renouvellement sans nouvelle bi-clé n'est pas autorisé dans le cadre de cette PC Type.

### III.4. Identification et validation d'une demande de révocation

L'AC doit mettre en place des procédures pour authentifier les demandes de révocation. Les demandes peuvent émaner :
- Du RC
- Du MC (le cas échéant)
- De l'entité responsable du service applicatif

**Tous niveaux**
L'authentification de la demande de révocation doit permettre de s'assurer de l'identité du demandeur et de son autorisation à demander la révocation.

---

## IV. Exigences opérationnelles sur le cycle de vie des certificats

### IV.1. Demande de certificat

#### IV.1.1. Origine d'une demande de certificat

La demande de certificat peut provenir :
- Du RC directement
- Du MC pour le compte de son entité

#### IV.1.2. Processus et responsabilités pour l'établissement d'une demande de certificat

Le RC ou MC doit :
- Fournir les informations nécessaires à l'identification du service applicatif
- Accepter les conditions d'utilisation du certificat
- Fournir la preuve de possession de la clé privée (si bi-clé générée par le service)

### IV.2. Traitement d'une demande de certificat

#### IV.2.1. Exécution des processus d'identification et de validation de la demande

L'AE vérifie :
- L'identité du demandeur
- La légitimité de la demande
- La complétude du dossier d'enregistrement

#### IV.2.2. Acceptation ou rejet de la demande

L'AE peut rejeter une demande si :
- Les informations fournies sont incomplètes ou incorrectes
- L'identité du demandeur ne peut être vérifiée
- Le demandeur n'est pas autorisé à effectuer la demande

#### IV.2.3. Durée d'établissement du certificat

L'AC doit définir dans sa DPC les délais de traitement des demandes de certificats.

### IV.3. Délivrance du certificat

#### IV.3.1. Actions de l'AC concernant la délivrance du certificat

Après validation de la demande, l'AC :
- Génère le certificat
- Le signe avec sa clé privée
- Le transmet au RC

#### IV.3.2. Notification de la délivrance du certificat au RC

L'AC doit notifier le RC de la délivrance du certificat par un moyen sécurisé.

### IV.4. Acceptation du certificat

#### IV.4.1. Procédure d'acceptation du certificat

Le RC doit vérifier le contenu du certificat et confirmer son acceptation.

#### IV.4.2. Publication du certificat

La publication des certificats de services applicatifs n'est pas obligatoire, sauf si l'AC le prévoit dans sa PC.

#### IV.4.3. Notification de la délivrance du certificat aux autres entités

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

### IV.5. Usages de la bi-clé et du certificat

#### IV.5.1. Utilisation de la clé privée et du certificat par le RC

Le RC doit :
- Utiliser la clé privée uniquement pour l'usage prévu dans le certificat
- Protéger la clé privée contre tout accès non autorisé
- Ne pas divulguer la clé privée

#### IV.5.2. Utilisation du certificat par les utilisateurs de certificats

Les utilisateurs de certificats doivent :
- Vérifier la validité du certificat (dates, révocation)
- Vérifier la chaîne de certification
- Respecter l'usage prévu dans le certificat

### IV.6. Renouvellement d'un certificat

Le renouvellement sans changement de bi-clé n'est pas autorisé.

### IV.7. Délivrance d'un nouveau certificat suite au changement de la bi-clé

#### IV.7.1. Causes possibles de changement de bi-clé

- Expiration du certificat
- Révocation du certificat
- Changement des informations contenues dans le certificat
- Mise à jour des algorithmes cryptographiques

#### IV.7.2. Origine d'une demande de nouveau certificat

La demande suit le processus défini en IV.1.

#### IV.7.3. Traitement d'une demande de nouveau certificat

Le traitement suit le processus défini en IV.2 et IV.3.

### IV.8. Modification du certificat

La modification d'un certificat n'est pas autorisée. Toute modification nécessite l'émission d'un nouveau certificat.

### IV.9. Révocation et suspension des certificats

#### IV.9.1. Causes possibles de révocation

Les causes de révocation incluent :
- Compromission ou suspicion de compromission de la clé privée
- Changement des informations contenues dans le certificat
- Non-respect des conditions d'utilisation
- Cessation d'activité du service applicatif
- Demande du RC ou du MC
- Décision de l'AC

#### IV.9.2. Origine d'une demande de révocation

La demande de révocation peut provenir :
- Du RC
- Du MC
- De l'entité responsable du service applicatif
- De l'AC (révocation d'office)

#### IV.9.3. Procédure de traitement d'une demande de révocation

L'AC doit :
- Authentifier le demandeur
- Vérifier la légitimité de la demande
- Procéder à la révocation
- Mettre à jour la LCR

#### IV.9.4. Délai accordé au RC pour formuler la demande de révocation

Le RC doit formuler sa demande de révocation sans délai dès qu'il a connaissance d'une cause de révocation.

#### IV.9.5. Délai de traitement par l'AC d'une demande de révocation

| Niveau | Délai maximum de traitement |
|--------|---------------------------|
| (\*\*\*) | 24 heures |
| (\*\*) | 24 heures |
| (\*) | 72 heures |

#### IV.9.6. Exigences de vérification de la révocation par les utilisateurs de certificats

Les utilisateurs de certificats doivent vérifier le statut de révocation des certificats avant utilisation.

#### IV.9.7. Fréquence d'établissement des LCR

| Niveau | Fréquence de publication des LCR |
|--------|----------------------------------|
| (\*\*\*) | Au minimum toutes les 24 heures |
| (\*\*) | Au minimum toutes les 24 heures |
| (\*) | Au minimum tous les 7 jours |

#### IV.9.8. Délai maximum de publication d'une LCR

La LCR doit être publiée immédiatement après sa génération.

#### IV.9.9. Disponibilité d'un système de vérification en ligne du statut des certificats

L'AC peut mettre en place un service OCSP (Online Certificate Status Protocol) en complément des LCR.

**Niveau (\*\*\*)**
Un service OCSP est recommandé.

#### IV.9.10. Exigences de vérification en ligne de la révocation par les utilisateurs de certificats

Si un service OCSP est disponible, les utilisateurs de certificats peuvent l'utiliser pour vérifier le statut de révocation.

#### IV.9.11. Autres moyens d'information disponibles concernant les révocations

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

#### IV.9.12. Exigences spécifiques en cas de compromission de la clé privée de l'AC

En cas de compromission de la clé privée de l'AC :
- L'AC doit révoquer tous les certificats émis avec cette clé
- L'AC doit informer les utilisateurs de certificats
- L'AC doit publier l'information sur son site

#### IV.9.13. Suspension de certificats

La suspension n'est pas autorisée dans le cadre de cette PC Type.

### IV.10. Fonction d'information sur l'état des certificats

#### IV.10.1. Caractéristiques opérationnelles

Le service d'information sur l'état des certificats doit être disponible en permanence.

| Niveau | Disponibilité |
|--------|---------------|
| (\*\*\*) | 99,9% |
| (\*\*) | 99% |
| (\*) | 95% |

#### IV.10.2. Disponibilité de la fonction

Le service de publication des LCR et, le cas échéant, le service OCSP doivent être accessibles 24h/24, 7j/7.

### IV.11. Fin de la relation entre le RC et l'AC

La fin de la relation peut résulter de :
- L'expiration du certificat
- La révocation du certificat
- La résiliation du contrat

### IV.12. Séquestre de clé et recouvrement

Le séquestre des clés privées n'est pas autorisé pour les certificats de services applicatifs couverts par cette PC Type.

---

## V. Mesures de sécurité non techniques

### V.1. Mesures de sécurité physiques

#### V.1.1. Situation géographique et construction des sites

Les locaux hébergeant les composantes de l'IGC doivent être situés dans des bâtiments offrant un niveau de protection physique adapté.

**Niveau (\*\*\*)**
- Locaux dédiés et sécurisés
- Contrôle d'accès strict
- Surveillance permanente

**Niveau (\*\*)**
- Locaux sécurisés
- Contrôle d'accès
- Surveillance

**Niveau (\*)**
- Protection physique appropriée
- Contrôle d'accès

#### V.1.2. Accès physique

| Niveau | Exigences |
|--------|-----------|
| (\*\*\*) | Double contrôle d'accès, journalisation, vidéosurveillance |
| (\*\*) | Contrôle d'accès, journalisation |
| (\*) | Contrôle d'accès approprié |

#### V.1.3. Alimentation électrique et climatisation

Les équipements de l'IGC doivent être protégés contre les pannes d'alimentation et les variations de température.

**Niveaux (\*\*) et (\*\*\*)**
- Alimentation secourue (onduleurs, groupes électrogènes)
- Climatisation redondante

#### V.1.4. Vulnérabilité aux dégâts des eaux

Les locaux doivent être protégés contre les risques d'inondation.

#### V.1.5. Prévention et protection incendie

Les locaux doivent être équipés de systèmes de détection et d'extinction d'incendie.

#### V.1.6. Conservation des supports

Les supports de sauvegarde et les archives doivent être conservés dans des conditions garantissant leur intégrité et leur confidentialité.

#### V.1.7. Mise hors service des supports

Les supports contenant des informations sensibles doivent être détruits de manière sécurisée conformément au guide [972-1].

#### V.1.8. Sauvegarde hors site

Les sauvegardes doivent être stockées dans un site distinct du site principal.

### V.2. Mesures de sécurité procédurales

#### V.2.1. Rôles de confiance

Les rôles de confiance au sein de l'IGC incluent :
- Responsable sécurité
- Administrateur système
- Opérateur
- Auditeur

**Niveaux (\*\*) et (\*\*\*)**
Les rôles doivent être séparés (une personne ne peut cumuler certains rôles incompatibles).

#### V.2.2. Nombre de personnes requises par tâche

| Tâche | Niveau (\*) | Niveau (\*\*) | Niveau (\*\*\*) |
|-------|------------|--------------|----------------|
| Génération de clé d'AC | 1 | 2 | 2 |
| Activation de clé d'AC | 1 | 2 | 2 |
| Cérémonies de clés | 1 + témoins | 2 + témoins (dont 1 externe) | 2 + témoins (dont 1 externe) |

#### V.2.3. Identification et authentification pour chaque rôle

Chaque personne occupant un rôle de confiance doit être identifiée et authentifiée de manière forte.

#### V.2.4. Rôles exigeant une séparation des attributions

**Niveaux (\*\*) et (\*\*\*)**
Les rôles suivants doivent être séparés :
- Responsable sécurité et opérateur
- Administrateur et auditeur

### V.3. Mesures de sécurité vis-à-vis du personnel

#### V.3.1. Qualifications, compétences et habilitations requises

Le personnel de l'IGC doit posséder les qualifications et compétences nécessaires à l'exercice de ses fonctions.

#### V.3.2. Procédures de vérification des antécédents

**Niveaux (\*\*) et (\*\*\*)**
Des vérifications des antécédents doivent être effectuées pour le personnel occupant des rôles de confiance.

#### V.3.3. Exigences en matière de formation initiale

Le personnel doit recevoir une formation adaptée à ses fonctions avant de les exercer.

#### V.3.4. Exigences et fréquence en matière de formation continue

Le personnel doit recevoir une formation continue pour maintenir ses compétences.

#### V.3.5. Fréquence et séquence de rotation entre différentes attributions

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

#### V.3.6. Sanctions en cas d'actions non autorisées

L'AC doit définir les sanctions applicables en cas de manquement aux règles de sécurité.

#### V.3.7. Exigences vis-à-vis du personnel des prestataires externes

Les prestataires externes doivent respecter les mêmes exigences de sécurité que le personnel de l'AC.

#### V.3.8. Documentation fournie au personnel

Le personnel doit disposer de la documentation nécessaire à l'exercice de ses fonctions.

### V.4. Procédures de constitution des données d'audit

#### V.4.1. Type d'évènements à enregistrer

Les évènements suivants doivent être enregistrés :
- Démarrage et arrêt des systèmes
- Connexions et déconnexions
- Émission de certificats
- Révocations
- Publication des LCR
- Modification des paramètres de sécurité
- Accès physiques aux locaux

#### V.4.2. Fréquence de traitement des journaux d'évènements

| Niveau | Fréquence de revue |
|--------|-------------------|
| (\*\*\*) | Hebdomadaire |
| (\*\*) | Mensuelle |
| (\*) | Trimestrielle |

#### V.4.3. Période de conservation des journaux d'évènements

Les journaux d'évènements doivent être conservés pendant au moins :
- 3 ans pour les évènements courants
- 10 ans pour les évènements relatifs aux certificats

#### V.4.4. Protection des journaux d'évènements

Les journaux doivent être protégés en intégrité et en confidentialité.

#### V.4.5. Procédure de sauvegarde des journaux d'évènements

Les journaux doivent être sauvegardés régulièrement.

#### V.4.6. Système de collecte des journaux d'évènements

L'AC peut utiliser un système centralisé de collecte des journaux.

#### V.4.7. Notification de l'enregistrement d'un évènement au responsable de l'évènement

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

#### V.4.8. Évaluation des vulnérabilités

**Niveaux (\*\*) et (\*\*\*)**
Des évaluations de vulnérabilités doivent être réalisées régulièrement.

### V.5. Archivage des données

#### V.5.1. Types de données à archiver

Les données suivantes doivent être archivées :
- Politique de Certification
- DPC
- Contrats avec les RC
- Dossiers d'enregistrement
- Certificats émis
- LCR publiées
- Journaux d'évènements

#### V.5.2. Période de conservation des archives

| Type de données | Durée de conservation |
|-----------------|----------------------|
| PC et DPC | Durée de vie de l'AC + 5 ans |
| Certificats | Durée de validité + 5 ans minimum |
| Dossiers d'enregistrement | Durée de validité du certificat + 5 ans minimum |
| LCR | 10 ans minimum |

#### V.5.3. Protection des archives

Les archives doivent être protégées en intégrité et en confidentialité.

#### V.5.4. Procédures de sauvegarde des archives

Les archives doivent être sauvegardées régulièrement sur des supports appropriés.

#### V.5.5. Exigences d'horodatage des données

Les données archivées doivent être horodatées.

#### V.5.6. Système de collecte des archives

L'AC peut utiliser un système d'archivage électronique certifié.

#### V.5.7. Procédures de récupération et de vérification des archives

Les archives doivent pouvoir être récupérées et leur intégrité vérifiée.

### V.6. Changement de clé d'AC

L'AC doit prévoir les procédures de changement de clé d'AC :
- Génération d'une nouvelle bi-clé
- Continuité du service pendant la transition
- Information des utilisateurs de certificats

### V.7. Compromission et plan de reprise après sinistre

#### V.7.1. Procédures de remontée et de traitement des incidents et des compromissions

L'AC doit disposer de procédures de gestion des incidents couvrant :
- La détection et le signalement des incidents
- L'analyse et la qualification des incidents
- Le traitement et la résolution des incidents
- La communication vers les parties prenantes

#### V.7.2. Procédures de reprise en cas de corruption des ressources informatiques

L'AC doit disposer d'un plan de continuité d'activité (PCA) et d'un plan de reprise d'activité (PRA).

#### V.7.3. Procédures de reprise en cas de compromission de la clé privée de l'AC

En cas de compromission de la clé privée de l'AC :
- Révocation immédiate du certificat d'AC compromis
- Notification aux utilisateurs de certificats
- Révocation de tous les certificats émis par cette AC
- Génération d'une nouvelle bi-clé d'AC

### V.8. Fin de vie de l'AC

En cas de cessation d'activité de l'AC :
- Information des RC et des utilisateurs de certificats
- Révocation de tous les certificats en cours de validité
- Archivage des données pendant la durée requise
- Transfert éventuel des activités à une autre AC
- Destruction sécurisée des clés privées

---

## VI. Mesures de sécurité techniques

### VI.1. Génération et installation de bi-clés

#### VI.1.1. Génération des bi-clés

##### VI.1.1.1. Clés de l'AC

**Niveau (\*\*\*)**
Les cérémonies de clés doivent se dérouler sous le contrôle d'au moins deux personnes ayant des rôles de confiance et en présence de plusieurs témoins dont au moins un est externe à l'AC et est impartial. Les témoins attestent, de façon objective et factuelle, du déroulement de la cérémonie par rapport au script préalablement défini.

**Niveau (\*\*)**
Les cérémonies de clés doivent se dérouler sous le contrôle d'au moins deux personnes ayant des rôles de confiance et en présence de plusieurs témoins dont au moins un est externe à l'AC et est impartial.

**Niveau (\*)**
Les cérémonies de clés doivent se dérouler sous le contrôle d'au moins une personne ayant au moins un rôle de confiance et en présence de plusieurs témoins.

##### VI.1.1.2. Clés du service applicatif générées par l'AC

Si la bi-clé du service applicatif est générée par l'AC :
- La génération doit être effectuée dans un environnement sécurisé
- Les bi-clés doivent être générées soit directement dans le dispositif de protection des éléments secrets du service applicatif, soit dans un module cryptographique conforme puis transférées de manière sécurisée

##### VI.1.1.3. Clés du service applicatif générées au niveau du service applicatif

Si la bi-clé est générée au niveau du service applicatif, cette génération doit être effectuée dans un dispositif répondant aux exigences du chapitre XII. L'AC doit s'en assurer auprès du RC, au minimum au travers d'un engagement contractuel clair et explicite.

#### VI.1.2. Transmission de la clé privée au service applicatif

Si l'AC génère la bi-clé du service applicatif, la clé privée doit être transmise au service applicatif de manière sécurisée, afin d'en assurer la confidentialité et l'intégrité.

**Niveau (\*\*\*)**
Si la vérification de l'identité du RC par l'AE via un face-à-face physique n'a pas eu lieu au moment de l'enregistrement, celle-ci doit être effectuée lors de la remise de la bi-clé générée par l'AC en présence du RC.

**Niveau (\*\*)**
Si la vérification de l'identité du RC par l'AE via un face-à-face physique ou via l'emploi d'un procédé de signature électronique conforme au minimum aux exigences du niveau (\*\*) n'a pas eu lieu au moment de l'enregistrement, celle-ci doit être effectuée lors de la remise de la bi-clé générée par l'AC en présence du RC.

Il est interdit à l'autorité de certification de conserver ou de dupliquer cette clé privée.

#### VI.1.3. Transmission de la clé publique à l'AC

En cas de transmission de la requête de demande de certificat au format PKCS#10 ou équivalent, la clé devra être protégée en intégrité et son origine devra en être authentifiée.

#### VI.1.4. Transmission de la clé publique de l'AC aux utilisateurs de certificats

Les clés publiques de vérification de signature de l'AC doivent être diffusées auprès des utilisateurs de certificats par un moyen qui en assure l'intégrité de bout en bout et qui en authentifie l'origine.

Un certificat racine autosigné ne permet pas de garantir par lui-même que la clé publique correspondante appartient bien à l'AC considérée. Sa diffusion doit s'accompagner de la diffusion, via des sources de confiance, de l'empreinte numérique du certificat et d'une déclaration qu'il s'agit bien d'une clé publique de l'AC.

#### VI.1.5. Tailles des clés

Les clés d'AC et de services applicatifs doivent respecter les exigences de caractéristiques (tailles, algorithmes, etc.) du document [RGS_A4].

#### VI.1.6. Vérification de la génération des paramètres des bi-clés et de leur qualité

L'équipement de génération de bi-clés doit utiliser des paramètres respectant les normes de sécurité propres à l'algorithme correspondant à la bi-clé (cf. [RGS_A4]).

Les paramètres et les algorithmes utilisés doivent être documentés par l'AC.

#### VI.1.7. Objectifs d'usage de la clé

L'utilisation d'une clé privée d'AC et du certificat associé est strictement limitée à la signature de certificats, de LCR / LAR et/ou de réponses OCSP.

L'utilisation de la clé privée du service applicatif et du certificat associé est strictement limitée à la fonction de sécurité concernée (cachet, signature de codes, authentification serveur).

### VI.2. Mesures de sécurité pour la protection des clés privées et pour les modules cryptographiques

#### VI.2.1. Standards et mesures de sécurité pour les modules cryptographiques

##### VI.2.1.1. Modules cryptographiques de l'AC

Les modules cryptographiques utilisés par l'AC pour la génération et la mise en œuvre de ses clés de signature, ainsi que le cas échéant pour la génération des clés des services applicatifs, doivent être des modules cryptographiques répondant au minimum aux exigences du chapitre XI pour le niveau de sécurité considéré.

##### VI.2.1.2. Dispositifs de protection des éléments secrets du service applicatif

Les dispositifs de protection des clés privées des services applicatifs doivent respecter les exigences du chapitre XII pour le niveau de sécurité considéré.

Si l'AC ne fournit pas elle-même ce dispositif au RC, elle doit s'assurer auprès du RC de la conformité du dispositif mis en œuvre, au minimum au travers d'un engagement contractuel clair et explicite.

Lorsque l'AC fournit ce dispositif au RC, elle doit s'assurer que :
- La préparation des dispositifs de protection est contrôlée de façon sécurisée
- Les dispositifs de protection sont stockés et distribués de façon sécurisée
- Les désactivations et réactivations des dispositifs protection sont contrôlées de façon sécurisée

#### VI.2.2. Contrôle de la clé privée par plusieurs personnes

**Niveaux (\*\*) et (\*\*\*)**
Le contrôle des clés privées de signature de l'AC doit être assuré par du personnel de confiance (porteurs de secrets d'IGC) et via un outil mettant en œuvre le partage des secrets (systèmes où n exploitants parmi m doivent s'authentifier, avec n au moins égal à 2).

**Niveau (\*)**
Le contrôle des clés privées de signature de l'AC doit être assuré par du personnel de confiance (porteurs de secrets d'IGC).

#### VI.2.3. Séquestre de la clé privée

Ni les clés privées d'AC, ni les clés privées des services applicatifs ne doivent en aucun cas être séquestrées.

#### VI.2.4. Copie de secours de la clé privée

Les clés privées des services applicatifs ou d'AC peuvent faire l'objet de copie de secours.

Ces copies peuvent être effectuées :
- Soit dans un module cryptographique conforme aux exigences du chapitre XI
- Soit hors d'un module cryptographique mais dans ce cas sous forme chiffrée et avec un mécanisme de contrôle d'intégrité

Le chiffrement correspondant doit offrir un niveau de sécurité équivalent ou supérieur au stockage au sein du module cryptographique, conformément aux règles du document [RGS_B_1].

Les opérations de chiffrement et de déchiffrement doivent être effectuées à l'intérieur du module cryptographique de telle manière que les clés privées d'AC ne soient à aucun moment en clair en dehors du module cryptographique.

#### VI.2.5. Archivage de la clé privée

- Les clés privées de l'AC ne doivent en aucun cas être archivées
- Les clés privées des services applicatifs ne doivent en aucun cas être archivées ni par l'AC ni par aucune des composantes de l'IGC

#### VI.2.6. Transfert de la clé privée vers / depuis le module cryptographique

Si l'AC génère les clés privées des services applicatifs en dehors du dispositif de protection des éléments secrets, le transfert doit se faire conformément aux exigences du chapitre VI.1.1.2.

Pour les clés privées d'AC, tout transfert doit se faire sous forme chiffrée, conformément aux exigences du chapitre VI.2.4.

#### VI.2.7. Stockage de la clé privée dans un module cryptographique

Il est recommandé de stocker les clés privées d'AC dans un module cryptographique répondant au minimum aux exigences du chapitre XI pour le niveau de sécurité considéré.

Dans le cas des copies de secours, le stockage peut être effectué en dehors d'un module cryptographique moyennant le respect des exigences du chapitre VI.2.4.

Quel que soit le moyen utilisé, l'AC doit garantir que les clés privées d'AC ne sont pas compromises pendant leur stockage ou leur transport.

#### VI.2.8. Méthode d'activation de la clé privée

##### VI.2.8.1. Clés privées d'AC

La méthode d'activation des clés privées d'AC dans un module cryptographique doit permettre de répondre aux exigences définies dans le chapitre XI pour le niveau de sécurité considéré.

**Niveaux (\*\*) et (\*\*\*)**
L'activation des clés privées d'AC dans un module cryptographique doit être contrôlée via des données d'activation et doit faire intervenir au moins deux personnes dans des rôles de confiance (par exemple, responsable sécurité et opérateur).

**Niveau (\*)**
L'activation des clés privées d'AC dans un module cryptographique doit être contrôlée via des données d'activation et doit faire intervenir au moins une personne ayant au moins un rôle de confiance (par exemple, responsable sécurité).

##### VI.2.8.2. Clés privées des services applicatifs

La méthode d'activation de la clé privée du service applicatif dépend du dispositif utilisé. L'activation de la clé privée du service applicatif doit au minimum être contrôlée via des données d'activation et doit permettre de répondre aux exigences définies dans le chapitre XII pour le niveau de sécurité considéré.

#### VI.2.9. Méthode de désactivation de la clé privée

##### VI.2.9.1. Clés privées d'AC

La désactivation des clés privées d'AC dans un module cryptographique doit être automatique dès que l'environnement du module évolue : arrêt ou déconnexion du module, déconnexion de l'opérateur, etc.

Une clé privée d'AC peut également être désactivée après une certaine période d'inactivité.

##### VI.2.9.2. Clés privées des services applicatifs

Les conditions de désactivation de la clé privée d'un service applicatif doivent permettre de répondre aux exigences définies dans le chapitre XII pour le niveau de sécurité considéré.

#### VI.2.10. Méthode de destruction des clés privées

##### VI.2.10.1. Clés privées d'AC

La méthode de destruction des clés privées d'AC doit permettre de répondre aux exigences définies dans le chapitre XI pour le niveau de sécurité considéré.

En fin de vie d'une clé privée d'AC, normale ou anticipée (révocation), cette clé doit être systématiquement détruite, ainsi que toute copie et tout élément permettant de la reconstituer.

##### VI.2.10.2. Clés privées des services applicatifs

Si les clés privées des services applicatifs sont générées par l'AC dans un module cryptographique hors du dispositif de protection des éléments secrets, la méthode de destruction de ces clés privées après leur exportation hors du module cryptographique doit permettre de répondre aux exigences définies dans le chapitre XII pour le niveau de sécurité considéré.

#### VI.2.11. Niveau de qualification du module cryptographique et des dispositifs de protection

Les exigences de qualification des produits de sécurité de type module cryptographique et dispositif de protection des éléments secrets ne s'appliquent que lorsque :
- Le PSCE fait l'objet d'une procédure de qualification de son offre de certificats électroniques, et
- Les dispositifs de protection sont délivrés par le PSCE

Ces exigences sont précisées aux chapitres XI et XII.

### VI.3. Autres aspects de la gestion des bi-clés

#### VI.3.1. Archivage des clés publiques

Les clés publiques de l'AC et des services applicatifs sont archivées dans le cadre de l'archivage des certificats correspondants.

#### VI.3.2. Durées de vie des bi-clés et des certificats

Les bi-clés et les certificats des services applicatifs couverts par la présente PC Type doivent avoir une **durée de vie maximale de 3 ans**.

La fin de validité d'un certificat d'AC doit être postérieure à la fin de vie des certificats de services applicatifs qu'elle émet.

À titre d'exemple :
- Un certificat d'AC racine peut avoir une durée de vie de 12 ans
- Un certificat d'AC intermédiaire peut avoir une durée de vie de 6 ans
- Un certificat de service applicatif peut avoir une durée de vie de 3 ans

### VI.4. Données d'activation

#### VI.4.1. Génération et installation des données d'activation

##### VI.4.1.1. Données d'activation correspondant à la clé privée de l'AC

La génération et l'installation des données d'activation d'un module cryptographique de l'IGC doivent se faire lors de la phase d'initialisation et de personnalisation de ce module.

Si les données d'activation ne sont pas choisies et saisies par les responsables de ces données eux-mêmes, elles doivent leur être transmises de manière à en garantir la confidentialité et l'intégrité.

Ces données d'activation ne doivent être connues que par les responsables nommément identifiés dans le cadre des rôles qui leurs sont attribués.

##### VI.4.1.2. Données d'activation correspondant à la clé privée du service applicatif

Si l'AC génère la clé privée du service applicatif, elle a pour obligation de transmettre au RC les données d'activation correspondantes par le biais d'un chemin garantissant la protection en intégrité et en confidentialité des données.

Notamment, la remise de la donnée d'activation doit être séparée dans le temps ou dans l'espace de la remise de la clé privée.

Si les données d'activation sont sous forme de mots de passe, le RC doit être informé de la politique de constitution des mots de passe (par exemple, longueur d'au moins 8 caractères, présence d'au moins un caractère spécial, etc.).

#### VI.4.2. Protection des données d'activation

##### VI.4.2.1. Données d'activation correspondant à la clé privée de l'AC

Les données d'activation qui sont générées par l'AC pour les modules cryptographiques de l'IGC doivent être protégées en intégrité et en confidentialité jusqu'à la remise à leur destinataire.

Ce destinataire a ensuite la responsabilité d'en assurer la confidentialité, l'intégrité et la disponibilité.

##### VI.4.2.2. Données d'activation correspondant aux clés privées des services applicatifs

Si les données d'activation des dispositifs de protection des clés privées des services applicatifs sont générées par l'AC, elles doivent être protégées en intégrité et en confidentialité jusqu'à la remise aux RC.

Si ces données d'activation sont également sauvegardées par l'AC, elles doivent être protégées en intégrité et en confidentialité.

#### VI.4.3. Autres aspects liés aux données d'activation

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

### VI.5. Mesures de sécurité des systèmes informatiques

Les mesures de sécurité relatives aux systèmes informatiques doivent satisfaire aux objectifs de sécurité qui découlent de l'analyse de risque que l'AC peut mener.

Le PSCE doit être en mesure de justifier, par tout moyen, qu'il a pris les mesures nécessaires pour assurer la protection des échanges d'information entre les différentes composantes de l'IGC.

Le moyen privilégié consiste en un audit technique réalisé par un prestataire d'audit de la sécurité des systèmes d'information qualifié.

#### VI.5.1. Exigences de sécurité technique spécifiques aux systèmes informatiques

Un niveau minimal d'assurance de la sécurité offerte sur les systèmes informatiques de l'IGC doit être défini dans la DPC de l'AC. Il doit au moins répondre aux objectifs de sécurité suivants :

- Identification et authentification forte des utilisateurs pour l'accès au système (authentification à deux facteurs, de nature physique et/ou logique)
- Gestion des droits des utilisateurs (permettant de mettre en œuvre la politique de contrôle d'accès définie par l'AC, notamment pour implémenter les principes de moindres privilèges, de contrôles multiples et de séparation des rôles)
- Gestion de sessions d'utilisation (déconnexion après un temps d'inactivité, accès aux fichiers contrôlé par rôle et nom d'utilisateur)
- Protection contre les virus informatiques et toutes formes de logiciels compromettants ou non-autorisés et mises à jour des logiciels
- Gestion des comptes des utilisateurs, notamment la modification et la suppression rapide des droits d'accès
- Protection du réseau contre toute intrusion d'une personne non autorisée
- Protection du réseau afin d'assurer la confidentialité et l'intégrité des données qui y transitent
- Fonctions d'audits (non-répudiation et nature des actions effectuées)
- Éventuellement, gestion des reprises sur erreur

Des dispositifs de surveillance (avec alarme automatique) et des procédures d'audit des paramétrages du système (en particulier des éléments de routage) doivent être mis en place.

#### VI.5.2. Niveau de qualification des systèmes informatiques

**Niveaux (\*\*) et (\*\*\*)**
Lorsque le PSCE souhaite faire qualifier son offre de certificats électroniques, il est recommandé que les systèmes informatiques de l'IGC mettant en œuvre le module cryptographique fassent l'objet d'une qualification conformément à l'[ORDONNANCE], au niveau standard défini par le [RGS] et en respectant les exigences du [CWA 14167-1].

### VI.6. Mesures de sécurité des systèmes durant leur cycle de vie

#### VI.6.1. Mesures de sécurité liées au développement des systèmes

L'implémentation d'un système permettant de mettre en œuvre les composantes de l'IGC doit être documentée et doit respecter dans la mesure du possible des normes de modélisation et d'implémentation.

La configuration du système des composantes de l'IGC ainsi que toute modification et mise à niveau doivent être documentées et contrôlées.

L'AC doit :
- Garantir que les objectifs de sécurité sont définis lors des phases de spécification et de conception
- Utiliser des systèmes et des produits fiables qui sont protégés contre toute modification

#### VI.6.2. Mesures liées à la gestion de la sécurité

Toute évolution significative d'un système d'une composante de l'IGC doit être signalée à l'AC pour validation.

Elle doit être documentée et doit apparaître dans les procédures de fonctionnement interne de la composante concernée et être conforme au schéma de maintenance de l'assurance de conformité, dans le cas de produits évalués.

#### VI.6.3. Niveau d'évaluation sécurité du cycle de vie des systèmes

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

### VI.7. Mesures de sécurité réseau

L'interconnexion vers des réseaux publics doit être protégée par des passerelles de sécurité configurées pour n'accepter que les protocoles nécessaires au fonctionnement de la composante au sein de l'IGC.

L'AC doit garantir que les composants du réseau local (routeurs, par exemple) sont maintenus dans un environnement physiquement sécurisé et que leurs configurations sont périodiquement auditées en vue de vérifier leur conformité avec les exigences spécifiées par l'AC.

Les échanges entre composantes au sein de l'IGC peuvent nécessiter la mise en place de mesures particulières en fonction du niveau de sensibilité des informations (utilisation de réseaux séparés / isolés, mise en œuvre de mécanismes cryptographiques à l'aide de clés d'infrastructure et de contrôle, etc.).

### VI.8. Horodatage / Système de datation

Plusieurs exigences de la présente PC Type nécessitent la datation par les différentes composantes de l'IGC d'évènements liés aux activités de l'IGC.

Pour dater ces évènements, les différentes composantes de l'IGC peuvent recourir :
- Soit à une autorité d'horodatage, interne ou externe à l'IGC, conforme à la politique d'horodatage [RGS_A5]
- Soit en utilisant l'heure système de l'IGC en assurant une synchronisation des horloges des systèmes de l'IGC entre elles, au minimum à la minute près, et par rapport à une source fiable de temps UTC, au minimum à la seconde près

Pour les opérations faites hors ligne (ex : administration d'une AC Racine), cette précision de synchronisation par rapport au temps UTC n'est pas requise. Le système devra toutefois pouvoir ordonner les évènements avec une précision suffisante.

Pour la synchronisation par rapport au temps UTC, il est recommandé de se référer à un système comprenant au moins deux sources indépendantes de temps.

---

## VII. Profils des certificats, OCSP et des LCR

Le document [RGS_A4] liste les règles concernant les profils des certificats, des listes de révocation (LCR) et OCSP.

Elles portent notamment sur :
- Les algorithmes et longueurs des clés cryptographiques
- Limitation exclusive de l'usage du certificat à la fonction de sécurité concernée

---

## VIII. Audit de conformité et autres évaluations

Les audits et les évaluations concernent, d'une part, ceux réalisés en vue de la délivrance d'une attestation de qualification au sens de l'[ORDONNANCE] (schéma de qualification des prestataires de services de confiance conformément au [DécretRGS]) et, d'autre part, ceux que doit réaliser, ou faire réaliser, l'AC afin de s'assurer que l'ensemble de son IGC, ainsi que le cas échéant le ou les MC, est bien conforme à ses engagements affichés dans sa PC et aux pratiques identifiées dans sa DPC.

### VIII.1. Fréquences et / ou circonstances des évaluations

Avant la première mise en service d'une composante de son IGC ou suite à toute modification significative au sein d'une composante, l'AC doit procéder à un contrôle de conformité de cette composante.

L'AC doit également procéder régulièrement à un contrôle de conformité de l'ensemble de son IGC, suivant la fréquence suivante :

| Niveau | Fréquence de contrôle |
|--------|----------------------|
| (\*) | 1 fois tous les 3 ans |
| (\*\*) | 1 fois tous les 2 ans |
| (\*\*\*) | 1 fois par an |

### VIII.2. Identités / qualifications des évaluateurs

Le contrôle d'une composante doit être assigné par l'AC à une équipe d'auditeurs compétents en sécurité des systèmes d'information et dans le domaine d'activité de la composante contrôlée.

### VIII.3. Relations entre évaluateurs et entités évaluées

L'équipe d'audit ne doit pas appartenir à l'entité opérant la composante de l'IGC contrôlée, quelle que soit cette composante, et être dûment autorisée à pratiquer les contrôles visés.

### VIII.4. Sujets couverts par les évaluations

Les contrôles de conformité portent sur une composante de l'IGC (contrôles ponctuels) ou sur l'ensemble de l'architecture de l'IGC (contrôles périodiques) et visent à vérifier le respect des engagements et pratiques définies dans la PC de l'AC et dans la DPC qui y répond ainsi que des éléments qui en découlent (procédures opérationnelles, ressources mises en œuvre, etc.).

Le PSCE doit être en mesure de justifier, par tout moyen, aux auditeurs, qu'il a pris les mesures nécessaires pour assurer la protection des échanges d'information entre les différentes composantes de l'IGC.

Le moyen privilégié consiste en un audit technique réalisé par un prestataire d'audit de la sécurité des systèmes d'information qualifié.

### VIII.5. Actions prises suite aux conclusions des évaluations

À l'issue d'un contrôle de conformité, l'équipe d'audit rend à l'AC un avis parmi les suivants : "réussite", "échec", "à confirmer".

Selon l'avis rendu, les conséquences du contrôle sont les suivantes :

- **En cas d'échec** : et selon l'importance des non-conformités, l'équipe d'audit émet des recommandations à l'AC qui peuvent être la cessation (temporaire ou définitive) d'activité, la révocation du certificat de la composante, la révocation de l'ensemble des certificats émis depuis le dernier contrôle positif, etc. Le choix de la mesure à appliquer est effectué par l'AC et doit respecter ses politiques de sécurité internes.

- **En cas de résultat "à confirmer"** : l'AC remet à la composante un avis précisant sous quel délai les non-conformités doivent être levées. Puis, un contrôle de « confirmation » permettra de vérifier que tous les points critiques ont bien été résolus.

- **En cas de réussite** : l'AC confirme à la composante contrôlée la conformité aux exigences de la PC et la DPC.

### VIII.6. Communication des résultats

Les résultats des audits de conformité doivent être tenus à la disposition de l'organisme de qualification en charge de la qualification de l'AC.

---

## IX. Autres problématiques métiers et légales

### IX.1. Tarifs

#### IX.1.1. Tarifs pour la fourniture ou le renouvellement de certificats

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

#### IX.1.2. Tarifs pour accéder aux certificats

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

#### IX.1.3. Tarifs pour accéder aux informations d'état et de révocation des certificats

L'accès aux LCR et, éventuellement, deltaLCR doit être en accès libre en lecture.

#### IX.1.4. Tarifs pour d'autres services

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

#### IX.1.5. Politique de remboursement

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

### IX.2. Responsabilité financière

Conformément à ses obligations, l'AC doit prendre les dispositions nécessaires pour couvrir, éventuellement financièrement, ses responsabilités liées à ses opérations et/ou activités.

#### IX.2.1. Couverture par les assurances

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

#### IX.2.2. Autres ressources

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

#### IX.2.3. Couverture et garantie concernant les entités utilisatrices

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

### IX.3. Confidentialité des données professionnelles

#### IX.3.1. Périmètre des informations confidentielles

Les informations considérées comme confidentielles sont au moins les suivantes :
- La partie non-publique de la DPC de l'AC
- Les clés privées de l'AC, des composantes et des serveurs
- Les données d'activation associées aux clés privées d'AC et des services applicatifs
- Tous les secrets de l'IGC
- Les journaux d'évènements des composantes de l'IGC
- Les dossiers d'enregistrement des services applicatifs et des RC
- Les causes de révocations, sauf accord explicite du RC

#### IX.3.2. Informations hors du périmètre des informations confidentielles

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

#### IX.3.3. Responsabilités en termes de protection des informations confidentielles

L'AC est tenue d'appliquer des procédures de sécurité pour garantir la confidentialité des informations identifiées au chapitre IX.3.1, en particulier en ce qui concerne l'effacement définitif ou la destruction des supports ayant servi à leur stockage.

De plus, lorsque ces données sont échangées, l'AC doit en garantir l'intégrité.

L'AC est tenue de respecter la législation et la réglementation en vigueur sur le territoire français. En particulier, elle peut devoir mettre à disposition les dossiers d'enregistrement des certificats de services applicatifs à des tiers dans le cadre de procédures légales. Elle doit également donner l'accès à ces informations au RC et au MC.

### IX.4. Protection des données à caractère personnel

#### IX.4.1. Politique de protection des données à caractère personnel

Il est entendu que toute collecte et tout usage de données à caractère personnel par l'AC et l'ensemble de ses composantes sont réalisés dans le strict respect de la législation et de la réglementation en vigueur sur le territoire français, en particulier de la loi [CNIL].

#### IX.4.2. Données à caractère personnel

Les données considérées comme personnelles sont au moins les suivantes :
- Les causes de révocation des certificats des services applicatifs (qui sont considérées comme confidentielles sauf accord explicite du RC)
- Les dossiers d'enregistrement des RC

#### IX.4.3. Données à caractère non personnel

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

#### IX.4.4. Responsabilité en termes de protection des données à caractère personnel

Cf. législation et réglementation en vigueur sur le territoire français.

#### IX.4.5. Notification et consentement d'utilisation des données à caractère personnel

Conformément à la législation et réglementation en vigueur sur le territoire français, les informations personnelles remises par les porteurs à l'AC ne doivent ni être divulguées ni transférées à un tiers sauf dans les cas suivants : consentement préalable du porteur, décision judiciaire ou autre autorisation légale.

#### IX.4.6. Conditions de divulgation de données personnelles aux autorités judiciaires ou administratives

Cf. législation et réglementation en vigueur sur le territoire français.

#### IX.4.7. Autres circonstances de divulgation de données personnelles

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

### IX.5. Droits de propriété intellectuelle

La présente PC Type ne formule pas d'exigence spécifique sur le sujet. Application de la législation et de la réglementation en vigueur sur le territoire français.

### IX.6. Interprétations contractuelles et garanties

Les obligations communes aux composantes de l'IGC sont les suivantes :
- Protéger et garantir l'intégrité et la confidentialité de leurs clés secrètes et/ou privées
- N'utiliser leurs clés cryptographiques qu'aux fins prévues lors de leur émission et avec les outils spécifiés dans les conditions fixées par la PC de l'AC
- Respecter et appliquer la partie de la DPC leur incombant
- Se soumettre aux contrôles de conformité effectués par l'équipe d'audit mandatée par l'AC et l'organisme de qualification
- Respecter les accords ou contrats qui les lient entre elles ou aux RC
- Documenter leurs procédures internes de fonctionnement
- Mettre en œuvre les moyens (techniques et humains) nécessaires à la réalisation des prestations auxquelles elles s'engagent dans des conditions garantissant qualité et sécurité

#### IX.6.1. Autorités de Certification

L'AC a pour obligation de :
- Pouvoir démontrer aux utilisateurs de ses certificats qu'elle a émis un certificat pour un service applicatif donné et que le RC correspondant a accepté le certificat
- Garantir et maintenir la cohérence de sa DPC avec sa PC
- Prendre toutes les mesures raisonnables pour s'assurer que ses RC sont au courant de leurs droits et obligations

L'AC est responsable de la conformité de sa Politique de Certification avec les exigences émises dans la présente PC Type pour le niveau de sécurité considéré.

L'AC assume toute conséquence dommageable résultant du non-respect de sa PC, conforme aux exigences de la présente PC Type, par elle-même ou l'une de ses composantes.

L'AC reconnaît engager sa responsabilité en cas de faute ou de négligence qui aurait pour conséquence la lecture, l'altération ou le détournement des données personnelles des RC à des fins frauduleuses.

L'AC reconnaît avoir à sa charge un devoir général de surveillance quant à la sécurité et l'intégrité des certificats délivrés.

En cas de non-respect ponctuel des obligations décrites dans la présente PC Type, l'Administration se réserve le droit de refuser temporairement ou définitivement les certificats de l'AC conformément à la réglementation en vigueur.

#### IX.6.2. Service d'enregistrement

Cf. les obligations pertinentes du chapitre IX.6.1.

#### IX.6.3. RC

Le RC a le devoir de :
- Communiquer des informations exactes et à jour lors de la demande ou du renouvellement du certificat
- Protéger la clé privée du service applicatif dont il a la responsabilité par des moyens appropriés à son environnement
- Protéger les données d'activation de cette clé privée et, le cas échéant, les mettre en œuvre
- Protéger l'accès à la base de certificats du service applicatif
- Respecter les conditions d'utilisation de la clé privée du service applicatif et du certificat correspondant
- Informer l'AC de toute modification concernant les informations contenues dans le certificat électronique
- Faire, sans délai, une demande de révocation du certificat électronique en cas de compromission ou de suspicion de compromission de la clé privée correspondante (ou de ses données d'activation)

La relation entre le RC et l'AC ou ses composantes est formalisée par un engagement du RC visant à certifier l'exactitude des renseignements et des documents fournis.

Ces informations s'appliquent également aux MC.

#### IX.6.4. Utilisateurs de certificats

Les utilisateurs de la sphère publique utilisant les certificats doivent :
- Vérifier et respecter l'usage pour lequel un certificat a été émis
- Contrôler que le certificat émis par l'AC est référencé au niveau de sécurité et pour le service de confiance requis par l'application
- Pour chaque certificat de la chaîne de certification, du certificat du service applicatif jusqu'à l'AC Racine, vérifier la signature numérique de l'AC émettrice du certificat considéré et contrôler la validité de ce certificat (dates de validité, statut de révocation)
- Vérifier et respecter les obligations des utilisateurs de certificats exprimées dans la présente PC Type

L'AC ne doit pas émettre dans sa propre PC d'obligations supplémentaires, par rapport aux obligations de la présente PC Type, à l'encontre des utilisateurs de la sphère publique.

#### IX.6.5. Autres participants

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

### IX.7. Limite de garantie

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

### IX.8. Limite de responsabilité

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

### IX.9. Indemnités

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

### IX.10. Durée et fin anticipée de validité de la PC

#### IX.10.1. Durée de validité

La PC de l'AC doit rester en application au moins jusqu'à la fin de vie du dernier certificat émis au titre de cette PC.

#### IX.10.2. Fin anticipée de validité

La publication d'une nouvelle version de la présente PC Type peut entraîner, en fonction des évolutions apportées, la nécessité pour l'AC de faire évoluer sa PC correspondante.

En fonction de la nature et de l'importance des évolutions apportées à la PC Type, le délai de mise en conformité sera arrêté conformément aux modalités prévues par la réglementation en vigueur.

De plus, la mise en conformité n'impose pas le renouvellement anticipé des certificats déjà émis, sauf cas exceptionnel lié à la sécurité.

#### IX.10.3. Effets de la fin de validité et clauses restant applicables

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

### IX.11. Notifications individuelles et communications entre les participants

En cas de changement de toute nature intervenant dans la composition de l'IGC, l'AC devra :
- Au plus tard un mois avant le début de l'opération, faire valider ce changement au travers d'une expertise technique, afin d'évaluer les impacts sur le niveau de qualité et de sécurité des fonctions de l'AC et de ses différentes composantes
- Au plus tard un mois après la fin de l'opération, en informer l'organisme de qualification

### IX.12. Amendements à la PC

#### IX.12.1. Procédures d'amendements

L'AC devra contrôler que tout projet de modification de sa PC reste conforme aux exigences de la présente PC Type et des éventuels documents complémentaires du [RGS].

En cas de changement important, il est recommandé à l'AC de faire appel à une expertise technique pour en contrôler l'impact.

#### IX.12.2. Mécanisme et période d'information sur les amendements

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

#### IX.12.3. Circonstances selon lesquelles l'OID doit être changé

L'OID de la PC de l'AC étant inscrit dans les certificats qu'elle émet, toute évolution de cette PC ayant un impact majeur sur les certificats déjà émis (par exemple, augmentation des exigences en matière d'enregistrement des RC, qui ne peuvent donc pas s'appliquer aux certificats déjà émis) doit se traduire par une évolution de l'OID, afin que les utilisateurs puissent clairement distinguer quels certificats correspondent à quelles exigences.

### IX.13. Dispositions concernant la résolution de conflits

L'AC doit mettre en place des politiques et procédures pour le traitement des réclamations et le règlement des litiges émanant des entités pour lesquelles elle fournit des services électroniques de confiance ou d'autres points qui y sont liés.

### IX.14. Juridictions compétentes

La présente PC Type ne formule pas d'exigence spécifique sur le sujet. Application de la législation et de la réglementation en vigueur sur le territoire français.

### IX.15. Conformité aux législations et réglementations

Les textes législatifs et réglementaires applicables à la présente PC Type sont, notamment, ceux indiqués au chapitre X.

### IX.16. Dispositions diverses

#### IX.16.1. Accord global

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

#### IX.16.2. Transfert d'activités

Cf. chapitre V.8.

#### IX.16.3. Conséquences d'une clause non valide

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

#### IX.16.4. Application et renonciation

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

#### IX.16.5. Force majeure

Sont considérés comme cas de force majeure tous ceux habituellement retenus par les tribunaux français, notamment le cas d'un évènement irrésistible, insurmontable et imprévisible.

### IX.17. Autres dispositions

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

---

## X. Annexe 1 : Documents cités en référence

### X.1. Réglementation

| Renvoi | Document |
|--------|----------|
| [CNIL] | Loi n° 78-17 du 6 janvier 1978 relative à l'informatique, aux fichiers et aux libertés, modifiée par la loi n° 2004-801 du 6 août 2004 |
| [ORDONNANCE] | Ordonnance n° 2005-1516 du 8 décembre 2005 relative aux échanges électroniques entre les usagers et les autorités administratives et entre les autorités administratives |
| [DécretRGS] | Décret pris pour l'application des articles 9, 10 et 12 de l'ordonnance n° 2005-1516 du 8 décembre 2005 |
| [LOI-TOUBON] | Loi n°94-665 du 4 août 1994 relative à l'emploi de la langue française |

### X.2. Documents techniques

| Renvoi | Document |
|--------|----------|
| [RGS] | Référentiel Général de Sécurité – Version 2.0 |
| [RGS_A1] | RGS - Fonction de sécurité « Signature » - Version 3.0 |
| [RGS_A4] | RGS - Politiques de Certification Types - Profils de certificats, de LCR et OCSP et algorithmes cryptographiques – Version 3.0 |
| [RGS_B_1] | Règles et recommandations concernant le choix et le dimensionnement des mécanismes cryptographiques, ANSSI, Version 1.20 |
| [GEVC] | Exigences relatives à l'émission et à la gestion des certificats électroniques SSL Extended Validation, CA Browser Forum, 1er octobre 2009, version 1.2 |
| [CWA14167-1] | CWA 14167-1 (2003-06) Security Requirements for Trustworthy Systems Managing Certificates for Electronic Signatures – Part 1 |
| [ETSI_NQCP] | ETSI TS 102 042 V1.3.4 (décembre 2007) Policy Requirements for Certification Authorities issuing public key certificates |
| [ExigencesSitesPerso] | Exigences de sécurité des sites de personnalisation, V1.0 (août 2007) |
| [PROG_ACCRED] | COFRAC - Programme d'accréditation pour la qualification des prestataires de services de confiance – CEPE REF 21 |
| [RFC3647] | IETF - Internet X.509 Public Key Infrastructure - Certificate Policy and Certification Practice Framework - novembre 2003 |
| [RFC2527] | IETF - Internet X.509 Public Key Infrastructure - Certificate Policy and Certification Practice Framework - march 1999 |
| [X.509] | Information Technology – Open Systems Interconnection – The Directory: Public-key and attribute certificate frameworks, Recommendation X.509 |
| [972-1] | DCSSI - Guide Technique pour la confidentialité des informations enregistrées sur les disques durs à recycler ou exporter – N° 972-1/SGDN/DCSSI du 17/07/2003 |

---

## XI. Annexe 2 : Exigences de sécurité du module cryptographique de l'AC

### XI.1. Exigences sur les objectifs de sécurité

Le module cryptographique, utilisé par l'AC pour générer et mettre en œuvre ses clés de signature (pour la génération des certificats électroniques, des LCR / LAR et, éventuellement, des réponses OCSP), ainsi que, le cas échéant, générer les bi-clés des services applicatifs, doit répondre aux exigences de sécurité suivantes :

- Si les bi-clés des services applicatifs sont générées par ce module, garantir que ces générations sont réalisées exclusivement par des utilisateurs autorisés et garantir la robustesse cryptographique des bi-clés générées
- Si les bi-clés des services applicatifs sont générées par ce module, assurer la confidentialité des clés privées et l'intégrité des clés privées et publiques des services applicatifs lorsqu'elles sont sous la responsabilité de l'AC et pendant leur transfert vers le dispositif de protection des éléments secrets du service applicatif et assurer leur destruction sûre après ce transfert
- Assurer la confidentialité et l'intégrité des clés privées de signature de l'AC durant tout leur cycle de vie, et assurer leur destruction sûre en fin de vie
- Être capable d'identifier et d'authentifier ses utilisateurs
- Limiter l'accès à ses services en fonction de l'utilisateur et du rôle qui lui a été assigné
- Être capable de mener une série de tests pour vérifier qu'il fonctionne correctement et entrer dans un état sûr s'il détecte une erreur
- Permettre de créer une signature électronique sécurisée, pour signer les certificats générés par l'AC, qui ne révèle pas les clés privées de l'AC et qui ne peut pas être falsifiée sans la connaissance de ces clés privées
- Créer des enregistrements d'audit pour chaque modification concernant la sécurité
- Si une fonction de sauvegarde et de restauration des clés privées de l'AC est offerte, garantir la confidentialité et l'intégrité des données sauvegardées et réclamer au minimum un double contrôle des opérations de sauvegarde et de restauration

**Niveaux (\*\*) et (\*\*\*)**
Il est recommandé que le module cryptographique de l'AC détecte les tentatives d'altérations physiques et entre dans un état sûr quand une tentative d'altération est détectée.

### XI.2. Exigences sur la qualification

Les exigences suivantes ne sont applicables que lorsque le PSCE souhaite faire qualifier son offre de certificats de service applicatif au(x) niveau(x) de sécurité considéré(s) selon la procédure décrite dans le [DécretRGS].

**Niveau (\*\*\*)**
Le module cryptographique utilisé par l'AC doit être qualifié au **niveau renforcé**, selon le processus décrit dans le [RGS], et être conforme aux exigences du chapitre XI.1.

**Niveau (\*\*)**
Le module cryptographique utilisé par l'AC doit être qualifié au minimum au **niveau standard**, selon le processus décrit dans le [RGS], et être conforme aux exigences du chapitre XI.1.
Il est toutefois recommandé d'utiliser un module cryptographique qualifié au niveau renforcé.

**Niveau (\*)**
Le module cryptographique utilisé par l'AC doit être qualifié au minimum au **niveau élémentaire**, selon le processus décrit dans le [RGS], et être conforme aux exigences du chapitre XI.1.
Il est toutefois recommandé d'utiliser un module cryptographique qualifié au niveau standard.

---

## XII. Annexe 3 : Exigences de sécurité du dispositif de protection

Le dispositif de protection des éléments secrets, utilisé par le service applicatif pour stocker et mettre en œuvre sa clé privée et, le cas échéant, générer sa bi-clé, doit répondre aux exigences de sécurité suivantes :

- Si la bi-clé du service applicatif est générée par le dispositif, garantir que cette génération est réalisée exclusivement par des utilisateurs autorisés et garantir la robustesse cryptographique de la bi-clé générée
- Assurer la correspondance entre la clé privée et la clé publique
- Générer un cachet ou une authentification qui ne peut être falsifiée sans la connaissance de la clé privée

Par ailleurs, des mesures de sécurité organisationnelles, procédurales ou techniques doivent être mises en place afin de :
- Détecter les défauts lors des phases d'initialisation, et d'opération et disposer de techniques sûres de destruction de la clé privée en cas de re-génération de la clé privée
- Garantir la confidentialité et l'intégrité de la clé privée
- Permettre de garantir l'authenticité et l'intégrité de la clé publique lors de son export hors du dispositif

### Cachet

- Assurer pour le serveur légitime uniquement la fonction de génération des cachets électroniques et protéger la clé privée contre toute utilisation par des tiers

### Authentification Serveur

- Assurer pour le serveur légitime uniquement, d'une part, la fonction d'authentification et, d'autre part, la fonction de déchiffrement de clés symétriques de session, et protéger la clé privée contre toute utilisation par des tiers
- Permettre de garantir l'authenticité et l'intégrité de la clé symétrique de session, une fois déchiffrée, lors de son export hors du dispositif à destination de l'application de déchiffrement des données

**Nota** : Les dispositifs matériels, de types cartes à puces ou modules cryptographiques qualifiés par l'ANSSI, respectent ces exigences. Toutefois, des solutions logicielles sont susceptibles de respecter ces exigences pourvues que des mesures de sécurité additionnelles propres à l'environnement dans lequel est déployé la clé privée soient mises en place. Cet environnement dans lequel est déployée la clé privée doit faire l'objet d'un audit de sécurité.

---

## Synthèse des exigences par niveau de sécurité

| Exigence | Niveau (\*) | Niveau (\*\*) | Niveau (\*\*\*) |
|----------|------------|--------------|----------------|
| **Vérification identité RC** | Tout moyen approprié | Face-à-face ou signature niveau (\*\*) | Face-à-face obligatoire |
| **Délai révocation** | 72h | 24h | 24h |
| **Fréquence LCR** | 7 jours | 24h | 24h |
| **Disponibilité service** | 95% | 99% | 99,9% |
| **Contrôle clé AC** | 1 personne | 2 personnes + partage secrets | 2 personnes + partage secrets |
| **Cérémonie clés** | 1 + témoins | 2 + témoins (1 externe) | 2 + témoins (1 externe) |
| **Fréquence audit** | 3 ans | 2 ans | 1 an |
| **Module crypto AC** | Qualifié élémentaire | Qualifié standard | Qualifié renforcé |

---

*Document source : Annexe A3 au RGS v2.0*
*Version : 3.0*
*Date : 27 février 2014*
*Diffusion : PUBLIC*
