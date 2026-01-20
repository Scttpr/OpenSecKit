# Guide d'hygiène informatique

**Renforcer la sécurité de son système d'information en 42 mesures**

*ANSSI - Agence nationale de la sécurité des systèmes d'information*

**Version 2.0 - Septembre 2017**

Licence Ouverte/Open Licence (Etalab - V1)

---

## Avant-propos

Dans le monde actuel, la quasi-totalité des organisations, qu'il s'agisse d'entreprises ou d'administrations publiques, dépend aujourd'hui fortement de leur système d'information. Avec l'accélération des échanges électroniques, ce phénomène connaît une croissance soutenue.

Les incidents de sécurité, qu'il s'agisse d'attaques informatiques ou de négligences internes, sont de plus en plus fréquents et leurs impacts de plus en plus lourds. Pour faire face à ces menaces, l'ANSSI propose ce guide de bonnes pratiques qui présente, au travers de 42 mesures, les règles essentielles de la sécurité du numérique.

Ce guide est l'occasion de rappeler que la sécurité informatique est l'affaire de tous, au sein de l'entité.

## Mode d'emploi

Ce guide présente des règles d'hygiène informatique réparties en 10 sections thématiques. Chaque règle, ou mesure de sécurité, est accompagnée d'un niveau d'exigence :

- **Standard** : niveau de sécurité minimal que toute organisation devrait atteindre
- **Renforcé** : niveau de sécurité supérieur pour les organisations disposant d'un SI particulièrement sensible ou ciblé

Les mesures présentées dans ce guide constituent le socle minimum de règles à respecter pour protéger les informations de l'entité. Elles ne sont pas suffisantes à elles seules pour répondre aux besoins de sécurité de chaque organisation. Une analyse de risques doit permettre d'identifier les mesures complémentaires à mettre en œuvre.

---

## I - Sensibiliser et former

### Mesure 1 : Former les équipes opérationnelles à la sécurité des systèmes d'information

**Niveau : Standard**

Les équipes opérationnelles (administrateurs système, réseau et sécurité, développeurs) doivent suivre des formations régulières sur les aspects techniques de la sécurité des systèmes d'information. Cette formation doit porter sur :

- La configuration sécurisée des systèmes qu'ils administrent
- Les mécanismes de journalisation et de surveillance
- Les procédures de gestion des incidents
- Les bonnes pratiques de développement sécurisé (pour les développeurs)

### Mesure 2 : Sensibiliser les utilisateurs aux bonnes pratiques élémentaires de sécurité informatique

**Niveau : Standard**

Tous les utilisateurs du système d'information doivent être sensibilisés aux risques informatiques et aux bonnes pratiques de sécurité. Cette sensibilisation doit être régulière et adaptée aux profils des utilisateurs. Elle doit couvrir notamment :

- La gestion des mots de passe
- La reconnaissance des tentatives de phishing
- L'utilisation des supports amovibles
- La protection des informations sensibles
- Les risques liés au nomadisme

Une charte d'utilisation des moyens informatiques doit être établie et signée par les utilisateurs.

### Mesure 3 : Maîtriser les risques de l'infogérance

**Niveau : Standard**

Lorsque tout ou partie du système d'information est confié à un prestataire (hébergement, administration, développement), l'entité doit s'assurer :

- De la formalisation des exigences de sécurité dans le contrat
- De la qualification du prestataire (ISO 27001, SecNumCloud, etc.)
- Des clauses de réversibilité et d'auditabilité
- De la localisation des données

**Niveau renforcé** : Privilégier les prestataires qualifiés par l'ANSSI.

---

## II - Connaître le système d'information

### Mesure 4 : Identifier les informations et serveurs les plus sensibles et maintenir un schéma du réseau

**Niveau : Standard**

L'entité doit :

- Identifier les informations et données les plus sensibles (données personnelles, propriété intellectuelle, données stratégiques)
- Identifier les serveurs qui les hébergent et les traitent
- Maintenir un schéma à jour du réseau indiquant les différentes zones, les équipements et les flux principaux

**Niveau renforcé** : Réaliser une classification formelle des données selon leur niveau de sensibilité.

### Mesure 5 : Disposer d'un inventaire exhaustif des comptes privilégiés et le maintenir à jour

**Niveau : Standard**

