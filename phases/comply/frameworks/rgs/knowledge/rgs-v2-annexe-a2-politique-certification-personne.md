# Référentiel Général de Sécurité version 2.0

## Annexe A2

### Politique de Certification Type « certificats électroniques de personne »

**Version 3.0 du 27 février 2014**

---

**Premier ministre**

| Organisation | Rôle |
|--------------|------|
| Agence nationale de la sécurité des systèmes d'information (ANSSI) | Autorité nationale en matière de SSI |
| Secrétariat général pour la modernisation de l'action publique (SGMAP) | Modernisation de l'administration |

---

## Historique des versions

| Date | Version | Évolution du document | Rédacteur |
|------|---------|----------------------|-----------|
| 06/11/2006 | 2.1 | Document constitutif de la Politique de Référencement Intersectorielle de Sécurité – PRISv2.1. | DCSSI / SDAE |
| 12/12/2008 | 2.2 | Document constitutif du Référentiel Général de Sécurité – RGSv0.98, annexe A9. Modifications principales : Réécriture d'exigences conformément à la norme ETSI TS 101456 ; Introduction de la notion de qualification des produits de sécurité et des offres de prestataires de services de certification électronique conformément à l'ordonnance n° 2005-1516. | DCSSI / DGME |
| 11/02/2010 | 2.3 | Document constitutif du Référentiel Général de Sécurité – RGSv1.0, annexe A9. Modifications principales : Suppression de la notion de référencement ; Suppression de l'obligation pour l'AC de réaliser une analyse de risques pour les niveaux * et ** ; Modification des exigences sur les certificats de recette / test ; Modification des variables de temps (cf annexe A13) ; Réécriture des exigences sur l'enregistrement d'une demande de certificat d'authentification serveur ; Réécriture des chapitres III.2.6, III.3.1, VI.2.11, XI.2 et XII.2. | ANSSI / DGME |
| 27/02/2014 | 3.0 | Document constitutif du Référentiel Général de Sécurité – RGSv2.0, annexe A2. Modifications principales : fusion des PC Types v2.3 relatives aux usages de signature électronique, d'authentification de personne, de confidentialité et de double usage authentification et signature électronique ; suppression de l'obligation de mettre en place au minimum un service d'information sur l'état de révocation d'un certificat basé sur des LCR. Le choix est laissé à l'AC qui peut choisir de mettre en place un mécanisme de LCR ou OCSP. La durée de vie des clés d'AC n'est plus bornée à 10 ans. Introduction de la notion de LAR et préconisation sur leur fréquence de publication. Obligation de mise en place de mesures pour assurer la protection des échanges d'information entre les différentes composantes de l'IGC. Précisions sur le rôle de confiance « auditeur ». Suppression du chapitre relatif aux certifications croisées. Introduction d'une recommandation sur la durée de validité des LCR et dLCR. Suppression de l'obligation de publier les LCRs au format LDAP. | ANSSI |

**Contact:**
- Agence nationale de la sécurité des systèmes d'information (SGDSN/ANSSI)
- 51 boulevard de La Tour-Maubourg
- 75700 Paris 07 SP
- Email: rgs@ssi.gouv.fr

---

## Sommaire

