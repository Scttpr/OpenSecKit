---
title: "Plan de Continuité d'Activité (PCA) - Conforme RGS"
template_version: "1.0.0"
domain: "government-rgs"
constitutional_principle: "I - Threat Modeling, VII - Patch Management"
ssdlc_phase: "operations"
last_updated: "2025-01-15"
description: "Modèle de Plan de Continuité d'Activité pour systèmes gouvernementaux. Définit les procédures pour maintenir les activités critiques en cas de sinistre."
tags:
  - pca
  - business-continuity
  - rgs
  - disaster-recovery
  - resilience
difficulty: "advanced"
estimated_time: "4-8 heures"
prerequisites:
  - "Analyse EBIOS RM complétée"
  - "DICP évalués"
  - "Architecture système documentée"
auto_fill_sources:
  - ".osk/rgs-context.yaml"
  - "docs/context/meta.md"
  - "docs/security/risks/risk-register.yaml"
  - "docs/security/rgs/EBIOS-RM-*.md"
---

# Plan de Continuité d'Activité (PCA)

<!--
============================================================================
INSTRUCTIONS DE PRÉ-REMPLISSAGE AUTOMATIQUE

Ce template peut être partiellement pré-rempli par OSK depuis :
- .osk/rgs-context.yaml : Organisation, DICP, RTO/RPO, contacts
- docs/context/meta.md : Infrastructure, stack technique
- docs/security/risk-register.yaml : Scénarios de risques
- Étape 6 du wizard /osk-rgs : Fournisseurs et hébergement

Les sections marquées [AUTO] peuvent être extraites automatiquement.
Les sections marquées [MANUEL] nécessitent une saisie humaine.
============================================================================
-->

## Informations Documentaires

| Champ | Valeur |
|-------|--------|
| **Système** | [AUTO: systeme.nom depuis rgs-context.yaml] |
| **Version du document** | 1.0 |
| **Date de création** | [DATE] |
| **Dernière mise à jour** | [DATE] |
| **Prochaine revue** | [DATE + 1 an] |
| **Classification** | [AUTO: classification.classification_donnees] |
| **Statut** | BROUILLON / VALIDÉ / EN VIGUEUR |

### Historique des Versions

| Version | Date | Auteur | Modifications |
|---------|------|--------|---------------|
| 1.0 | [DATE] | [AUTEUR] | Création initiale |

### Approbations

| Rôle | Nom | Date | Signature |
|------|-----|------|-----------|
| RSSI | [AUTO: organisation.rssi.nom] | | |
| Autorité d'homologation | [AUTO: organisation.autorite_homologation.nom] | | |
| Direction Métier | [MANUEL] | | |

---

## 1. Objet et Périmètre

### 1.1 Objet du Document

Ce Plan de Continuité d'Activité (PCA) définit les dispositions permettant d'assurer la continuité des activités critiques du système **[AUTO: systeme.nom]** en cas de sinistre majeur affectant son fonctionnement normal.

### 1.2 Périmètre

**Système concerné** : [AUTO: systeme.nom]

**Description** : [AUTO: systeme.description]

**URL de production** : [AUTO: systeme.url_production]

**Composants inclus** :
<!-- [AUTO: Extraire de perimetre.inclus dans rgs-context.yaml] -->
- [ ] [Composant 1]
- [ ] [Composant 2]
- [ ] [Composant 3]

**Composants exclus** :
<!-- [AUTO: Extraire de perimetre.exclus dans rgs-context.yaml] -->
- [ ] [Composant exclu 1]
- [ ] [Composant exclu 2]

### 1.3 Objectifs de Continuité

<!-- [AUTO: Extraire de besoins_securite.disponibilite dans rgs-context.yaml] -->