L'entité doit disposer d'un inventaire exhaustif et maintenu à jour de tous les comptes à privilèges (administrateurs systèmes, réseau, bases de données, applications métier). Cet inventaire doit préciser :

- L'identité du détenteur du compte
- Les privilèges associés
- La date de création et de dernière modification

**Niveau renforcé** : Utiliser une solution de gestion des accès privilégiés (PAM - Privileged Access Management).

### Mesure 6 : Organiser les procédures d'arrivée, de départ et de changement de fonction des utilisateurs

**Niveau : Standard**

Des procédures formalisées doivent définir les actions à réaliser lors de l'arrivée, du départ ou du changement de fonction d'un utilisateur :

- Création/modification/suppression des comptes
- Attribution/révocation des droits d'accès
- Récupération/distribution des équipements
- Formation/sensibilisation

### Mesure 7 : Autoriser la connexion au réseau de l'entité aux seuls équipements maîtrisés

**Niveau : Standard**

Seuls les équipements maîtrisés par l'entité (fournis et configurés par le service informatique) doivent être autorisés à se connecter au réseau interne. Les équipements personnels (BYOD) doivent être interdits ou, à défaut, isolés sur un réseau dédié sans accès aux ressources sensibles.

**Niveau renforcé** : Mettre en place un mécanisme de contrôle d'accès au réseau (NAC - Network Access Control).

---

## III - Authentifier et contrôler les accès

### Mesure 8 : Identifier nommément chaque personne accédant au système et distinguer les rôles utilisateur/administrateur

**Niveau : Standard**

Chaque utilisateur doit disposer d'un compte nominatif permettant de l'identifier. L'utilisation de comptes génériques doit être proscrite. Les rôles utilisateur et administrateur doivent être clairement distingués avec des comptes séparés.

**Niveau renforcé** : Implémenter une séparation stricte entre les comptes d'administration et les comptes utilisateurs, y compris sur des postes physiquement distincts.

### Mesure 9 : Attribuer les bons droits sur les ressources sensibles du système d'information

**Niveau : Standard**

Les droits d'accès aux ressources sensibles doivent être attribués selon le principe du moindre privilège : chaque utilisateur ne doit disposer que des droits strictement nécessaires à l'exercice de ses fonctions.

**Niveau renforcé** : Mettre en place des revues régulières des droits d'accès.

### Mesure 10 : Définir et vérifier des règles de choix et de dimensionnement des mots de passe

**Niveau : Standard**

Une politique de mots de passe doit être définie et appliquée techniquement. Elle doit préciser :

- La longueur minimale (au moins 12 caractères pour les comptes utilisateurs, 15 pour les comptes à privilèges)
- La complexité requise (majuscules, minuscules, chiffres, caractères spéciaux)
- La fréquence de renouvellement
- L'interdiction de réutilisation des anciens mots de passe

**Niveau renforcé** : Utiliser des phrases de passe de plus de 16 caractères.

### Mesure 11 : Protéger les mots de passe stockés sur les systèmes

**Niveau : Standard**

Les mots de passe ne doivent jamais être stockés en clair. Ils doivent être protégés par des fonctions de hachage robustes (bcrypt, Argon2, PBKDF2) avec salage individuel.

### Mesure 12 : Changer les éléments d'authentification par défaut sur les équipements et services

**Niveau : Standard**

Tous les mots de passe par défaut des équipements (routeurs, commutateurs, serveurs, applications) doivent être modifiés avant leur mise en production. Les comptes par défaut non utilisés doivent être désactivés ou supprimés.

### Mesure 13 : Privilégier lorsque c'est possible une authentification forte

**Niveau : Standard**

L'authentification forte (multi-facteurs) doit être privilégiée, en particulier pour :

- Les accès distants (VPN)
- Les comptes à privilèges
- Les applications sensibles

**Niveau renforcé** : Déployer l'authentification forte pour tous les utilisateurs.

---

## IV - Sécuriser les postes

### Mesure 14 : Mettre en place un niveau de sécurité minimal sur l'ensemble du parc informatique

**Niveau : Standard**

Une configuration de sécurité minimale doit être déployée sur tous les postes de travail :

- BIOS/UEFI protégé par mot de passe
- Démarrage sur le seul disque système
- Antivirus à jour avec analyse en temps réel
- Système d'exploitation et applications à jour
- Désactivation des services non nécessaires

