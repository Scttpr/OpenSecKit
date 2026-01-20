# RGS v2.0 - Annexe A1

## Règles relatives à la mise en œuvre des fonctions de sécurité basées sur l'emploi de certificats électroniques

**Version 3.0 du 27 février 2014**

---

## Historique des versions

| Date | Version | Évolution du document | Rédacteur |
|------|---------|----------------------|-----------|
| 06/11/2006 | 2.1 | Document constitutif de la Politique de Référencement Intersectorielle de Sécurité – PRISv2.1 | DCSSI / SDAE |
| 12/12/2008 | 2.2 | Document constitutif du RGSv0.98, annexe A1. Restructuration du document | DCSSI / DGME |
| 11/02/2010 | 2.3 | Document constitutif du RGSv1.0, annexe A1. Suppression des exigences des chapitres III.2, III.3.2 et III.4.2 et III.5.2. Rajout de chapitres relatifs à la qualification des produits de sécurité et des offres de PSCE | ANSSI / DGME |
| 27/02/2014 | 3.0 | Document constitutif du RGSv2.0, annexe A1. Fusion des annexes A1 à A5 du RGS version 1.0. Modifications des exigences sur les dispositifs de protection des clés privées des services applicatifs. Modifications des exigences de qualification des applications de création de signature électronique | ANSSI |

**Contact:** SGDSN/ANSSI - 51 boulevard de La Tour-Maubourg, 75700 Paris 07 SP - rgs@ssi.gouv.fr

---

## Sommaire