| Métrique | Objectif | Justification |
|----------|----------|---------------|
| **RTO** (Recovery Time Objective) | [AUTO: besoins_securite.disponibilite.rto_heures] heures | Délai maximum acceptable d'interruption |
| **RPO** (Recovery Point Objective) | [AUTO: besoins_securite.disponibilite.rpo_heures] heures | Perte de données maximum acceptable |
| **SLA Disponibilité** | [AUTO: besoins_securite.disponibilite.sla_disponibilite] | Taux de disponibilité cible |
| **MTPD** (Maximum Tolerable Period of Disruption) | [MANUEL] heures | Durée maximale tolérable d'interruption |

---

## 2. Analyse d'Impact sur l'Activité (BIA)

### 2.1 Processus Métier Critiques

<!-- [MANUEL: Lister les processus métier supportés par le système] -->

| # | Processus Métier | Criticité | RTO | RPO | Dépendances |
|---|------------------|-----------|-----|-----|-------------|
| 1 | [Processus 1] | CRITIQUE | 4h | 1h | [Composants] |
| 2 | [Processus 2] | IMPORTANT | 8h | 4h | [Composants] |
| 3 | [Processus 3] | STANDARD | 24h | 12h | [Composants] |

**Échelle de criticité** :
- **CRITIQUE** : Interruption = impact légal, financier majeur ou atteinte grave aux usagers
- **IMPORTANT** : Interruption = dégradation significative du service
- **STANDARD** : Interruption = gêne limitée, alternatives disponibles

### 2.2 Ressources Critiques

#### 2.2.1 Infrastructure Technique

<!-- [AUTO: Extraire de meta.md et fournisseurs dans rgs-context.yaml] -->

| Ressource | Fournisseur | Criticité | Redondance | Contact Support |
|-----------|-------------|-----------|------------|-----------------|
| Hébergement | [AUTO: organisation.exploitant.nom] | CRITIQUE | [Oui/Non] | [Contact] |
| Base de données | [AUTO: depuis meta.md] | CRITIQUE | [Oui/Non] | [Contact] |
| Stockage fichiers | [MANUEL] | IMPORTANT | [Oui/Non] | [Contact] |
| CDN | [AUTO: depuis fournisseurs] | IMPORTANT | [Oui/Non] | [Contact] |
| DNS | [MANUEL] | CRITIQUE | [Oui/Non] | [Contact] |
| Email | [AUTO: depuis fournisseurs] | STANDARD | [Oui/Non] | [Contact] |

#### 2.2.2 Ressources Humaines Clés

<!-- [AUTO: Extraire de organisation dans rgs-context.yaml] -->

| Rôle | Titulaire | Suppléant | Compétences Critiques |
|------|-----------|-----------|----------------------|
| RSSI | [AUTO: organisation.rssi.nom] | [MANUEL] | Gestion de crise sécurité |
| Responsable Technique | [MANUEL] | [MANUEL] | Administration système |
| DBA | [MANUEL] | [MANUEL] | Restauration bases de données |
| DevOps | [MANUEL] | [MANUEL] | Déploiement, infrastructure |

### 2.3 Impacts par Durée d'Interruption

<!-- [MANUEL: Évaluer les impacts selon la durée] -->

| Durée | Impact Opérationnel | Impact Financier | Impact Réputationnel | Impact Légal |
|-------|---------------------|------------------|----------------------|--------------|
| 0-1h | Faible | Négligeable | Aucun | Aucun |
| 1-4h | Moyen | [Estimer] | Faible | Faible |
| 4-8h | Important | [Estimer] | Moyen | Moyen |
| 8-24h | Critique | [Estimer] | Important | Important |
| >24h | Catastrophique | [Estimer] | Critique | Critique |

---

## 3. Scénarios de Sinistre

### 3.1 Scénarios Identifiés

