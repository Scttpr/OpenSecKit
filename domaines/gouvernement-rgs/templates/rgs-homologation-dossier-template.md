---
title: "Modèle de Dossier d'Homologation RGS"
template_version: "1.0.0"
domain: "government-rgs"
constitutional_principles:
  - "I - Modélisation des Menaces"
  - "II - Analyse des Risques"
  - "III - Sécurité par Conception"
regulatory_references:
  - "Décret n°2010-112 du 2 février 2010"
  - "Arrêté du 13 juin 2014 portant création d'un traitement de données à caractère personnel relatif aux systèmes d'information"
  - "RGS v2.0 - Annexe B1 (Homologation)"
ssdlc_phase: "all"
difficulty: "avancé"
estimated_time: "40-80 heures (dossier complet)"
last_updated: "2025-01-19"
reviewers:
  - name: "Expert Sécurité ANSSI"
    role: "Conformité Réglementaire"
  - name: "RSSI (Responsable de la Sécurité des Systèmes d'Information)"
    role: "Autorité de Sécurité"
---

# Modèle de Dossier d'Homologation RGS

# Dossier d'Homologation de Sécurité (RGS)

## Objectif

Ce modèle fournit une structure complète pour créer un **Dossier d'Homologation RGS** (Dossier d'Homologation de Sécurité), qui est **obligatoire** pour les systèmes d'information des administrations françaises traitant des données sensibles.

**L'homologation** (accréditation de sécurité) est le processus formel par lequel une **Autorité d'Homologation** accepte les risques résiduels de sécurité d'un système après examen du dossier de sécurité.

## Quand Utiliser ce Modèle

- [ ] **Obligatoire** pour les systèmes d'information des administrations publiques françaises
- [ ] Requis pour les systèmes traitant des données personnelles (RGPD + RGS)
- [ ] Requis pour les systèmes avec classification RGS\* (standard), RGS\*\* (renforcé), ou RGS\*\*\* (élevé)
- [ ] Requis avant le déploiement en production
- [ ] Renouvellement requis tous les **3-5 ans** ou après des modifications majeures du système

## Contexte Réglementaire

### Base Légale

- **Décret n°2010-112** (2 février 2010) : Obligation d'homologation pour les systèmes gouvernementaux
- **Arrêté du 13 juin 2014** : Lignes directrices de mise en œuvre
- **RGS v2.0 - Annexe B1** : Méthodologie d'homologation

### Concepts Clés

| Terme | Définition |
|-------|------------|
| **Homologation** | Acceptation formelle des risques résiduels par l'autorité |
| **Autorité d'Homologation** | Décisionnaire (généralement directeur/secrétaire général) |
| **RSSI** | Responsable de la Sécurité des Systèmes d'Information - Prépare le dossier, conseille l'autorité |
| **Dossier d'Homologation** | Package complet de documentation de sécurité |
| **Décision d'Homologation** | Approbation formelle avec période de validité |
| **MCS** | Maintien en Condition de Sécurité (continu) |

---

## Vue d'Ensemble du Processus d'Homologation

```
┌─────────────────────────────────────────────────────────────────┐
│                    CYCLE DE VIE DE L'HOMOLOGATION                │
└─────────────────────────────────────────────────────────────────┘

Phase 1 : ÉTUDE
├── Modélisation des menaces (EBIOS Risk Manager)
├── Définition des exigences de sécurité
├── Classification RGS (RGS*, RGS**, RGS***)
└── Évaluation initiale des risques

Phase 2 : DOSSIER (Préparation du Dossier)
├── Description du système
├── Architecture de sécurité
├── Analyse des risques et plan de traitement
├── Preuves de mise en œuvre des mesures de sécurité
└── Documentation des risques résiduels

Phase 3 : DÉCISION
├── Le RSSI présente le dossier à l'Autorité d'Homologation
├── L'Autorité examine les risques résiduels
├── Décision d'Homologation signée
└── Période de validité définie (généralement 3-5 ans)

Phase 4 : MCS (Maintien Continu)
├── Surveillance continue
├── Gestion des correctifs
├── Réponse aux incidents
├── Revues de sécurité annuelles
└── Ré-homologation en cas de changements majeurs
```

---

## SECTION 1 : SYNTHÈSE EXÉCUTIVE

### 1.1 Identification du Système

| Champ | Valeur |
|-------|--------|
| **Nom du Système** | [ex. : "Plateforme Nationale des Services Publics Numériques"] |
| **Code Système** | [ex. : "PNSPN-2025"] |
| **Organisation Propriétaire** | [ex. : "Ministère de la Transformation et de la Fonction Publiques"] |
| **Entité Responsable** | [ex. : "Direction Interministérielle du Numérique (DINUM)"] |
| **RSSI** | [Nom, email, téléphone] |
| **Autorité d'Homologation** | [Nom, titre] |
| **Type de Système** | ☐ Application web ☐ Application mobile ☐ API ☐ Application bureau ☐ Infrastructure ☐ Autre : ______ |
| **Environnement de Déploiement** | ☐ Sur site ☐ Cloud gouvernemental ☐ Cloud privé ☐ Hybride |

### 1.2 Contexte Métier

**Énoncé de Mission** (2-3 phrases) :
> [Décrivez l'objectif du système et comment il soutient la mission de l'organisation]
>
> Exemple : "Cette plateforme fournit aux citoyens français un accès unifié à plus de 150 services publics (actes de naissance, déclarations d'impôts, prestations sociales) via un portail unique authentifié par FranceConnect. Elle dessert 12 millions d'utilisateurs actifs et traite 50 millions de transactions par an."

**Contexte Légal et Réglementaire** :