1. [Objet et contenu du document](#i-objet-et-contenu-du-document)
2. [Présentation des fonctions de sécurité](#ii-présentation-des-fonctions-de-sécurité)
3. [Exigences relatives à la mise en œuvre des fonctions de sécurité](#iii-exigences-relatives-à-la-mise-en-œuvre-des-fonctions-de-sécurité)
4. [Documents de référence](#iv-documents-de-référence)

---

## I. Objet et contenu du document

Le présent document fait partie des documents constitutifs du Référentiel Général de Sécurité [RGS]. Il en constitue l'annexe [RGS_A1].

Il fixe les règles de sécurité applicables aux différents « composants » nécessaires à la mise en œuvre des fonctions de sécurité basées sur l'emploi des certificats électroniques.

### Fonctions de sécurité couvertes

- Signature électronique
- Authentification de personne
- Double usage signature électronique et authentification
- Confidentialité
- Cachet
- Authentification de serveur

### Composants concernés

- Les bi-clés et certificats électroniques délivrés par des prestataires de service de certification électronique (PSCE)
- Le dispositif de protection des éléments secrets
- Les applications qui assurent l'interface avec les usagers (ou les machines), les dispositifs de protection et les éléments secrets

### Public cible

Ce document s'adresse aux autorités administratives (AA) qui ont décidé, après analyse des risques, de mettre en œuvre, pour un niveau de sécurité donné parmi *, ** et ***, l'une ou plusieurs des fonctions de sécurité du RGS.

**Convention de notation:**
- Les règles spécifiques à une fonction sont précédées du nom entre crochets (ex: `[Signature électronique]`)
- Les règles pour personnes physiques: `[Personne]`
- Les règles pour services applicatifs: `[Service applicatif]`

---

## II. Présentation des fonctions de sécurité

### II.1. Fonction de sécurité « signature électronique »

La signature électronique apporte de la confiance dans les échanges dématérialisés entre usagers et autorités administratives ou entre autorités administratives.

#### Types de relations couverts

- Signature électronique par un usager, vérification par un téléservice d'une AA
- Signature électronique par un usager, vérification par un agent d'une AA
- Signature électronique par un agent d'une AA, vérification par un usager
- Signature électronique par un agent d'un acte administratif, vérification par un autre agent

#### Contextes d'utilisation

La signature électronique peut être mise en œuvre depuis:
- Un ordinateur personnel
- Une borne d'accès dans un lieu public (mairie, CPAM, etc.)

> **Note:** Le recours à la signature électronique est imposé seulement pour la validité des actes administratifs établis sous forme électronique (Art. 8 de l'Ordonnance).

#### Principe de fonctionnement

1. **Application de création de signature:** Déployée sur une machine (PC, borne publique, serveur), elle réalise les premières itérations de calcul d'un condensat à l'aide d'une fonction de hachage
2. **Transmission au dispositif:** Les informations nécessaires (informations à signer, condensat partiel) sont transmises au dispositif de création de signature (carte à puce, clé USB)
3. **Dispositif de création de signature:** Réalise les itérations restantes du calcul du condensat, puis un calcul cryptographique de signature en utilisant la clé privée (activée par code PIN)
4. **Retour de la signature:** Le condensat signé (signature électronique) est retourné à l'application
5. **Vérification:** Effectuée à l'aide d'un module de vérification et du certificat électronique du PSCE

> **Important:** L'utilisation de la clé privée de signature et du certificat mono-usage associé est strictement limitée à la signature électronique. Les certificats « double usage » (authentification et signature) sont tolérés.

#### Présomption de fiabilité

La mise en œuvre d'un procédé de signature électronique respectant les exigences du **niveau *** permet de bénéficier de la présomption de fiabilité (article 1316-4 du code civil), conformément aux articles 6 et 3 du [Décret2001-272].

---

### II.2. Fonction de sécurité « confidentialité »

La confidentialité apporte de la confiance dans les échanges dématérialisés entre usagers et autorités administratives ou entre autorités administratives.

#### Types de relations couverts

- Chiffrement par un service d'une AA, à destination d'un usager ou d'un agent
- Chiffrement par un service, à destination d'un agent d'une AA
- Chiffrement par un usager ou un agent, à destination d'un agent d'une AA

Le chiffrement garantit que les données échangées ne seront accessibles que par le(s) destinataire(s), lors de l'échange ou du stockage.

#### Principe de fonctionnement

1. **Chiffrement des données:** Effectué à l'aide d'une clé symétrique dite « clé de session »
2. **Échange de la clé de session:** Via mécanisme cryptographique asymétrique ou Diffie-Hellman
3. **Module de chiffrement:** Utilise la clé publique du destinataire (trouvée dans son certificat électronique)
4. **Déchiffrement:** Le destinataire déchiffre à l'aide de sa clé privée confinée dans un dispositif de stockage

> **Alternative:** Les données peuvent être chiffrées directement avec la clé publique du destinataire et déchiffrées avec sa clé privée (sans clé de session).

> **Important:** L'utilisation de la clé privée de déchiffrement et du certificat mono-usage associé est strictement limitée au service de confidentialité.

---

### II.3. Fonction de sécurité « authentification »

L'authentification apporte de la confiance dans les échanges dématérialisés entre usagers et autorités administratives ou entre autorités administratives.

#### Types de relations couverts

- Authentification d'un usager vis-à-vis d'un service de l'administration
- Authentification d'un usager vis-à-vis d'un agent d'une AA
- Authentification d'un agent d'une AA vis-à-vis d'un usager

> **Note:** Ce document ne traite que de l'authentification basée sur des mécanismes cryptographiques asymétriques.

#### Principe de fonctionnement

1. **Application de création de cachet:** Calcule un condensat à partir des informations à signer
2. **Transmission au dispositif:** Le condensat est transmis au dispositif de création de cachet
3. **Dispositif de création de cachet:** Réalise un calcul cryptographique de signature du condensat en utilisant la clé privée (activée par code PIN)
4. **Retour du cachet:** Le condensat signé est retourné à l'application
5. **Vérification:** Effectuée à l'aide d'un module de vérification et du certificat électronique du PSCE

> **Important:** L'utilisation de la clé privée d'authentification et du certificat mono-usage associé est strictement limitée à l'authentification. Les certificats « double usage » sont tolérés.

---

### II.4. Fonction de sécurité « cachet »

Le cachet, apposé par un service de création de cachet, apporte de la confiance dans les échanges dématérialisés. Le terme « cachet » est utilisé par un service applicatif, se différenciant de la « signature électronique » réservée à une personne physique.

#### Types de relations couverts

- Apposition d'un cachet par un service applicatif d'une AA, vérification par un usager
- Apposition d'un cachet par un service applicatif, vérification par un agent d'une AA
- Apposition d'un cachet par un service applicatif, vérification par un autre service applicatif

#### Principe de fonctionnement

1. **Application de création de cachet:** Réalise les premières itérations de calcul d'un condensat
2. **Transmission au dispositif:** Les informations nécessaires sont transmises au dispositif de création de cachet (carte à puce, clé USB)
3. **Dispositif de création de cachet:** Réalise les itérations restantes du calcul du condensat, puis retourne le condensat signé (cachet) à l'application
4. **Vérification:** Effectuée à l'aide d'un module de vérification et du certificat électronique du PSCE

> **Important:** L'utilisation de la clé privée du service de création de cachet et du certificat mono-usage associé est strictement limitée au service de cachet.

---

### II.5. Fonction de sécurité « authentification serveur »

L'authentification d'un serveur apporte de la confiance dans les échanges dématérialisés entre usagers et autorités administratives ou entre autorités administratives.

#### Types de relations couverts

- Établissement d'une session sécurisée entre un serveur d'une AA et un usager
- Établissement d'une session sécurisée entre un serveur et un agent d'une AA
- Établissement d'une session sécurisée entre deux serveurs

#### Principe de fonctionnement

1. **Application d'authentification:** Transmet une requête d'authentification (« challenge ») au dispositif d'authentification
2. **Dispositif d'authentification:** Réalise un calcul cryptographique de signature du challenge en utilisant la clé privée (activée par code PIN)
3. **Retour du challenge signé:** Le challenge signé est retourné à l'application
4. **Vérification:** Effectuée à l'aide d'un module de vérification et du certificat électronique du PSCE

> **Important:** L'utilisation de la clé privée d'authentification du serveur et du certificat mono-usage associé est strictement limitée à l'authentification et à l'établissement de session sécurisée.

---

## III. Exigences relatives à la mise en œuvre des fonctions de sécurité

### III.1. Les certificats délivrés par les PSCE

Les exigences que doit respecter un PSCE sont définies dans les politiques de certification type (PC Type):
- **[RGS_A2]:** PC Type « Personne » (signature électronique, authentification, confidentialité)
- **[RGS_A3]:** PC Type « Service applicatif » (cachet, authentification serveur)

#### Niveaux de sécurité

| Fonction | Niveaux disponibles |
|----------|-------------------|
| Signature électronique | *, **, *** |
| Authentification | *, **, *** |
| Confidentialité | *, **, *** |
| Cachet | *, **, *** |
| Authentification serveur | *, **, *** |
| Double usage (Authentification + Signature) | *, ** |

#### Règles d'utilisation

- Il est autorisé d'utiliser un certificat de niveau supérieur, sous réserve de cohérence entre le niveau du dispositif de protection et celui du certificat
- Les PC Types s'appuient sur l'annexe [RGS_A4] pour les profils de certificats, LCR, OCSP et algorithmes cryptographiques
- Un PSCE peut faire qualifier son offre selon les modalités du [DécretRGS]

---

### III.2. Les dispositifs de protection des éléments secrets

Le dispositif de protection des éléments secrets est un logiciel ou matériel (carte à puce) qui:
- Stocke la clé privée dédiée à une fonction de sécurité
- Stocke les éléments de déverrouillage (code PIN)
- Permet la mise en œuvre et éventuellement la génération des clés

#### III.2.1. Dispositifs de protection des éléments secrets d'une personne physique

##### III.2.1.1. Exigences de sécurité

Les exigences sont décrites dans l'annexe 3 des PC Type [RGS_A2].

**Exigences communes (tous niveaux):**

- Si la bi-clé est générée par le dispositif: garantir que la génération est réalisée exclusivement par des utilisateurs autorisés et garantir la robustesse cryptographique
- Détecter les défauts lors des phases d'initialisation, de personnalisation et d'opération
- Garantir la confidentialité et l'intégrité de la clé privée
- Assurer la correspondance entre la clé privée et la clé publique
- Permettre de garantir l'authenticité et l'intégrité de la clé publique lors de son export

**[Tous usages sauf Confidentialité]:**
- Disposer de techniques sûres de destruction de la clé privée en cas de re-génération ou de clés non utilisées

**[Confidentialité]:**
- Permettre de garantir la confidentialité, l'authenticité et l'intégrité de la clé symétrique lors de son export vers l'application de déchiffrement

##### III.2.1.2. Exigences sur la qualification

Ces exigences ne sont requises que pour la qualification de l'offre de certificats électroniques selon le [DécretRGS].

| Niveau | Qualification requise | Recommandation |
|--------|----------------------|----------------|
| *** | Niveau renforcé | - |
| ** | Niveau standard (minimum) | Niveau renforcé |
| * | Niveau élémentaire (minimum) | Niveau standard |

#### III.2.2. Dispositifs de protection des éléments secrets d'un service applicatif

##### III.2.2.1. Exigences de sécurité

Les exigences sont décrites dans l'annexe 3 des PC Type [RGS_A3].

**Exigences communes (tous niveaux):**

- Si la bi-clé est générée par le dispositif: garantir la génération par utilisateurs autorisés et la robustesse cryptographique
- Assurer la correspondance entre la clé privée et la clé publique

**Mesures organisationnelles, procédurales ou techniques à mettre en place:**

- Détecter les défauts lors des phases d'initialisation et d'opération
- Disposer de techniques sûres de destruction de la clé privée
- Garantir la confidentialité et l'intégrité de la clé privée
- Permettre de garantir l'authenticité et l'intégrité de la clé publique lors de son export

**[Cachet]:**
- Assurer pour le serveur légitime uniquement la fonction de génération des cachets électroniques
- Protéger la clé privée contre toute utilisation par des tiers

**[Authentification Serveur]:**
- Assurer pour le serveur légitime la fonction d'authentification et de déchiffrement de clés symétriques de session
- Protéger la clé privée contre toute utilisation par des tiers
- Permettre de garantir l'authenticité et l'intégrité de la clé symétrique de session lors de son export

> **Nota:** Les dispositifs matériels (cartes à puces, modules cryptographiques) qualifiés par l'ANSSI respectent ces exigences. Des solutions logicielles peuvent également les respecter avec des mesures de sécurité additionnelles.

##### III.2.2.2. Exigences en terme d'évaluation et d'audit

Les composantes de l'IGC qui mettent en œuvre la clé privée doivent faire l'objet d'un **audit de sécurité**.

**Pour les niveaux ** et ***:** Audit technique minimum tous les 2 ans, comprenant:
- Audit de l'architecture réseau (liaison entre zones, filtrage)
- Audit de configuration (équipements réseau et sécurité, serveurs d'infrastructure)
- Audit organisationnel

L'environnement de déploiement de la clé privée peut également faire l'objet d'un audit.

---

### III.3. Les Applications

#### III.3.1. Exigences de sécurité

**[Signature]:**
- Recommandé: Application de création de signature conforme au profil de protection [PP_Appli]
- Recommandé: Module de vérification de signature conforme au profil de protection [PP_Vérif]

**[Confidentialité] - Module de chiffrement:**

| Exigence | Description |
|----------|-------------|
| Robustesse cryptographique | Garantir la robustesse de la clé symétrique générée |
| Détection des défauts | Lors des phases d'initialisation, personnalisation et opération |
| Destruction des clés | Techniques sûres de destruction des clés non utilisées |
| Protection des clés | Garantir confidentialité et intégrité de la clé symétrique et des données |
| Contrôle d'accès | Accès à la clé exclusivement par utilisateurs autorisés |

**[Confidentialité] - Module de déchiffrement:**

| Exigence | Description |
|----------|-------------|
| Détection des défauts | Lors des phases d'initialisation, personnalisation et opération |
| Destruction des clés | Techniques sûres de destruction des clés non utilisées |
| Protection des clés | Garantir confidentialité et intégrité de la clé symétrique et des données |
| Contrôle d'accès | Accès à la clé exclusivement par utilisateurs autorisés |

#### III.3.2. Exigences sur la qualification

| Niveau | Recommandation |
|--------|---------------|
| *** | Applications qualifiées au niveau standard |
| ** | Applications qualifiées au niveau standard |
| * | Applications qualifiées au niveau élémentaire |

#### III.3.3. Bonnes pratiques

Avant de se fier à un certificat électronique, vérifier que:
- Il contient une indication d'usage conforme à ce qui est attendu
- Il est valide et n'est pas révoqué
- Sa chaîne de certification est correcte à tous les niveaux
- Il correspond au niveau de sécurité cohérent avec l'usage prévu

> **Recommandation:** Élaborer une politique de vérification des certificats électroniques.

---

### III.4. Environnement d'utilisation

**[Personne]:** Fonctions « Signature », « Authentification » et « Confidentialité »
- Mise en œuvre sur borne publique ou ordinateur (privé ou professionnel)
- Usage par une personne physique

**[Service applicatif]:** Fonctions « Cachet » et « Authentification Serveur »
- Mise en œuvre sur serveur(s) hébergeant un service applicatif
- Usage relevant d'une personne morale, sous contrôle d'une personne physique

#### Mesures de sécurité recommandées

- Protection contre les virus avec mises à jour régulières
- Contrôle et limitation des échanges réseau
- Restriction de l'accès aux fonctions d'administration (différenciation utilisateur/administrateur)
- Installation et mise à jour de logiciels sous contrôle de l'administrateur
- Refus d'exécuter des applications téléchargées de sources non sûres
- Mise à jour des composants logiciels et systèmes

**[Personne]:** Pour l'utilisation d'une carte à puce, il est recommandé (particulièrement au niveau ***) d'utiliser un **lecteur de carte avec PIN/PAD intégré qualifié** permettant de saisir et vérifier le code sans transiter par la machine hôte.

**[Confidentialité]:** Au niveau ***, il est recommandé que les informations à protéger ne soient jamais présentes en clair sur une machine reliée au réseau de transit des données chiffrées.

---

## IV. Documents de référence

### IV.1. Réglementation

| Référence | Document |
|-----------|----------|
| [ORDONNANCE] | Ordonnance n° 2005-1516 du 8 décembre 2005 relative aux échanges électroniques entre les usagers et les autorités administratives et entre les autorités administratives |
| [DécretRGS] | Décret pris pour l'application des articles 9, 10 et 12 de l'ordonnance n° 2005-1516 du 8 décembre 2005 |
| [Décret2001-272] | Décret n° 2001-272 du 30 mars 2001 pris pour l'application de l'article 1316-4 du code civil et relatif à la signature électronique |

### IV.2. Documents techniques

| Référence | Document |
|-----------|----------|
| [PP_Appli] | Profil de protection application de création de signature électronique Version 1.6 d'août 2008 |
| [PP_Vérif] | Profil de protection module de vérification de signature électronique Version 1.6 d'août 2008 |
| [RGS] | Référentiel Général de Sécurité - Version 2.0 |
| [RGS_A2] | Politique de Certification Type « Personne » - Version 3.0 |
| [RGS_A3] | Politique de Certification Type « Service applicatif » - Version 3.0 |
| [RGS_A4] | Profils de certificats, de LCR et OCSP et algorithmes cryptographiques – Version 2.3 |