### Mesure 15 : Se protéger des menaces relatives à l'utilisation de supports amovibles

**Niveau : Standard**

L'utilisation des supports amovibles (clés USB, disques externes) doit être encadrée :

- Interdiction des supports personnels
- Analyse antivirus systématique
- Chiffrement des supports contenant des données sensibles

**Niveau renforcé** : Désactiver l'exécution automatique et limiter l'utilisation des ports USB aux seuls périphériques autorisés.

### Mesure 16 : Utiliser un outil de gestion centralisée afin d'homogénéiser les politiques de sécurité

**Niveau : Standard**

Un outil de gestion centralisée (type Active Directory, SCCM, solution MDM) doit permettre de déployer et maintenir les configurations de sécurité de manière homogène sur l'ensemble du parc.

**Niveau renforcé** : Implémenter des stratégies de groupe (GPO) ou équivalent pour appliquer automatiquement les configurations de sécurité.

### Mesure 17 : Activer et configurer le pare-feu local des postes de travail

**Niveau : Standard**

Le pare-feu local de chaque poste de travail doit être activé et configuré pour bloquer les flux non explicitement autorisés, en particulier les flux entrants.

### Mesure 18 : Chiffrer les données sensibles transmises par voie Internet

**Niveau : Standard**

Toutes les données sensibles transmises via Internet doivent être chiffrées (TLS 1.2 minimum, TLS 1.3 recommandé). Les protocoles obsolètes (SSLv3, TLS 1.0, TLS 1.1) doivent être désactivés.

**Niveau renforcé** : Utiliser des certificats issus d'une autorité de certification de confiance et mettre en place HSTS.

---

## V - Sécuriser le réseau

### Mesure 19 : Segmenter le réseau et mettre en place un cloisonnement entre ces zones

**Niveau : Standard**

Le réseau doit être segmenté en zones de sécurité distinctes (utilisateurs, serveurs, administration, DMZ) avec filtrage des flux entre zones. Le cloisonnement doit respecter le principe du moindre privilège.

**Niveau renforcé** : Implémenter une architecture de type "Zero Trust" avec vérification systématique des accès.

### Mesure 20 : S'assurer de la sécurité des réseaux d'accès Wi-Fi et de la séparation des usages

**Niveau : Standard**

Les réseaux Wi-Fi doivent être sécurisés :

- Utilisation de WPA3 (ou WPA2 avec AES minimum)
- Séparation des réseaux (entreprise, invités, objets connectés)
- Authentification des utilisateurs (802.1X pour le réseau entreprise)

**Niveau renforcé** : Isoler complètement le réseau Wi-Fi invités du réseau interne.

### Mesure 21 : Utiliser des protocoles sécurisés dès qu'ils existent

**Niveau : Standard**

Les protocoles non sécurisés doivent être remplacés par leurs équivalents sécurisés :

- HTTP → HTTPS
- FTP → SFTP ou FTPS
- Telnet → SSH
- SMTP → SMTPS ou STARTTLS

### Mesure 22 : Mettre en place une passerelle d'accès sécurisé à Internet

**Niveau : Standard**

Une passerelle sécurisée doit contrôler tous les accès Internet :

- Pare-feu de nouvelle génération
- Filtrage d'URL
- Analyse antivirale des flux
- Inspection des flux chiffrés (si juridiquement possible)

### Mesure 23 : Cloisonner les services visibles depuis Internet du reste du système d'information

**Niveau : Standard**

Les services exposés sur Internet (serveurs web, messagerie) doivent être isolés dans une DMZ, avec filtrage strict des flux vers le réseau interne.

**Niveau renforcé** : Utiliser des proxys applicatifs et des WAF (Web Application Firewall).

### Mesure 24 : Protéger sa messagerie professionnelle

**Niveau : Standard**

La messagerie doit être protégée contre les menaces :

- Filtrage anti-spam et anti-phishing
- Analyse antivirale des pièces jointes
- Authentification des messages (SPF, DKIM, DMARC)

**Niveau renforcé** : Sandboxing des pièces jointes suspectes.

### Mesure 25 : Sécuriser les interconnexions réseau dédiées avec les partenaires

**Niveau : Standard**

Les interconnexions avec les partenaires (fournisseurs, clients) doivent être sécurisées :

