---
title: "Modèle EBIOS Risk Manager - Conforme RGS"
template_version: "1.0.0"
domain: "government-rgs"
constitutional_principle: "I - Threat Modeling, II - Risk Analysis"
ssdlc_phase: "planning"
last_updated: "2025-01-15"
reviewers:
  - rssi
  - security-champion-team
description: "Modèle de méthodologie EBIOS Risk Manager pour projets gouvernement français. Obligatoire pour conformité RGS. Fournit analyse de risques structurée alignée sur directives ANSSI."
tags:
  - ebios
  - risk-analysis
  - threat-modeling
  - rgs
  - anssi
  - french-government
difficulty: "avancé"
estimated_time: "8-16 heures (format atelier recommandé)"
prerequisites:
  - "Architecture système définie"
  - "Parties prenantes projet identifiées"
  - "Formation méthodologie EBIOS RM"
related_templates:
  - "../../templates/threat-modeling/stride-threat-model-template-planning.md"
  - "../../templates/risk-analysis/risk-scoring-template-planning.md"
compliance_frameworks:
  - "RGS v2.0"
  - "ANSSI EBIOS Risk Manager"
  - "ISO 27005"
---

# Modèle EBIOS Risk Manager - Conforme RGS

## Vue d'ensemble

**EBIOS Risk Manager** (EBIOS RM) est la méthode officielle d'analyse de risques de l'ANSSI, obligatoire pour l'homologation RGS.

### Les 5 ateliers EBIOS RM

```
Atelier 1: Socle de sécurité
    ↓
Atelier 2: Sources de risques
    ↓
Atelier 3: Scénarios stratégiques
    ↓
Atelier 4: Scénarios opérationnels
    ↓
Atelier 5: Traitement du risque
```

**Durée totale** : 2-4 jours en atelier

---

## Atelier 1 : Socle de sécurité

### Objectif

Définir les besoins de sécurité fondamentaux et identifier les biens supports essentiels.

### 1.1 Périmètre du système

**Nom du système** : [Exemple: Plateforme de téléprocédures administratives]

**Description** :
[Décrire le système étudié]

**Limites** :
- Dans le périmètre : [Composants inclus]
- Hors périmètre : [Composants exclus]

---

### 1.2 Missions et valeurs métier

| Mission métier | Valeur métier | Criticité |
|----------------|---------------|-----------|
| Délivrance de certificats administratifs | Continuité de service public | **Élevée** |
| Gestion des dossiers usagers | Protection des données personnelles | **Élevée** |
| [Mission] | [Valeur] | [Faible/Moyenne/Élevée] |

---

### 1.3 Biens supports

#### Biens essentiels

| Bien essentiel | Type | Propriétaire | Sensibilité |
|----------------|------|--------------|-------------|
| Base de données usagers | Données | RSSI | **Confidentiel DR** |
| Serveurs d'application | Système | DSI | **Diffusion Restreinte** |
| Identités FranceConnect | Données | Responsable traitement | **Secret** |
| [Asset] | Données/Système/Personnel | [Propriétaire] | [Public/DR/Confidentiel DR/Secret Défense] |

#### Classification RGS des données

- **Public** : Diffusable sans restriction
- **Diffusion Restreinte (DR)** : Usage interne uniquement
- **Confidentiel Diffusion Restreinte** : Accès limité aux habilités
- **Secret Défense** : Protection maximale

---

### 1.4 Besoins de sécurité

Pour chaque bien essentiel, évaluer les besoins en **DICP** (échelle 0-4) :

#### Bien : Base de données usagers

| Critère | Niveau | Justification |
|---------|--------|---------------|
| **D**isponibilité | **3** | Service public 24/7, interruption > 1h inacceptable |
| **I**ntégrité | **4** | Données officielles, toute altération = non-conformité légale |
| **C**onfidentialité | **4** | Données personnelles sensibles (RGPD), secret professionnel |
| **P**reuve | **3** | Traçabilité requise pour audits, opposabilité juridique |

**Échelle DICP** :
- **0** : Aucun besoin
- **1** : Faible (impact limité)
- **2** : Moyen (impact significatif)
- **3** : Fort (impact important)
- **4** : Très fort (impact critique, conformité légale)

---

### 1.5 Événements redoutés

| ID | Événement redouté | Bien impacté | Gravité | Vraisemblance |
|----|-------------------|--------------|---------|---------------|
| ER-01 | Divulgation de données personnelles | Base de données usagers | **4 - Critique** | Moyenne |
| ER-02 | Altération de dossiers administratifs | Base de données usagers | **4 - Critique** | Faible |
| ER-03 | Indisponibilité prolongée (> 4h) | Serveurs d'application | **3 - Important** | Moyenne |
| ER-04 | Usurpation d'identité via FranceConnect | Identités FranceConnect | **4 - Critique** | Faible |