- [I. Introduction](#i-introduction)
  - [I.1. Présentation générale](#i1-présentation-générale)
  - [I.2. Identification du document](#i2-identification-du-document)
  - [I.3. Définitions et acronymes](#i3-définitions-et-acronymes)
  - [I.4. Entités intervenant dans l'IGC](#i4-entités-intervenant-dans-ligc)
  - [I.5. Usage des certificats](#i5-usage-des-certificats)
  - [I.6. Gestion de la PC](#i6-gestion-de-la-pc)
- [II. Responsabilités concernant la mise à disposition des informations devant être publiées](#ii-responsabilités-concernant-la-mise-à-disposition-des-informations-devant-être-publiées)
- [III. Identification et authentification](#iii-identification-et-authentification)
- [IV. Exigences opérationnelles sur le cycle de vie des certificats](#iv-exigences-opérationnelles-sur-le-cycle-de-vie-des-certificats)
- [V. Mesures de sécurité non techniques](#v-mesures-de-sécurité-non-techniques)
- [VI. Mesures de sécurité techniques](#vi-mesures-de-sécurité-techniques)
- [VII. Profils des certificats, OCSP et des LCR](#vii-profils-des-certificats-ocsp-et-des-lcr)
- [VIII. Audit de conformité et autres évaluations](#viii-audit-de-conformité-et-autres-évaluations)
- [IX. Autres problématiques métiers et légales](#ix-autres-problématiques-métiers-et-légales)
- [X. Annexe 1 : Documents cités en référence](#x-annexe-1--documents-cités-en-référence)
- [XI. Annexe 2 : Exigences de sécurité du module cryptographique de l'AC](#xi-annexe-2--exigences-de-sécurité-du-module-cryptographique-de-lac)
- [XII. Annexe 3 : Exigences de sécurité du dispositif de protection des éléments secrets](#xii-annexe-3--exigences-de-sécurité-du-dispositif-de-protection-des-éléments-secrets)

---

## I. Introduction

### I.1. Présentation générale

#### I.1.1. Objet du document

Le présent document « Politique de Certification Type, certificats électroniques de personne » (PC Type Personne) fait partie du Référentiel Général de Sécurité [RGS]. Il en constitue l'annexe [RGS_A2].

Ce référentiel technique liste les règles que les prestataires de services de certification électronique (PSCE), délivrant des certificats électroniques à des personnes doivent respecter. Les PSCE délivrant des certificats électroniques à des services applicatifs se reporteront à l'annexe [RGS_A3].

Ce document distingue **trois niveaux de sécurité** aux exigences croissantes : *, ** et ***. Il distingue par ailleurs **quatre usages** de certificats électroniques : signature électronique, authentification, confidentialité, signature électronique + authentification. Enfin, il distingue **trois types de porteur** du certificat électronique délivré : particulier, agent d'une autorité administrative, employé d'une entreprise privée.

Conformément à l'[Ordonnance], il est du ressort de l'autorité administrative (AA) de déterminer le niveau de sécurité ainsi que les fonctions de sécurité qu'elle souhaite mettre en place au sein de son SI. Elle peut, par conséquent, décider de recourir à la fonction de sécurité « Signature », « Authentification », « Confidentialité » ou « Authentification et Signature » basée sur des mécanismes cryptographiques asymétriques nécessitant l'usage de certificats électroniques. Le cas échéant, une fois le niveau de sécurité déterminé parmi *, ** et ***, l'AA doit recourir à des certificats électroniques délivrés par des PSCE conformes à la présente PC Type au dit niveau.

Un PSCE peut demander la qualification de son offre de services selon les modalités précisées dans le [DécretRGS]. Ce label permet d'attester de la conformité de l'offre du PSCE aux exigences du présent document, pour un ou plusieurs niveaux de sécurité, un ou plusieurs usages et types de porteur de certificats électroniques.

> **Note importante** : Dans le cas précis des certificats électroniques à double usage « authentification + signature électronique », seuls les deux premiers niveaux de sécurité (*) et (**) sont disponibles. De tels certificats électroniques ne peuvent donc pas prétendre au niveau (***).

Les exigences, communes à tous les niveaux et particulières à un niveau donné, spécifiées dans la présente PC Type doivent être respectées intégralement par les PSCE moyennant l'exception suivante : dans la présente PC Type, un certain nombre de recommandations sont formulées. Les PSCE sont incités à les respecter également dès maintenant car ces recommandations, qui ne sont pas d'application obligatoire dans la présente version de ce document, devraient le devenir dans une version ultérieure.

Cette PC Type n'est pas une PC à part entière : elle ne peut pas être utilisée telle quelle par un PSCE en tant que PC pour être mentionnée dans ses certificats et sa DPC. Un PSCE souhaitant être qualifié par rapport à un des niveaux de sécurité de la présente PC Type doit en reprendre, dans sa propre PC, l'ensemble des exigences correspondant au niveau visé.

Afin de favoriser l'interopérabilité, dans le cadre de la sécurisation des échanges électroniques entre AA et usagers et entre AA, des règles et recommandations sur les formats de certificats et de listes de révocations, compatibles avec la norme [X.509] sont formulées dans le document [RGS_A4].

Dans le cas particulier des certificats de signature électronique, les exigences du niveau *** couvrent celles de l'article 3 du décret n° 2001-272 relatif à la signature électronique et de l'annexe de l'arrêté du 26 juillet 2004. Sous réserve du respect des autres exigences du décret précité, un procédé de signature électronique employant des certificats électroniques conformes au niveau *** sera présumé fiable.

#### I.1.2. Conventions de rédaction

De manière à mettre en exergue les règles spécifiques à un niveau de sécurité, à un type d'usage ou à un type de porteur, celles-ci seront présentées dans un encadré, le titre du cadre précisant son périmètre d'application (usage du certificat électronique, niveau de sécurité et type de porteur du certificat électronique). La forme est la suivante :

```
[Usage] [Niveau de sécurité] [Type de porteur]
Intitulé de la règle …
```

Les exigences qui ne sont pas encadrées s'appliquent de manière identique aux trois niveaux.

### I.2. Identification du document

La présente PC Type est dénommée "RGS - Politique de Certification Type - certificats électroniques de personne". Elle peut être identifiée par son nom, son numéro de version et sa date de mise à jour.

### I.3. Définitions et acronymes

#### I.3.1. Acronymes

| Acronyme | Signification |
|----------|---------------|
| AA | Autorité Administrative |
| AC | Autorité de Certification |
| AE | Autorité d'Enregistrement |
| AED | Autorité d'Enregistrement Déléguée |
| AH | Autorité d'Horodatage |
| ANSSI | Agence nationale de la sécurité des systèmes d'information |
| CEN | Comité Européen de Normalisation |
| DGME | Direction Générale de la Modernisation de l'État |
| DN | Distinguished Name |
| DPC | Déclaration des Pratiques de Certification |
| ETSI | European Telecommunications Standards Institute |
| IGC | Infrastructure de Gestion de Clés |
| LAR | Liste des certificats d'AC Révoqués |
| LCR | Liste des Certificats Révoqués |
| MC | Mandataire de Certification |
| OC | Opérateur de Certification |
| OCSP | Online Certificate Status Protocol |
| OID | Object Identifier |
| PC | Politique de Certification |
| PP | Profil de Protection |
| PSCE | Prestataire de Services de Certification Électronique |
| RSA | Rivest Shamir Adelman |
| SP | Service de Publication |
| SSI | Sécurité des Systèmes d'Information |
| URL | Uniform Resource Locator |

#### I.3.2. Définitions

**Agent** - Personne physique agissant pour le compte d'une autorité administrative.

**Applications utilisatrices** - Services applicatifs exploitant les certificats émis par l'Autorité de Certification pour des besoins d'authentification, de chiffrement ou de signature du porteur du certificat ou des besoin d'authentification ou de cachet du serveur auquel le certificat est rattaché.

**Autorités administratives** - Ce terme générique, défini à l'article 1 de l'[ORDONNANCE], désigne les administrations de l'Etat, les collectivités territoriales, les établissements publics à caractère administratif, les organismes gérant des régimes de protection sociale et les autres organismes chargés de la gestion d'un service public administratif.

**Autorité d'horodatage** - Autorité responsable de la gestion d'un service d'horodatage (cf. politique d'horodatage type du [RGS]).

**Autorité de certification (AC)** - Au sein d'un PSCE, une Autorité de Certification a en charge, au nom et sous la responsabilité de ce PSCE, l'application d'au moins une politique de certification et est identifiée comme telle, en tant qu'émetteur (champ "issuer" du certificat), dans les certificats émis au titre de cette politique de certification.

**Certificat électronique** - Document sous forme électronique attestant du lien entre une clé publique et l'identité de son propriétaire. Cette attestation prend la forme d'une signature électronique réalisée par un prestataire de service de certification électronique (PSCE). Il est délivré par une Autorité de Certification. Le certificat est valide pendant une durée donnée précisée dans celui-ci. Les usages des certificats électroniques régis par le présent document sont la signature électronique, l'authentification, la confidentialité ainsi que le double usage signature électronique + authentification.

**Composante** - Plate-forme opérée par une entité et constituée d'au moins un poste informatique, une application et, le cas échéant, un moyen de cryptologie et jouant un rôle déterminé dans la mise en œuvre opérationnelle d'au moins une fonction de l'IGC.

**Déclaration des pratiques de certification (DPC)** - Une DPC identifie les pratiques (organisation, procédures opérationnelles, moyens techniques et humains) que l'AC applique dans le cadre de la fourniture de ses services de certification électronique aux usagers et en conformité avec la ou les politiques de certification qu'elle s'est engagée à respecter.

**Dispositif de protection des éléments secrets** - Un dispositif de protection des éléments secrets désigne un dispositif de stockage des éléments secrets remis au porteur (exemples : clé privée, code PIN, etc). Il peut prendre la forme d'une carte à puce, d'une clé USB à capacités cryptographique ou se présenter au format logiciel (exemple fichier PKCS#12).

**Entité** - Désigne une autorité administrative ou une entreprise au sens le plus large, c'est-à-dire également les personnes morales de droit privé de type associations.

**Infrastructure de gestion de clés (IGC)** - Ensemble de composantes, fonctions et procédures dédiées à la gestion de clés cryptographiques et de leurs certificats utilisés par des services de confiance. Une IGC peut être composée d'une autorité de certification, d'un opérateur de certification, d'une autorité d'enregistrement centralisée et/ou locale, de mandataires de certification, d'une entité d'archivage, d'une entité de publication, etc.

**Politique de certification (PC)** - Ensemble de règles, identifié par un nom (OID), définissant les exigences auxquelles une AC se conforme dans la mise en place et la fourniture de ses prestations et indiquant l'applicabilité d'un certificat à une communauté particulière et/ou à une classe d'applications avec des exigences de sécurité communes.

**Porteur de certificat** - Personne physique identifiée dans le certificat et qui est le détenteur de la clé privée correspondant à la clé publique.

**Prestataire de services de certification électronique (PSCE)** - L'[ORDONNANCE] introduit et définit les prestataires de service de confiance (PSCO). Un PSCE est un type de PSCO particulier. Un PSCE se définit comme toute personne ou entité qui est responsable de la gestion de certificats électroniques tout au long de leur cycle de vie, vis-à-vis des porteurs et utilisateurs de ces certificats.

**Produit de sécurité** - Un dispositif, logiciel ou matériel, qui met en œuvre des fonctions de sécurité nécessaires à la sécurisation d'une information ou d'un système.

**Promoteur d'application** - Un responsable d'un service de la sphère publique accessible par voie électronique.

**Qualification d'un prestataire de services de certification électronique** - Le [DécretRGS] décrit la procédure de qualification des PSCO. Un PSCE étant un PSCO particulier, la qualification d'un PSCE est un acte par lequel un organisme de certification atteste de la conformité de l'offre de certification électronique d'un PSCE aux exigences de ce document pour un niveau de sécurité et un usage donnés.

**Qualification d'un produit de sécurité** - Acte par lequel l'ANSSI atteste de la capacité d'un produit à assurer, avec un niveau de robustesse donné, les fonctions de sécurité objet de la qualification.

**Système d'information** – Tout ensemble de moyens destinés à élaborer, traiter, stocker ou transmettre des informations faisant l'objet d'échanges par voie électronique entre autorités administratives et usagers ainsi qu'entre autorités administratives.

**Usager** - Personne physique agissant pour son propre compte ou pour le compte d'une personne morale et procédant à des échanges électroniques avec des autorités administratives. Selon le contexte, un usager peut être un porteur ou un utilisateur de certificats.

**Utilisateur de certificat** - Entité ou personne physique qui utilise un certificat et qui s'y fie pour vérifier une signature électronique ou une valeur d'authentification provenant d'un porteur de certificat ou chiffrer des données à destination d'un porteur de certificat.

### I.4. Entités intervenant dans l'IGC

#### I.4.1. Autorités de certification

L'AC a en charge la fourniture des prestations de gestion des certificats tout au long de leur cycle de vie (génération, diffusion, renouvellement, révocation,...) et s'appuie pour cela sur une infrastructure technique : une infrastructure de gestion de clés (IGC).

Les prestations de l'AC sont le résultat de différentes fonctions qui correspondent aux différentes étapes du cycle de vie des bi-clés et des certificats.

La décomposition fonctionnelle d'une IGC qui est retenue dans la présente PC Type est la suivante :

- **Autorité d'enregistrement (AE)** - Cette fonction vérifie et valide les informations d'identification du futur porteur d'un certificat, ainsi qu'éventuellement d'autres attributs spécifiques, avant de transmettre la demande correspondante à la fonction adéquate de l'IGC.

- **Fonction de génération des certificats** - Cette fonction génère (création du format, signature électronique avec la clé privée de l'AC) les certificats à partir des informations transmises par l'autorité d'enregistrement et de la clé publique du porteur.

- **Fonction de génération des éléments secrets du porteur** - Cette fonction génère les éléments secrets à destination du porteur, si l'AC a en charge une telle génération.

- **Fonction de remise au porteur** - Cette fonction remet au porteur au minimum son certificat ainsi que, le cas échéant, les autres éléments fournis par l'AC.

- **Fonction de publication** - Cette fonction met à disposition des différentes parties concernées, les conditions générales, politiques et pratiques publiées par l'AC, les certificats d'AC et toute autre information pertinente.

- **Fonction de gestion des révocations** - Cette fonction traite les demandes de révocation (notamment identification et authentification du demandeur) et détermine les actions à mener.

- **Fonction d'information sur l'état des certificats** - Cette fonction fournit aux utilisateurs de certificats des informations sur l'état des certificats (révoqués, suspendus, etc.). Cette fonction peut être mise en œuvre selon un mode de publication d'informations mises à jour à intervalles réguliers (LCR, LAR) ou selon un mode requête / réponse temps réel (OCSP).

**[Confidentialité]** - Une IGC gérant des certificats de confidentialité doit assurer au surplus les fonctions suivantes :

- **Fonction de gestion des recouvrements** - Cette fonction traite les demandes de recouvrement de clés privées des porteurs.

- **Fonction de séquestre et recouvrement** - Cette fonction fournit la capacité de séquestrer de manière sécurisée les clés privées de confidentialité des porteurs, puis de les recouvrer en cas de besoin.

#### I.4.2. Autorité d'enregistrement

L'AE a pour rôle de vérifier l'identité du futur porteur de certificat. Pour cela, l'AE assure les tâches suivantes :

- l'établissement et la transmission de la demande de certificat à la fonction adéquate de l'IGC suivant l'organisation de cette dernière et les prestations offertes ;
- l'archivage des pièces du dossier d'enregistrement (ou l'envoi vers la composante chargée de l'archivage) ;
- la conservation et la protection en confidentialité et en intégrité des données personnelles d'authentification du porteur ou, le cas échéant, du MC.

Il est à noter que l'AE peut déléguer tout ou partie de ses fonctions à des unités de proximité désignées sous le nom d'autorités d'enregistrement déléguées (AED).

#### I.4.3. Porteurs de certificats

Dans le contexte du présent référentiel, un porteur de certificats ne peut être qu'une personne physique qui utilise sa clé privée et le certificat électronique associé :

- pour son propre compte, dans le cas des particuliers ;
- pour ses activités en lien avec l'entité, identifiée dans le certificat électronique, avec laquelle il a une relation contractuelle, hiérarchique ou réglementaire, dans le cas agents d'AA ou d'employés d'entreprises privées.

#### I.4.4. Utilisateurs de certificats

**[Confidentialité]** - Un utilisateur de certificats électroniques de confidentialité peut être notamment :
- Un service en ligne qui utilise un dispositif de chiffrement pour chiffrer des données ou un message à destination du porteur du certificat ;
- Une personne qui émet un message chiffré à l'intention du porteur du certificat électronique.

**[Authentification]** - Un utilisateur de certificats électroniques d'authentification peut être notamment :
- Un service en ligne qui utilise un certificat et un dispositif de vérification d'authentification soit pour valider une demande d'accès faite par le porteur du certificat dans le cadre d'un contrôle d'accès, soit pour authentifier l'origine d'un message ou de données transmises par le porteur du certificat ;
- Un usager destinataire d'un message ou de données et qui utilise un certificat et un dispositif de vérification d'authentification afin d'en authentifier l'origine.

**[Signature]** - Un utilisateur de certificats de signature électronique peut être notamment :
- Un service en ligne qui utilise un dispositif de vérification de signature pour vérifier la signature électronique apposée sur des données ou un message par le porteur du certificat ;
- Un usager qui signe électroniquement un document ou un message ;
- Un usager destinataire d'un message ou de données et qui utilise un certificat et un dispositif de vérification de signature afin de vérifier la signature électronique apposée par le porteur du certificat sur ce message ou sur ces données.

#### I.4.5. Autres participants

##### I.4.5.1. Composantes de l'IGC

La décomposition en fonctions de l'IGC est présentée au chapitre I.4.1 ci-dessus. Les composantes de l'IGC mettant en œuvre ces fonctions devront être présentées dans la DPC de l'AC.

##### I.4.5.2. Mandataire de certification

**[Entreprise] [Administration]**

Le recours à un mandataire de certification (MC) n'est pas obligatoire pour une entité. Une même entité peut s'appuyer sur un ou plusieurs MC.

Dans le cas où elle y a recours, le MC doit être formellement désigné par un représentant légal de l'entité concernée. Le MC est en relation directe avec l'AE de l'IGC.

Les engagements du MC à l'égard de l'AC doivent être précisés dans un contrat écrit avec l'entité responsable du MC. Ce contrat stipule notamment que le MC doit :
- effectuer correctement et de façon indépendante les contrôles d'identité des futurs porteurs de l'entité pour laquelle il est MC,
- respecter les parties de la PC et de la DPC de l'AC qui lui incombent.

L'entité doit signaler à l'AC, si possible préalablement mais au moins sans délai, le départ du MC de ses fonctions et, éventuellement, lui désigner un successeur.

Le MC ne doit en aucun cas avoir accès aux moyens qui lui permettraient d'activer et d'utiliser la clé privée associée à la clé publique contenue dans le certificat délivré au porteur.

### I.5. Usage des certificats

#### I.5.1. Domaines d'utilisation applicables

##### I.5.1.1. Bi-clés et certificats des porteurs

**[Confidentialité]** - Lorsque le certificat électronique délivré par le PSCE est un certificat de confidentialité, les usages sont :
- Déchiffrement : à l'aide de sa clé privée, un porteur déchiffre les données qui lui ont été transmises dans le cadre d'échanges dématérialisés, chiffrées à partir de sa clé publique ;
- Chiffrement : à l'aide de la publique du destinataire, une personne chiffre des données.

**[Authentification]** - Lorsque le certificat électronique délivré par le PSCE est un certificat d'authentification, les usages sont l'authentification des porteurs auprès de serveurs distants ou auprès d'autres personnes. Il peut s'agir d'authentification dans le cadre d'un contrôle d'accès à un serveur ou une application, ou de l'authentification de l'origine de données dans le cadre de la messagerie électronique.

**[Signature]** - Lorsque le certificat électronique délivré par le PSCE est un certificat de signature électronique, les usages sont la signature électronique de données. Une telle signature électronique apporte, outre l'authenticité et l'intégrité des données ainsi signées, la manifestation du consentement du signataire quant au contenu de ces données.

**[Authentification et Signature]** - Lorsque le certificat électronique délivré par le PSCE est un certificat double usage signature électronique + authentification, les usages sont l'ensemble de ceux identifiés ci-dessus pour les usages séparés d'authentification et de signature.

**Niveaux de sécurité :**

| Niveau | Description |
|--------|-------------|
| *** | Les certificats électronique objets de la présente PC Type sont utilisés par des applications pour lesquelles les besoins de sécurité sont **très forts** eu égard aux risques **très élevés** qui les menacent (usurpation d'identité, …). |
| ** | Les certificats électronique objets de la présente PC Type sont utilisés par des applications pour lesquelles les besoins de sécurité sont **forts** eu égard aux risques **élevés** qui les menacent (usurpation d'identité, …). |
| * | Les certificats électronique objets de la présente PC Type sont utilisés par des applications pour lesquelles les besoins de sécurité sont **moyens** eu égard aux risques qui les menacent. |

##### I.5.1.2. Bi-clés et certificats d'AC et de composantes

Cette PC Type comporte également des exigences concernant les bi-clés et certificats de l'AC (signature des certificats des porteurs, des LCR / LAR ou des réponses OCSP) ainsi que des clés, bi-clés et certificats des composantes de l'IGC.

L'AC génère et signe différents types d'objets : certificats, LCR / LAR ou réponses OCSP. Pour signer ces objets, l'AC dispose d'au moins une bi-clé, mais il est recommandé qu'elle mette en œuvre des bi-clés séparées en particulier pour les réponses OCSP.

**Le cas n°2 est interdit** (AC dispose d'une seule bi-clé et le certificat correspondant est un certificat racine).

La présente PC Type recommande la mise en œuvre du cas n°5 (bi-clés distinctes rattachées à une AC de niveau supérieur), qui permet notamment à l'AC de niveau supérieur de générer et diffuser de manière plus simple des LAR en cas de révocations des certificats d'AC de niveau inférieur.

#### I.5.2. Domaines d'utilisation interdits

Les restrictions d'utilisation des bi-clés et des certificats sont définies au chapitre IV.5 ci-dessous, en fonction du niveau de sécurité. L'AC doit respecter ces restrictions et imposer leur respect par ses porteurs et ses utilisateurs de certificats.

### I.6. Gestion de la PC

#### I.6.1. Entité gérant la PC

La direction de l'AC est responsable de la validation et de la gestion de la PC répondant aux exigences de la présente PC Type.

#### I.6.2. Point de contact

À préciser dans la PC de l'AC.

#### I.6.3. Entité déterminant la conformité d'une DPC avec cette PC

L'AC doit être pourvue d'une direction ayant autorité et une responsabilité finale pour déterminer la conformité de la DPC avec la PC.

#### I.6.4. Procédures d'approbation de la conformité de la DPC

L'AC doit mettre en place un processus d'approbation de la conformité de la DPC avec la PC.

L'AC est responsable de la gestion (mise à jour, révisions) de la DPC. Toute demande de mise à jour de la DPC doit suivre le processus d'approbation mis en place. Toute nouvelle version de la DPC doit être publiée, conformément aux exigences du paragraphe II.2 sans délai.

---

## II. Responsabilités concernant la mise à disposition des informations devant être publiées

### II.1. Entités chargées de la mise à disposition des informations

Pour la mise à disposition des informations devant être publiées à destination des porteurs et des utilisateurs de certificats, l'AC doit mettre en œuvre au sein de son IGC une fonction de publication et une fonction d'information sur l'état des certificats.

La PC de l'AC doit préciser les méthodes de mise à disposition et les URL correspondantes (annuaire accessible par le protocole LDAP et/ou HTTP, serveur Web, serveur OCSP, etc.).

### II.2. Informations devant être publiées

L'AC a pour obligation de publier au minimum les informations suivantes à destination des porteurs et utilisateurs de certificats :

- sa politique de certification, couvrant l'ensemble des rubriques du [RFC3647] et conforme à la présente PC Type, ainsi que les éventuels documents complémentaires ;
- la liste des certificats révoqués (porteurs et AC) ;
- les certificats de l'AC, en cours de validité ;
- si l'AC est rattachée à une hiérarchie d'AC, les certificats en cours de validité des AC de cette hiérarchie, les différentes politiques de certification correspondantes et les éventuels documents complémentaires, ceci jusqu'à l'AC Racine ;
- pour les certificats d'AC autosignés (AC Racine), les informations permettant aux utilisateurs de certificats de s'assurer de l'origine de ces certificats et de leur état.

L'AC a l'obligation de publier, à destination des porteurs et utilisateurs de certificats, sa déclaration des pratiques de certification ainsi que toute autre documentation pertinente pour rendre possible l'évaluation de la conformité avec sa politique de certification.

L'AC a également pour obligation de publier, à destination des porteurs de certificats, les différents formulaires nécessaires pour la gestion des certificats (demande d'enregistrement, demande de révocation, demande de renouvellement, etc.).

De plus, compte tenu de la complexité de lecture d'une PC pour des porteurs ou des utilisateurs de certificats non spécialistes du domaine, il est obligatoire que l'AC publie également des **conditions générales d'utilisation** correspondant aux "PKI Disclosure Statement" (PDS).

Il est recommandé que ces conditions générales reprennent, à destination des porteurs et des utilisateurs de certificats, les informations pertinentes de la PC de l'AC :

- les conditions d'usages des certificats et leurs limites,
- l'identifiant : OID de la PC applicable,
- les obligations et responsabilités des différentes parties,
- les garanties et limites de garanties de l'AC,
- les informations sur comment vérifier un certificat,
- la durée de conservation des dossiers d'enregistrement et des journaux d'évènements,
- les procédures pour la résolution des réclamations et des litiges,
- le système légal applicable,
- si l'AC a été déclarée conforme à la politique identifiée et dans ce cas au travers de quel schéma.

### II.3. Délais et fréquences de publication

Les informations liées à l'IGC (nouvelle version de la PC, formulaires, etc.) doivent être publiées dès que nécessaire afin que soit assurée à tout moment la cohérence entre les informations publiées et les engagements, moyens et procédures effectifs de l'AC.

Les certificats d'AC doivent être diffusés préalablement à toute diffusion de certificats de porteurs et/ou de LCR correspondants et les systèmes les publiant doivent avoir une disponibilité de **24h/24 et 7j/7**.

### II.4. Contrôle d'accès aux informations publiées

L'ensemble des informations publiées à destination des utilisateurs de certificats doit être libre d'accès en lecture.

**Niveau (***)**

L'accès en modification aux systèmes de publication (ajout, suppression, modification des informations publiées) doit être strictement limité aux fonctions internes habilitées de l'IGC, au travers d'un **contrôle d'accès fort** (basé sur une authentification au moins à deux facteurs).

**Niveau (**)**

L'accès en modification aux systèmes de publication des informations d'état des certificats (ajout, suppression, modification des informations publiées) doit être strictement limité aux fonctions internes habilitées de l'IGC, au travers d'un **contrôle d'accès fort** (basé sur une authentification au moins à deux facteurs).

L'accès en modification aux systèmes de publication des autres informations doit être strictement limité aux fonctions internes habilitées de l'IGC, au moins au travers d'un **contrôle d'accès de type mots de passe** basé sur une politique de gestion stricte des mots de passe.

**Niveau (*)**

L'accès en modification aux systèmes de publication (ajout, suppression, modification des informations publiées) doit être strictement limité aux fonctions internes habilitées de l'IGC, au moins au travers d'un **contrôle d'accès de type mots de passe** basé sur une politique de gestion stricte des mots de passe.

---

## III. Identification et authentification

### III.1. Nommage

#### III.1.1. Types de noms

Les noms utilisés doivent être conformes aux spécifications de la norme [X.500].

Dans chaque certificat conforme à la norme [X.509], l'AC émettrice (issuer) et le porteur (subject) sont identifiés par un "Distinguished Name" (DN) répondant aux exigences de la norme [X.501].

Des règles sur la construction du DN de ces champs sont précisées dans le document [RGS_A_4].

#### III.1.2. Nécessité d'utilisation de noms explicites

Les noms choisis pour désigner les porteurs de certificats doivent être explicites.

Lorsqu'un pseudonyme est utilisé, il doit être explicitement identifié comme tel dans le DN.

#### III.1.3. Pseudonymisation des porteurs

L'AC doit pouvoir à tout moment être en mesure de fournir, moyennant le respect de ses obligations en matière de protection des données personnelles, l'identité réelle du porteur en conservant les caractéristiques et références des documents présentés par le porteur pour justifier de son identité.

L'identifiant d'un porteur dans son certificat peut être un pseudonyme à condition d'être identifié comme tel.

#### III.1.4. Règles d'interprétation des différentes formes de nom

Le document [RGS_A4] fournit des règles à ce sujet. Le cas échéant des précisions seront fournies par l'AC dans sa PC.

#### III.1.5. Unicité des noms

Le DN du champ "subject" de chaque certificat de porteur doit permettre d'identifier de façon unique le porteur correspondant au sein du domaine de l'AC.

Ce DN doit pour cela respecter les règles correspondantes définies dans le document [RGS_A4], notamment pour le traitement des cas d'homonymie au sein du domaine de l'AC.

Durant toute la durée de vie de l'AC, un DN attribué à un porteur de certificats ne peut être attribué à un autre porteur.

#### III.1.6. Identification, authentification et rôle des marques déposées

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

L'AC est responsable de l'unicité des noms de ses porteurs et de la résolution des litiges portant sur la revendication d'utilisation d'un nom.

### III.2. Validation initiale de l'identité

L'enregistrement d'un porteur peut se faire soit directement auprès de l'AE, soit via un mandataire de certification de l'entité dans le cas des certificats délivrés à des agents d'AA ou des employés d'entreprises. Dans ce dernier cas, le MC doit être préalablement enregistré par l'AE.

La vérification et la validation initiales de l'identité d'une entité, d'une personne physique et éventuellement de son rattachement à une entité, est ainsi réalisée dans les cas suivants :

**[Entreprise] [Administration]**
- Enregistrement d'un porteur sans MC : validation par l'AE de l'identité "personne morale" de l'entité de rattachement du porteur, de l'identité "personne physique" du futur porteur et du rattachement du futur porteur à l'entité.
- Enregistrement d'un MC : validation de l'identité "personne morale" de l'entité pour laquelle le MC interviendra, de l'identité "personne physique" du futur MC et du rattachement du futur MC à l'entité.
- Enregistrement d'un porteur via un MC : validation par le MC de l'identité "personne physique" du futur porteur et de son rattachement à l'entité pour laquelle le MC intervient.

**[Particulier]**
- Enregistrement d'un porteur : validation par l'AE de l'identité "personne physique" du futur porteur.

#### III.2.1. Méthode pour prouver la possession de la clé privée

Lorsque c'est le porteur qui génère sa bi-clé, il doit alors fournir à l'AC, via le MC le cas échéant, une preuve de possession de sa clé privée correspondant à la clé publique contenue dans la demande de certificat.

#### III.2.2. Validation de l'identité d'un organisme

Cf. chapitre III.2.3

#### III.2.3. Validation de l'identité d'un individu

##### III.2.3.1. Enregistrement d'un porteur [Particulier]

**Dossier d'enregistrement :**

Le dossier d'enregistrement, déposé auprès de l'AE, doit au moins comprendre :
- une demande de certificat écrite signée, et datée de moins de 3 mois, par le futur porteur,
- un document officiel d'identité en cours de validité du futur porteur comportant une photographie d'identité (notamment carte nationale d'identité, passeport ou carte de séjour), qui est présenté à l'AE qui en conserve une copie,
- l'adresse postale et / ou l'adresse mail permettant à l'AC de contacter le porteur,
- les conditions générales d'utilisation signées.

**[Signature] Niveau (***)**
- l'engagement relatif à l'utilisation d'un dispositif sécurisé de création de signature conforme aux exigences de l'annexe 3, dans le cas où le PSCE ne le délivre pas.

**Procédure de vérification de l'identité du porteur :**

| Niveau | Exigence |
|--------|----------|
| *** | La vérification de l'identité du porteur par l'AE est réalisée lors d'un **face-à-face physique**. |
| ** | L'authentification du porteur par l'AE est réalisée lors d'un **face-à-face physique** ou sous forme dématérialisée à condition que la demande soit signée par le porteur à l'aide d'un procédé de signature électronique conforme au minimum aux exigences du niveau (**) décrites dans le document [RGS_A1] et que la signature soit vérifiée et valide au moment de l'enregistrement. |
| * | L'authentification du porteur peut notamment se faire : soit par l'envoi du dossier papier à l'AE accompagné d'une photocopie du document d'identité du futur porteur certifiée conforme par lui-même ; soit via une demande d'enregistrement dématérialisée signée électroniquement ; soit par la communication d'un élément propre au futur porteur permettant de l'identifier au sein d'une base de données administrative pré-établie. |

##### III.2.3.2. Enregistrement d'un porteur [Entreprise] / [Administration] sans MC

L'enregistrement du futur porteur représentant une entité nécessite, l'identification de cette entité, l'identification de la personne physique et la preuve du rattachement de la personne physique à l'entité.

**Dossier d'enregistrement :**

Le dossier d'enregistrement, déposé directement auprès de l'AE, doit au moins comprendre :
- un mandat signé, et daté de moins de 3 mois, par un représentant légal de l'entité désignant le futur porteur auquel le certificat doit être délivré. Ce mandat doit être signé pour acceptation par le futur porteur bénéficiaire,
- [Entreprise] toute pièce, valide lors de la demande de certificat (extrait Kbis ou Certificat d'Identification au Répertoire National des Entreprises et de leurs Établissements ou inscription au répertoire des métiers, ...), attestant de l'existence de l'entreprise et portant le numéro SIREN de celle-ci,
- [Administration] une pièce, valide au moment de l'enregistrement, portant délégation ou subdélégation de l'autorité responsable de la structure administrative,
- un document officiel d'identité en cours de validité du futur porteur ou une carte professionnelle délivrée par une autorité administrative,
- l'adresse postale et / ou l'adresse mail permettant à l'AC de contacter le porteur,
- les conditions générales d'utilisation signées.

##### III.2.3.3. Enregistrement d'un Mandataire de Certification

**[Entreprise] [Administration]**

Le dossier d'enregistrement d'un MC doit au moins comprendre :
- un mandat signé, et daté de moins de 3 mois, par un représentant légal de l'entité désignant le MC. Ce mandat doit être signé par le MC pour acceptation,
- un engagement signé, et daté de moins de 3 mois, du MC, auprès de l'AC, à effectuer correctement et de façon indépendante les contrôles des dossiers des demandeurs,
- un engagement signé, et daté de moins de 3 mois, du MC à signaler à l'AE son départ de l'entité,
- [Entreprise] toute pièce attestant de l'existence de l'entreprise et portant le numéro SIREN,
- [Administration] une pièce portant délégation ou subdélégation de l'autorité responsable de la structure administrative,
- un document officiel d'identité en cours de validité du MC ou une carte professionnelle.

##### III.2.3.4. Enregistrement d'un porteur [Entreprise] / [Administration] via un MC

**Dossier d'enregistrement :**

Le dossier d'enregistrement, déposé auprès d'un MC, doit au moins comprendre :
- une demande de certificat, datée de moins de 3 mois, indiquant l'identité du porteur, co-signé par le porteur et le MC,
- un document officiel d'identité en cours de validité du futur porteur ou une carte professionnelle délivrée par une autorité administrative,
- l'adresse postale et / ou l'adresse mail permettant à l'AC de contacter le porteur,
- les conditions générales d'utilisation signées.

### III.3. Identification et validation d'une demande de renouvellement des clés

Le renouvellement de la bi-clé d'un porteur entraîne automatiquement la génération et la fourniture d'un nouveau certificat. De plus, un nouveau certificat ne peut pas être fourni au porteur sans renouvellement de la bi-clé correspondante.

#### III.3.1. Identification et validation pour un renouvellement courant

**Niveau (**) et (***)**

Lors du premier renouvellement, l'AC doit au minimum s'assurer que les informations du dossier d'enregistrement initial sont toujours valides et que le certificat à renouveler existe, et est toujours valide.

**Niveau (*)**

Lors du premier renouvellement, la vérification de l'identité du porteur est optionnelle. Elle est laissée à l'appréciation de l'AC qui engage sa responsabilité quant à la validité des informations contenues dans le certificat renouvelé.

Lors du renouvellement suivant, l'AE, saisie de la demande, identifiera le porteur selon la même procédure que pour l'enregistrement initial ou une procédure offrant un niveau de garantie équivalent.

#### III.3.2. Identification et validation pour un renouvellement après révocation

Suite à la révocation définitive d'un certificat, quelle qu'en soit la cause, la procédure d'identification et de validation de la demande de renouvellement doit être identique à la procédure d'enregistrement initial ou doit être une procédure offrant un niveau de garantie équivalent.

### III.4. Identification et validation d'une demande de révocation

**Niveau (***)**

Si la demande de révocation est faite via un service téléphonique ou via un service en ligne (serveur web), le demandeur doit être formellement authentifié : vérification de l'identité du demandeur et de son autorité par rapport au certificat à révoquer.

Par exemple : série d'au moins 4 ou 5 questions / réponses sur des informations propres au demandeur, dont au moins une réponse ne peut réellement être connue que du demandeur.

**Niveau (**)**

Si la demande de révocation est faite via un service téléphonique ou via un service en ligne (serveur web), le demandeur doit être formellement authentifié : série d'au moins 3 ou 4 questions / réponses sur des informations propres au demandeur.

**Niveau (*)**

Si la demande de révocation est faite via un service téléphonique ou via un service en ligne (serveur web), elle doit faire l'objet d'un minimum d'authentification : vérification d'une ou deux informations de base du demandeur (adresse, n° de téléphone, etc.) et de son autorité par rapport au certificat à révoquer.

Une demande de révocation peut également être faite par courrier ou par télécopie. Elle doit alors être signée par le demandeur et le service de gestion des révocations doit s'assurer de l'identité du demandeur et de son autorité par rapport au certificat à révoquer.

---

## IV. Exigences opérationnelles sur le cycle de vie des certificats

### IV.1. Demande de certificat

#### IV.1.1. Origine d'une demande de certificat

**[Entreprise] / [Administration]**

Un certificat peut être demandé par un représentant légal de l'entité ou un MC dûment mandaté pour cette entité, après consentement préalable du futur porteur ou, si les raisons du service l'exige, pour une administration, après vérification que le futur porteur a été informé de ses responsabilités et les a accepté au sein d'une attestation personnelle de responsabilité.

**[Particulier]**

Un certificat ne peut être demandé que par le futur porteur ou par le représentant d'un incapable majeur ou d'un mineur.

#### IV.1.2. Processus et responsabilités pour l'établissement d'une demande de certificat

Les informations suivantes doivent au moins faire partie de la demande de certificat :
- le nom du porteur à utiliser dans le certificat (nom réel ou pseudonyme) ;
- les données personnelles d'identification du porteur ;
- [Entreprise] / [Administration] les données d'identification de l'entité (sauf si l'enregistrement est effectué par l'intermédiaire d'un MC).

**[Confidentialité]**

Le cas échéant, la demande de certificat doit intégrer les informations concernant la demande de séquestre de la clé privée du porteur correspondant au certificat sur lequel porte la demande et la durée souhaitée de conservation de la clé privée séquestrée.

### IV.2. Traitement d'une demande de certificat

#### IV.2.1. Exécution des processus d'identification et de validation de la demande

Cf. chapitre III.2

#### IV.2.2. Acceptation ou rejet de la demande

Suite à l'enregistrement de la demande, le certificat ne peut être délivré au porteur qu'après acceptation de celle-ci par l'AC. L'AC est libre de définir ses procédures de rejet. Celles-ci seront définies dans sa DPC.

#### IV.2.3. Durée d'établissement du certificat

L'AC indique dans sa DPC le délai entre la réception de la demande complète de création de certificat et la production de ce certificat.

### IV.3. Délivrance du certificat

#### IV.3.1. Actions de l'AC concernant la délivrance du certificat

L'AC ou l'entité mandatée est responsable de la délivrance du certificat auprès du porteur.

#### IV.3.2. Notification par l'AC de la délivrance du certificat au porteur

L'AC notifie au porteur la délivrance de son certificat soit en mettant à disposition ce certificat, soit par l'envoi d'un courrier ou d'un message électronique.

### IV.4. Acceptation du certificat

#### IV.4.1. Démarche d'acceptation du certificat

Après remise du certificat au porteur, celui-ci doit vérifier les informations contenues dans son certificat et indiquer explicitement à l'AE (ou au MC), par tout moyen défini dans le processus, son acceptation ou son refus des termes et conditions applicables à son utilisation du certificat.

En signant le formulaire de demande de certificat et / ou en signant les conditions générales d'utilisation du certificat et, le cas échéant, en activant son certificat au moment du retrait, le porteur accepte le certificat et engage sa responsabilité sur la validité et l'exactitude de tout ou partie des informations contenues dans son certificat.

#### IV.4.2. Publication du certificat

La publication des certificats porteurs n'est pas obligatoire.

Si l'AC publie les certificats de ses porteurs, cette publication doit être soumise à l'accord de chaque porteur dans le respect de la législation en vigueur, et l'AC doit informer le porteur des conditions de publication.

#### IV.4.3. Notification par l'AC aux autres entités de la délivrance du certificat

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

### IV.5. Usages de la bi-clé et du certificat

#### IV.5.1. Utilisation de la clé privée et du certificat par le porteur

Le porteur doit utiliser sa clé privée et le certificat correspondant uniquement aux fins pour lesquelles ils ont été émis et avec les outils spécifiés par l'AC.

Le porteur ne doit utiliser sa clé privée et le certificat correspondant que pour créer des signatures électroniques, s'authentifier ou déchiffrer des données qui lui ont été transmises en fonction de l'usage pour lequel le certificat a été émis.

#### IV.5.2. Utilisation de la clé publique et du certificat par l'utilisateur du certificat

L'utilisateur d'un certificat doit vérifier la validité du certificat, sa non-révocation et respecter le champ d'utilisation du certificat.

### IV.6. Renouvellement d'un certificat

Au sens du [RFC3647], il s'agit de l'émission d'un nouveau certificat pour le porteur sans changement de bi-clé ni d'aucune autre information du certificat. Cette pratique est interdite par la présente PC Type.

### IV.7. Délivrance d'un nouveau certificat suite à changement de la bi-clé

#### IV.7.1. Causes possibles de changement d'une bi-clé

Un changement de bi-clé peut être demandé dans les situations suivantes :
- fin de la période de validité de la bi-clé,
- altération ou destruction de la clé privée et/ou de son support,
- perte du mot de passe ou de la donnée d'activation nécessaire à l'utilisation de la clé privée,
- changement de support du certificat.

#### IV.7.2. Origine d'une demande d'un nouveau certificat

Cf. chapitre IV.1.1 et chapitre III.3.

#### IV.7.3. Procédure de traitement d'une demande d'un nouveau certificat

Cf. chapitre IV.2.

### IV.8. Modification du certificat

Conformément au [RFC3647], la modification d'un certificat correspond à des modifications d'informations sans changement de la clé publique et autres qu'uniquement la modification des dates de validité.

La modification de certificat n'est pas recommandée dans la présente PC Type. Toutefois, si elle est mise en œuvre, elle doit modifier le numéro de série du certificat, révoquer le certificat initial et ne concerner que les certificats d'utilisateurs finaux.

### IV.9. Révocation et suspension des certificats

#### IV.9.1. Causes possibles d'une révocation

##### IV.9.1.1. Certificats de porteurs

Les circonstances suivantes peuvent être à l'origine de la révocation du certificat d'un porteur :

- les informations du porteur figurant dans son certificat ne sont plus en conformité avec l'identité ou l'utilisation prévue dans le certificat ;
- le porteur n'a pas respecté les modalités applicables d'utilisation du certificat ;
- le porteur et/ou, le cas échéant, le MC / l'entité n'ont pas respecté leurs obligations découlant de la PC de l'AC ;
- une erreur (intentionnelle ou non) a été détectée dans le dossier d'enregistrement du porteur ;
- la clé privée du porteur est suspectée de compromission, est compromise, est perdue ou est volée ;
- le porteur ou une entité autorisée demande la révocation du certificat ;
- le décès du porteur ou la cessation d'activité de l'entité du porteur.

##### IV.9.1.2. Certificats d'une composante de l'IGC

Les circonstances suivantes peuvent être à l'origine de la révocation d'un certificat d'une composante de l'IGC (y compris un certificat d'AC pour la génération de certificats, de LCR et/ou de réponses OCSP) :

- la composante de l'IGC n'a pas respecté les modalités applicables d'utilisation du certificat ;
- une erreur (intentionnelle ou non) a été détectée dans le certificat ;
- la clé privée de la composante de l'IGC est suspectée de compromission, est compromise, est perdue ou est volée ;
- la cessation d'activité de la composante de l'IGC.

#### IV.9.2. Origine d'une demande de révocation

Les acteurs suivants peuvent faire une demande de révocation du certificat du porteur :
- le porteur ;
- l'AC ou l'une de ses composantes ;
- le MC, le cas échéant ;
- une personne autorisée de l'entité à laquelle appartient le porteur, le cas échéant.

#### IV.9.3. Procédure de traitement d'une demande de révocation

L'AC doit vérifier l'identité du demandeur (cf. chapitre III.4) avant de procéder à la révocation.

#### IV.9.4. Délai accordé au porteur pour formuler la demande de révocation

Le porteur doit informer l'AC de la nécessité de révoquer son certificat sans délai.

#### IV.9.5. Délai de traitement par l'AC d'une demande de révocation

| Niveau | Délai de traitement |
|--------|---------------------|
| *** | Le service de révocation est accessible 24h/24 et 7j/7. Le traitement de la demande de révocation et la publication de la révocation s'effectuent dans un délai maximal de **24h**. |
| ** | Le service de révocation est accessible 24h/24 et 7j/7. Le traitement de la demande de révocation et la publication de la révocation s'effectuent dans un délai maximal de **24h**. |
| * | Le service de révocation est accessible pendant les jours et heures ouvrés. Le traitement de la demande de révocation et la publication de la révocation s'effectuent dans un délai maximal de **3 jours ouvrés**. |

#### IV.9.6. Exigences de vérification de la révocation par les utilisateurs de certificats

L'AC doit imposer aux utilisateurs de certificats de vérifier l'état du certificat avant son utilisation.

#### IV.9.7. Fréquence d'établissement et durée de validité des LCR

| Niveau | Fréquence de publication | Durée de validité recommandée |
|--------|-------------------------|-------------------------------|
| *** | Une nouvelle LCR doit être publiée au moins **toutes les 24 heures** ou sans délai en cas de révocation. | Fréquence de publication + 2h |
| ** | Une nouvelle LCR doit être publiée au moins **toutes les 24 heures** ou sans délai en cas de révocation. | Fréquence de publication + 2h |
| * | Une nouvelle LCR doit être publiée au moins **tous les 5 jours ouvrés** ou sans délai en cas de révocation. | Fréquence de publication + 24h |

#### IV.9.8. Délai maximum de publication d'une LCR

Les LCR et LAR sont publiées dans l'heure qui suit leur génération.

### IV.10. Fonction d'information sur l'état des certificats

#### IV.10.1. Caractéristiques opérationnelles

La fonction d'information sur l'état des certificats peut être mise en œuvre selon un mode de publication d'informations mises à jour à intervalles réguliers (LCR, LAR) ou selon un mode requête / réponse temps réel (OCSP).

L'AC doit mettre en place au minimum un de ces services, LCR ou OCSP.

#### IV.10.2. Disponibilité de la fonction d'information sur l'état des certificats

| Niveau | Disponibilité |
|--------|---------------|
| *** | **24h/24 et 7j/7** |
| ** | **24h/24 et 7j/7** |
| * | Les jours et heures ouvrés |

### IV.11. Fin de la relation entre le porteur et l'AC

Cf. chapitres IV.7 (renouvellement) et IV.9 (révocation).

### IV.12. Séquestre de clé et recouvrement

**[Confidentialité]**

#### IV.12.1. Politique et pratiques de recouvrement par séquestre des clés

Le séquestre des clés privées de confidentialité des porteurs et la possibilité de les recouvrer sont autorisés.

Le séquestre des clés privées de signature électronique ou d'authentification des porteurs est interdit.

L'AC doit indiquer dans sa PC et sa DPC, d'une part si elle prend en charge le séquestre et le recouvrement des clés privées des porteurs, et d'autre part, si c'est le cas, les conditions et les modalités.

---

## V. Mesures de sécurité non techniques

Les exigences définies dans la suite du présent chapitre sont les exigences minimales que l'AC doit respecter. Elles doivent être complétées et déclinées en mesures de sécurité en fonction de l'environnement réel de l'IGC.

### V.1. Mesures de sécurité physique

#### V.1.1. Situation géographique et construction des sites

La présente PC Type ne formule pas d'exigence spécifique concernant la localisation géographique de l'IGC et de ses composantes.

La construction des sites doit respecter les règlements et normes en vigueur ainsi qu'éventuellement des exigences spécifiques face à des risques de type tremblement de terre ou explosion.

#### V.1.2. Accès physique

Afin d'éviter toute perte, dommage et compromission des ressources de l'IGC et l'interruption des services de l'AC, les accès aux locaux des différentes composantes de l'IGC doivent être contrôlés.

En outre, toute personne entrant dans ces zones physiquement sécurisées ne doit pas être laissée, pendant une période de temps significative, sans la surveillance d'une personne autorisée.

**Niveau (***)**

Pour les fonctions de génération des certificats, de génération des éléments secrets du porteur et de gestion des révocations et, le cas échéant, pour les fonctions de gestion des recouvrements et de séquestre et recouvrement :

L'accès doit être strictement limité aux seules personnes **nominativement autorisées** à pénétrer dans les locaux et la **traçabilité des accès** doit être assurée. En dehors des heures ouvrables, la sécurité doit être renforcée par la mise en œuvre de **moyens de détection d'intrusion** physique et logique.

Afin d'assurer la disponibilité des systèmes, l'accès aux machines doit être limité aux seules personnes autorisées à effectuer des opérations nécessitant l'accès physique aux machines. Pour cela, les composantes concernées de l'IGC doivent définir un **périmètre de sécurité physique** où sont installées ces machines.

**Niveau (**)**

Pour les fonctions de génération des certificats, de génération des éléments secrets du porteur et de gestion des révocations et, le cas échéant, pour les fonctions de gestion des recouvrements et de séquestre et recouvrement :

L'accès doit être strictement limité aux seules personnes autorisées à pénétrer dans les locaux et la traçabilité des accès doit être assurée. En dehors des heures ouvrables, la sécurité doit être renforcée par la mise en œuvre de moyens de détection d'intrusion physique et logique.

### V.2. Mesures de sécurité procédurales

#### V.2.1. Rôles de confiance

Les différents rôles de confiance au sein de l'IGC doivent être clairement définis. Au minimum, les rôles de confiance suivants doivent être identifiés :

- **responsable de la sécurité** : responsable de la mise en œuvre de la politique de sécurité ;
- **responsable de l'enregistrement** : responsable de l'approbation des demandes de certificats ;
- **responsable de la révocation** : responsable du traitement des demandes de révocation ;
- **responsable du système** : responsable de l'administration des systèmes ;
- **responsable des opérateurs** : responsable de la désignation des personnes pouvant accéder au système ;
- **auditeur** : responsable de la surveillance de l'activité de l'IGC.

#### V.2.2. Nombre de personnes requises par tâches

| Niveau | Exigence |
|--------|----------|
| *** | Certaines tâches sensibles (notamment la génération des clés d'AC, l'activation de la clé privée d'AC, la génération des supports cryptographiques contenant les clés des porteurs, etc.) nécessitent au minimum la présence de **deux (2) personnes de confiance**. |
| ** | Certaines tâches sensibles (notamment la génération des clés d'AC, l'activation de la clé privée d'AC) nécessitent au minimum la présence de **deux (2) personnes de confiance**. |
| * | Aucune exigence spécifique. |

#### V.2.3. Identification et authentification pour chaque rôle

L'identification et l'authentification des personnes ayant un rôle de confiance doivent être effectuées avant l'accès aux locaux sécurisés et aux fonctions d'administration.

#### V.2.4. Rôles exigeant une séparation des attributions

Les rôles exigeant une séparation des attributions sont les suivants :

| Niveau | Exigence |
|--------|----------|
| *** | Les rôles suivants doivent être séparés et ne peuvent pas être cumulés : responsable de la sécurité, responsable de l'enregistrement, responsable de la révocation, responsable du système, auditeur. |
| ** | Les rôles suivants doivent être séparés : responsable de la sécurité, responsable du système, auditeur. |
| * | Les rôles de responsable de la sécurité et d'auditeur doivent être séparés. |

### V.3. Mesures de sécurité vis-à-vis du personnel

#### V.3.1. Qualifications, compétences et habilitations requises

Le personnel affecté à des rôles de confiance doit posséder les qualifications et compétences nécessaires à l'exercice de ses fonctions.

#### V.3.2. Procédures de vérification des antécédents

L'AC doit vérifier les antécédents des personnes ayant un rôle de confiance.

#### V.3.3. Exigences en matière de formation initiale

Le personnel affecté à des rôles de confiance doit recevoir une formation initiale adaptée à ses fonctions.

#### V.3.4. Exigences et fréquence en matière de formation continue

Le personnel affecté à des rôles de confiance doit recevoir une formation continue pour maintenir ses compétences.

#### V.3.5. Fréquence et séquence de rotation entre différentes attributions

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

#### V.3.6. Sanctions en cas d'actions non autorisées

L'AC doit prévoir des sanctions en cas d'actions non autorisées de la part de son personnel.

### V.4. Procédures de constitution des données d'audit

#### V.4.1. Type d'évènements à enregistrer

Les évènements suivants doivent être enregistrés dans les journaux d'évènements :

- démarrage et arrêt de l'équipement de l'AC et des applications ;
- tentatives de création, de suppression, de définition de mots de passe ou de modification des droits du système des utilisateurs privilégiés ;
- modifications à la configuration de l'AC ou apportées aux clés ou aux certificats de l'AC ;
- modifications au cycle de vie des certificats (demandes de certificats, révocation, renouvellement, etc.) ;
- opérations sur les clés cryptographiques (génération, sauvegarde, stockage, recouvrement, archivage, révocation, destruction) ;
- toutes les actions effectuées par les opérateurs de l'AC et les administrateurs ;
- toutes les demandes et opérations liées aux révocations.

#### V.4.2. Fréquence de traitement des journaux d'évènements

Les journaux d'évènements doivent être traités régulièrement afin de détecter toute anomalie.

#### V.4.3. Période de conservation des journaux d'évènements

Les journaux d'évènements doivent être conservés pendant au moins **7 années**.

#### V.4.4. Protection des journaux d'évènements

Les journaux d'évènements doivent être protégés en intégrité et en confidentialité.

### V.5. Archivage des données

#### V.5.1. Types de données à archiver

Les données suivantes doivent être archivées :

- les dossiers d'enregistrement des porteurs ;
- les certificats de clés de porteurs et d'AC ;
- les LCR / LAR produites ;
- les réponses OCSP produites ;
- les journaux d'évènements.

#### V.5.2. Période de conservation des archives

**Dossiers d'enregistrement :**

| Niveau | Période de conservation |
|--------|------------------------|
| *** | Au moins **10 ans** après la fin de validité du dernier certificat délivré au porteur. |
| ** | Au moins **7 ans** après la fin de validité du dernier certificat délivré au porteur. |
| * | Au moins **5 ans** après la fin de validité du dernier certificat délivré au porteur. |

**Certificats, LCR et réponses OCSP émis par l'AC :**

Les certificats de clés de porteurs et d'AC, ainsi que les LCR / LAR produites, doivent être archivés pendant au moins **5 années** après leur expiration.

Les réponses OCSP produites doivent être archivées pendant au moins **3 mois** après leur expiration.

**Journaux d'évènements :**

Les journaux d'évènements seront archivés pendant **7 années** après leur génération.

#### V.5.3. Protection des archives

Pendant tout le temps de leur conservation, les archives, et leurs sauvegardes, doivent :
- être protégées en intégrité ;
- être accessibles aux personnes autorisées ;
- pouvoir être relues et exploitées.

#### V.5.7. Procédures de récupération et de vérification des archives

Les archives (papier et électroniques) doivent pouvoir être récupérées dans un délai inférieur à **2 jours ouvrés**.

### V.6. Changement de clé d'AC

L'AC doit documenter les procédures de changement de clé d'AC, notamment la génération de nouvelles bi-clés d'AC, la création des nouveaux certificats d'AC, la publication des nouveaux certificats d'AC et la notification aux porteurs et utilisateurs de certificats.

### V.7. Reprise suite à compromission et sinistre

#### V.7.1. Procédures de remontée et de traitement des incidents et des compromissions

L'AC doit définir et documenter les procédures de remontée et de traitement des incidents et des compromissions.

#### V.7.2. Procédures de reprise en cas de corruption des ressources informatiques

L'AC doit définir et documenter les procédures de reprise en cas de corruption des ressources informatiques (matériels, logiciels et / ou données).

#### V.7.3. Procédures de reprise en cas de compromission de la clé privée d'une composante

L'AC doit définir et documenter les procédures de reprise en cas de compromission de la clé privée d'une composante.

#### V.7.4. Capacités de continuité d'activité suite à un sinistre

L'AC doit définir et documenter les capacités de continuité d'activité suite à un sinistre.

### V.8. Fin de vie de l'IGC

L'AC doit définir et documenter les procédures de fin de vie de l'IGC, notamment :
- la révocation des certificats d'AC ;
- la notification aux porteurs et utilisateurs de certificats ;
- le transfert des archives ;
- la destruction des clés privées d'AC.

---

## VI. Mesures de sécurité techniques

### VI.1. Génération et installation de bi-clés

#### VI.1.1. Génération des bi-clés

**Bi-clés d'AC :**

| Niveau | Exigence |
|--------|----------|
| *** | Les bi-clés d'AC doivent être générées dans un **module cryptographique qualifié** au niveau renforcé. |
| ** | Les bi-clés d'AC doivent être générées dans un **module cryptographique qualifié** au niveau standard. |
| * | Les bi-clés d'AC doivent être générées dans un **module cryptographique qualifié** au niveau élémentaire ou dans un module cryptographique certifié EAL 4+. |

**Bi-clés des porteurs :**

| Niveau | Exigence |
|--------|----------|
| *** | Les bi-clés des porteurs doivent être générées dans un **dispositif qualifié** ou dans le module cryptographique de l'AC. |
| ** | Les bi-clés des porteurs peuvent être générées par le porteur ou par l'AC. |
| * | Les bi-clés des porteurs peuvent être générées par le porteur ou par l'AC. |

#### VI.1.5. Tailles des clés

Les tailles des clés doivent respecter les recommandations de l'annexe [RGS_B1].

### VI.2. Mesures de sécurité pour la protection des clés privées et pour les modules cryptographiques

#### VI.2.1. Standards et mesures de sécurité pour les modules cryptographiques

Les modules cryptographiques utilisés doivent respecter les exigences de qualification définies dans le [RGS].

#### VI.2.8. Méthode d'activation de la clé privée

| Niveau | Exigence |
|--------|----------|
| *** | La clé privée du porteur doit être protégée par une **donnée d'activation** (code PIN, mot de passe, données biométriques, etc.) dont la mise en œuvre est obligatoire avant chaque utilisation de la clé. |
| ** | La clé privée du porteur doit être protégée par une **donnée d'activation** dont la mise en œuvre est obligatoire avant chaque utilisation de la clé. |
| * | Il est recommandé que la clé privée du porteur soit protégée par une donnée d'activation. |

### VI.3. Autres aspects de la gestion des bi-clés

#### VI.3.2. Durées de vie des bi-clés et des certificats

Les bi-clés et les certificats des porteurs couverts par la présente PC Type doivent avoir une **durée de vie maximale** de :

| Type de porteur | Niveau * | Niveau ** | Niveau *** |
|-----------------|----------|-----------|------------|
| Particulier | 5 ans | 5 ans | 5 ans |
| Agent | 3 ans | 3 ans | 3 ans |
| Entreprise | 3 ans | 3 ans | 3 ans |

Cette durée de vie s'entend à compter de la première utilisation de la clé, ou de la première émission d'un certificat associé à cette clé.

La fin de validité d'un certificat d'AC doit être postérieure à la fin de vie des certificats porteurs qu'elle émet.

### VI.4. Données d'activation

#### VI.4.1. Génération et installation des données d'activation

Les données d'activation doivent être générées de manière aléatoire ou pseudo-aléatoire.

#### VI.4.2. Protection des données d'activation

Les données d'activation doivent être protégées en confidentialité et en intégrité.

### VI.5. Mesures de sécurité des systèmes informatiques

#### VI.5.1. Exigences de sécurité technique spécifiques aux systèmes informatiques

Les systèmes informatiques de l'IGC doivent être protégés contre les accès non autorisés.

### VI.6. Mesures de sécurité des systèmes durant leur cycle de vie

#### VI.6.1. Mesures de sécurité liées au développement des systèmes

Les systèmes informatiques de l'IGC doivent être développés selon des pratiques de développement sécurisé.

#### VI.6.2. Mesures liées à la gestion de la sécurité

Les systèmes informatiques de l'IGC doivent être gérés selon des pratiques de gestion de la sécurité.

### VI.7. Mesures de sécurité réseau

Les systèmes informatiques de l'IGC doivent être protégés par des mesures de sécurité réseau appropriées (pare-feu, segmentation, etc.).

### VI.8. Horodatage / Système de datation

Les évènements enregistrés dans les journaux d'évènements doivent être datés.

| Niveau | Exigence |
|--------|----------|
| *** | L'AC utilise un **système d'horodatage qualifié** ou assure une synchronisation de ses systèmes sur une source de temps fiable. |
| ** | L'AC assure une synchronisation de ses systèmes sur une source de temps fiable. |
| * | L'AC assure une synchronisation de ses systèmes sur une source de temps. |

---

## VII. Profils des certificats, OCSP et des LCR

Les profils de certificats, de LCR et de réponses OCSP doivent être conformes aux exigences de l'annexe [RGS_A4].

---

## VIII. Audit de conformité et autres évaluations

### VIII.1. Fréquences et / ou circonstances des évaluations

L'AC doit effectuer des audits de conformité réguliers.

| Niveau | Fréquence |
|--------|-----------|
| *** | Audit au moins **annuel**. |
| ** | Audit au moins **tous les 2 ans**. |
| * | Audit au moins **tous les 3 ans**. |

### VIII.2. Identités / qualifications des évaluateurs

Les évaluateurs doivent être indépendants de l'AC et posséder les qualifications nécessaires.

### VIII.3. Relations entre évaluateurs et entités évaluées

Les évaluateurs doivent être indépendants de l'AC.

### VIII.4. Sujets couverts par les évaluations

Les audits doivent couvrir l'ensemble des exigences de la présente PC Type.

### VIII.5. Actions prises suite aux conclusions des évaluations

L'AC doit mettre en œuvre les actions correctives identifiées lors des audits.

### VIII.6. Communication des résultats

Les résultats des audits doivent être communiqués à la direction de l'AC.

---

## IX. Autres problématiques métiers et légales

### IX.1. Tarifs

À préciser par l'AC dans sa PC.

### IX.2. Responsabilité financière

#### IX.2.1. Couverture par les assurances

L'AC doit disposer d'une couverture par les assurances adéquate.

### IX.3. Confidentialité des données professionnelles

#### IX.3.1. Périmètre des informations confidentielles

Les informations confidentielles comprennent notamment :
- les clés privées ;
- les données d'activation ;
- les données d'enregistrement ;
- les journaux d'évènements.

### IX.4. Protection des données à caractère personnel

#### IX.4.1. Politique de protection des données à caractère personnel

L'AC doit respecter la législation en vigueur en matière de protection des données à caractère personnel.

### IX.5. Droits de propriété intellectuelle

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

### IX.6. Interprétations contractuelles et garanties

#### IX.6.1. Autorités de Certification

L'AC doit s'engager vis-à-vis des porteurs et utilisateurs de certificats à respecter les exigences de la présente PC Type.

L'AC est responsable de la conformité de sa Politique de Certification avec les exigences émises dans la présente PC Type pour le niveau de sécurité considéré. L'AC assume toute conséquence dommageable résultant du non-respect de sa PC, conforme aux exigences de la présente PC Type, par elle-même ou l'une de ses composantes.

#### IX.6.3. Porteurs de certificats

Le porteur a le devoir de :
- communiquer des informations exactes et à jour lors de la demande ou du renouvellement du certificat ;
- protéger sa clé privée par des moyens appropriés à son environnement ;
- protéger ses données d'activation et, le cas échéant, les mettre en œuvre ;
- protéger l'accès à sa base de certificats ;
- respecter les conditions d'utilisation de sa clé privée et du certificat correspondant ;
- informer l'AC de toute modification concernant les informations contenues dans son certificat ;
- faire, sans délai, une demande de révocation de son certificat auprès de l'AE, du MC de son entreprise ou de l'AC en cas de compromission ou de suspicion de compromission de sa clé privée.

#### IX.6.4. Utilisateurs de certificats

Les utilisateurs de la sphère publique utilisant les certificats doivent :
- vérifier la validité du certificat avant son utilisation ;
- vérifier la non-révocation du certificat ;
- respecter le champ d'utilisation du certificat.

### IX.7. Limite de garantie

À préciser par l'AC dans sa PC.

### IX.8. Limite de responsabilité

À préciser par l'AC dans sa PC.

### IX.10. Durée et fin anticipée de validité de la PC

#### IX.10.1. Durée de validité

La PC est valide tant qu'elle n'est pas remplacée par une nouvelle version ou retirée.

#### IX.10.2. Fin anticipée de validité

L'AC peut mettre fin à sa PC de manière anticipée.

### IX.11. Notifications individuelles et communications entre les participants

À préciser par l'AC dans sa PC.

### IX.12. Amendements à la PC

#### IX.12.1. Procédures d'amendements

L'AC doit définir les procédures d'amendements à sa PC.

### IX.13. Dispositions concernant la résolution de conflits

À préciser par l'AC dans sa PC.

### IX.14. Juridictions compétentes

Les tribunaux français sont compétents.

### IX.15. Conformité aux législations et réglementations

L'AC doit respecter la législation et la réglementation en vigueur.

### IX.16. Dispositions diverses

#### IX.16.1. Accord global

La présente PC Type ne formule pas d'exigence spécifique sur le sujet.

#### IX.16.2. Transfert d'activités

L'AC doit définir les procédures de transfert d'activités.

#### IX.16.5. Force majeure

À préciser par l'AC dans sa PC.

---

## X. Annexe 1 : Documents cités en référence

### X.1. Réglementation

- **[Ordonnance]** - Ordonnance n° 2005-1516 du 8 décembre 2005 relative aux échanges électroniques entre les usagers et les autorités administratives et entre les autorités administratives.
- **[DécretRGS]** - Décret n° 2010-112 du 2 février 2010 pris pour l'application des articles 9, 10 et 12 de l'ordonnance n° 2005-1516 du 8 décembre 2005 relative aux échanges électroniques entre les usagers et les autorités administratives et entre les autorités administratives.

### X.2. Documents techniques

- **[RGS]** - Référentiel Général de Sécurité, version 2.0
- **[RGS_A1]** - Règles relatives à la mise en œuvre des fonctions de sécurité basées sur l'emploi de certificats électroniques
- **[RGS_A3]** - Politique de Certification Type « certificats électroniques de services applicatifs »
- **[RGS_A4]** - Profils de Certificats / LCR / OCSP et Algorithmes Cryptographiques
- **[RGS_B1]** - Mécanismes cryptographiques - Règles et recommandations concernant le choix et le dimensionnement des mécanismes cryptographiques
- **[RFC3647]** - Internet X.509 Public Key Infrastructure - Certificate Policy and Certification Practices Framework
- **[X.500]** - ITU-T Recommendation X.500 - Information technology - Open Systems Interconnection - The Directory: Overview of concepts, models and services
- **[X.501]** - ITU-T Recommendation X.501 - Information technology - Open Systems Interconnection - The Directory: Models
- **[X.509]** - ITU-T Recommendation X.509 - Information technology - Open Systems Interconnection - The Directory: Public-key and attribute certificate frameworks
- **[ETSI_QCP]** - ETSI TS 101 456 - Electronic Signatures and Infrastructures (ESI); Policy requirements for certification authorities issuing qualified certificates
- **[ETSI_NQCP]** - ETSI TS 102 042 - Electronic Signatures and Infrastructures (ESI); Policy requirements for certification authorities issuing public key certificates
- **[CWA14167-1]** - CWA 14167-1 - Security requirements for trustworthy systems managing certificates for electronic signatures - Part 1: System Security Requirements

---

## XI. Annexe 2 : Exigences de sécurité du module cryptographique de l'AC

### XI.1. Exigences sur les objectifs de sécurité

Le module cryptographique de l'AC doit assurer :
- la confidentialité des clés privées de l'AC ;
- l'intégrité des clés de l'AC ;
- l'authentification des opérateurs autorisés à utiliser le module.

### XI.2. Exigences sur la qualification

| Niveau | Exigence de qualification |
|--------|---------------------------|
| *** | Le module cryptographique doit être **qualifié au niveau renforcé** ou certifié selon un profil de protection reconnu au niveau EAL 4+. |
| ** | Le module cryptographique doit être **qualifié au niveau standard** ou certifié selon un profil de protection reconnu au niveau EAL 4+. |
| * | Le module cryptographique doit être **qualifié au niveau élémentaire** ou certifié selon un profil de protection reconnu au niveau EAL 4+. |

---

## XII. Annexe 3 : Exigences de sécurité du dispositif de protection des éléments secrets

### XII.1. Exigences sur les objectifs de sécurité

Le dispositif de protection des éléments secrets doit assurer :
- la confidentialité de la clé privée du porteur ;
- l'intégrité de la clé privée du porteur ;
- l'authentification du porteur avant l'utilisation de la clé privée.

### XII.2. Exigences sur la qualification

**[Signature] Niveau (***)**

Le dispositif sécurisé de création de signature doit être **qualifié** au sens du décret n° 2001-272 relatif à la signature électronique.

**Niveaux (*) et (**) et autres usages**

Il est recommandé que le dispositif de protection des éléments secrets soit qualifié ou certifié selon un profil de protection reconnu.

---

*Document source : ANSSI - Référentiel Général de Sécurité v2.0 - Annexe A2*
*Conversion en Markdown : janvier 2026*