- Filtrage strict des flux
- Chiffrement des communications (VPN IPsec)
- Monitoring des accès

### Mesure 26 : Contrôler et protéger l'accès aux salles serveurs et aux locaux techniques

**Niveau : Standard**

L'accès physique aux salles serveurs et locaux techniques doit être contrôlé :

- Contrôle d'accès par badge ou biométrie
- Journalisation des accès
- Vidéosurveillance

**Niveau renforcé** : Double authentification pour l'accès aux zones les plus sensibles.

---

## VI - Sécuriser l'administration

### Mesure 27 : Interdire l'accès à Internet depuis les postes ou serveurs utilisés pour l'administration du système d'information

**Niveau : Standard**

Les postes et serveurs d'administration ne doivent pas avoir accès à Internet pour limiter les risques de compromission. La navigation web et la messagerie doivent être effectuées depuis des postes distincts.

### Mesure 28 : Utiliser un réseau dédié et cloisonné pour l'administration du système d'information

**Niveau : Standard**

Un réseau d'administration dédié et cloisonné doit être mis en place pour les opérations d'administration. Ce réseau doit être physiquement ou logiquement séparé du réseau de production.

**Niveau renforcé** : Utiliser des postes d'administration dédiés (PAW - Privileged Access Workstation).

### Mesure 29 : Limiter au strict besoin opérationnel les droits d'administration sur les postes de travail

**Niveau : Standard**

Les utilisateurs ne doivent pas disposer de droits d'administration sur leurs postes de travail. L'installation de logiciels et les modifications de configuration doivent être réalisées par le service informatique.

**Niveau renforcé** : Implémenter une solution d'élévation de privilèges temporaires pour les cas exceptionnels.

---

## VII - Gérer le nomadisme

### Mesure 30 : Prendre des mesures de sécurisation physique des terminaux nomades

**Niveau : Standard**

Les équipements nomades (ordinateurs portables, tablettes, smartphones) doivent être sécurisés physiquement :

- Câble antivol
- Marquage des équipements
- Sensibilisation des utilisateurs à la vigilance

**Niveau renforcé** : Utiliser des filtres de confidentialité pour les écrans.

### Mesure 31 : Chiffrer les données sensibles, en particulier sur le matériel potentiellement perdable

**Niveau : Standard**

Les équipements nomades doivent disposer d'un chiffrement intégral du disque (BitLocker, FileVault, LUKS) pour protéger les données en cas de perte ou de vol.

**Niveau renforcé** : Utiliser des solutions de chiffrement qualifiées par l'ANSSI.

### Mesure 32 : Sécuriser la connexion réseau des postes utilisés en situation de nomadisme

**Niveau : Standard**

Les connexions depuis l'extérieur de l'entité doivent être sécurisées par VPN avec authentification forte. Les connexions sur des réseaux non maîtrisés (Wi-Fi publics, hôtels) doivent être évitées ou systématiquement tunnelisées.

### Mesure 33 : Adopter des politiques de sécurité dédiées aux terminaux mobiles

**Niveau : Standard**

Une politique de sécurité spécifique aux terminaux mobiles (smartphones, tablettes) doit être définie :

- Verrouillage par code PIN ou biométrie
- Chiffrement des données
- Effacement à distance en cas de perte/vol
- Gestion des applications autorisées

**Niveau renforcé** : Utiliser une solution EMM/MDM pour la gestion centralisée des terminaux mobiles.

---

## VIII - Maintenir le système d'information à jour

### Mesure 34 : Définir une politique de mise à jour des composants du système d'information

**Niveau : Standard**

Une politique de mise à jour doit être définie et appliquée :

- Veille sur les vulnérabilités (CERT-FR, éditeurs)
- Priorisation selon la criticité
- Procédure de test avant déploiement
- Délais de déploiement selon la criticité

**Niveau renforcé** : Mettre en place un processus de gestion des vulnérabilités avec outil de suivi.

### Mesure 35 : Anticiper la fin de la maintenance des logiciels et systèmes et limiter les adhérences logicielles

**Niveau : Standard**

L'entité doit :

- Tenir à jour un inventaire des logiciels et de leurs dates de fin de support
- Planifier les migrations vers les versions maintenues
- Limiter les dépendances à des technologies propriétaires ou obsolètes