**Gravité** :
- **1** : Négligeable
- **2** : Limitée
- **3** : Importante
- **4** : Critique

---

## Atelier 2 : Sources de risques

### Objectif

Identifier les acteurs malveillants (cybercriminels, États, hacktivistes) et leurs objectifs.

### 2.1 Cartographie des sources de risques

| Type de source | Exemples | Motivation | Niveau de ressources | Menace pour ce système |
|----------------|----------|------------|---------------------|------------------------|
| **Cybercriminels** | Groupes ransomware, fraudeurs | Financière | Moyen à Élevé | Élevée - Données revendables |
| **États** | APT, services de renseignement | Espionnage, déstabilisation | Très élevé | Moyenne - Cible gouvernementale |
| **Hacktivistes** | Anonymous, groupes militants | Idéologique, protestation | Faible à Moyen | Moyenne - Visibilité publique |
| **Initiés malveillants** | Employés mécontents | Vengeance, sabotage | Variable | Élevée - Accès privilégié |
| **Concurrents** | Autres administrations, privé | Avantage concurrentiel | Moyen | Faible - Secteur public |

### 2.2 Objectifs visés (OV)

Que cherchent-ils à atteindre sur ce système ?

| ID | Objectif visé | Sources de risques | Priorité |
|----|---------------|--------------------|----------|
| OV-01 | Voler les données personnelles des usagers | Cybercriminels | **Élevée** |
| OV-02 | Perturber le service public (DDoS, défacement) | Hacktivistes, États | **Élevée** |
| OV-03 | Altérer les dossiers administratifs | Initiés malveillants, Cybercriminels | **Moyenne** |
| OV-04 | Compromettre l'infrastructure pour rebond | États (APT) | **Moyenne** |

---

## Atelier 3 : Scénarios stratégiques

### Objectif

Croiser les événements redoutés (ER) et les objectifs visés (OV) pour identifier les scénarios stratégiques.

### 3.1 Matrice ER × OV

| Événement redouté ↓ / Objectif visé → | OV-01: Vol données | OV-02: Perturbation service | OV-03: Altération dossiers | OV-04: Compromission infra |
|---------------------------------------|--------------------|-----------------------------|----------------------------|----------------------------|
| **ER-01: Divulgation données** | ✓ **SS-01** | - | - | - |
| **ER-02: Altération dossiers** | - | - | ✓ **SS-02** | - |
| **ER-03: Indisponibilité** | - | ✓ **SS-03** | - | - |
| **ER-04: Usurpation identité** | ✓ **SS-04** | - | ✓ **SS-05** | - |

### 3.2 Scénarios stratégiques identifiés

#### SS-01 : Vol de données personnelles par cybercriminels

**Événement redouté** : ER-01 (Divulgation de données personnelles)
**Objectif visé** : OV-01 (Voler les données personnelles des usagers)
**Sources de risques** : Cybercriminels (ransomware, fraude)

**Gravité** : **4 - Critique** (RGPD, atteinte aux droits des personnes)
**Vraisemblance** : **Moyenne** (cibles attractives, mais défenses en place)
**Niveau de risque initial** : **Élevé**

---

#### SS-02 : Altération de dossiers par initié malveillant

**Événement redouté** : ER-02 (Altération de dossiers administratifs)
**Objectif visé** : OV-03 (Altérer les dossiers administratifs)
**Sources de risques** : Initiés malveillants