- [ ] RGPD (Règlement Général sur la Protection des Données)
- [ ] RGS (Référentiel Général de Sécurité) - Niveau : ☐ RGS\* ☐ RGS\*\* ☐ RGS\*\*\*
- [ ] RGI (Référentiel Général d'Interopérabilité)
- [ ] RGAA (Référentiel Général d'Amélioration de l'Accessibilité)
- [ ] NIS2 (si infrastructure critique)
- [ ] Autre : _______________

### 1.3 Classification RGS

**Détermination du Niveau de Sécurité** :

| Critère | Score | Justification |
|---------|-------|---------------|
| **Sensibilité des données** | 1-5 | [ex. : "4 - Traite des données de santé et informations fiscales (données personnelles sensibles)"] |
| **Nombre d'utilisateurs** | 1-5 | [ex. : "5 - 12 millions d'utilisateurs actifs"] |
| **Impact de l'indisponibilité** | 1-5 | [ex. : "4 - Critique pour l'accès des citoyens aux prestations sociales"] |
| **Exigences réglementaires** | 1-5 | [ex. : "5 - Conformité RGPD + RGS obligatoire"] |

**Score Total** : ____ / 20

**Classification RGS** :

- ☐ **RGS\*** (Standard) : Score 5-10, services e-gov basiques
- ☐ **RGS\*\*** (Renforcé) : Score 11-15, données sensibles, base utilisateurs importante
- ☐ **RGS\*\*\*** (Élevé) : Score 16-20, données classifiées, sécurité nationale

### 1.4 Besoins de Sécurité DICP

Basé sur l'Atelier 1 d'EBIOS Risk Manager - évaluer les besoins pour chaque fonction de sécurité :

| Besoin de Sécurité | Score (0-4) | Justification | Mesures Requises |
|--------------------|-------------|---------------|------------------|
| **Disponibilité** | [0-4] | [ex. : "4 - Disponibilité 99,9% requise, max 8h d'indisponibilité/an"] | [ex. : "Répartition de charge, PRA avec RTO 4h"] |
| **Intégrité** | [0-4] | [ex. : "4 - Modification de données = conséquences juridiques"] | [ex. : "Signatures numériques, journaux d'audit"] |
| **Confidentialité** | [0-4] | [ex. : "4 - Données de santé, données fiscales protégées par la loi"] | [ex. : "Chiffrement AES-256, RBAC"] |
| **Preuve** | [0-4] | [ex. : "3 - Traçabilité légale requise"] | [ex. : "Conservation des journaux 3 ans, journaux inaltérables"] |

**Légende** :

- **0** : Pas de besoin spécifique
- **1** : Faible (dégradation acceptable)
- **2** : Moyen (impact significatif)
- **3** : Élevé (impact grave)
- **4** : Très élevé (impact catastrophique)

---

## SECTION 2 : DESCRIPTION DU SYSTÈME

### 2.1 Périmètre et Limites du Système

**Dans le Périmètre** :

- [x] [Composant 1 : ex. : "Frontend web (React SPA)"]
- [x] [Composant 2 : ex. : "Passerelle API (Kong)"]
- [x] [Composant 3 : ex. : "Services backend (Java Spring Boot)"]
- [x] [Composant 4 : ex. : "Cluster base de données PostgreSQL"]
- [x] [Composant 5 : ex. : "Intégration SSO FranceConnect"]

**Hors Périmètre** :

- [ ] [ex. : "Processeur de paiement tiers (homologation séparée)"]
- [ ] [ex. : "Appareils utilisateurs (BYOD, non contrôlés)"]

**Dépendances Externes** :

1. **FranceConnect** (authentification) : ☐ Homologué ☐ En attente ☐ Non requis
2. **France Data Centers** (hébergement) : ☐ Homologué ☐ En attente ☐ Non requis
3. **API France** (interopérabilité) : ☐ Homologué ☐ En attente ☐ Non requis

### 2.2 Vue d'Ensemble de l'Architecture

**Diagramme d'Architecture Haut Niveau** :

```
┌─────────────────────────────────────────────────────────────────┐
│                        ZONE EXTERNE                              │
├─────────────────────────────────────────────────────────────────┤
│  Utilisateurs Internet (12M)  →  CDN (CloudFlare)  →  WAF       │
└────────────────────────────┬────────────────────────────────────┘
                             │
┌────────────────────────────▼────────────────────────────────────┐
│                          ZONE DMZ                                │
├─────────────────────────────────────────────────────────────────┤
│  Répartiteur de Charge (HAProxy)  →  Passerelle API (Kong)      │
│                             ↓                                    │
│                      FranceConnect                               │
│                      (OAuth 2.0 + OIDC)                          │
└────────────────────────────┬────────────────────────────────────┘
                             │
┌────────────────────────────▼────────────────────────────────────┐
│                      ZONE APPLICATION                            │
├─────────────────────────────────────────────────────────────────┤
│  Services Backend (Java Spring Boot)                             │
│    ├── Service Utilisateur                                      │
│    ├── Service Document                                         │
│    ├── Service Notification                                     │
│    └── Service Audit                                            │
└────────────────────────────┬────────────────────────────────────┘
                             │
┌────────────────────────────▼────────────────────────────────────┐
│                         ZONE DONNÉES                             │
├─────────────────────────────────────────────────────────────────┤
│  Cluster PostgreSQL (Primaire + 2 Réplicas)                      │
│  Redis (Cache de Session)                                        │
│  Stockage compatible S3 (Documents - chiffrés au repos)          │
└─────────────────────────────────────────────────────────────────┘
```

**Segmentation Réseau** :

- [ ] Zone externe (face à Internet, confiance minimale)
- [ ] Zone DMZ (proxy inverse, passerelle d'authentification)
- [ ] Zone application (services backend, pas d'accès Internet direct)
- [ ] Zone données (bases de données, stockage chiffré)
- [ ] Zone administration (accès admin, MFA requis, réseau séparé)

### 2.3 Diagramme de Flux de Données (DFD)

**Flux de Données Critiques** :

```
[Citoyen] --1--> [Navigateur] --2--> [WAF] --3--> [Passerelle API] --4--> [FranceConnect]
                                                    │
                                                    5
                                                    ↓
                                            [Service Utilisateur] --6--> [PostgreSQL]
                                                    │
                                                    7
                                                    ↓
                                            [Service Document] --8--> [Stockage S3]
                                                    │
                                                    9
                                                    ↓
                                            [Service Audit] --10--> [Journaux Audit]
```

**Contrôles de Sécurité des Flux de Données** :

| Flux # | Description | Confidentialité | Intégrité | Disponibilité |
|--------|-------------|-----------------|-----------|---------------|
| 1-3 | Citoyen → Passerelle API | TLS 1.3 | HTTPS + HSTS | CDN + Protection DDoS |
| 4 | Authentification FranceConnect | OAuth 2.0 + OIDC | Signature jeton (RS256) | SLA 99,9% |
| 6 | Service → Base de données | TLS 1.3, AES-256 au repos | Requêtes préparées (pas de SQLi) | Cluster PostgreSQL HA |
| 8 | Service → Stockage fichiers | Chiffrement AES-256-GCM | Hachage intégrité fichier (SHA-256) | Versionnement S3 + sauvegarde |
| 10 | Journalisation audit | TLS 1.3 vers SIEM | Journaux inaltérables (stockage WORM) | Stockage journaux redondant |

### 2.4 Stack Technologique

**Frontend** :

- Framework : React 18.2.0
- Langage : TypeScript 5.0
- Build : Vite 4.3
- Sécurité : Content-Security-Policy, SRI (Subresource Integrity)

**Backend** :

- Framework : Java Spring Boot 3.1.5
- Langage : Java 17 LTS
- API : RESTful (spécification OpenAPI 3.0)
- Authentification : Spring Security + OAuth2

**Base de Données** :

- Primaire : PostgreSQL 15.3 (chiffré avec LUKS)
- Cache : Redis 7.0 (chiffré en transit)
- Recherche : Elasticsearch 8.8 (chiffré au repos)

**Infrastructure** :

- Hébergement : OVHcloud Sovereign Cloud (France)
- Orchestration conteneurs : Kubernetes 1.27
- Service mesh : Istio 1.18 (mTLS entre services)
- Surveillance : Prometheus + Grafana
- Journalisation : Stack ELK (Elasticsearch, Logstash, Kibana)

---

## SECTION 3 : MODÉLISATION DES MENACES

### 3.1 Résumé EBIOS Risk Manager

**Référence** : Voir documentation EBIOS Risk Manager complète (domaines/gouvernement-rgs/templates/ebios-risk-manager-template.md)

**Synthèse Exécutive** :

- **Atelier 1** : 12 actifs essentiels identifiés, besoins DICP évalués
- **Atelier 2** : 5 catégories d'acteurs menaçants analysées (cybercriminels, États-nations, hacktivistes, initiés, concurrents)
- **Atelier 3** : 8 scénarios stratégiques identifiés, gravité/vraisemblance évaluées
- **Atelier 4** : 15 scénarios opérationnels (chemins d'attaque) documentés avec détails techniques
- **Atelier 5** : Plan de traitement des risques avec 22 mesures d'atténuation, budget de 92k€

**Top 3 des Scénarios Stratégiques** :

| Scénario | Événement Redouté | Objectif Visé | Gravité | Vraisemblance | Niveau de Risque |
|----------|-------------------|---------------|---------|---------------|------------------|
| **SS-01** | Violation de données (exfiltration données santé/fiscales) | Gain financier | 4 (Grave) | 3 (Probable) | Élevé |
| **SS-02** | Indisponibilité de service (DDoS, ransomware) | Atteinte à la réputation | 3 (Significatif) | 3 (Probable) | Moyen-Élevé |
| **SS-03** | Compromission de l'intégrité des données (documents frauduleux) | Objectifs politiques/activistes | 4 (Grave) | 2 (Possible) | Moyen-Élevé |

### 3.2 Analyse des Menaces STRIDE

**Référence** : Voir modèle de menaces STRIDE (templates/threat-modeling/stride-threat-model-template-planning.md)

**Résumé STRIDE** :

| Catégorie de Menace | # Menaces Identifiées | Menace à Plus Haut Risque | Statut d'Atténuation |
|---------------------|----------------------|---------------------------|----------------------|
| **Usurpation (Spoofing)** | 4 | Prise de contrôle de compte via bourrage d'identifiants | ✅ Atténué (MFA, limitation de débit) |
| **Altération (Tampering)** | 6 | Injection SQL dans la recherche de documents | ✅ Atténué (requêtes préparées, validation des entrées) |
| **Répudiation** | 2 | Actions admin non journalisées | ✅ Atténué (journalisation audit, conservation 3 ans) |
| **Divulgation d'Information** | 8 | Exposition de données personnelles via API | ✅ Atténué (RBAC, minimisation des données) |
| **Déni de Service** | 3 | Attaque DDoS sur points d'accès publics | ✅ Atténué (CDN, limitation de débit, auto-scaling) |
| **Élévation de Privilèges** | 5 | Escalade de privilèges via IDOR | ✅ Atténué (ABAC, autorisation au niveau objet) |

**Total des Menaces Identifiées** : 28
**Atténuées** : 26
**Acceptées** : 2 (gravité faible)

---

## SECTION 4 : ARCHITECTURE DE SÉCURITÉ

### 4.1 Fonctions de Sécurité RGS

#### Fonction 1 : Identification et Authentification

**Mécanisme Principal** : **FranceConnect** (homologué ANSSI, certifié RGS\*\*)

**Mise en Œuvre** :

- [x] **Intégration OAuth 2.0 + OpenID Connect** (OIDC)
- [x] **Fournisseurs d'Identité** : Les 9 IdP FranceConnect supportés (Impots.gouv.fr, Ameli.fr, La Poste, etc.)
- [x] **Niveau eIDAS** : Substantiel
- [x] **Gestion des Sessions** :
  - Délai d'inactivité : 30 minutes
  - Délai absolu : 8 heures
  - Cookies sécurisés (HttpOnly, Secure, SameSite=Strict)

**Authentification de Secours** (pour utilisateurs admin) :

- [x] **MFA (Authentification Multi-Facteurs)** : TOTP (RFC 6238) via application d'authentification
- [x] **Politique de Mot de Passe** :
  - Minimum 14 caractères (exigence RGS\*\*)
  - Complexité : majuscule, minuscule, chiffre, caractère spécial
  - Historique des mots de passe : 12 précédents
  - Facteur de travail bcrypt : 12
- [x] **Verrouillage de Compte** : 5 tentatives échouées → verrouillage 15 minutes

**Conformité** :

- ✅ RGS v2.0 Annexe B2 (Authentification)
- ✅ Recommandations ANSSI relatives à l'authentification multifacteur (2021)

#### Fonction 2 : Intégrité

**Intégrité des Données** :

- [x] **Niveau base de données** : Contraintes PostgreSQL, clés étrangères, triggers
- [x] **Niveau application** : Validation des entrées (OWASP ASVS V5), requêtes préparées
- [x] **Intégrité des fichiers** : Sommes de contrôle SHA-256 pour tous les documents téléversés
- [x] **Signatures numériques** : XMLDSig pour documents gouvernementaux officiels (algorithmes approuvés ANSSI)

**Intégrité des Communications** :

- [x] **TLS 1.3** pour toutes les communications (TLS 1.2 minimum)
- [x] **HSTS** (HTTP Strict Transport Security) avec max-age de 1 an
- [x] **Certificate Pinning** pour les points d'accès API critiques

**Intégrité du Code** :

- [x] **Commits signés** : Signature GPG pour tous les commits de production
- [x] **Vérification des dépendances** : npm audit, OWASP Dependency-Check
- [x] **Signature des images conteneur** : Cosign pour images Kubernetes

**Conformité** :

- ✅ RGS v2.0 Annexe B3 (Intégrité)
- ✅ Recommandations ANSSI de sécurité relatives à TLS (2021)

#### Fonction 3 : Confidentialité

**Chiffrement au Repos** :

- [x] **Base de données** : PostgreSQL Transparent Data Encryption (TDE) avec AES-256
- [x] **Stockage fichiers** : AES-256-GCM pour tous les documents
- [x] **Sauvegardes** : Chiffrées avec GPG (clé RSA-4096)
- [x] **Gestion des Clés** : HashiCorp Vault avec intégration HSM

**Chiffrement en Transit** :

- [x] **Externe** : TLS 1.3 (suite de chiffrement TLS_AES_256_GCM_SHA384)
- [x] **Interne** : mTLS (TLS mutuel) entre microservices via Istio service mesh
- [x] **Connexions base de données** : TLS 1.3 pour toutes les connexions

**Contrôle d'Accès** :

- [x] **RBAC (Contrôle d'Accès Basé sur les Rôles)** : 8 rôles définis (Citoyen, Admin, Auditeur, RSSI, etc.)
- [x] **ABAC (Contrôle d'Accès Basé sur les Attributs)** : Politiques dynamiques pour données sensibles
- [x] **Minimisation des Données** : Seuls les champs nécessaires exposés par rôle
- [x] **Pseudonymisation** : ID utilisateurs pseudonymisés dans analytique/journaux

**Algorithmes Cryptographiques** (approuvés ANSSI) :

- ✅ **Symétrique** : AES-256 (modes CBC, GCM)
- ✅ **Asymétrique** : RSA-4096, ECDSA P-384
- ✅ **Hachage** : SHA-256, SHA-384, SHA-512
- ✅ **Dérivation de Clé** : PBKDF2 (minimum 100 000 itérations)

**Conformité** :

- ✅ RGS v2.0 Annexe B4 (Confidentialité)
- ✅ Mécanismes cryptographiques ANSSI - Règles et recommandations (2021)

#### Fonction 4 : Traçabilité

**Journalisation d'Audit** :

- [x] **Événements journalisés** :
  - Authentification (succès/échec, flux FranceConnect)
  - Autorisation (accès accordé/refusé)
  - Accès aux données (opérations lecture/écriture DCP)
  - Actions administratives (création utilisateur, modifications de permissions)
  - Événements de sécurité (échecs de connexion, activité suspecte)
- [x] **Format des journaux** : JSON (journalisation structurée)
- [x] **Champs des journaux** : Horodatage (ISO 8601), ID Utilisateur, Action, Ressource, IP, User-Agent, Résultat
- [x] **Inaltérabilité** : Stockage WORM (Write-Once-Read-Many)
- [x] **Conservation** : **3 ans minimum** (exigence RGS)

**Intégration SIEM** :

- [x] **Outil** : Stack ELK (Elasticsearch, Logstash, Kibana)
- [x] **Alertes temps réel** : Événements de sécurité déclenchent alertes (PagerDuty)
- [x] **Règles de corrélation** : 15 règles pour détection d'anomalies (ex. : voyage impossible, escalade de privilèges)

**Conformité** :

- ✅ RGS v2.0 Annexe B5 (Traçabilité)
- ✅ Recommandations ANSSI relatives à la journalisation (2021)

### 4.2 Défense en Profondeur

**Couches de Sécurité** :

| Couche | Contrôles |
|--------|-----------|
| **Réseau** | Pare-feu (avec état), IDS/IPS (Snort), Segmentation réseau (VLANs), Protection DDoS (Cloudflare) |
| **Application** | WAF (ModSecurity), Validation des entrées (OWASP ASVS V5), Encodage de sortie, Jetons CSRF, En-têtes CSP |
| **Données** | Chiffrement au repos (AES-256), Chiffrement en transit (TLS 1.3), Contrôle d'accès (RBAC/ABAC), Masquage des données |
| **Identité** | MFA (TOTP), SSO FranceConnect, Gestion des sessions, Politique de mot de passe (14+ caractères) |
| **Surveillance** | SIEM (ELK), Détection d'intrusion (Wazuh), Scan de vulnérabilités (Nessus), Tests d'intrusion (annuels) |
| **Physique** | Contrôle d'accès datacenter (biométrique), Vidéosurveillance, HSM pour stockage des clés |

---

## SECTION 5 : ANALYSE ET TRAITEMENT DES RISQUES

### 5.1 Méthodologie de Scoring des Risques

**Formule de Risque** : `Score de Risque = Gravité × Vraisemblance`

**Échelle de Gravité (Impact sur DICP)** :

- **1** : Négligeable (pas d'impact significatif)
- **2** : Limité (dégradation mineure, récupération rapide)
- **3** : Significatif (interruption de service, exposition de données)
- **4** : Grave (impact étendu, conséquences réglementaires)
- **5** : Catastrophique (sécurité nationale, violation massive de données)

**Échelle de Vraisemblance** :

- **1** : Très improbable (< 5% probabilité annuelle)
- **2** : Improbable (5-25% probabilité annuelle)
- **3** : Possible (25-50% probabilité annuelle)
- **4** : Probable (50-75% probabilité annuelle)
- **5** : Très probable (> 75% probabilité annuelle)

**Matrice de Risque** :

```
Vraisemblance│  Gravité →
     ↓       │   1    2    3    4    5
─────────────┼────────────────────────────
     5       │   5   10   15   20   25  (Critique)
     4       │   4    8   12   16   20  (Élevé)
     3       │   3    6    9   12   15  (Moyen)
     2       │   2    4    6    8   10  (Faible)
     1       │   1    2    3    4    5   (Minimal)
```

**Seuils de Risque** :

- **20-25** : Critique → Décision de l'Autorité d'Homologation requise
- **12-19** : Élevé → Approbation RSSI + plan d'atténuation obligatoire
- **6-11** : Moyen → Mesures d'atténuation standard
- **3-5** : Faible → Surveiller et documenter
- **1-2** : Minimal → Accepter

### 5.2 Registre des Risques

**Risque #1 : Attaque par Bourrage d'Identifiants sur Comptes Admin**

| Champ | Valeur |
|-------|--------|
| **ID Risque** | R-001 |
| **Source de Menace** | Cybercriminels externes (bots automatisés) |
| **Vulnérabilité** | Mots de passe réutilisés provenant d'autres violations |
| **Impact** | Accès non autorisé au panneau admin → violation de données (données santé/fiscales) |
| **Impact DICP** | Confidentialité (4), Intégrité (3) |
| **Gravité** | 4 (Grave - violation RGPD, sanctions CNIL) |
| **Vraisemblance (Inhérente)** | 4 (Probable - 10M+ identifiants admin dans violations publiques) |
| **Risque Inhérent** | 4 × 4 = **16 (Élevé)** |
| **Contrôles Existants** | Politique de complexité de mot de passe, verrouillage de compte après 5 tentatives |
| **Mesures d'Atténuation** | **M-001** : Imposer MFA (TOTP) pour tous les comptes admin (couverture 100%)<br>**M-002** : Implémenter limitation de débit (max 10 tentatives de connexion/heure/IP)<br>**M-003** : Déployer détection de bourrage d'identifiants (API HaveIBeenPwned) |
| **Responsable** | Équipe Sécurité |
| **Échéance** | T1 2025 |
| **Coût** | 8 000€ (implémentation MFA + formation) |
| **Vraisemblance (Résiduelle)** | 2 (Improbable - MFA bloque 99,9% des attaques) |
| **Risque Résiduel** | 4 × 2 = **8 (Moyen)** |
| **Décision de Traitement** | ✅ **Réduire** (implémenter les atténuations) |

---

**Risque #2 : Injection SQL dans la Recherche de Documents**

| Champ | Valeur |
|-------|--------|
| **ID Risque** | R-002 |
| **Source de Menace** | Attaquants externes, scanners automatisés |
| **Vulnérabilité** | Concaténation de chaînes dans requêtes SQL (code legacy) |
| **Impact** | Compromission de base de données → 12M d'enregistrements citoyens exfiltrés |
| **Impact DICP** | Confidentialité (4), Intégrité (4) |
| **Gravité** | 5 (Catastrophique - notification ANSSI, amende CNIL 20M€) |
| **Vraisemblance (Inhérente)** | 3 (Possible - code legacy toujours en production) |
| **Risque Inhérent** | 5 × 3 = **15 (Moyen-Élevé)** |
| **Contrôles Existants** | WAF avec règles de détection SQLi |
| **Mesures d'Atténuation** | **M-004** : Refactoriser toutes les requêtes pour utiliser des requêtes préparées (couverture 100%)<br>**M-005** : Déployer SAST (SonarQube) en CI/CD pour détecter les patterns SQLi<br>**M-006** : Implémenter comptes BD avec moindres privilèges (lecture seule pour recherche) |
| **Responsable** | Équipe Développement |
| **Échéance** | T1 2025 (haute priorité) |
| **Coût** | 15 000€ (temps développeur + licence SAST) |
| **Vraisemblance (Résiduelle)** | 1 (Très improbable - défense en profondeur) |
| **Risque Résiduel** | 5 × 1 = **5 (Faible)** |
| **Décision de Traitement** | ✅ **Réduire** (implémenter les atténuations) |

---

**Risque #3 : Attaque DDoS sur Points d'Accès Publics**

| Champ | Valeur |
|-------|--------|
| **ID Risque** | R-003 |
| **Source de Menace** | Hacktivistes, concurrents, acteurs étatiques |
| **Vulnérabilité** | Bande passante limitée, pas d'atténuation DDoS |
| **Impact** | Indisponibilité de service → 12M d'utilisateurs incapables d'accéder aux services critiques |
| **Impact DICP** | Disponibilité (4) |
| **Gravité** | 4 (Grave - couverture médiatique nationale, pression politique) |
| **Vraisemblance (Inhérente)** | 3 (Possible - 2 attaques DDoS sur sites gouvernementaux français en 2024) |
| **Risque Inhérent** | 4 × 3 = **12 (Moyen-Élevé)** |
| **Contrôles Existants** | CDN (Cloudflare) avec protection DDoS basique |
| **Mesures d'Atténuation** | **M-007** : Passer à Cloudflare Enterprise (atténuation DDoS 100 Gbps)<br>**M-008** : Implémenter auto-scaling (Kubernetes HPA) pour pics de trafic<br>**M-009** : Déployer limitation de débit (Passerelle API) - 100 req/min/IP |
| **Responsable** | Équipe Infrastructure |
| **Échéance** | T2 2025 |
| **Coût** | 25 000€/an (abonnement Cloudflare Enterprise) |
| **Vraisemblance (Résiduelle)** | 2 (Improbable - disponibilité 99,99% avec atténuation) |
| **Risque Résiduel** | 4 × 2 = **8 (Moyen)** |
| **Décision de Traitement** | ✅ **Réduire** (implémenter les atténuations) |

---

**[Continuer pour tous les risques identifiés... généralement 10-20 risques pour les systèmes RGS\*\*]**

### 5.3 Résumé des Risques Résiduels

**Total des Risques Identifiés** : 18

| Niveau de Risque | Nombre | Traitement |
|------------------|--------|------------|
| **Critique (20-25)** | 0 | N/A |
| **Élevé (12-19)** | 2 | Réduire (plan d'atténuation approuvé) |
| **Moyen (6-11)** | 8 | Réduire (contrôles standard) |
| **Faible (3-5)** | 6 | Surveiller |
| **Minimal (1-2)** | 2 | Accepter |

**Budget Total d'Atténuation** : 92 000€ (voir Plan de Traitement des Risques)

---

## SECTION 6 : MESURES DE SÉCURITÉ

### 6.1 Mesures de Sécurité Techniques

**Catégorie : Contrôle d'Accès**

| ID Mesure | Description | Statut d'Implémentation | Preuve |
|-----------|-------------|------------------------|--------|
| T-001 | SSO FranceConnect (OAuth 2.0 + OIDC) | ✅ Implémenté | Fichiers de configuration, tests d'intégration |
| T-002 | MFA (TOTP) pour comptes admin | ✅ Implémenté (couverture 100%) | Captures panneau admin, journaux audit |
| T-003 | RBAC avec 8 rôles | ✅ Implémenté | Documents politique IAM, définitions de rôles |
| T-004 | Délai d'expiration session (30 min inactivité, 8h absolu) | ✅ Implémenté | Fichiers configuration, code gestion sessions |

**Catégorie : Cryptographie**

| ID Mesure | Description | Statut d'Implémentation | Preuve |
|-----------|-------------|------------------------|--------|
| T-010 | Chiffrement AES-256-GCM au repos (base de données, fichiers) | ✅ Implémenté | Configuration chiffrement, politiques gestion clés |
| T-011 | TLS 1.3 pour toutes les communications | ✅ Implémenté | Note A+ SSL Labs, config suite chiffrement |
| T-012 | mTLS entre microservices (Istio) | ✅ Implémenté | Configuration Istio, gestion certificats |
| T-013 | RSA-4096 pour signatures numériques | ✅ Implémenté | Infrastructure à clé publique, workflow signature |

**Catégorie : Surveillance et Journalisation**

| ID Mesure | Description | Statut d'Implémentation | Preuve |
|-----------|-------------|------------------------|--------|
| T-020 | Journalisation audit (conservation 3 ans) | ✅ Implémenté | Tableaux de bord SIEM, échantillons journaux |
| T-021 | Alertes SIEM temps réel | ✅ Implémenté | Règles d'alerte, playbooks réponse incidents |
| T-022 | Détection d'intrusion (Wazuh) | ✅ Implémenté | Configuration IDS, historique alertes |
| T-023 | Scan de vulnérabilités (hebdomadaire) | ✅ Implémenté | Rapports scan Nessus, suivi remédiation |

**Catégorie : Sécurité Applicative**

| ID Mesure | Description | Statut d'Implémentation | Preuve |
|-----------|-------------|------------------------|--------|
| T-030 | WAF (ModSecurity avec OWASP CRS) | ✅ Implémenté | Règles WAF, journaux attaques bloquées |
| T-031 | Validation des entrées (OWASP ASVS V5) | ✅ Implémenté | Rapports revue de code, bibliothèque validation |
| T-032 | Requêtes préparées (couverture 100%) | ✅ Implémenté | Résultats scan SAST (0 finding SQLi) |
| T-033 | En-têtes CSP (Content-Security-Policy) | ✅ Implémenté | Configuration en-têtes HTTP, journaux CSP report-uri |

### 6.2 Mesures de Sécurité Organisationnelles

| ID Mesure | Description | Statut d'Implémentation | Preuve |
|-----------|-------------|------------------------|--------|
| O-001 | Formation sensibilisation sécurité (annuelle, 100% personnel) | ✅ Implémenté | Registres formation, certificats d'achèvement |
| O-002 | Plan de réponse aux incidents (testé trimestriellement) | ✅ Implémenté | Document plan RI, rapports exercices sur table |
| O-003 | Processus de gestion des changements (2+ approbations) | ✅ Implémenté | Journaux demandes de changement, workflow approbation |
| O-004 | Vérifications antécédents pour utilisateurs admin | ✅ Implémenté | Registres vérification RH |
| O-005 | Évaluations sécurité tierces (pentest annuel) | ✅ Implémenté | Rapports tests d'intrusion (2024), preuves remédiation |

### 6.3 Matrice de Conformité

**Conformité RGS v2.0** :

| Article RGS | Exigence | Statut Conformité | Preuve |
|-------------|----------|-------------------|--------|
| **Annexe B2** | Authentification (FranceConnect ou équivalent) | ✅ Conforme | Intégration FranceConnect, journaux OAuth |
| **Annexe B3** | Intégrité (intégrité données, signatures numériques) | ✅ Conforme | Sommes de contrôle SHA-256, implémentation XMLDSig |
| **Annexe B4** | Confidentialité (AES-256, TLS 1.3) | ✅ Conforme | Configuration chiffrement, rapport SSL Labs |
| **Annexe B5** | Traçabilité (conservation journaux 3 ans) | ✅ Conforme | Politique conservation SIEM, journaux audit |

**Conformité RGPD** :

| Article RGPD | Exigence | Statut Conformité | Preuve |
|--------------|----------|-------------------|--------|
| **Art. 32** | Sécurité du traitement (chiffrement, pseudonymisation) | ✅ Conforme | AIPD, implémentation chiffrement |
| **Art. 33** | Notification de violation (72h à la CNIL) | ✅ Conforme | Plan réponse incidents, coordonnées CNIL |
| **Art. 35** | AIPD pour traitement à haut risque | ✅ Conforme | Document AIPD (domaines/rgpd/templates/gdpr-dpia-template.md) |

---

## SECTION 7 : TESTS ET VALIDATION

### 7.1 Résultats des Tests de Sécurité

**Tests d'Intrusion** (Annuels) :

| Date Test | Prestataire | Périmètre | Constats | Statut Remédiation |
|-----------|-------------|-----------|----------|-------------------|
| 2024-11-15 | Synacktiv (certifié ANSSI) | Application complète + infrastructure | 12 constats (2 Élevés, 5 Moyens, 5 Faibles) | ✅ 100% remédiés |

**Résumé des Constats** :

- **Gravité Élevée** :
  - H-01 : Limitation de débit manquante sur endpoint réinitialisation mot de passe → **Corrigé** : Implémenté 3 requêtes/heure/email
  - H-02 : Désérialisation non sécurisée dans gestion des sessions → **Corrigé** : Migration vers JWT avec vérification de signature
- **Gravité Moyenne** : [5 constats, tous remédiés]
- **Gravité Faible** : [5 constats, tous remédiés]

**SAST (Tests de Sécurité Applicative Statiques)** :

| Outil | Date Scan | Critique | Élevé | Moyen | Faible | Statut |
|-------|-----------|----------|-------|-------|--------|--------|
| SonarQube | 2025-01-15 | 0 | 0 | 3 | 12 | ✅ Critique/Élevé : 0 |

**DAST (Tests de Sécurité Applicative Dynamiques)** :

| Outil | Date Scan | Critique | Élevé | Moyen | Faible | Statut |
|-------|-----------|----------|-------|-------|--------|--------|
| OWASP ZAP | 2025-01-10 | 0 | 1 | 4 | 8 | ✅ Critique : 0, Élevé remédié |

**SCA (Analyse de Composition Logicielle)** :

| Outil | Date Scan | Critique | Élevé | Moyen | Faible | Statut |
|-------|-----------|----------|-------|-------|--------|--------|
| OWASP Dependency-Check | 2025-01-18 | 0 | 2 | 8 | 15 | ✅ Critique : 0, Élevé corrigé |

### 7.2 Audits de Conformité

**Audit RGPD** (CNIL) :

- **Date d'Audit** : 2024-09-20
- **Résultat** : ✅ Conforme (aucun constat)
- **Prochain Audit** : 2027-09-20 (cycle 3 ans)

**Audit RGS** (ANSSI) :

- **Date d'Audit** : 2024-10-15
- **Résultat** : ✅ Certification RGS\*\* accordée
- **Validité** : 2024-10-15 au 2027-10-15 (3 ans)
- **Numéro de Certification** : RGS-2024-FR-00123

---

## SECTION 8 : REPRISE D'ACTIVITÉ ET CONTINUITÉ D'ACTIVITÉ (PRA/PCA)

### 8.1 Stratégie de Sauvegarde

**Fréquence de Sauvegarde** :

- **Sauvegarde complète** : Hebdomadaire (dimanche 2h00 CET)
- **Sauvegarde incrémentale** : Quotidienne (2h00 CET)
- **Journaux de transactions** : Réplication temps réel (réplication streaming PostgreSQL)

**Conservation des Sauvegardes** :

- **Sauvegardes quotidiennes** : 30 jours
- **Sauvegardes hebdomadaires** : 1 an
- **Sauvegardes annuelles** : 7 ans (exigence légale)

**Chiffrement des Sauvegardes** :

- ✅ Toutes les sauvegardes chiffrées avec GPG (RSA-4096)
- ✅ Clés de chiffrement stockées en HSM (Hardware Security Module)
- ✅ Stockage de sauvegarde hors site (datacenters géographiquement séparés)

### 8.2 Reprise d'Activité

**Objectifs de Récupération** :

- **RTO (Objectif de Temps de Récupération)** : 4 heures
- **RPO (Objectif de Point de Récupération)** : 1 heure

**Sites de Reprise** :

- **Primaire** : OVH Roubaix (France)
- **Secondaire** : OVH Strasbourg (France) - distance 400 km
- **Sauvegarde** : OVH Gravelines (France)

**Tests de Reprise** :

- **Fréquence** : Trimestrielle
- **Date Dernier Test** : 2025-01-05
- **Résultat du Test** : ✅ RTO atteint (3,5 heures), RPO atteint (45 minutes)

### 8.3 Continuité d'Activité

**Fonctions Métier Critiques** :

1. **Authentification citoyen** (RTO : 2 heures, RPO : 30 minutes)
2. **Récupération de documents** (RTO : 4 heures, RPO : 1 heure)
3. **Soumission de demandes de service** (RTO : 8 heures, RPO : 2 heures)

**Mesures de Continuité** :

- [ ] Connexions Internet redondantes (2 FAI)
- [ ] Répartition de charge sur 3 zones de disponibilité
- [ ] Basculement automatique (contrôles de santé toutes les 30 secondes)
- [ ] Équipe de réponse aux incidents d'astreinte 24/7

---

## SECTION 9 : SÉCURITÉ DE LA CHAÎNE D'APPROVISIONNEMENT

### 9.1 Gestion des Risques Tiers

**Fournisseurs Critiques** :

| Fournisseur | Service | Statut RGS | Fin de Contrat | Niveau de Risque |
|-------------|---------|------------|----------------|------------------|
| **OVHcloud** | Hébergement infrastructure | ✅ Qualifié (Prestataire de Confiance) | 2026-12-31 | Faible |
| **FranceConnect** | Authentification | ✅ Homologué ANSSI | N/A (service gouvernemental) | Minimal |
| **Cloudflare** | CDN + Protection DDoS | ⚠️ Non-UE (société US, résidence données UE) | 2025-06-30 | Moyen |

**Exigences de Sécurité Fournisseurs** :

- [ ] Certification ISO 27001 (obligatoire)
- [ ] Conformité RGPD (Accord de Traitement de Données signé)
- [ ] Audits de sécurité annuels (rapports partagés)
- [ ] Notification d'incident (SLA 24 heures)

### 9.2 Chaîne d'Approvisionnement Logicielle

**Gestion des Dépendances** :

- [ ] SBOM (Software Bill of Materials) généré pour chaque release
- [ ] Scan de dépendances (OWASP Dependency-Check, Snyk)
- [ ] Conformité licences (pas de GPL dans code propriétaire)
- [ ] Surveillance des vulnérabilités (alertes GitHub Dependabot)

**Sécurité des Conteneurs** :

- [ ] Images de base uniquement depuis registres de confiance (Red Hat UBI, Alpine officiel)
- [ ] Signature des images avec Cosign
- [ ] Scan de vulnérabilités conteneurs (Trivy)
- [ ] Infrastructure immuable (pas de modifications au runtime)

---

## SECTION 10 : MAINTIEN EN CONDITION DE SÉCURITÉ (MCS)

### 10.1 Surveillance Continue

**Tableau de Bord des Métriques de Sécurité** :

| Métrique | Objectif | Actuel | Statut |
|----------|----------|--------|--------|
| **Disponibilité** | 99,9% | 99,95% | ✅ Dans l'objectif |
| **Tentatives de connexion échouées** | < 1000/jour | 450/jour | ✅ Normal |
| **Vulnérabilités critiques** | 0 | 0 | ✅ Dans l'objectif |
| **Systèmes non patchés** | 0 | 0 | ✅ Dans l'objectif |
| **Temps de réponse incidents** | < 4h | 2,5h (moy.) | ✅ Dépasse l'objectif |

**Outils de Surveillance** :

- **SIEM** : Stack ELK (surveillance 24/7)
- **Scanner de Vulnérabilités** : Nessus (scans hebdomadaires)
- **IDS** : Wazuh (détection d'intrusion temps réel)
- **APM** : New Relic (performance applicative)
- **Disponibilité** : Pingdom (surveillance externe, intervalles de 1 minute)

### 10.2 Gestion des Correctifs

**SLA de Patch** :

- **Vulnérabilités critiques** : 7 jours
- **Vulnérabilités élevées** : 30 jours
- **Vulnérabilités moyennes** : 90 jours
- **Vulnérabilités faibles** : Prochaine release trimestrielle

**Test des Correctifs** :

1. Test en environnement de développement (24 heures)
2. Test en environnement de préproduction (48 heures)
3. Déploiement en production (déploiement graduel, 10% → 50% → 100%)
4. Surveillance des régressions (période d'observation 48 heures)

**Correctifs Récents** :

- 2025-01-15 : PostgreSQL 15.3 → 15.5 (CVE-2024-XXXX corrigé)
- 2025-01-10 : Spring Boot 3.1.5 → 3.1.8 (CVE-2024-YYYY corrigé)

### 10.3 Revues de Sécurité Annuelles

**Planning des Revues** :

- **T1** : Tests d'intrusion (prestataire externe)
- **T2** : Audit de conformité RGPD (mené par DPO)
- **T3** : Revue de conformité RGS (menée par RSSI)
- **T4** : Exercice de continuité d'activité (exercice sur table + test technique PRA)

**Livrables des Revues** :

- Rapport de posture de sécurité (présenté à l'Autorité d'Homologation)
- Mises à jour du registre des risques
- Feuille de route sécurité pour l'année suivante
- Allocation budgétaire pour améliorations de sécurité

---

## SECTION 11 : GESTION DES INCIDENTS

### 11.1 Classification des Incidents

| Gravité | Définition | Temps de Réponse | Notification |
|---------|------------|------------------|--------------|
| **P0 (Critique)** | Violation de données, ransomware, panne complète | 15 minutes | ANSSI (24h), CNIL (72h), Autorité d'Homologation (immédiat) |
| **P1 (Élevé)** | Panne partielle, tentative de violation, vulnérabilité haute gravité | 1 heure | RSSI, Direction |
| **P2 (Moyen)** | Dégradation de performance, vulnérabilité moyenne gravité | 4 heures | RSSI |
| **P3 (Faible)** | Problèmes mineurs, vulnérabilité faible gravité | 24 heures | Équipe IT |

### 11.2 Plan de Réponse aux Incidents

**Phase 1 : Détection et Analyse** (0-1 heure)

- [ ] Alerte déclenchée (SIEM, IDS, signalement manuel)
- [ ] Classification de l'incident (P0-P3)
- [ ] Commandant d'incident désigné
- [ ] Début de collecte des preuves forensiques

**Phase 2 : Confinement** (1-4 heures)

- [ ] Isoler les systèmes affectés (segmentation réseau)
- [ ] Désactiver les comptes compromis
- [ ] Déployer correctifs/atténuations d'urgence
- [ ] Préserver journaux et preuves forensiques

**Phase 3 : Éradication** (4-24 heures)

- [ ] Supprimer malware/portes dérobées
- [ ] Corriger les vulnérabilités
- [ ] Réinitialiser les identifiants compromis
- [ ] Reconstruire les systèmes affectés (si nécessaire)

**Phase 4 : Récupération** (24-48 heures)

- [ ] Restaurer depuis sauvegardes propres
- [ ] Restauration graduelle du service (10% → 50% → 100%)
- [ ] Surveillance renforcée contre réinfection
- [ ] Vérifier l'intégrité du système

**Phase 5 : Post-Incident** (48 heures - 1 semaine)

- [ ] Rapport d'analyse de cause racine
- [ ] Réunion de retour d'expérience
- [ ] Mise à jour des mesures de sécurité
- [ ] Notifications réglementaires (ANSSI, CNIL si applicable)

### 11.3 Exigences de Notification

**Notification ANSSI** (si infrastructure critique) :

- **Délai** : 24 heures pour incidents significatifs
- **Méthode** : Portail de signalement d'incidents ANSSI
- **Contenu** : Nature de l'incident, impact, systèmes affectés, actions de réponse

**Notification CNIL** (si violation de données personnelles) :

- **Délai** : 72 heures après prise de connaissance
- **Méthode** : Formulaire en ligne CNIL
- **Contenu** : Catégories de données affectées, nombre de personnes concernées, conséquences de la violation, mesures d'atténuation

**Notification aux Utilisateurs** (si risque élevé pour les personnes concernées) :

- **Délai** : Sans délai excessif
- **Méthode** : Email, notification in-app, annonce publique
- **Contenu** : Description en langage clair, point de contact, actions recommandées

---

## SECTION 12 : DÉCISION D'HOMOLOGATION

### 12.1 Acceptation des Risques Résiduels

**Recommandation du RSSI** :

> "Après examen de l'analyse des risques et des mesures de sécurité mises en œuvre, je, [Nom du RSSI], Responsable de la Sécurité des Systèmes d'Information, recommande **l'homologation** du système pour une période de **3 ans** (2025-01-20 au 2028-01-20).
>
> **Justification** :
>
> - Tous les risques critiques et élevés ont été atténués à des niveaux acceptables (risque résiduel ≤ 8)
> - Conformité RGS\*\* obtenue (certification ANSSI #RGS-2024-FR-00123)
> - Conformité RGPD vérifiée (audit CNIL 2024-09-20, aucun constat)
> - Les mesures de sécurité sont alignées sur les bonnes pratiques ANSSI
> - Plan de maintien en condition de sécurité (MCS) en place
>
> **Conditions** :
>
> - Revues de sécurité trimestrielles (T1, T2, T3, T4 2025-2028)
> - Tests d'intrusion annuels (prochain : T4 2025)
> - Ré-homologation immédiate si changements majeurs du système
> - Allocation budgétaire pour mesures d'atténuation (92 000€ approuvés)
>
> **Risques résiduels nécessitant acceptation de l'Autorité** :
>
> - **R-001** (Bourrage d'identifiants) : Risque résiduel 8 (Moyen) - Accepté avec implémentation MFA
> - **R-003** (DDoS) : Risque résiduel 8 (Moyen) - Accepté avec Cloudflare Enterprise
>
> Signé : _________________ Date : 2025-01-20
> [Nom du RSSI], RSSI"

### 12.2 Décision de l'Autorité d'Homologation

**Modèle de Décision** :

> **DÉCISION D'HOMOLOGATION DE SÉCURITÉ**
>
> **Système** : [Nom du Système]
> **Date** : 2025-01-20
> **Autorité d'Homologation** : [Nom, Titre]
>
> Après examen du dossier d'homologation préparé par le RSSI, comprenant :
>
> - Description et architecture du système
> - Analyse EBIOS Risk Manager
> - Plan de traitement des risques et risques résiduels
> - Preuves de mise en œuvre des mesures de sécurité
> - Certifications de conformité (RGS\*\*, RGPD)
>
> Je, [Nom de l'Autorité], en ma qualité d'Autorité d'Homologation, par la présente :
>
> ☐ **APPROUVE** l'homologation de ce système
> ☐ **APPROUVE SOUS CONDITIONS** (préciser ci-dessous)
> ☐ **REPORTE** en attente de mesures de sécurité supplémentaires (préciser ci-dessous)
> ☐ **REJETTE** (préciser les raisons ci-dessous)
>
> **Période de Validité de l'Homologation** : 2025-01-20 au 2028-01-20 (3 ans)
>
> **Conditions** (si applicable) :
>
> - [ex. : "Implémenter M-007 (atténuation DDoS) d'ici T2 2025"]
> - [ex. : "Rapports trimestriels de posture de sécurité à présenter à l'Autorité"]
>
> **Déclencheurs de Ré-homologation** :
>
> - Changements architecturaux majeurs (ex. : migration cloud)
> - Introduction de nouvelles catégories de données sensibles
> - Incident de sécurité significatif (P0)
> - Fin de la période de validité (2028-01-20)
>
> **Risques Résiduels Acceptés** :
>
> - R-001 (Bourrage d'identifiants, risque résiduel 8) - Accepté
> - R-003 (DDoS, risque résiduel 8) - Accepté
>
> Signé : _________________ Date : 2025-01-20
> [Nom de l'Autorité], [Titre]

---

## SECTION 13 : ANNEXES

### Annexe A : Glossaire

| Terme | Définition |
|-------|------------|
| **Autorité d'Homologation** | Décisionnaire qui accepte les risques résiduels (généralement directeur/secrétaire général) |
| **RSSI** | Responsable de la Sécurité des Systèmes d'Information - Responsable de la sécurité, prépare le dossier d'homologation |
| **Homologation** | Processus formel d'approbation pour la sécurité du système |
| **MCS** | Maintien en Condition de Sécurité - Surveillance et mises à jour continues pour maintenir la posture de sécurité |
| **DICP** | Disponibilité, Intégrité, Confidentialité, Preuve - 4 propriétés de sécurité fondamentales dans le RGS |
| **ANSSI** | Agence Nationale de la Sécurité des Systèmes d'Information - Autorité réglementaire pour la sécurité IT gouvernementale |

### Annexe B : Documents de Référence

1. **EBIOS Risk Manager** (analyse complète) : `domaines/gouvernement-rgs/templates/ebios-risk-manager-template.md`
2. **Modèle de Menaces STRIDE** : `templates/threat-modeling/stride-threat-model-template-planning.md`
3. **AIPD RGPD** : `domaines/rgpd/templates/gdpr-dpia-template.md`
4. **Diagrammes d'Architecture de Sécurité** : `docs/architecture/` (fichiers Visio)
5. **Rapport de Test d'Intrusion** : `audits/2024-11-15-pentest-synacktiv.pdf`
6. **Certification RGS** : `certifications/RGS-2024-FR-00123.pdf`

### Annexe C : Informations de Contact

| Rôle | Nom | Email | Téléphone |
|------|-----|-------|-----------|
| **RSSI** | [Nom] | <rssi@example.gouv.fr> | +33 1 XX XX XX XX |
| **Autorité d'Homologation** | [Nom] | <autorite@example.gouv.fr> | +33 1 XX XX XX XX |
| **DPO (Délégué à la Protection des Données)** | [Nom] | <dpo@example.gouv.fr> | +33 1 XX XX XX XX |
| **Réponse aux Incidents** | Équipe Sécurité | <soc@example.gouv.fr> | +33 1 XX XX XX XX (24/7) |
| **Contact ANSSI** | - | <cert-fr@ssi.gouv.fr> | +33 1 71 75 84 68 |
| **Contact CNIL** | - | <cnil@cnil.fr> | +33 1 53 73 22 22 |

---

## Liste de Contrôle de Validation

**Avant de soumettre ce dossier à l'Autorité d'Homologation** :

- [ ] **Section 1** : Synthèse exécutive complète, classification RGS justifiée
- [ ] **Section 2** : Architecture système documentée avec diagrammes
- [ ] **Section 3** : EBIOS Risk Manager complété (5 ateliers)
- [ ] **Section 4** : 4 fonctions de sécurité RGS (DICP) implémentées et documentées
- [ ] **Section 5** : Registre des risques complet, tous les risques élevés atténués
- [ ] **Section 6** : Mesures de sécurité implémentées (techniques + organisationnelles)
- [ ] **Section 7** : Tests de sécurité complétés (pentest, SAST, DAST, SCA)
- [ ] **Section 8** : Plan PRA/PCA testé et validé
- [ ] **Section 9** : Risques tiers évalués, contrats en place
- [ ] **Section 10** : Plan MCS défini avec SLA de surveillance et patching
- [ ] **Section 11** : Plan de réponse aux incidents testé (exercice sur table)
- [ ] **Section 12** : Recommandation RSSI signée
- [ ] **Annexes** : Tous les documents de référence joints
- [ ] **Revue Finale** : 2+ champions sécurité ont revu le dossier
- [ ] **Revue Juridique** : Service juridique a approuvé (si requis)

---

## Exemple : Plateforme Télécoms Gouvernementale (Dossier Complété)

**Système** : Plateforme Nationale des Services Publics Numériques (PNSPN)
**Classification** : RGS\*\* (Renforcé)
**Utilisateurs** : 12 millions de citoyens actifs
**Sensibilité des Données** : Dossiers médicaux, informations fiscales, prestations sociales
**RSSI** : Marie Dupont (<marie.dupont@dinum.gouv.fr>)
**Autorité** : Directeur Général, Direction Interministérielle du Numérique

**Décision d'Homologation** : ✅ **APPROUVÉE** (2025-01-20 au 2028-01-20)

**Mesures de Sécurité Clés** :

- Authentification FranceConnect (100% des utilisateurs)
- Chiffrement AES-256 au repos, TLS 1.3 en transit
- Conservation des journaux d'audit 3 ans (SIEM)
- Tests d'intrusion annuels (Synacktiv)
- Disponibilité 99,95% (RTO 4h, RPO 1h)

**Risques Résiduels Acceptés** :

- R-001 : Bourrage d'identifiants (risque résiduel 8) - MFA implémenté
- R-003 : DDoS (risque résiduel 8) - Cloudflare Enterprise déployé

**Prochaine Revue** : 2026-01-20 (revue de sécurité annuelle)

---

**Version du Modèle** : 1.0.0
**Dernière Mise à Jour** : 2025-01-19
**Mainteneur** : SSDLC Toolkit - Domaine Gouvernement-RGS

Pour questions ou contributions, voir [Guide de Contribution](../../../CONTRIBUTING.txt).
