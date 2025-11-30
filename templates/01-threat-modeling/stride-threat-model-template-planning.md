---
title: "Modèle de Menaces STRIDE - Phase de Planification"
template_version: "1.0.0"
constitutional_principle: "I - modélisation des menaces"
ssdlc_phase: "planification"
last_updated: "2025-01-15"
reviewers:
  - security-champion-team
  - architecture-team
description: "Modèle de modélisation des menaces basé sur STRIDE pour identifier les menaces de sécurité pendant la phase de planification. Aide les équipes à analyser systématiquement les menaces de Spoofing, Tampering, Repudiation, Information Disclosure, Denial of Service et Elevation of Privilege."
tags:
  - stride
  - threat-modeling
  - security-architecture
  - planning
difficulty: "intermédiaire"
estimated_time: "2-4 heures"
prerequisites:
  - "Diagramme d'architecture système de haut niveau"
  - "Compréhension des flux de données"
related_templates:
  - "attack-tree-template-planning.md"
  - "data-flow-diagram-template-design.md"
  - "../02-risk-analysis/risk-scoring-template-planning.md"
compliance_frameworks:
  - "OWASP SAMM"
  - "NIST SSDF"
  - "ISO 27001"
---

# Modèle de Menaces STRIDE - Phase de Planification

## Aperçu

### Objectif

Ce modèle vous guide dans la création d'un modèle de menaces basé sur STRIDE pour votre système. STRIDE est un cadre d'identification des menaces de sécurité en les catégorisant en six types : **S**poofing (Usurpation d'identité), **T**ampering (Altération), **R**epudiation (Répudiation), **I**nformation Disclosure (Divulgation d'informations), **D**enial of Service (Déni de service), et **E**levation of Privilege (Élévation de privilèges).

### Quand utiliser

- **Phase de Planification** : Avant le début de la conception détaillée
- **Revues d'Architecture** : Lors de l'évaluation de la sécurité de l'architecture système
- **Planification de Fonctionnalités** : Lors de l'ajout de nouvelles fonctionnalités aux systèmes existants
- **Exigences de Conformité** : Lorsque la modélisation des menaces de sécurité est requise

### Qui devrait utiliser

- Architectes de sécurité
- Architectes système
- Champions de sécurité
- Responsables techniques
- Équipes de sécurité produit

### Résultats attendus

- Liste complète des menaces potentielles
- Catégorisation des menaces par type STRIDE
- Priorisation des menaces par risque
- Contre-mesures identifiées pour les menaces hautement prioritaires

## Prérequis

Avant de compléter ce modèle, assurez-vous d'avoir :

- [ ] Diagramme d'architecture système de haut niveau
- [ ] Compréhension des flux de données à travers le système
- [ ] Liste des actifs système et des frontières de confiance
- [ ] Connaissance des mécanismes d'authentification et d'autorisation

## Instructions

### Étape 1 : Définir le périmètre système

Définir clairement ce que vous modélisez en termes de menaces.

### Étape 2 : Identifier les actifs

Lister les actifs de valeur nécessitant une protection.

### Étape 3 : Créer un diagramme de flux de données

Documenter comment les données se déplacent à travers votre système (ou référencer un diagramme existant).

### Étape 4 : Identifier les frontières de confiance

Marquer où les données franchissent les frontières de confiance (ex : utilisateur vers serveur, interne vers externe).

### Étape 5 : Appliquer STRIDE à chaque composant

Pour chaque composant/interaction, poser systématiquement les questions STRIDE.

### Étape 6 : Documenter les menaces

Enregistrer les menaces identifiées avec détails.

### Étape 7 : Prioriser les menaces

Utiliser la notation de risque pour prioriser les menaces.

### Étape 8 : Identifier les contre-mesures

Documenter les atténuations pour les menaces hautement prioritaires.

---

## 1. Périmètre système

### Nom du système

**Nom** : [Nom de Votre Système]

**Description** : [Brève description du système modélisé]

### Dans le périmètre

**Composants** :
- [Composant 1] - [Brève description]
- [Composant 2] - [Brève description]
- [Composant 3] - [Brève description]

**Fonctionnalités** :
- [Fonctionnalité 1]
- [Fonctionnalité 2]
- [Fonctionnalité 3]

**Interfaces** :
- [API externe]
- [Interface utilisateur]
- [Intégrations tierces]

### Hors périmètre