**Niveau renforcé** : Isoler les systèmes qui ne peuvent pas être mis à jour (systèmes industriels, legacy).

---

## IX - Superviser, auditer, réagir

### Mesure 36 : Activer et configurer les journaux des composants les plus importants

**Niveau : Standard**

La journalisation doit être activée sur tous les composants critiques :

- Authentifications (succès et échecs)
- Actions d'administration
- Accès aux données sensibles
- Événements de sécurité (pare-feu, antivirus)

Les journaux doivent être centralisés et protégés contre la modification.

**Niveau renforcé** : Mettre en place un SIEM pour la corrélation et l'analyse des événements.

### Mesure 37 : Définir et appliquer une politique de sauvegarde des composants critiques

**Niveau : Standard**

Une politique de sauvegarde doit être définie :

- Identification des données critiques
- Fréquence des sauvegardes (quotidienne minimum)
- Rétention adaptée aux besoins
- Stockage sécurisé et externalisé
- Tests de restauration réguliers

**Niveau renforcé** : Mettre en place des sauvegardes hors ligne (air-gapped) pour se protéger des ransomwares.

### Mesure 38 : Procéder à des contrôles et audits de sécurité réguliers puis appliquer les actions correctives associées

**Niveau : Standard**

Des contrôles et audits de sécurité réguliers doivent être réalisés :

- Tests d'intrusion (au moins annuels)
- Audits de configuration
- Revues de code pour les applications critiques

Les vulnérabilités identifiées doivent faire l'objet d'un plan d'action de remédiation.

**Niveau renforcé** : Faire appel à des prestataires qualifiés PASSI par l'ANSSI.

### Mesure 39 : Désigner un référent en sécurité des systèmes d'information et le faire connaître auprès du personnel

**Niveau : Standard**

Un référent SSI (ou RSSI) doit être désigné et identifié par l'ensemble du personnel. Il est responsable de :

- La définition et le suivi de la politique de sécurité
- La sensibilisation des utilisateurs
- La gestion des incidents de sécurité
- La veille sécuritaire

**Niveau renforcé** : Constituer une équipe sécurité dédiée.

### Mesure 40 : Définir une procédure de gestion des incidents de sécurité

**Niveau : Standard**

Une procédure de gestion des incidents de sécurité doit être définie et testée :

- Détection et qualification des incidents
- Escalade et notification
- Réponse et confinement
- Éradication et récupération
- Analyse post-incident et amélioration

**Niveau renforcé** : Disposer d'une capacité de réponse à incidents (SOC/CSIRT) interne ou externalisée.

---

## X - Pour aller plus loin

### Mesure 41 : Mener une analyse de risques formelle

**Niveau : Renforcé**

Une analyse de risques formelle doit être menée selon une méthode reconnue (EBIOS Risk Manager, ISO 27005) pour identifier les risques spécifiques à l'entité et définir les mesures de sécurité complémentaires appropriées.

### Mesure 42 : Privilégier l'usage de produits et de services qualifiés par l'ANSSI

**Niveau : Renforcé**

Pour les besoins de sécurité, l'utilisation de produits et services qualifiés par l'ANSSI doit être privilégiée. Les catalogues de produits et prestataires qualifiés sont disponibles sur le site de l'ANSSI.

---

## Outil de suivi

### I - Sensibiliser et former

| # | Mesure | Standard | Renforcé |
|---|--------|----------|----------|
| 1 | Former les équipes opérationnelles à la sécurité des systèmes d'information | ☐ | - |
| 2 | Sensibiliser les utilisateurs aux bonnes pratiques élémentaires de sécurité informatique | ☐ | - |
| 3 | Maîtriser les risques de l'infogérance | ☐ | - |

### II - Connaître le système d'information

| # | Mesure | Standard | Renforcé |
|---|--------|----------|----------|
| 4 | Identifier les informations et serveurs les plus sensibles et maintenir un schéma du réseau | ☐ | ☐ |
| 5 | Disposer d'un inventaire exhaustif des comptes privilégiés et le maintenir à jour | ☐ | ☐ |
| 6 | Organiser les procédures d'arrivée, de départ et de changement de fonction des utilisateurs | ☐ | - |
| 7 | Autoriser la connexion au réseau de l'entité aux seuls équipements maîtrisés | ☐ | ☐ |

### III - Authentifier et contrôler les accès

