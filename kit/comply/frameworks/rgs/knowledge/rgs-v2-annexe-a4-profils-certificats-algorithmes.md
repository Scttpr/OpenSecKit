# Référentiel Général de Sécurité version 2.0

## Annexe A4 - Profils de Certificats / LCR / OCSP et Algorithmes Cryptographiques

**Version 3.0 du 27 février 2014**

---

## Historique des versions

| Date | Version | Évolution du document | Rédacteur |
|------|---------|----------------------|-----------|
| 06/11/2006 | 2.1 | Document constitutif de la Politique de Référencement Intersectorielle de Sécurité – PRISv2.1 | DCSSI / SDAE |
| 12/12/2008 | 2.2 | Document constitutif du RGSv0.98, annexe A14. Ajout extension pour certificat de signature qualifiée ; Modification extension pour certificat authentification serveur ; Modification du champ reasonCode des LCR ; Précisions sur le codage des DN et des ICD ; Réécriture du chapitre 5 | DCSSI / DGME |
| 11/02/2010 | 2.3 | Document constitutif du RGSv1.0, annexe A14. Précisions sur les champs keyUsage et ExtendedKeyUsage ; Précisions sur le champ nextUpdate ; Réécriture du chapitre 5 | ANSSI / DGME |
| 27/02/2014 | 3.0 | Document constitutif du RGSv2.0, annexe A4. Précisions sur le champ basicConstraints ; Précisions sur le champ authorityInformationAccess ; Précisions sur le champ crlDistributionPoint ; Introduction d'une recommandation sur la génération des numéros de série | ANSSI |

---

## Sommaire