<!-- [AUTO: Extraire des scénarios stratégiques de l'EBIOS RM] -->

| # | Scénario | Probabilité | Impact | Stratégie |
|---|----------|-------------|--------|-----------|
| S1 | Panne hébergeur majeure | Faible | CRITIQUE | Basculement site secours |
| S2 | Cyberattaque (ransomware) | Moyenne | CRITIQUE | Restauration depuis backup |
| S3 | Panne base de données | Moyenne | IMPORTANT | Failover automatique |
| S4 | Indisponibilité service tiers critique | Moyenne | IMPORTANT | Mode dégradé |
| S5 | Perte de données (corruption, suppression) | Faible | CRITIQUE | Restauration backup |
| S6 | Incident de sécurité (compromission) | Moyenne | CRITIQUE | Isolation + reconstruction |

### 3.2 Détail des Scénarios Critiques

#### S1 : Panne Hébergeur Majeure

**Description** : Indisponibilité totale de l'infrastructure hébergée chez [AUTO: organisation.exploitant.nom].

**Causes possibles** :
- Incident datacenter (incendie, inondation, coupure électrique)
- Panne réseau majeure du fournisseur
- Faillite ou cessation de service du fournisseur

**Impact** :
- RTO estimé sans PCA : > 72h
- Perte potentielle : Totale si pas de backup externe

**Stratégie de continuité** : [Voir section 4.1]

---

#### S2 : Cyberattaque (Ransomware)

**Description** : Chiffrement des données et systèmes par un ransomware.

**Causes possibles** :
- Phishing réussi sur un administrateur
- Exploitation d'une vulnérabilité non patchée
- Compromission d'un fournisseur (supply chain)

**Impact** :
- RTO estimé sans PCA : Indéfini (si pas de backup sain)
- Perte potentielle : Données depuis dernier backup sain

**Stratégie de continuité** : [Voir section 4.2]

---

#### S3 : Panne Base de Données

**Description** : Corruption ou indisponibilité de la base de données principale.

**Causes possibles** :
- Défaillance matérielle
- Bug logiciel
- Erreur humaine (DROP TABLE)
- Saturation espace disque

**Impact** :
- RTO estimé avec failover : < 5 min
- RTO estimé sans failover : 1-4h (restauration)

**Stratégie de continuité** : [Voir section 4.3]

---

## 4. Stratégies de Continuité

### 4.1 Stratégie Hébergement

<!-- [AUTO: Adapter selon organisation.exploitant.type dans rgs-context.yaml] -->

#### Option A : Multi-AZ (Availability Zones)

**Applicable si** : Hébergeur cloud (AWS, OVH, Azure, GCP)

```
┌─────────────────────────────────────────────────────────────────┐
│                     ARCHITECTURE MULTI-AZ                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│    ┌─────────────┐         ┌─────────────┐                      │
│    │    AZ-1     │         │    AZ-2     │                      │
│    │  (Primary)  │◄───────►│  (Standby)  │                      │
│    │             │  Sync   │             │                      │
│    │  App + DB   │         │  App + DB   │                      │
│    └─────────────┘         └─────────────┘                      │
│           │                       │                              │
│           └───────────┬───────────┘                              │
│                       │                                          │
│              ┌────────▼────────┐                                │
│              │  Load Balancer  │                                │
│              │  (Health Check) │                                │
│              └─────────────────┘                                │
│                                                                  │
│  RTO : < 5 minutes (failover automatique)                       │
│  RPO : < 1 minute (réplication synchrone)                       │
└─────────────────────────────────────────────────────────────────┘
```

**Configuration requise** :
- [ ] Réplication synchrone de la base de données
- [ ] Load balancer avec health checks
- [ ] Stockage répliqué entre AZ
- [ ] Basculement automatique configuré

#### Option B : Site de Secours (DR Site)

**Applicable si** : On-premise ou besoin de souveraineté maximale

```
┌─────────────────────────────────────────────────────────────────┐
│                   ARCHITECTURE DR SITE                           │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│   Site Principal                    Site Secours                │
│   ┌─────────────┐                  ┌─────────────┐              │
│   │   Actif     │    Réplication   │   Passif    │              │
│   │             │ ═══════════════► │   (Cold)    │              │
│   │  App + DB   │    Asynchrone    │  App + DB   │              │
│   └─────────────┘                  └─────────────┘              │
│                                                                  │
│  RTO : 4-8 heures (activation manuelle)                         │
│  RPO : 1-4 heures (réplication asynchrone)                      │
└─────────────────────────────────────────────────────────────────┘
```

**Configuration requise** :
- [ ] Site secondaire géographiquement distant (> 100 km)
- [ ] Réplication asynchrone des données
- [ ] Procédure d'activation documentée
- [ ] Tests de basculement semestriels

### 4.2 Stratégie Sauvegarde et Restauration

<!-- [AUTO: Adapter selon les fournisseurs identifiés] -->

#### Politique de Sauvegarde

| Type | Fréquence | Rétention | Stockage | Chiffrement |
|------|-----------|-----------|----------|-------------|
| Complète | Hebdomadaire | 3 mois | [Fournisseur backup] | AES-256 |
| Incrémentale | Quotidienne | 1 mois | [Fournisseur backup] | AES-256 |
| Transaction logs | Toutes les 15 min | 7 jours | [Fournisseur backup] | AES-256 |
| Snapshots infra | Quotidien | 2 semaines | [Cloud provider] | AES-256 |

#### Règle 3-2-1

- **3** copies des données (production + 2 backups)
- **2** types de support différents (disque + objet storage)
- **1** copie hors site (autre région/datacenter)

#### Procédure de Restauration

```
┌─────────────────────────────────────────────────────────────────┐
│            ARBRE DE DÉCISION RESTAURATION                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  Incident détecté                                               │
│        │                                                        │
│        ▼                                                        │
│  ┌─────────────┐                                                │
│  │ Données     │──► OUI ──► Restaurer dernier backup sain      │
│  │ corrompues? │           (vérifier intégrité avant)          │
│  └─────────────┘                                                │
│        │ NON                                                    │
│        ▼                                                        │
│  ┌─────────────┐                                                │
│  │ Infra       │──► OUI ──► Restaurer snapshot infra           │
│  │ compromise? │           ou reconstruire depuis IaC          │
│  └─────────────┘                                                │
│        │ NON                                                    │
│        ▼                                                        │
│  ┌─────────────┐                                                │
│  │ Service     │──► OUI ──► Failover vers site secours         │
│  │ tiers KO?   │           ou activer mode dégradé             │
│  └─────────────┘                                                │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### 4.3 Stratégie Mode Dégradé

**Objectif** : Maintenir les fonctions essentielles si une restauration complète est impossible rapidement.

| Fonction | Mode Normal | Mode Dégradé | Délai Activation |
|----------|-------------|--------------|------------------|
| Authentification | [FranceConnect + local] | [Local uniquement] | Immédiat |
| Consultation données | Temps réel | Cache + lecture seule | 15 min |
| Création/modification | Disponible | Formulaire hors-ligne | 30 min |
| Notifications | Email + push | Email différé | Immédiat |
| Reporting | Temps réel | Différé J+1 | N/A |

---

## 5. Organisation de Crise

### 5.1 Cellule de Crise

<!-- [AUTO: Extraire de organisation dans rgs-context.yaml] -->

```
┌─────────────────────────────────────────────────────────────────┐
│                    CELLULE DE CRISE                              │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│                 ┌──────────────────┐                            │
│                 │ Directeur de     │                            │
│                 │ Crise            │                            │
│                 │ [AUTO: autorite_ │                            │
│                 │  homologation]   │                            │
│                 └────────┬─────────┘                            │
│                          │                                       │
│         ┌────────────────┼────────────────┐                     │
│         │                │                │                     │
│         ▼                ▼                ▼                     │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐                │
│  │ Resp.      │  │ Resp.      │  │ Resp.      │                │
│  │ Technique  │  │ Sécurité   │  │ Comm.      │                │
│  │ [MANUEL]   │  │ [RSSI]     │  │ [MANUEL]   │                │
│  └────────────┘  └────────────┘  └────────────┘                │
│         │                │                │                     │
│         ▼                ▼                ▼                     │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐                │
│  │ Équipe     │  │ Équipe     │  │ Support    │                │
│  │ Ops/DevOps │  │ SOC/CERT   │  │ Utilisateur│                │
│  └────────────┘  └────────────┘  └────────────┘                │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### 5.2 Contacts d'Urgence

<!-- [AUTO: Extraire de organisation dans rgs-context.yaml + fournisseurs] -->

| Rôle | Nom | Téléphone | Email | Astreinte |
|------|-----|-----------|-------|-----------|
| Directeur de crise | [AUTO: autorite_homologation.nom] | [MANUEL] | [AUTO: email] | 24/7 |
| RSSI | [AUTO: organisation.rssi.nom] | [MANUEL] | [AUTO: email] | 24/7 |
| Responsable Technique | [MANUEL] | [MANUEL] | [MANUEL] | 24/7 |
| Support Hébergeur | [AUTO: exploitant.nom] | [MANUEL] | [MANUEL] | 24/7 |
| Support BDD | [MANUEL] | [MANUEL] | [MANUEL] | HO |
| ANSSI (si incident majeur) | CERT-FR | 01 71 75 84 68 | cert-fr@ssi.gouv.fr | 24/7 |

### 5.3 Niveaux d'Alerte

| Niveau | Critères | Actions | Qui décide |
|--------|----------|---------|------------|
| **VERT** | Fonctionnement normal | Surveillance standard | Automatique |
| **JAUNE** | Dégradation partielle, RTO non menacé | Mobilisation équipe technique | Resp. Technique |
| **ORANGE** | Interruption partielle, RTO < 50% consommé | Activation cellule de crise | RSSI |
| **ROUGE** | Interruption totale, RTO menacé | Cellule de crise complète + Direction | Directeur de crise |

---

## 6. Procédures de Continuité

### 6.1 Procédure d'Activation du PCA

```
┌─────────────────────────────────────────────────────────────────┐
│           PROCÉDURE D'ACTIVATION PCA                             │
├─────────────────────────────────────────────────────────────────┤
│                                                                  │
│  T+0    │ Détection incident (monitoring, alerte, signalement)  │
│         │                                                        │
│  T+15m  │ Qualification par équipe technique                    │
│         │ → Incident mineur ? → Procédure standard              │
│         │ → Incident majeur ? → Continuer ↓                     │
│         │                                                        │
│  T+30m  │ Évaluation impact et estimation RTO                   │
│         │ → RTO tenable ? → Résolution standard                 │
│         │ → RTO menacé ? → Activation PCA ↓                     │
│         │                                                        │
│  T+45m  │ Notification cellule de crise                         │
│         │ • Directeur de crise                                  │
│         │ • RSSI                                                │
│         │ • Responsable technique                               │
│         │                                                        │
│  T+1h   │ Réunion cellule de crise                              │
│         │ • Choix stratégie (basculement, restauration, dégradé)│
│         │ • Attribution des rôles                               │
│         │ • Communication initiale                              │
│         │                                                        │
│  T+Xh   │ Exécution stratégie choisie                           │
│         │ [Voir procédures spécifiques sections 6.2-6.4]        │
│         │                                                        │
│  T+RTO  │ Vérification reprise service                          │
│         │ • Tests fonctionnels                                  │
│         │ • Validation données                                  │
│         │ • Communication reprise                               │
│         │                                                        │
│  Post   │ Retour d'expérience (REX)                             │
│         │ • Analyse causes                                      │
│         │ • Mise à jour PCA si nécessaire                       │
│                                                                  │
└─────────────────────────────────────────────────────────────────┘
```

### 6.2 Procédure : Basculement Site Secours

**Déclencheur** : Indisponibilité site principal > 1h ou > 50% RTO

**Prérequis** :
- [ ] Site secours opérationnel (vérifié lors du dernier test)
- [ ] Réplication à jour (vérifier timestamp dernier sync)
- [ ] Accès réseau au site secours confirmé

**Étapes** :

| # | Action | Responsable | Durée | Vérification |
|---|--------|-------------|-------|--------------|
| 1 | Confirmer indisponibilité site principal | Ops | 5 min | Ping, health checks |
| 2 | Activer site secours | DevOps | 15 min | Services UP |
| 3 | Vérifier intégrité données | DBA | 30 min | Checksums, counts |
| 4 | Basculer DNS | DevOps | 5 min | Propagation DNS |
| 5 | Tests fonctionnels | QA | 30 min | Smoke tests |
| 6 | Communication reprise | Comm | 10 min | Email, status page |

**Rollback** : Si site principal redevient disponible et données intègres, planifier retour arrière en heures creuses.

### 6.3 Procédure : Restauration Backup

**Déclencheur** : Corruption de données ou attaque ransomware

**Prérequis** :
- [ ] Identifier le dernier backup sain (avant incident)
- [ ] Environnement de restauration isolé disponible
- [ ] Espace disque suffisant

**Étapes** :

| # | Action | Responsable | Durée | Vérification |
|---|--------|-------------|-------|--------------|
| 1 | Isoler l'environnement compromis | SecOps | 15 min | Coupure réseau |
| 2 | Identifier backup sain | DBA | 30 min | Logs, dates |
| 3 | Provisionner environnement propre | DevOps | 1h | Infra neuve |
| 4 | Restaurer backup | DBA | 2-4h | Selon volume |
| 5 | Vérifier intégrité données | DBA + Métier | 1h | Échantillonnage |
| 6 | Appliquer patches/correctifs | SecOps | 1h | Vuln corrigées |
| 7 | Tests de sécurité | SecOps | 1h | Scan vulnérabilités |
| 8 | Basculer trafic | DevOps | 15 min | DNS + LB |
| 9 | Monitoring renforcé | Ops | Continu | 72h post-incident |

### 6.4 Procédure : Activation Mode Dégradé

**Déclencheur** : Service tiers critique indisponible ou restauration complète impossible rapidement

**Étapes** :

| # | Action | Responsable | Durée | Vérification |
|---|--------|-------------|-------|--------------|
| 1 | Identifier services impactés | Ops | 10 min | Status fournisseurs |
| 2 | Activer fallbacks configurés | DevOps | 15 min | Feature flags |
| 3 | Communication utilisateurs | Comm | 10 min | Bannière, email |
| 4 | Surveillance renforcée | Ops | Continu | Alerting |
| 5 | Retour mode normal dès service rétabli | DevOps | 15 min | Tests |

---

## 7. Communication de Crise

### 7.1 Modèles de Communication

#### Communication Interne (Équipes)

```
OBJET: [ALERTE PCA] - [NOM_SYSTÈME] - Niveau [JAUNE/ORANGE/ROUGE]

Équipe,

Un incident affecte actuellement [NOM_SYSTÈME].

SITUATION:
- Nature de l'incident : [Description]
- Impact : [Services affectés]
- Début : [Heure]
- RTO cible : [Heures]

ACTIONS EN COURS:
- [Action 1]
- [Action 2]

PROCHAINE COMMUNICATION: [Heure]

Cellule de crise
```

#### Communication Externe (Utilisateurs)

```
OBJET: [SERVICE] - Interruption temporaire

Chers utilisateurs,

Le service [NOM_SERVICE] est actuellement indisponible en raison
d'un incident technique.

Nos équipes sont mobilisées pour rétablir le service dans les
meilleurs délais.

Reprise estimée : [Heure]

Nous vous prions de nous excuser pour la gêne occasionnée.

[Signature]
```

#### Notification ANSSI (si incident de sécurité majeur)

**Obligatoire si** : OIV, NIS2, ou incident de sécurité significatif

```
À : cert-fr@ssi.gouv.fr
OBJET: [NOTIF INCIDENT] - [NOM_ORGANISME] - [NOM_SYSTÈME]

1. IDENTIFICATION
   - Organisme : [AUTO: organisation.entite_responsable.nom]
   - Système : [AUTO: systeme.nom]
   - Contact : [AUTO: organisation.rssi]

2. DESCRIPTION INCIDENT
   - Date/heure détection : [DATE]
   - Nature : [Type d'incident]
   - Impact : [Description]

3. ACTIONS EN COURS
   - [Actions]

4. DEMANDE D'ASSISTANCE
   - [Oui/Non] - [Préciser si oui]
```

### 7.2 Canaux de Communication

| Canal | Usage | Responsable |
|-------|-------|-------------|
| Status Page | Information temps réel utilisateurs | Ops |
| Email | Notifications formelles | Comm |
| Slack/Teams interne | Coordination équipes | Tous |
| Téléphone | Escalade urgente | Cellule de crise |
| Réseaux sociaux | Communication publique si majeur | Comm |

---

## 8. Tests et Maintenance du PCA

### 8.1 Programme de Tests

| Type de Test | Fréquence | Périmètre | Responsable |
|--------------|-----------|-----------|-------------|
| Revue documentaire | Trimestriel | Contacts, procédures | RSSI |
| Test de restauration | Mensuel | Backup → Environnement test | DBA |
| Exercice sur table | Semestriel | Simulation scénario | Cellule de crise |
| Test de basculement | Annuel | Site secours réel | DevOps + Direction |

### 8.2 Critères de Réussite

| Test | Critère | Seuil |
|------|---------|-------|
| Restauration backup | Durée restauration | < RTO cible |
| Restauration backup | Intégrité données | 100% |
| Basculement site | Durée basculement | < RTO cible |
| Basculement site | Fonctionnalités OK | > 95% |
| Exercice sur table | Temps réaction | < 1h mobilisation |

### 8.3 Historique des Tests

| Date | Type | Résultat | Observations | Actions |
|------|------|----------|--------------|---------|
| [DATE] | [Type] | ✅/⚠️/❌ | [Observations] | [Actions correctives] |

---

## 9. Annexes

### Annexe A : Checklist d'Activation PCA

```
CHECKLIST ACTIVATION PCA - [NOM_SYSTÈME]
Date/Heure : _______________
Incident : _________________

QUALIFICATION
[ ] Impact confirmé : _____________________________
[ ] RTO menacé : Oui / Non
[ ] Décision activation PCA : Oui / Non
[ ] Heure décision : _______________

MOBILISATION
[ ] Directeur de crise notifié : _______________
[ ] RSSI notifié : _______________
[ ] Équipe technique mobilisée : _______________
[ ] Salle de crise activée / Conf call : _______________

STRATÉGIE
[ ] Stratégie choisie : Basculement / Restauration / Dégradé
[ ] Responsable exécution : _______________
[ ] ETA reprise : _______________

COMMUNICATION
[ ] Communication interne envoyée : _______________
[ ] Status page mis à jour : _______________
[ ] Communication externe (si applicable) : _______________
[ ] ANSSI notifié (si applicable) : _______________

CLÔTURE
[ ] Service rétabli : _______________
[ ] Tests de validation : _______________
[ ] Communication de clôture : _______________
[ ] REX planifié : _______________
```

### Annexe B : Contacts Fournisseurs Critiques

<!-- [AUTO: Extraire de fournisseurs dans rgs-context.yaml] -->

| Fournisseur | Service | Contact Support | SLA | Escalade |
|-------------|---------|-----------------|-----|----------|
| [Hébergeur] | Infrastructure | [Tel/Email] | [SLA] | [Contact N2] |
| [BDD managée] | Base de données | [Tel/Email] | [SLA] | [Contact N2] |
| [CDN] | Distribution | [Tel/Email] | [SLA] | [Contact N2] |
| [Backup] | Sauvegarde | [Tel/Email] | [SLA] | [Contact N2] |

### Annexe C : Schéma Infrastructure

<!-- [MANUEL: Insérer schéma d'architecture avec points de failover] -->

```
[Insérer diagramme d'architecture]
```

---

## Références

- **RGS v2.0** - Référentiel Général de Sécurité
- **ISO 22301** - Systèmes de management de la continuité d'activité
- **Guide ANSSI** - Recommandations pour la mise en place d'un PCA
- **EBIOS RM** - [AUTO: Lien vers docs/security/EBIOS-RM-*.md]

---

**Document généré par OpenSecKit** | Version 1.0 | [DATE]