**Explicitement exclus** :
- [Composant/système exclu de ce modèle de menaces]
- [Justification de l'exclusion]

---

## 2. Actifs système

### Actifs de données

| Actif | Classification | Criticité | Localisation | Propriétaire |
|-------|----------------|-----------|--------------|--------------|
| Identifiants utilisateur | Confidentiel | Critique | DB Authentification | Équipe sécurité |
| Données personnelles utilisateur | Confidentiel | Élevé | DB Utilisateur | Équipe Produit |
| Informations de paiement | Confidentiel | Critique | Passerelle Paiement | Équipe Finance |
| [Nom de l'actif] | [Public/Interne/Confidentiel/Secret] | [Faible/Moyen/Élevé/Critique] | [Où stocké] | [Équipe responsable] |

### Actifs système

| Actif | Description | Criticité | Dépendances |
|-------|-------------|-----------|-------------|
| Service d'authentification | Valide l'identité utilisateur | Critique | DB Utilisateur, Store Session |
| Passerelle API | Route les requêtes externes | Élevé | Services backend |
| [Nom de l'actif] | [Description] | [Faible/Moyen/Élevé/Critique] | [Ce qui en dépend] |

### Frontières de confiance

**Frontière de confiance 1** : Internet → Web Application Firewall
- **Niveau de Menace** : Non fiable
- **Contrôles de sécurité** : WAF, Rate limiting, Validation d'entrée

**Frontière de confiance 2** : Application Web → Base de Données
- **Niveau de Menace** : Interne/Fiable
- **Contrôles de sécurité** : Authentification, Chiffrement en transit, Segmentation réseau

**Frontière de confiance 3** : [Nom de la frontière]
- **Niveau de Menace** : [Non fiable/Interne/Fiable]
- **Contrôles de sécurité** : [Lister les contrôles]

---

## 3. Diagramme de flux de données

### Diagramme

[Insérer ou référencer le diagramme de flux de données ici]

**Alternative** : Fournir une description textuelle si le diagramme n'est pas disponible :

```
Utilisateur → Navigateur → HTTPS → Load Balancer → Serveur Web → Serveur Application → Base de Données
                                                  ↓
                                            Service Auth
```

### Flux de données clés

**Flux 1** : Authentification utilisateur
- **Source** : Navigateur utilisateur
- **Destination** : Service d'authentification
- **Données** : Nom d'utilisateur, mot de passe
- **Protocole** : HTTPS
- **Frontière de confiance franchie** : Internet → Interne

**Flux 2** : [Nom du flux]
- **Source** : [Origine]
- **Destination** : [Cible]
- **Données** : [Quelles données]
- **Protocole** : [Comment transmis]
- **Frontière de confiance franchie** : [Quelle frontière]

---

## 4. Analyse des Menaces STRIDE

### Composant 1 : [Nom du composant, ex : "Service d'authentification"]

#### Spoofing (S) - Usurpation d'identité

**ID menace** : S01
- **Description** : L'attaquant usurpe l'identité d'un utilisateur légitime en volant le jeton de session
- **Vecteur d'Attaque** : Jeton de session transmis sur connexion non sécurisée ou stocké dans local storage vulnérable aux attaques XSS
- **Impact** : Accès non autorisé au compte utilisateur (Élevé)
- **Probabilité** : Moyenne
- **Score de risque** : Élevé (Impact × Probabilité)
- **Contrôles existants** : HTTPS pour le transport, cookies HttpOnly
- **Contre-mesures** :
  - Implémenter l'attribut SameSite pour les cookies
  - Ajouter des jetons CSRF
  - Implémenter timeout de session et ré-authentification pour actions sensibles
- **Statut** : Atténué / Accepté / Transféré / Évité

**ID menace** : S02
- **Description** : [Décrire la menace de spoofing]
- **Vecteur d'attaque** : [Comment cela pourrait-il se produire ?]
- **Impact** : [Que se passe-t-il en cas de succès ?] (Faible/Moyen/Élevé/Critique)
- **Probabilité** : [Quelle probabilité ?] (Faible/Moyen/Élevé)
- **Score de risque** : [Risque global]
- **Contrôles existants** : [Qu'est-ce qui est actuellement en place ?]
- **Contre-mesures** : [Que devrait-on faire ?]
- **Statut** : [Atténué/Accepté/Transféré/Évité]

#### Tampering (T) - Altération

**ID menace** : T01
- **Description** : L'attaquant modifie la requête d'authentification pour contourner la vérification de mot de passe
- **Vecteur d'attaque** : Attaque man-in-the-middle ou manipulation côté client
- **Impact** : Contournement de l'authentification (Critique)
- **Probabilité** : Faible (avec HTTPS) / Élevé (sans HTTPS)
- **Score de risque** : Critique
- **Contrôles existants** : HTTPS, Validation d'entrée
- **Contre-mesures** :
  - Appliquer HTTPS avec HSTS
  - Implémenter certificate pinning (applications mobiles)
  - Validation côté serveur de toutes les entrées
  - Implémenter la signature de requêtes
- **Statut** : Atténué

**ID menace** : T02
- **Description** : [Décrire la menace d'altération]
- **Vecteur d'attaque** : [Comment l'altération pourrait-elle se produire ?]
- **Impact** : [Conséquences]
- **Probabilité** : [Probabilité]
- **Score de risque** : [Niveau de risque]
- **Contrôles existants** : [Protections actuelles]
- **Contre-mesures** : [Atténuations supplémentaires]
- **Statut** : [Statut]

#### Repudiation (R) - Répudiation

**ID menace** : R01
- **Description** : L'utilisateur nie avoir effectué une action sensible (ex : changement de mot de passe, suppression de compte)
- **Vecteur d'attaque** : Absence de journalisation d'audit complète
- **Impact** : Impossible de prouver les actions utilisateur, violations de conformité (Moyen)
- **Probabilité** : Moyenne
- **Score de risque** : Moyen
- **Contrôles existants** : Logs applicatifs de base
- **Contre-mesures** :
  - Implémenter une journalisation d'audit complète (voir Principe VI)
  - Logger : ID utilisateur, horodatage, action, adresse IP, user agent
  - Implémenter protection d'intégrité des logs (logs signés, SIEM)
  - Conserver les logs selon exigences de conformité
- **Statut** : Planifié

**ID menace** : R02
- **Description** : [Décrire la menace de répudiation]
- **Vecteur d'attaque** : [Comment la répudiation pourrait-elle se produire ?]
- **Impact** : [Conséquences]
- **Probabilité** : [Probabilité]
- **Score de risque** : [Niveau de risque]
- **Contrôles existants** : [Journalisation/audit actuel]
- **Contre-mesures** : [Améliorations d'audit]
- **Statut** : [Statut]

#### Information Disclosure (I) - Divulgation d'informations

**ID menace** : I01
- **Description** : Les messages d'erreur sensibles révèlent les détails internes du système (structure base de données, chemins de fichiers)
- **Vecteur d'attaque** : Messages d'erreur verbeux en production
- **Impact** : Fuite d'informations aide les attaques ultérieures (Moyen)
- **Probabilité** : Élevée
- **Score de risque** : Élevé
- **Contrôles existants** : Aucun
- **Contre-mesures** :
  - Implémenter messages d'erreur génériques pour les utilisateurs
  - Logger les erreurs détaillées uniquement côté serveur
  - Désactiver les traces de pile en production
  - Assainir les réponses d'erreur
- **Statut** : Planifié

**ID menace** : I02
- **Description** : [Décrire la menace de divulgation d'informations]
- **Vecteur d'attaque** : [Comment les informations pourraient-elles fuiter ?]
- **Impact** : [Quelles informations pourraient être exposées ?]
- **Probabilité** : [Probabilité]
- **Score de risque** : [Niveau de risque]
- **Contrôles existants** : [Protection de données actuelle]
- **Contre-mesures** : [Protections supplémentaires]
- **Statut** : [Statut]

#### Denial of Service (D) - Déni de service

**ID menace** : D01
- **Description** : L'attaquant inonde l'endpoint d'authentification de requêtes, empêchant les utilisateurs légitimes de se connecter
- **Vecteur d'attaque** : Attaque par déni de service distribué (DDoS)
- **Impact** : Indisponibilité du service (Élevé)
- **Probabilité** : Moyenne
- **Score de risque** : Élevé
- **Contrôles existants** : Protection DDoS du fournisseur cloud
- **Contre-mesures** :
  - Implémenter rate limiting par adresse IP
  - Ajouter CAPTCHA pour tentatives échouées répétées
  - Utiliser CDN avec atténuation DDoS
  - Implémenter file d'attente et throttling des requêtes
- **Statut** : Partiellement atténué

**ID menace** : D02
- **Description** : [Décrire la menace de déni de service]
- **Vecteur d'attaque** : [Comment le service pourrait-il être refusé ?]
- **Impact** : [Impact sur la disponibilité]
- **Probabilité** : [Probabilité]
- **Score de risque** : [Niveau de risque]
- **Contrôles existants** : [Protections de disponibilité actuelles]
- **Contre-mesures** : [Mesures de résilience supplémentaires]
- **Statut** : [Statut]

#### Elevation of Privilege (E) - Élévation de privilèges

**ID menace** : E01
- **Description** : L'attaquant exploite une injection SQL pour obtenir des privilèges admin
- **Vecteur d'attaque** : Entrée non assainie dans la requête d'authentification
- **Impact** : Compromission complète du système (Critique)
- **Probabilité** : Faible (avec validation d'entrée appropriée)
- **Score de risque** : Élevé
- **Contrôles existants** : Requêtes paramétrées (ORM)
- **Contre-mesures** :
  - Utiliser exclusivement requêtes paramétrées / prepared statements
  - Implémenter comptes de base de données à moindre privilège
  - Ajouter Web Application Firewall (WAF) avec détection d'injection SQL
  - Analyse SAST/DAST régulière
- **Statut** : Atténué

**ID menace** : E02
- **Description** : [Décrire la menace d'escalade de privilèges]
- **Vecteur d'attaque** : [Comment les privilèges pourraient-ils être élevés ?]
- **Impact** : [Quel accès l'attaquant gagnerait-il ?]
- **Probabilité** : [Probabilité]
- **Score de risque** : [Niveau de risque]
- **Contrôles existants** : [Contrôles d'accès actuels]
- **Contre-mesures** : [Protections d'accès supplémentaires]
- **Statut** : [Statut]

---

### Composant 2 : [Nom du composant]

#### Spoofing (S)

**ID menace** : S03
- **Description** : [Description de la menace]
- [Continuer avec analyse complète comme ci-dessus]

#### Tampering (T)

**ID menace** : T03
- [Analyse]

#### Repudiation (R)

**ID menace** : R03
- [Analyse]

#### Information Disclosure (I)

**ID menace** : I03
- [Analyse]

#### Denial of Service (D)

**ID menace** : D03
- [Analyse]

#### Elevation of Privilege (E)

**ID menace** : E03
- [Analyse]

---

### Composant 3 : [Ajouter plus de composants selon les besoins]

[Répéter l'analyse STRIDE pour chaque composant]

---

## 5. Résumé des menaces

### Statistiques des menaces

| Catégorie STRIDE | Total Menaces | Critique | Élevé | Moyen | Faible |
|------------------|---------------|----------|-------|-------|--------|
| Spoofing         | 2             | 0        | 1     | 1     | 0      |
| Tampering        | 2             | 1        | 0     | 1     | 0      |
| Repudiation      | 2             | 0        | 0     | 2     | 0      |
| Information Disclosure | 2       | 0        | 1     | 1     | 0      |
| Denial of Service | 2            | 0        | 1     | 1     | 0      |
| Elevation of Privilege | 2       | 1        | 1     | 0     | 0      |
| **TOTAL**        | **12**        | **2**    | **4** | **6** | **0**  |

### Menaces hautement prioritaires

Lister les menaces nécessitant une atténuation immédiate (Critique et Élevé) :

1. **T01** - Altération de requête d'authentification (Critique) - Statut : Atténué
2. **E01** - Escalade de privilèges par injection SQL (Élevé) - Statut : Atténué
3. **S01** - Usurpation de jeton de session (Élevé) - Statut : Atténué
4. **I01** - Divulgation d'informations par messages d'erreur (Élevé) - Statut : Planifié
5. **D01** - DDoS sur endpoint d'authentification (Élevé) - Statut : Partiellement Atténué

---

## 6. Contre-mesures et atténuations

### Actions immédiates (menaces critiques/élevées)

| ID Menace | Contre-mesure | Propriétaire | Échéance | Statut |
|-----------|---------------|--------------|----------|--------|
| T01 | Appliquer HTTPS avec HSTS | Équipe DevOps | 2025-02-01 | En Cours |
| E01 | Revue de code pour requêtes paramétrées | Équipe Ingénierie | 2025-01-30 | Terminé |
| S01 | Implémenter cookies SameSite + jetons CSRF | Équipe sécurité | 2025-02-15 | Planifié |
| I01 | Assainir messages d'erreur | Équipe Ingénierie | 2025-02-10 | Planifié |
| D01 | Implémenter rate limiting | Équipe Plateforme | 2025-03-01 | En Cours |

### Améliorations futures (menaces moyennes/faibles)

| ID Menace | Contre-mesure | Propriétaire | Trimestre Cible | Statut |
|-----------|---------------|--------------|-----------------|--------|
| R01 | Journalisation d'audit complète | Équipe sécurité | T1 2025 | Planifié |
| [ID] | [Atténuation] | [Propriétaire] | [Quand] | [Statut] |

### Risques acceptés

| ID Menace | Risque | Justification de l'Acceptation | Approuvé Par | Date |
|-----------|--------|-------------------------------|--------------|------|
| [ID] | [Description] | [Pourquoi accepter ce risque ?] | [Nom/Rôle] | [Date] |

---

## 7. Liste de vérification

Utiliser cette liste pour vérifier l'exhaustivité de votre modèle de menaces :

- [ ] Périmètre système clairement défini
- [ ] Tous les composants dans le périmètre identifiés
- [ ] Actifs catalogués avec classification
- [ ] Frontières de confiance identifiées et documentées
- [ ] Diagramme de flux de données créé ou référencé
- [ ] Analyse STRIDE complétée pour tous les composants
- [ ] Chaque menace a : ID, description, vecteur d'attaque, impact, probabilité, score de risque
- [ ] Les menaces hautement prioritaires (Critique/Élevé) ont des contre-mesures identifiées
- [ ] Les contre-mesures ont des propriétaires et échéances
- [ ] Les risques acceptés ont une justification documentée et approbation
- [ ] Modèle de menaces revu par champion de sécurité
- [ ] Modèle de menaces lié au registre de risques (voir modèles 02-risk-analysis)

---

## 8. Revue et maintenance

### Historique des revues

| Version | Date | Réviseur | Modifications | Statut |
|---------|------|----------|---------------|--------|
| 1.0 | 2025-01-15 | [Nom du Champion de sécurité] | Modèle de menaces initial | Approuvé |
| [Version] | [Date] | [Nom] | [Ce qui a changé] | [Statut] |

### Prochaine date de revue

**Revue planifiée** : [Date, typiquement trimestrielle ou lors de changements significatifs]

### Déclencheurs de revue

Ré-exécuter ce modèle de menaces lorsque :
- [ ] Nouvelles fonctionnalités ajoutées au système
- [ ] Changements d'architecture
- [ ] Nouvelles intégrations ou dépendances tierces
- [ ] Incidents de sécurité liés à ce système
- [ ] Changements d'exigences de conformité
- [ ] Cycle de revue trimestrielle

---

## 9. Artefacts liés

### Références croisées

- **analyse de risques** : [Lien vers risk-register-template-all.md]
- **exigences de sécurité** : [Lien vers exigences de sécurité dérivées de ce modèle de menaces]
- **tests de sécurité** : [Lien vers plans de test abordant les menaces identifiées]
- **Architecture** : [Lien vers documentation d'architecture système]

### Ressources externes

- [OWASP Threat Modeling](https://owasp.org/www-community/Threat_Modeling)
- [Microsoft STRIDE](https://learn.microsoft.com/en-us/azure/security/develop/threat-modeling-tool-threats)
- [NIST SP 800-154 - Guide to Data-Centric System Threat Modeling](https://csrc.nist.gov/publications/detail/sp/800-154/draft)

---

## Annexe A : Référence rapide STRIDE

| Catégorie | Question | Exemples |
|-----------|----------|----------|
| **Spoofing** | L'attaquant peut-il se faire passer pour quelqu'un/quelque chose d'autre ? | Vol d'identifiants, détournement de session, IP spoofing |
| **Tampering** | L'attaquant peut-il modifier des données ou du code ? | Injection SQL, XSS, man-in-the-middle, manipulation de fichiers |
| **Repudiation** | L'attaquant peut-il nier ses actions ? | Logs manquants, transactions non signées, absence de piste d'audit |
| **Information Disclosure** | L'attaquant peut-il voir des données qu'il ne devrait pas voir ? | Injection SQL, directory traversal, erreurs verbeuses, données non chiffrées |
| **Denial of Service** | L'attaquant peut-il rendre le système indisponible ? | DDoS, épuisement de ressources, attaques de complexité algorithmique |
| **Elevation of Privilege** | L'attaquant peut-il obtenir des permissions non autorisées ? | Injection SQL, buffer overflow, contournement d'autorisation, défauts par défaut non sécurisés |

---

**Version du Modèle** : 1.0.0 | **Dernière Mise à Jour** : 2025-01-15 | **Prochaine Revue** : Trimestrielle