- [I. Introduction](#i-introduction)
- [II. Certificats](#ii-certificats)
  - [II.1. Certificats d'AC](#ii1-certificats-dac)
  - [II.2. Certificats porteurs](#ii2-certificats-porteurs)
  - [II.3. Certificats de services applicatifs](#ii3-certificats-de-services-applicatifs)
  - [II.4. Certificats de recette](#ii4-certificats-de-recette)
- [III. Liste de Certificats Révoqués](#iii-liste-de-certificats-révoqués)
- [IV. Protocoles d'État en ligne des Certificats](#iv-protocoles-détat-en-ligne-des-certificats)
- [V. Algorithmes et longueurs de clés](#v-algorithmes-et-longueurs-de-clés)
- [VI. Annexe 1 - Documents de référence](#vi-annexe-1---documents-de-référence)
- [VII. Annexe 2 - Exigences sur les identifiants d'AC, de porteurs et de machines](#vii-annexe-2---exigences-sur-les-identifiants-dac-de-porteurs-et-de-machines)

---

## I. Introduction

Les politiques de certification types (PC Types), annexes du [RGS], contiennent des règles sur les formats des certificats, des LCR et des requêtes / réponses OCSP (état en ligne des certificats) ainsi que sur les mécanismes cryptographiques.

Ces règles, communes à toutes les fonctions de sécurité à base de certificats traitées dans les PC Types, ont été factorisées dans le présent document. Celui-ci précise, lorsqu'il y en a, les différences entre les fonctions de sécurité et/ou les niveaux de sécurité.

---

## II. Certificats

### II.1. Certificats d'AC

Ce chapitre porte sur les certificats de clés d'AC utilisées pour la signature de certificats de porteurs ou de services applicatifs, et à la signature de LCR.

#### II.1.1. Champs de base

Un certificat d'AC conforme au [RGS] doit respecter, de base, les exigences correspondantes du [RFC5280], moyennant les compléments et/ou modifications d'exigences définis dans ce tableau.

| Champ | Intitulé de l'exigence |
|-------|------------------------|
| Version | La valeur de ce champ doit être "2", indiquant qu'il s'agit d'un certificat version 3 |
| Serial number | Il doit être généré pour être unique. Il est recommandé que le Serial Number soit non prédictible. Les AC en service lors de la parution de la V2.0 du RGS qui ne respecteraient pas cette exigence doivent le faire lors du renouvellement de leur certificat |
| Signature | Cf. chapitre V sur les exigences en matière d'algorithmes et de longueurs de clés |
| Issuer | Ce champ doit être un DN répondant aux exigences du chapitre VII |
| Validity | Pas d'exigence supplémentaire par rapport au [RFC5280] |
| Subject | Ce champ doit respecter les mêmes exigences que le champ "Issuer" |
| Subject Public Key Info | Cf. chapitre V sur les exigences en matière d'algorithmes et de longueurs de clés |
| Unique Identifiers (issuer et subject) | Les PC Types du [RGS] imposant l'unicité des DN des champs Issuer et Subject au sein du domaine de l'AC, les champs Unique Identifiers ne doivent pas être utilisés |
| Extensions | Cf. chapitre suivant |

#### II.1.2. Extensions

Une extension d'un certificat est caractérisée par :
- **Sa présence obligatoire ou non** dans le certificat (si l'AC émettrice a l'obligation ou non d'intégrer l'extension)
- **Sa criticité** (comment les utilisateurs de certificats doivent traiter l'extension conformément aux principes de [X.509])

| Champ | O | C | Intitulé de l'exigence |
|-------|---|---|------------------------|
| Authority Key Identifier | O | N | Cette extension doit être présente, être marquée "non critique" et contenir l'identifiant de la clé publique de l'AC émettrice |
| Key Usage | O | O | Cette extension doit être marquée "critique". Les bits keyCertSign et cRLSign doivent être à 1. Si l'AC signe des réponses OCSP, le bit digitalSignature doit être à 1 |
| Certificate Policies | O | N | Cette extension doit être conforme aux exigences du chapitre 4.2.1.4 du [RFC5280] |
| Subject Alternative Name / Issuer Alternative Name | N | N | Les champs peuvent être présents mais doivent obligatoirement être marqués "non critique" |
| CRL Distribution Points | N | N | S'il est proposé un service de LCR pour vérifier le statut de révocation du certificat d'AC concerné, alors cette extension doit être présente, marquée "non critique" |
| Authority Information Access | N | N | S'il est proposé un service de validation de certificat en ligne (type serveur OCSP), alors cette extension doit être renseignée |
| Basic Constraints | O | O | Pour les certificats d'AC utilisés pour la signature de certificats de personnes physiques ou de services applicatifs, le champ pathLenConstraint doit être positionné à « 0 » |

---

### II.2. Certificats porteurs

#### II.2.1. Champs de base

Un certificat porteur conforme au [RGS] doit respecter les exigences correspondantes du [RFC5280], du [RFC3739] et de [ETSI_QC] pour les certificats qualifiés.

| Champ | Intitulé de l'exigence |
|-------|------------------------|
| Version | La valeur de ce champ doit être "2", indiquant qu'il s'agit d'un certificat version 3 |
| Serial number | Il doit avoir été généré pour être unique. Il est recommandé que le Serial Number soit non prédictible. Les AC en service lors de la parution de la V2.0 du RGS qui ne respecteraient pas cette exigence doivent le faire sous trois ans |
| Signature | Cf. chapitre V sur les exigences en matière d'algorithmes et de longueurs de clés |
| Issuer | Ce champ est un DN répondant aux exigences du chapitre VII |
| Validity | Pas d'exigence supplémentaire par rapport au [RFC5280] |
| Subject | Ce champ doit être un DN répondant aux exigences du chapitre VII |
| Subject Public Key Info | Cf. chapitre V sur les exigences en matière d'algorithmes et de longueurs de clés |
| Unique Identifiers | Ne doivent pas être utilisés |

#### II.2.2. Extensions

| Champ | O | C | PC Type Signature | PC Type Authentification | PC Type Auth et Signature | PC Type Confidentialité |
|-------|---|---|-------------------|-------------------------|---------------------------|------------------------|
| Authority Key Identifier | O | N | Extension présente, marquée "non critique", contenant l'identifiant de la clé publique de l'AC émettrice ||||
| Key Usage | O | O | bit "nonRepudiation" à "1" | bit "digitalSignature" à "1" | bits "nonRepudiation" et "digitalSignature" à "1" | bit "keyEncipherment" (RSA) ou "keyAgreement" ou "dataEncipherment" à "1" |
| Certificate Policies | O | N | Conforme aux exigences du chapitre 3.2.3 du [RFC3739] ||||
| Subject Alternative Name | N | N | Peut être présent, marqué "non critique" ||||
| CRL Distribution Points | N | N | S'il est proposé un service de CRL, cette extension doit être présente ||||
| Freshest CRL | N | N | Si l'AC utilise des deltaLCR, cette extension doit être présente ||||
| Authority Information Access | N | N | S'il est proposé un service OCSP, cette extension doit être présente ||||
| QCStatements | - | - | Applicable uniquement pour les certificats de signature électronique ||||

**Extension QCStatements** (pour certificats de signature électronique) :
- **esi4-qcStatement-QcCompliance** : indique que le certificat émis est qualifié conformément à la législation en vigueur
- **esi4-qcStatement-QcSSCD** : indique que la clé privée correspondante est stockée dans un dispositif sécurisé de création de signature électronique (SSCD)

---

### II.3. Certificats de services applicatifs

#### II.3.1. Champs de base

Un certificat de services applicatifs (serveur par exemple), conforme aux exigences de la PC Type « authentification serveur » ou de la PC Type « cachet », doit respecter les exigences correspondantes du [RFC5280].

| Champ | Intitulé de l'exigence |
|-------|------------------------|
| Version | La valeur de ce champ doit être "2", indiquant qu'il s'agit d'un certificat version 3 |
| Serial number | Il doit avoir été généré pour être unique. Il est recommandé que le Serial Number soit non prédictible |
| Signature | Cf. chapitre V |
| Issuer | Ce champ est un DN répondant aux exigences du chapitre VII |
| Validity | Pas d'exigence supplémentaire par rapport au [RFC5280] |
| Subject | Ce champ doit être un DN répondant aux exigences du chapitre VII |
| Subject Public Key Info | Cf. chapitre V |
| Unique Identifiers | Ne doivent pas être utilisés |

#### II.3.2. Extensions

| Champ | O | C | Intitulé de l'exigence |
|-------|---|---|------------------------|
| Authority Key Identifier | O | N | Doit être présente, marquée "non critique" |
| Key Usage | O | O | **Pour serveur SSL/TLS ou IPsec** : si chiffrement RSA → bit « keyEncipherment » à 1 ; si Diffie-Hellman éphémère signé → bit « digitalSignature » à 1 ; si Diffie-Hellman avec clé publique fixe → bit « keyAgreement » à 1. **Pour serveur type client** : bits "digitalSignature" ou "keyAgreement" à "1". **Pour certificats cachet** : bit "digitalSignature" (et éventuellement "nonRepudiation") à "1". **Pour signature de code** : bit "digitalSignature" à "1" |
| Certificate Policies | O | N | Conforme aux exigences du chapitre 4.2.1.4 du [RFC5280] |
| Subject Alternative Name | O | N | Pour les certificats serveurs de type SSL/TLS, doit contenir au moins une entrée de type DNS Name correspondant à l'un des FQDN du service applicatif |
| Extended Key Usage | O | N | **Pour certificats cachet (horodatage)** : « id-kp-timeStamping ». **Pour répondeurs OCSP** : « id-kp-OCSPSigning ». **Pour signature de code** : « id-kp-codeSigning ». **Pour serveur SSL/TLS ou IPsec** : « id-kp-serverAuth ». **Pour serveurs « clients »** : « id-kp-clientAuth » |

---

### II.4. Certificats de recette

Les profils de certificat de recette sont identiques à ceux prévus par le présent document, avec les exceptions suivantes :

- **Pour les certificats de personnes** : les attributs commonName (CN), givenName (GN), surname (SN) ou pseudonym, s'ils sont présents, doivent débuter par la chaîne de caractères « TEST », suivie d'une espace, suivie de la valeur du champ prévue par la PC de l'AC

- **Pour les certificats de services applicatifs** : l'attribut commonName (CN) doit être présent sauf dans le cas d'un certificat de serveur SSL/TLS ou d'un certificat de signature de code. Si l'attribut CN est présent, il doit débuter par « TEST », suivi d'une espace, suivi de la valeur prévue. Si l'attribut CN n'est pas présent, l'extension SubjectAlternativeName ou le champ OU doit débuter par « TEST »

Les certificats de recette seront révoqués à la fin de la période de recette.

---

## III. Liste de Certificats Révoqués

### III.1. Champs de base

Une LCR conforme au [RGS] doit respecter les exigences correspondantes du [RFC5280]. Une LAR est une LCR qui ne contient que des certificats d'AC.

| Champ | Intitulé de l'exigence |
|-------|------------------------|
| Version | La valeur de ce champ doit être "1", indiquant qu'il s'agit d'une LCR version 2 |
| Signature | Cf. chapitre V |
| Issuer | Ce champ doit être identique au champ "Subject" du certificat d'AC dont la clé privée est utilisée pour signer la LCR |
| This Update | Pas d'exigence supplémentaire par rapport au [RFC5280] |
| Next Update | Pas d'exigence supplémentaire. Il est recommandé que ce champ soit fonction de la fréquence de publication définie dans le document [RGS_A_13] |
| Revoked Certificates | userCertificate, revocationDate : pas d'exigence supplémentaire ; crlEntryExtensions : cf. chapitre III.3 |

### III.2. Extensions de LCR

| Champ | O | C | Intitulé de l'exigence |
|-------|---|---|------------------------|
| Authority Key Identifier | O | N | Doit être présente, marquée "non critique" |
| Issuer Alternative Name | N | N | Peut être présent, doit être marqué "non critique" |
| CRL Number | O | N | Doit obligatoirement être présente, marquée "non critique". Ce numéro doit être incrémenté de 1 à chaque nouvelle CRL |
| Delta CRL Indicator | O* | O* | S'il s'agit d'une deltaLCR, doit être présente et marquée "critique" (*uniquement pour deltaLCR) |
| Freshest CRL | O** | N | Si l'AC utilise des deltaLCR, doit être présente dans les LCR complètes (**obligatoire si l'AC émet des deltaLCR) |

### III.3. Extensions d'entrée de LCR

Les extensions d'entrées de LCR doivent être conformes aux exigences du [RFC5280].

Pour les LCR qui comportent des numéros de série correspondant à des certificats d'unité d'horodatage, il est obligatoire de supporter l'extension d'entrée LCR : reasonCode.

---

## IV. Protocoles d'État en ligne des Certificats

Il n'y a pas d'exigence spécifique. Le service doit être conforme au [RFC2560].

---

## V. Algorithmes et longueurs de clés

Les règles à respecter concernant le choix et le dimensionnement des algorithmes cryptographiques et des longueurs de clés sont fixées dans le document [RGS_B_1].

Conformément au [RFC3279], l'identifiant utilisé dans le champ "subjectPublicKeyInfo" des certificats permet de spécifier l'algorithme cryptographique correspondant à la clé certifiée (rsaEncryption, id-dsa, id-ecPublicKey, dhpublicnumber), mais ne permet pas de spécifier l'éventuel algorithme de hachage à utiliser en liaison avec cet algorithme cryptographique.

L'information concernant les fonctions de hachage est donc fournie dans ce document à destination des applications et non pas à destination des AC. Si ce n'est pas l'AC qui génère le bi-clé du porteur / serveur, elle doit s'assurer que celle-ci est conforme aux exigences de ce chapitre.

Pour les algorithmes pouvant être utilisés pour divers usages cryptographiques (authentification, signature, confidentialité), l'usage de la clé doit être restreint au travers du champ "keyUsage" du certificat.

---

## VI. Annexe 1 - Documents de référence

### VI.1. Réglementation

| Renvoi | Document |
|--------|----------|
| [ORDONNANCE] | Ordonnance n° 2005-1516 du 8 décembre 2005 relative aux échanges électroniques entre les usagers et les autorités administratives et entre les autorités administratives |
| [Décret2001-272] | Décret n° 2001-272 du 20 mars 2001 pris pour l'application de l'article 1316-4 du code civil et relatif à la signature électronique |
| [DécretRGS] | Décret pris pour l'application des articles 9, 10 et 12 de l'ordonnance n° 2005-1516 |
| [RGS] | Référentiel Général de Sécurité – Version 1 |

### VI.2. Documents techniques

| Renvoi | Document |
|--------|----------|
| [ETSI_CERT] | ETSI - TS 102 280 - X.509 V3 Certificate Profile for Certificates Issued to Natural Persons, V1.1.1 mars 2004 |
| [ETSI_QC] | ETSI - TS 101 862 - Qualified certificate Profile, V1.3.3 janvier 2006 |
| [PKCS#1] | RSA Laboratories - PKCS #1 v2.1 - RSA Cryptography Standard, 14 juin 2002 |
| [RFC2560] | IETF - Internet X.509 Public Key Infrastructure - Online Certificate Status Protocol, RFC 2560 - juin 1999 |
| [RFC3279] | IETF - Algorithms and Identifiers for the Internet X.509 Public Key Infrastructure - Certificate and Certificate Revocation List (CRL) Profil - avril 2002 |
| [RFC3739] | IETF - Internet X.509 Public Key Infrastructure, Qualified Certificates Profile, RFC 3739 - mars 2004 |
| [RFC5280] | IETF - Internet X.509 Public Key Infrastructure, Certificate and CRL Profile, RFC 5280 - mai 2008 |
| [RGS_B_1] | Règles et recommandations concernant le choix et le dimensionnement des mécanismes cryptographiques, ANSSI, Version 1.20 |
| [X.509] | ITU - Information Technology – Open Systems Interconnection – The Directory: Public-key and attribute certificate frameworks, Recommendation X.509, version 08/2005 |

---

## VII. Annexe 2 - Exigences sur les identifiants d'AC, de porteurs et de machines

### VII.1. Identification d'une Autorité de Certification

Le DN qui se trouve dans le champ "Subject" d'un certificat d'AC, dans le champ "Issuer" d'un certificat d'AC ou d'utilisateur final, ainsi que dans le champ "Issuer" d'une LCR, doivent être conformes aux exigences des chapitres 4.1.2.4 de la RFC [5280], 3.1.1 de [RFC3739] et 5.2.4 de [ETSI_CERT].

Ce DN doit être encodé en **printableString** ou en **UTF8String**.

**Attributs requis :**
- **countryName** : doit être présent, indiquer le pays de l'autorité compétente auprès de laquelle l'entité est officiellement enregistrée, renseigné en lettres majuscules
- **organizationName** : doit être présent et contenir le nom officiel complet de l'entité
- **organizationalUnitName** : une instance doit être présente et contenir l'identification de l'entité selon la norme ISO 6523 au format « <ICD> <Identification de l'organisation> »

**Pour les entités de droit français :**
- ICD = 0002
- L'identification est le n° SIREN (9 caractères) ou le n° SIRET (14 caractères)

**Exemple de DN :** `C=FR, O= Société ABC, OU= 0002 243516879, OU= Centre de Paris`

### VII.2. Identification de porteur

#### VII.2.1. Certificats [ENTREPRISE] et [ADMINISTRATION]

Si le certificat n'est pas un certificat pseudonyme, une identification de l'entité à laquelle le porteur est rattaché est obligatoire.

- **countryName** : doit être présent
- **organizationName** : doit être présent et contenir le nom officiel complet de l'entité
- **organizationalUnitName** : une instance doit être présente avec l'identification selon ISO 6523
- **CN** : doit être conforme au § VII.2.2

#### VII.2.2. Certificats [ENTREPRISE], [ADMINISTRATION], [PARTICULIER]

L'attribut **countryName** doit être présent.

**Si certificat pseudonyme :** utiliser l'attribut **pseudonym** (pas givenName, surname, commonName)

**Si certificat non pseudonyme, deux possibilités :**

1. **Utilisation de givenName et surname** :
   - givenName : un des prénoms de l'état civil
   - surname : nom de l'état civil ou nom d'usage
   - Pour les prénoms/noms composés, utiliser le tiret comme séparateur
   - commonName peut être utilisé pour distinguer les homonymes

2. **Seul commonName utilisé** : prénom(s), espace, nom de l'état civil ou nom d'usage

**Exemples de DN :**
- `C=FR, O= Société DEF, OU= 0002 243516879, OU= Site de Toulouse, CN= Michel Martin`
- `C=FR, O= Société DEF, OU= 0002 243516879, OU= Site de Toulouse, GN= Michel + SN = Martin`

### VII.3. Identification d'un service applicatif

Est entendu par « service applicatif » :
- un service de création de cachet
- un service d'authentification de serveur

**Attributs requis :**
- **countryName** : doit être présent (lettres majuscules)
- **organizationName** : doit être présent avec le nom officiel complet de l'entité
- **organizationalUnitName** : une instance avec l'identification selon ISO 6523
- **commonName** : doit contenir un nom significatif du service applicatif, avec exceptions :
  - Pour certificat serveur SSL/TLS : CN facultatif ; s'il est présent, doit contenir un FQDN également présent dans SubjectAlternativeName
  - Pour certificat de signature de code : CN facultatif ; s'il est présent, ne doit pas contenir un FQDN

Les attributs **givenName** et **surname** ne doivent pas être utilisés.

**Exemple de DN :** `C=FR, O= Société ABC, OU= 0002 243516879, OU= Site de Toulouse, CN= www.abc.fr`

**Pour un service de création de cachet**, règle de nommage proposée pour le CN :
`[Nom de l'organisme].[Nom du bureau responsable du serveur].[Nom du service applicatif]`