**Gravité** : **4 - Critique** (intégrité des données officielles)
**Vraisemblance** : **Faible** (contrôles d'accès, traçabilité)
**Niveau de risque initial** : **Moyen**

---

#### SS-03 : DDoS par hacktivistes

**Événement redouté** : ER-03 (Indisponibilité prolongée)
**Objectif visé** : OV-02 (Perturber le service public)
**Sources de risques** : Hacktivistes, États

**Gravité** : **3 - Important** (continuité de service)
**Vraisemblance** : **Moyenne** (protestations, mouvements sociaux)
**Niveau de risque initial** : **Moyen**

---

## Atelier 4 : Scénarios opérationnels

### Objectif

Détailler les modes opératoires (techniques d'attaque) pour chaque scénario stratégique.

### 4.1 Scénario opérationnel SO-01 : SQL Injection → Vol de données

**Scénario stratégique parent** : SS-01 (Vol de données personnelles)

**Chemin d'attaque** :

```
1. Reconnaissance
   ├── Scan du site web (Nmap, Burp Suite)
   └── Identification des endpoints vulnérables

2. Exploitation initiale
   ├── SQL Injection sur formulaire de recherche
   └── Bypass de l'authentification (admin' OR '1'='1)

3. Escalade de privilèges
   ├── Récupération du hash admin
   └── Cracking du mot de passe (hashcat)

4. Exfiltration
   ├── Dump de la base de données (SQLMap)
   └── Transfert via canal chiffré (TOR)

5. Monétisation
   └── Revente des données sur le dark web
```

**Vraisemblance technique** : **Moyenne** (vulnérabilités SQL injection courantes mais détectables)

**Mesures de sécurité existantes** :
- [ ] WAF (Web Application Firewall) - ❌ Non implémenté
- [ ] Requêtes paramétrées - ✅ Partiellement (70% du code)
- [ ] SAST (analyse statique) - ❌ Non implémenté
- [ ] IDS/IPS - ✅ Déployé (Suricata)

**Niveau de vulnérabilité** : **Moyen** (certaines failles subsistent)

**Gravité × Vraisemblance = Niveau de risque** :
- Gravité : **4** (Critique)
- Vraisemblance : **2** (Moyenne)
- **Risque résiduel : 8/16 - Risque élevé**

---

### 4.2 Scénario opérationnel SO-02 : Phishing → Accès initié malveillant

**Scénario stratégique parent** : SS-02 (Altération de dossiers par initié)

**Chemin d'attaque** :

```
1. Ingénierie sociale
   ├── Email de phishing ciblé (spear phishing)
   └── Usurpation identité chef de service

2. Compromission du compte
   ├── Vol de credentials FranceConnect
   └── Bypass 2FA (interception SMS)

3. Accès au système
   ├── Authentification via FranceConnect
   └── Navigation vers interface d'administration

4. Altération de données
   ├── Modification de dossiers administratifs
   └── Suppression de traces (logs)

5. Persistance
   └── Création de backdoor pour accès futur
```

**Vraisemblance technique** : **Faible** (MFA activé, sensibilisation régulière)

**Mesures existantes** :
- [x] Formation anti-phishing - ✅ Trimestrielle
- [x] MFA obligatoire (TOTP) - ✅ Tous les comptes admin
- [x] Journalisation complète - ✅ SIEM (3 ans)
- [ ] Détection anomalies comportementales - ❌ Non implémenté

**Niveau de vulnérabilité** : **Faible**

**Risque résiduel : 4/16 - Risque moyen**

---

### 4.3 Scénario opérationnel SO-03 : DDoS volumétrique

**Scénario stratégique parent** : SS-03 (DDoS par hacktivistes)

**Chemin d'attaque** :

```
1. Préparation
   ├── Location de botnet (Mirai, etc.)
   └── Reconnaissance infrastructure (IP, DNS)

2. Attaque DDoS
   ├── Saturation bande passante (100 Gbps+)
   ├── SYN flood
   └── HTTP flood sur endpoints critiques

3. Amplification
   ├── Attaque réfléchie DNS (amplification x50)
   └── NTP amplification

4. Durée
   └── Maintien 4-24 heures (épuisement défenses)
```

**Vraisemblance technique** : **Moyenne** (botnets accessibles, coût faible)

**Mesures existantes** :
- [x] Protection DDoS FAI - ✅ Orange Cyberdéfense
- [ ] CDN avec anti-DDoS - ❌ Non implémenté
- [x] Rate limiting applicatif - ✅ Nginx (1000 req/s)
- [ ] Anycast DNS - ❌ Non implémenté

**Niveau de vulnérabilité** : **Moyen**

**Risque résiduel : 6/16 - Risque élevé**

---

## Atelier 5 : Traitement du risque

### Objectif

Définir les mesures de sécurité pour traiter chaque scénario opérationnel.

### 5.1 Stratégies de traitement

Pour chaque scénario, choisir :

- **Réduction** : Mesures de sécurité pour diminuer la vraisemblance ou la gravité
- **Évitement** : Supprimer l'activité à risque
- **Transfert** : Assurance cyber, externalisation
- **Acceptation** : Risque résiduel acceptable (avec validation autorité)

---

### 5.2 Plan de traitement - SO-01 (SQL Injection)

**Stratégie** : ☑ Réduction ☐ Évitement ☐ Transfert ☐ Acceptation

**Mesures de sécurité** :

| Mesure | Type | Propriétaire | Échéance | Coût estimé | Réduction de risque |
|--------|------|--------------|----------|-------------|---------------------|
| Refactoring SQL (requêtes paramétrées à 100%) | Technique | Équipe dev | Sprint +2 | 10 j·h | Vraisemblance : 2→1 |
| Déploiement WAF (ModSecurity) | Technique | DevOps | T1 2025 | 5 k€ | Vraisemblance : 2→1 |
| SAST en CI/CD (Semgrep) | Organisationnel | Security Champion | Sprint +1 | Gratuit | Détection précoce |
| Pentest annuel | Organisationnel | RSSI | Annuel | 15 k€ | Vérification |

**Risque résiduel après traitement** :
- Gravité : **4** (inchangée)
- Vraisemblance : **1** (Faible, après mesures)
- **Risque résiduel : 4/16 - Risque moyen** ✅ Acceptable

---

### 5.3 Plan de traitement - SO-02 (Phishing initié)

**Stratégie** : ☑ Réduction ☑ Transfert (assurance cyber)

**Mesures** :

| Mesure | Type | Propriétaire | Échéance | Coût | Réduction |
|--------|------|--------------|----------|------|-----------|
| UEBA (User Behavior Analytics) | Technique | RSSI | T2 2025 | 20 k€ | Détection anomalies |
| Simulation phishing mensuelle | Organisationnel | RH + RSSI | Continu | 2 k€/an | Sensibilisation |
| Assurance cyber (2M€ couverture) | Transfert | DAF | T1 2025 | 10 k€/an | Transfert financier |
| Revue trimestrielle des privilèges | Organisationnel | RSSI | Trimestriel | Interne | Moindre privilège |

**Risque résiduel : 2/16 - Risque faible** ✅ Acceptable

---

### 5.4 Plan de traitement - SO-03 (DDoS)

**Stratégie** : ☑ Réduction ☑ Acceptation partielle

**Mesures** :

| Mesure | Type | Propriétaire | Échéance | Coût | Réduction |
|--------|------|--------------|----------|------|-----------|
| CDN avec anti-DDoS (Cloudflare) | Technique | DevOps | T1 2025 | 15 k€/an | Vraisemblance : 2→1 |
| Architecture multi-AZ | Technique | Architecte | T2 2025 | 30 k€ | Résilience |
| Plan de communication de crise | Organisationnel | Direction | T1 2025 | Interne | Gestion de crise |

**Risque résiduel : 3/16 - Risque moyen** ✅ Acceptable (avec plan de continuité)

---

## Synthèse des risques

### Cartographie des risques

```
Gravité ↑
  4 │ [SS-01]         [SS-02]
    │   ↓               ↓
  3 │         [SS-03]
    │
  2 │
    │
  1 │
    └─────────────────────────────→ Vraisemblance
      1     2     3     4
```

**Légende** :
- [SS-XX] = Avant traitement
- ↓ = Après traitement

### Tableau de synthèse

| Scénario | Risque initial | Mesures | Budget | Risque résiduel | Statut |
|----------|----------------|---------|--------|-----------------|--------|
| SS-01 : SQL Injection | **8/16 Élevé** | WAF, SAST, refactoring | 15 k€ | **4/16 Moyen** | ✅ Acceptable |
| SS-02 : Phishing initié | **4/16 Moyen** | UEBA, simulations, assurance | 32 k€ | **2/16 Faible** | ✅ Acceptable |
| SS-03 : DDoS | **6/16 Élevé** | CDN, multi-AZ | 45 k€ | **3/16 Moyen** | ✅ Acceptable |
| **TOTAL** | - | - | **92 k€** | - | - |

---

## Validation et homologation

### Risques résiduels acceptables

Tous les risques résiduels sont **≤ Moyen** après traitement.

**Validation requise par** :
- [ ] RSSI (Responsable Sécurité des Systèmes d'Information)
- [ ] Autorité d'homologation
- [ ] Direction

### Prochaines étapes

1. Intégration dans le **dossier d'homologation RGS**
2. Implémentation des mesures de sécurité (backlog sécurité)
3. Revue EBIOS annuelle
4. Ré-homologation tous les 3-5 ans

---

## Annexes

### Annexe A : Glossaire EBIOS RM

- **Bien support** : Tout élément (système, données, personnel) nécessaire aux missions
- **Événement redouté (ER)** : Impact négatif sur les biens essentiels
- **Objectif visé (OV)** : But recherché par une source de risque
- **Scénario stratégique (SS)** : Croisement ER × OV
- **Scénario opérationnel (SO)** : Mode opératoire technique détaillé

### Annexe B : Références

- **ANSSI - Guide EBIOS RM** : https://cyber.gouv.fr/la-methode-ebios-risk-manager
- **RGS v2.0** : https://cyber.gouv.fr/le-referentiel-general-de-securite-rgs

---

**Version** : 1.0.0 | **Date** : 2025-01-15 | **Prochaine revue** : 2026-01-15