| # | Mesure | Standard | Renforcé |
|---|--------|----------|----------|
| 8 | Identifier nommément chaque personne accédant au système et distinguer les rôles utilisateur/administrateur | ☐ | ☐ |
| 9 | Attribuer les bons droits sur les ressources sensibles du système d'information | ☐ | ☐ |
| 10 | Définir et vérifier des règles de choix et de dimensionnement des mots de passe | ☐ | ☐ |
| 11 | Protéger les mots de passe stockés sur les systèmes | ☐ | - |
| 12 | Changer les éléments d'authentification par défaut sur les équipements et services | ☐ | - |
| 13 | Privilégier lorsque c'est possible une authentification forte | ☐ | ☐ |

### IV - Sécuriser les postes

| # | Mesure | Standard | Renforcé |
|---|--------|----------|----------|
| 14 | Mettre en place un niveau de sécurité minimal sur l'ensemble du parc informatique | ☐ | - |
| 15 | Se protéger des menaces relatives à l'utilisation de supports amovibles | ☐ | ☐ |
| 16 | Utiliser un outil de gestion centralisée afin d'homogénéiser les politiques de sécurité | ☐ | ☐ |
| 17 | Activer et configurer le pare-feu local des postes de travail | ☐ | - |
| 18 | Chiffrer les données sensibles transmises par voie Internet | ☐ | ☐ |

### V - Sécuriser le réseau

| # | Mesure | Standard | Renforcé |
|---|--------|----------|----------|
| 19 | Segmenter le réseau et mettre en place un cloisonnement entre ces zones | ☐ | ☐ |
| 20 | S'assurer de la sécurité des réseaux d'accès Wi-Fi et de la séparation des usages | ☐ | ☐ |
| 21 | Utiliser des protocoles sécurisés dès qu'ils existent | ☐ | ☐ |
| 22 | Mettre en place une passerelle d'accès sécurisé à Internet | ☐ | - |
| 23 | Cloisonner les services visibles depuis Internet du reste du système d'information | ☐ | ☐ |
| 24 | Protéger sa messagerie professionnelle | ☐ | ☐ |
| 25 | Sécuriser les interconnexions réseau dédiées avec les partenaires | ☐ | - |
| 26 | Contrôler et protéger l'accès aux salles serveurs et aux locaux techniques | ☐ | ☐ |

### VI - Sécuriser l'administration

| # | Mesure | Standard | Renforcé |
|---|--------|----------|----------|
| 27 | Interdire l'accès à Internet depuis les postes ou serveurs utilisés pour l'administration du système d'information | ☐ | - |
| 28 | Utiliser un réseau dédié et cloisonné pour l'administration du système d'information | ☐ | - |
| 29 | Limiter au strict besoin opérationnel les droits d'administration sur les postes de travail | ☐ | ☐ |

### VII - Gérer le nomadisme

| # | Mesure | Standard | Renforcé |
|---|--------|----------|----------|
| 30 | Prendre des mesures de sécurisation physique des terminaux nomades | ☐ | ☐ |
| 31 | Chiffrer les données sensibles, en particulier sur le matériel potentiellement perdable | ☐ | ☐ |
| 32 | Sécuriser la connexion réseau des postes utilisés en situation de nomadisme | ☐ | - |
| 33 | Adopter des politiques de sécurité dédiées aux terminaux mobiles | ☐ | ☐ |

### VIII - Maintenir à jour le système d'information

| # | Mesure | Standard | Renforcé |
|---|--------|----------|----------|
| 34 | Définir une politique de mise à jour des composants du système d'information | ☐ | ☐ |
| 35 | Anticiper la fin de la maintenance des logiciels et systèmes et limiter les adhérences logicielles | ☐ | - |

### IX - Superviser, auditer, réagir

| # | Mesure | Standard | Renforcé |
|---|--------|----------|----------|
| 36 | Activer et configurer les journaux des composants les plus importants | ☐ | ☐ |
| 37 | Définir et appliquer une politique de sauvegarde des composants critiques | ☐ | ☐ |
| 38 | Procéder à des contrôles et audits de sécurité réguliers puis appliquer les actions correctives associées | ☐ | ☐ |
| 39 | Désigner un référent en sécurité des systèmes d'information et le faire connaître auprès du personnel | ☐ | ☐ |
| 40 | Définir une procédure de gestion des incidents de sécurité | ☐ | ☐ |

