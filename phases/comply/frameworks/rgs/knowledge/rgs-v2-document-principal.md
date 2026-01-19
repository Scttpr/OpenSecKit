# Référentiel Général de Sécurité (RGS) - Version 2.0

**Premier ministre**

| Organisation | Rôle |
|--------------|------|
| Agence nationale de la sécurité des systèmes d'information (ANSSI) | Autorité nationale en matière de SSI |
| Secrétariat général pour la modernisation de l'action publique (SGMAP) | Modernisation de l'administration |

---

## Historique des versions

| Date | Version | Évolution du document |
|------|---------|----------------------|
| 06/05/2010 | 1.0 | Publication de la première version du Référentiel général de sécurité |
| 13/06/2014 | 2.0 | Publication de la deuxième version du Référentiel général de sécurité |

**Contact:**
- Agence nationale de la sécurité des systèmes d'information (SGDSN/ANSSI)
- Bureau de la maîtrise des risques et de la réglementation
- 51 boulevard de La Tour-Maubourg, 75700 Paris 07 SP
- Email: rgs [at] ssi.gouv.fr

**Base légale:**
- Décret n° 2010-112 du 2 février 2010
- Ordonnance n° 2005-1516 du 8 décembre 2005 (articles 9, 10 et 12)
- Arrêté du 13 juin 2014

---

## Sommaire