### X - Pour aller plus loin

| # | Mesure | Standard | Renforcé |
|---|--------|----------|----------|
| 41 | Mener une analyse de risques formelle | - | ☐ |
| 42 | Privilégier l'usage de produits et de services qualifiés par l'ANSSI | - | ☐ |

---

## Bibliographie

### Guides et méthodes

- **ANSSI**, *Bonnes pratiques pour l'acquisition et l'exploitation de noms de domaine*, guide, février 2015
  - https://www.ssi.gouv.fr/guide-bonnes-pratiques/

- **ANSSI**, *Expression des Besoins et Identification des Objectifs de Sécurité (EBIOS)*, méthode, janvier 2010
  - https://www.ssi.gouv.fr/ebios/

- **ANSSI**, *Guide de l'externalisation – Maîtriser les risques de l'infogérance*, guide, décembre 2010
  - https://www.ssi.gouv.fr/externalisation/

- **ANSSI**, *Maîtriser les risques de l'infogérance*, guide, décembre 2010
  - https://www.ssi.gouv.fr/infogerance/

- **ANSSI-CDSE**, *Passeport de conseils aux voyageurs*, bonnes pratiques, janvier 2010
  - https://www.ssi.gouv.fr/passeport-de-conseils-aux-voyageurs/

- **ANSSI**, *Charte d'utilisation des moyens informatiques et des outils numériques – Guide d'élaboration en 8 points clés pour les PME et ETI*, guide, juin 2017
  - https://www.ssi.gouv.fr/charte-utilisation-outils-numeriques/

### Notes techniques

- **ANSSI**, *Guide de définition d'une architecture de passerelle d'interconnexion sécurisée*, note technique, décembre 2011
  - https://www.ssi.gouv.fr/passerelle-interconnexion/

- **ANSSI**, *Recommandations de sécurité relatives aux mots de passe*, note technique, juin 2012
  - https://www.ssi.gouv.fr/mots-de-passe/

- **ANSSI**, *Recommandations pour la définition d'une politique de filtrage réseau d'un pare-feu*, note technique, mars 2013
  - https://www.ssi.gouv.fr/politique-filtrage-parefeu/

- **ANSSI**, *Recommandations de sécurité relatives aux réseaux Wi-Fi*, note technique, septembre 2013
  - https://www.ssi.gouv.fr/nt-wifi/

- **ANSSI**, *Recommandations pour la mise en œuvre d'une politique de restrictions logicielles sous Windows*, note technique, décembre 2013
  - https://www.ssi.gouv.fr/windows-restrictions-logicielles/

- **ANSSI**, *Recommandations de sécurité pour la mise en œuvre d'un système de journalisation*, note technique, décembre 2013
  - https://www.ssi.gouv.fr/journalisation/

- **ANSSI**, *Recommandations de sécurité relatives à Active Directory*, note technique, septembre 2014
  - https://www.ssi.gouv.fr/Active-Directory/

- **ANSSI**, *Recommandations relatives à l'administration sécurisée des systèmes d'information*, note technique, février 2015
  - https://www.ssi.gouv.fr/securisation-admin-si/

- **ANSSI**, *Recommandations de sécurité relatives aux ordiphones*, note technique, juillet 2015
  - https://www.ssi.gouv.fr/securisation-ordiphones/

- **ANSSI**, *Recommandations de sécurité relatives à IPsec pour la protection des flux réseau*, note technique, août 2015
  - https://www.ssi.gouv.fr/ipsec/

- **ANSSI**, *Recommandations de configuration d'un système GNU/Linux*, note technique, janvier 2016
  - https://www.ssi.gouv.fr/reco-securite-systeme-linux/

### Ressources en ligne

- **Site Web de l'ANSSI** : https://www.ssi.gouv.fr
- **Catalogue des produits et prestataires de service qualifiés** : https://www.ssi.gouv.fr/qualifications/
- **Twitter** : @ANSSI_FR - https://www.twitter.com/anssi_fr
- **CERT-FR** : https://www.cert.ssi.gouv.fr
- **CNIL** : https://www.cnil.fr

---

*ANSSI - 51, boulevard de la Tour-Maubourg - 75700 PARIS 07 SP*
*www.ssi.gouv.fr / communication@ssi.gouv.fr*