1. [Mise en conformité avec les exigences du « décret RGS »](#chapitre-1-mise-en-conformité-avec-les-exigences-du-décret-rgs)
2. [Description des étapes de la mise en conformité](#chapitre-2-description-des-étapes-de-la-mise-en-conformité)
3. [Règles relatives à la cryptographie et à la protection des échanges électroniques](#chapitre-3-règles-relatives-à-la-cryptographie-et-à-la-protection-des-échanges-électroniques)
4. [Règles relatives aux accusés d'enregistrement et aux accusés de réception](#chapitre-4-règles-relatives-aux-accusés-denregistrement-et-aux-accusés-de-réception)
5. [Qualification des produits de sécurité et des prestataires de services de confiance](#chapitre-5-qualification-des-produits-de-sécurité-et-des-prestataires-de-services-de-confiance)
6. [Validation des certificats par l'État](#chapitre-6-validation-des-certificats-par-létat)
7. [Recommandations relatives à l'application du référentiel](#chapitre-7-recommandations-relatives-à-lapplication-du-référentiel)
8. [Transition entre la première et la deuxième version du RGS](#chapitre-8-transition-entre-la-première-et-la-deuxième-version-du-rgs)
9. [Liste des annexes du RGS](#chapitre-9-liste-des-annexes-du-rgs)
10. [Références documentaires](#chapitre-10-références-documentaires)

---

## Chapitre 1. Mise en conformité avec les exigences du « décret RGS »

Le référentiel général de sécurité (RGS) vise à renforcer la confiance des usagers dans les services électroniques proposés par les autorités administratives, notamment lorsque ceux-ci traitent des données personnelles. Il s'applique aux systèmes d'information mis en œuvre par les autorités administratives dans leurs relations entre elles et avec les usagers. Il peut aussi être considéré comme un recueil de bonnes pratiques pour tous les autres organismes.

### Démarche en cinq étapes (décret n° 2010-112)

1. **Réalisation d'une analyse des risques** (art. 3 al. 1)
2. **Définition des objectifs de sécurité** (art. 3 al. 2)
3. **Choix et mise en œuvre des mesures appropriées** de protection et de défense du SI (art. 3 al. 3)
4. **Homologation de sécurité** du système d'information (art. 5)
5. **Suivi opérationnel** de la sécurité du SI

### Procédure simplifiée (SI existant ou modifié)

1. Réalisation d'un audit de la sécurité du système d'information (interne ou externalisé)
2. Réalisation d'une analyse des risques simplifiée
3. Mise en œuvre des mesures correctives fixées dans le rapport d'audit
4. Décision d'homologation de sécurité du système d'information
5. Suivi opérationnel de la sécurité du SI

### Points d'attention

- **Clauses contractuelles:** Veiller aux clauses relatives à la sécurité des contrats avec les prestataires (audit SSI, traitement d'incident, détection, externalisation, infogérance, cloud, TMA, etc.)
- **Facteur humain:** Sensibilisation du personnel aux questions de sécurité, formation des intervenants en SSI (surveillance, détection, prévention)

---

## Chapitre 2. Description des étapes de la mise en conformité

### 2.1 Analyse des risques

L'analyse de risques précise les besoins de sécurité du système d'information en fonction de la menace et des enjeux.

**Démarche:**
- Identifier les événements pouvant affecter la sécurité du système
- Estimer les conséquences et les impacts potentiels
- Décider des actions pour réduire le risque à un niveau acceptable

**Menaces à considérer:**
- Menaces réelles pesant sur le système
- Menaces sur les informations traitées, transmises et stockées
- Contexte environnemental du système

**Pour les certificats électroniques et horodatage:**
- Déterminer les usages (signature, authentification, confidentialité, etc.)
- Définir les niveaux de sécurité (*, ** ou ***)

**Références méthodologiques:**
- Norme ISO 27005 (cadre théorique de gestion des risques)
- Méthode EBIOS (Expression des besoins et identification des objectifs de sécurité)

### 2.2 Définition des objectifs de sécurité

**Trois domaines traditionnels:**
- **Disponibilité** des données et du système
- **Intégrité** des données et du système
- **Confidentialité** des données et des éléments critiques du système

**Deux domaines complémentaires:**
- **Authentification:** Garantir que la personne identifiée est effectivement celle qu'elle prétend être
- **Traçabilité:** Associer les actions sur les données et les processus aux personnes connectées, déceler les actions illégitimes

Les objectifs doivent être exprimés en termes de **protection** et de **défense** des systèmes d'information.

### 2.3 Choix et mise en œuvre des mesures de sécurité adaptées

**Mesures techniques:**
- Produits de sécurité (matériels ou logiciels)
- Prestations de services de confiance informatiques
- Autres dispositifs de sécurité (blindage, détecteur d'intrusion, etc.)

**Mesures organisationnelles:**
- Organisation des responsabilités (habilitation du personnel, contrôle des accès, protection physique)
- Gestion des ressources humaines (affectation d'agents responsables, formation, sensibilisation)

Les mesures peuvent être sélectionnées dans des référentiels existants, adaptées ou créées ex nihilo.

### 2.4 Homologation de sécurité du système d'information

Les systèmes d'information dans le champ de l'ordonnance du 8 décembre 2005 doivent faire l'objet d'une **décision d'homologation de sécurité** avant leur mise en service opérationnelle.

**Caractéristiques:**
- Également dénommée « attestation formelle » (art. 5, al. 1 du décret RGS)
- Prononcée par une **autorité d'homologation** désignée par l'autorité administrative
- Atteste que le SI est protégé conformément aux objectifs de sécurité
- Confirme que les risques résiduels sont acceptés
- S'appuie sur un **dossier d'homologation**
- Pour les téléservices: décision rendue accessible aux usagers

**Recommandation:** Revue périodique des systèmes homologués.

**Référence:** Guide d'homologation publié par l'ANSSI

### 2.5 Suivi opérationnel de la sécurité du système d'information

Le suivi opérationnel consiste à:
- Collecter et analyser les journaux d'événements et les alarmes
- Mener des audits réguliers
- Appliquer des mesures correctives après un audit ou un incident
- Mettre en œuvre une chaîne d'alerte en cas d'intrusion
- Gérer les droits d'accès des utilisateurs
- Assurer une veille sur les menaces et les vulnérabilités
- Entretenir des plans de continuité et de reprise d'activité
- Sensibiliser le personnel
- Gérer les crises

---

## Chapitre 3. Règles relatives à la cryptographie et à la protection des échanges électroniques

Les règles techniques du RGS portent sur la sécurisation des infrastructures utilisées pour les échanges électroniques:
- Entre autorités administratives et usagers
- Entre autorités administratives

Le RGS n'impose aucune technologie particulière mais fixe des exigences relatives à certaines fonctions (certification, horodatage, audit).

### 3.1 Règles relatives à la cryptographie

Pour les mesures de sécurité comprenant des mécanismes cryptographiques, respecter:
- **[RGS_B1]** et **[RGS_B2]:** Règles communes à tous les mécanismes cryptographiques
- **[RGS_B3]:** Règles dédiées aux mécanismes d'authentification

### 3.2 Règles relatives à la protection des échanges électroniques

Les fonctions de sécurité (authentification, signature, confidentialité, horodatage) reposent sur:
- **Contremarques de temps** pour l'horodatage électronique
- **Certificats électroniques** pour toutes les autres fonctions

#### a) Règles relatives aux certificats électroniques

**Annexes applicables:**
- **[RGS_A2]:** Politique de certification type - Personne physique
- **[RGS_A3]:** Politique de certification type - Services applicatifs

**Types de certificats disponibles:**

| Type | Niveaux | Usage |
|------|---------|-------|
| Mono-usage | *, **, *** | Authentification personne/serveur, signature, cachet, confidentialité |
| Double usage | *, ** | Authentification de personne physique + signature électronique |

#### a.1) L'authentification d'une entité par certificat électronique

L'authentification vérifie l'identité dont se réclame une personne ou une machine.

**Niveaux de sécurité:** (*), (**), (***)

**Composants couverts (annexe [RGS_A1]):**
- Bi-clé et certificat électronique d'authentification
- Dispositif d'authentification
- Module de vérification d'authentification
- Application d'authentification

#### a.2) La signature et le cachet électroniques

**Signature électronique (personne):**
- Garantit l'identité du signataire
- Garantit l'intégrité du document signé
- Établit le lien entre document et signature
- Traduit le consentement du signataire

**Cachet (machine):**
- Garantit l'intégrité des informations échangées
- Identifie le service ayant « cacheté »
- Équivalent de la signature pour les services applicatifs

**Niveaux de sécurité:** (*), (**), (***)

**Composants couverts (annexe [RGS_A1]):**
- Bi-clé et certificat électronique de signature/cachet
- Dispositif de création de signature/cachet
- Application de création de signature/cachet
- Module de vérification de signature/cachet

**Cas particuliers:**

1. **Signature des actes administratifs** (ordonnance du 8 décembre 2005, art. 8):
   - L'autorité administrative détermine le niveau de sécurité requis

2. **Signature « présumée fiable »** (art. 1316-4 du Code civil):
   - Exigences de l'arrêté du 26 juillet 2004 (certificats « qualifiés »)
   - Une signature conforme au niveau (***) de [RGS_A2] est de facto « présumée fiable »

#### a.3) La confidentialité

Le chiffrement est le mécanisme essentiel de protection de la confidentialité.

**Mesures complémentaires recommandées:**
- Gestion des droits d'accès (lecture, écriture, modification)
- Mécanismes robustes implémentés au plus près du stockage

**Niveaux de sécurité:** (*), (**), (***)

**Composants couverts (annexe [RGS_A1]):**
- Bi-clé et certificat électronique de chiffrement
- Dispositif de chiffrement
- Module de chiffrement
- Module de déchiffrement

#### b) Règles relatives à l'horodatage électronique

**Annexe applicable:** [RGS_A5] - Politique d'horodatage type

L'horodatage atteste qu'une donnée existe à un instant donné via une **contremarque de temps** générée par un PSHE (Prestataire de Services d'Horodatage Électronique).

**Niveau de sécurité:** Unique

**Cas particulier - Horodatage « présumé fiable »** (art. 1369-7 et 1369-8 du code civil):
- Décret n° 2011-434 du 20 avril 2011
- Module d'horodatage certifié (décret n° 2002-535)
- Un horodatage « présumé fiable » est de facto conforme à [RGS_A5]

#### c) Rattachement à l'IGC/A

L'ANSSI offre un service de certification consistant à signer l'AC racine d'une autorité administrative par l'AC de l'**Infrastructure de Gestion de la Confiance de l'Administration (IGC/A)**.

**Caractéristiques:**
- L'ANSSI est l'autorité racine de l'IGC/A
- Règles définies dans les politiques de certification [PC_IGC/A]
- Certificats racines publiés au Journal officiel

---

## Chapitre 4. Règles relatives aux accusés d'enregistrement et aux accusés de réception

Les accusés d'enregistrement et de réception (art. 5 de l'ordonnance du 8 décembre 2005) ne sont pas des fonctions de sécurité en soi, mais peuvent s'appuyer sur: signature, cachet, horodatage.

**Recommandations pour les accusés (cas général):**
- Horodatage avec contremarques de temps conformes à [RGS_A_5]
- Signature par un agent ou cachet par une machine, conformément à [RGS_A_2] et [RGS_A_3]
- Mécanismes cryptographiques conformes à [RGS_B_1] et [RGS_B_2]

**Cas particulier (art. 5.II):** Lorsque la date fait foi, un niveau de fiabilité supplémentaire est requis.

**Gestion:** Sauvegarde obligatoire tant que des réclamations peuvent survenir.

---

## Chapitre 5. Qualification des produits de sécurité et des prestataires de services de confiance

Conformément à l'article 4 du décret RGS, les autorités administratives doivent:
- Recourir à des produits/prestataires **qualifiés**
- Ou s'assurer de la conformité au RGS et attester formellement cette conformité à l'ANSSI

### 5.1 Qualification des produits de sécurité

**Trois niveaux de qualification:**

| Niveau | Document | Base de certification |
|--------|----------|----------------------|
| Élémentaire | [QE] | CSPN |
| Standard | [QS] | Critères communs |
| Renforcé | [QR] | Critères communs |

**Procédure:**
- L'ANSSI instruit les demandes et délivre les attestations
- Certification préalable requise (décret n° 2002-535, instruction n° 1414/ANSSI/SR)
- Contrôles complémentaires de l'ANSSI (cible de sécurité, tests cryptographiques)

**Recommandation:** Cible de sécurité conforme à un profil de protection ANSSI

### 5.2 Qualification des prestataires de services de confiance (PSCO)

**Caractéristiques:**
- Qualification distincte par type de prestation
- Attestation délivrée par un organisme de qualification accrédité (COFRAC) et habilité (ANSSI)
- Durée: 3 ans
- Pour les administrations d'État: l'ANSSI peut procéder directement à l'évaluation

#### a) Prestataires de services de certification électronique (PSCE)

**Annexes:** [RGS_A2] et [RGS_A3]

**Niveaux:** (*), (**), (***)

**Usages couverts:** chiffrement, authentification personne, signature, authentification machine, cachet, double usage

#### b) Prestataires de services d'horodatage électronique (PSHE)

**Annexe:** [RGS_A5]

**Niveau:** Unique

#### c) Prestataires d'audit de la sécurité des systèmes d'information

**Annexe:** [RGS_C]

**Activités couvertes:**
- Audit organisationnel
- Audit de code source
- Audit de configuration et d'architecture
- Tests d'intrusion

---

## Chapitre 6. Validation des certificats par l'État

### 6.1 Champ d'application

Les certificats délivrés aux autorités administratives et à leurs agents doivent être **validés par l'État** (art. 10 de l'ordonnance, chapitre V du décret RGS).

**Fonctions de sécurité concernées:**
- Authentification d'une personne et d'un serveur
- Signature électronique et cachet
- Confidentialité

**Procédure de délivrance des certificats:**
- Identification et vérification de l'identité des agents
- Fabrication technique des certificats
- Remise des certificats aux porteurs
- Publication des certificats, de leur statut et de la PC
- Révocation et renouvellement des certificats

### 6.2 Règles de sécurité

**L'autorité de certification doit:**
- Rédiger une PC par fonction, usage et niveau de sécurité (conforme aux PC-Types)
- Utiliser des clés cryptographiques dédiées aux AA et leurs agents
- Générer des certificats à destination exclusive des AA ou agents
- Établir des procédures avec les prestataires externes
- Faire approuver la PC par l'AA et la respecter

### 6.3 Procédure de validation

**Délais:**
- 3 ans à compter de la publication du RGS pour obtenir la validation
- 3 mois après délivrance pour les certificats émis après ce délai
- 2 mois pour l'instruction du dossier par l'ANSSI

**Dossier de demande:**
- Identification de l'AA et de l'AC
- PC de l'AC et chaîne de certification
- Procédures avec prestataires externes
- Résultats d'audit de conformité
- Attestation de qualification (le cas échéant)
- Certificat de l'AC

**En cas de non-conformité:** 3 mois pour corriger les procédures.

### 6.4 Liste des informations à mettre à disposition

- PC de l'AC
- Certificat de l'AC
- Mention de l'obtention de la validation

---

## Chapitre 7. Recommandations relatives à l'application du référentiel

### 7.1 Organiser la sécurité des systèmes d'information

#### a) Organiser les responsabilités

**Organisation recommandée:**
- Dirigée par un représentant de l'autorité administrative
- Disposant des moyens matériels nécessaires
- Capable de gérer risques, crises et incidents

**Chaîne fonctionnelle SSI:**
- RSSI (Responsable de la Sécurité des Systèmes d'Information)
- OSSI (Officier de la Sécurité des Systèmes d'Information)
- Correspondants SSI

**Missions:**
- Coordination des actions SSI dans les contrats
- Formalisation de la répartition des responsabilités
- Relations avec les autorités externes de défense des SI

#### b) Système de management de la sécurité (SMSI)

Recommandé selon ISO 27001 pour:
- Planifier et mettre en œuvre les mesures de protection
- Vérifier la pertinence et la conformité

#### c) Politique de sécurité des systèmes d'information (PSSI)

Élaborer et formaliser une PSSI (guide ANSSI « Politique SSI »).

### 7.2 Impliquer les instances décisionnelles

Les instances décisionnelles doivent:
- Donner les orientations (investissement humain et financier)
- Valider les objectifs de sécurité et orientations stratégiques

### 7.3 Adapter l'effort de protection aux enjeux

**Guides ANSSI recommandés:**
- « Maturité SSI »
- « GISSIP » (Gestion et Intégration de la SSI dans les Projets)

### 7.4 Adopter une démarche globale

**Recommandations:**
- Considérer tous les aspects (techniques et non techniques)
- Envisager tous les risques et menaces
- Prendre en compte la SSI à tous les niveaux hiérarchiques
- Responsabiliser tous les acteurs
- Intégrer la SSI tout au long du cycle de vie des SI
- Prendre en compte la sécurité dès la phase de définition des objectifs

### 7.5 Informer et sensibiliser le personnel

Formation adaptée et régulière pour tous les agents sur:
- Principes de base de la sécurité
- Règles élémentaires de bonne utilisation

**Référence:** Bonnes pratiques ANSSI

### 7.6 Prendre en compte la sécurité dans les contrats et les achats

**Points à traiter:**
- Intégrer les référentiels ANSSI aux cahiers des charges
- Demander les éléments permettant d'apprécier le niveau de sécurité
- Préciser les clauses de maintenance
- Préciser les conditions d'intervention des sous-traitants
- Préciser les clauses de qualité et sécurité
- Préciser les conditions de propriété des codes sources
- Prévoir la réversibilité et la portabilité des données
- Préciser les tableaux de bord et mécanismes de suivi
- Prévoir les modalités de réaction aux crises et incidents
- Prévoir des points de contact compétents
- Vérifier la couverture des exigences sécurité

### 7.7 Sécurité dans l'externalisation et le cloud

**Guide ANSSI:** « Maîtriser les risques de l'infogérance – Externalisation des systèmes d'information »

**Contenu:**
- Démarche de prise en compte de la SSI
- Clauses types et base d'exigences de sécurité

### 7.8 Mécanismes de défense des systèmes d'information

**Mesures complémentaires:**
- Connaissance des systèmes (cartographie, répertoire des interconnexions)
- Détection des malveillances, erreurs et imprudences
- Traçabilité des actions et accès (journalisation)
- Pérennisation des savoir-faire et compétences
- Conservation de la preuve des infractions

### 7.9 Utiliser les produits et prestataires labellisés

**Recommandations:**
- Utiliser des produits de sécurité qualifiés ANSSI
- Recourir à des PSCO qualifiés
- Considérer la certification ISO 27001 des prestataires
- Considérer la certification des personnels

### 7.10 Plans de traitement d'incidents, continuité et reprise d'activité

**Documents à élaborer:**
- Plan de continuité d'activité (PCA)
- Plan de reprise d'activité (PRA)

Ces plans doivent être régulièrement mis à jour et testés.

### 7.11 Audits réguliers de la sécurité

**Annexe applicable:** [RGS_C]

Faire appel à des prestataires d'audit qualifiés.

### 7.12 Veille sur les menaces et les vulnérabilités

**Sources d'information:**
- CERT-FR (www.cert.ssi.gouv.fr)
- Sites des éditeurs de logiciels et matériels

### 7.13 Favoriser l'interopérabilité

**Références:**
- Référentiel Général d'Interopérabilité (RGI)
- Arrêté du 18 janvier 2012 relatif au référencement

---

## Chapitre 8. Transition entre la première et la deuxième version du RGS

**Obligation de conformité:** Les autorités administratives devaient être conformes au RGS v1 dans les 3 ans suivant sa publication.

**Mesures transitoires pour certificats et horodatage:**

| Certificats | Émission | Acceptation |
|-------------|----------|-------------|
| Conformes RGS v1 | Jusqu'à 1 an après entrée en vigueur v2 | Pendant leur durée de vie (max 3 ans), soit max 4 ans après v2 |
| Conformes RGS v2 | Dès entrée en vigueur v2 | Au plus tard 1 an après v2 |

---

## Chapitre 9. Liste des annexes du RGS

### 9.1 Documents concernant les certificats électroniques

| Référence | Titre | Version |
|-----------|-------|---------|
| [RGS_A1] | Règles relatives à la mise en œuvre des fonctions de sécurité basées sur l'emploi de certificats électroniques | 3.0 |
| [RGS_A2] | Politique de Certification Type « certificats électroniques de personne » | 3.0 |
| [RGS_A3] | Politique de Certification Type « services applicatifs » | 3.0 |
| [RGS_A4] | Profils de certificats, CRL, OCSP et algorithmes cryptographiques | 3.0 |
| [RGS_A5] | Politique d'Horodatage Type | 3.0 |

### 9.2 Documents concernant les mécanismes cryptographiques

| Référence | Titre | Version |
|-----------|-------|---------|
| [RGS_B1] | Règles et recommandations concernant le choix et le dimensionnement des mécanismes cryptographiques | 2.03 |
| [RGS_B2] | Règles et recommandations concernant la gestion des clés utilisées dans des mécanismes cryptographiques | 2.00 |
| [RGS_B3] | Règles et recommandations concernant les mécanismes d'authentification | 1.0 |

### 9.3 Référentiel d'exigences pour les prestataires d'audit

| Référence | Titre | Version |
|-----------|-------|---------|
| [RGS_C] | Référentiel d'exigences applicables aux prestataires d'audit de la SSI | 1.1 |

---

## Chapitre 10. Références documentaires

### 10.1 Références réglementaires

| Référence | Description |
|-----------|-------------|
| [loi120400] | Loi n° 2000-321 du 12 avril 2000 relative aux droits des citoyens dans leurs relations avec les administrations |
| [DécretRGS] | Décret n° 2010-112 du 2 février 2010 |
| [Décret2001-272] | Décret n° 2001-272 du 30 mars 2001 relatif à la signature électronique |
| [Décret2002-535] | Décret n° 2002-535 du 18 avril 2002 relatif à l'évaluation et à la certification de la sécurité |
| [Décret2011-434] | Décret n° 2011-434 du 20 avril 2011 relatif à l'horodatage |
| [Arrêté260704] | Arrêté du 26 juillet 2004 relatif à la qualification des PSCE |
| [Arrêté200411] | Arrêté du 20 avril 2011 relatif à la qualification des PSHE |
| [IGI1300] | Instruction générale interministérielle n° 1300/SGDSN/PSE/PSD sur la protection du secret |
| [I-CSPN] | Instruction n° 1414/ANSSI/SR du 30 mai 2011 relative à la CSPN |
| [HabilitationOQ] | Instruction n°1001/ANSSI/SR du 8 avril 2011 relative à l'habilitation des organismes de qualification |
| [Stratégie_France] | Défense et sécurité des systèmes d'information - Stratégie de la France |

### 10.2 Références techniques

| Référence | Description |
|-----------|-------------|
| [ISO27001] | ISO/CEI 27001:2013 - Systèmes de management de la sécurité de l'information |
| [ISO27002] | ISO/CEI 27002:2013 - Code de bonne pratique pour le management de la sécurité |
| [ISO27005] | ISO/CEI 27005:2011 - Gestion des risques liés à la sécurité de l'information |
| [ISO27035] | ISO/CEI 27035:2011 - Gestion des incidents de sécurité |
| [PCI-DSS] | PCI Data Security Standard, version 3.0 |
| [PSSI] | Guide « Politique SSI » de l'ANSSI |
| [MaturitéSSI] | Guide « maturité SSI » de l'ANSSI |
| [EBIOS 2010] | Méthode d'analyse de risque de l'ANSSI |
| [GISSIP] | Guide « Gestion et Intégration de la SSI dans les Projets » |
| [Guide Externalisation] | Maîtriser les risques de l'infogérance |
| [Guide homologation] | L'homologation de sécurité en neuf étapes simples |
| [AvisCertsIGC/A] | Avis relatif aux certificats de l'IGC/A |
| [GHI] | Guide d'hygiène informatique |
| [SI-industriels] | La cybersécurité des systèmes industriels |
| [CC] | Common Criteria |
| [QE], [QS], [QR] | Processus de qualification des produits de sécurité |
| [PC_IGC/A] | Politiques de Certification de l'IGC/A |
