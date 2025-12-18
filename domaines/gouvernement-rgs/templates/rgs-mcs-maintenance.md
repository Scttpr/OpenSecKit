---
title: "Maintien en Condition de Sécurité (MCS) - RGS Article 13"
template_version: "1.0.0"
domain: "government-rgs"
constitutional_principles:
  - "VII - Patch Management"
  - "II - Risk Analysis"
  - "IV - Security Testing"
regulatory_references:
  - "RGS v2.0 - Article 13"
  - "ANSSI - Guide du maintien en condition de sécurité"
  - "NIS2 - Article 21 (mesures de gestion des risques)"
ssdlc_phase: "VII - Maintenance"
difficulty: "intermediate"
estimated_time: "Processus continu (revue trimestrielle)"
---

# Maintien en Condition de Sécurité (MCS) - RGS Article 13

## Objectif

Ce template définit les **procédures de maintien en condition de sécurité (MCS)** obligatoires après l'homologation RGS, conformément à l'**Article 13 du RGS v2.0**.

**Contexte Réglementaire** :
- **RGS v2.0 Article 13** : "L'autorité d'homologation veille au maintien dans le temps du niveau de sécurité du système"
- **ANSSI** : Le MCS est une phase continue du cycle de vie de la sécurité
- **NIS2** : Obligations de gestion des risques et de notification des incidents

**Alignement Constitutionnel** :
- **Principe VII (Patch Management)** : Gestion proactive des mises à jour de sécurité
- **Principe II (Risk Analysis)** : Réévaluation continue des risques
- **Principe IV (Security Testing)** : Tests réguliers de sécurité

---

## 1. Vue d'Ensemble du MCS

### 1.1 Cycle de Vie de l'Homologation

```
┌─────────────────────────────────────────────────────────────────────┐
│                    CYCLE D'HOMOLOGATION RGS                          │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│   ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐     │
│   │  ÉTUDE   │───▶│ DOSSIER  │───▶│ DÉCISION │───▶│   MCS    │     │
│   │ SÉCURITÉ │    │ HOMOLOG. │    │ HOMOLOG. │    │          │     │
│   └──────────┘    └──────────┘    └──────────┘    └────┬─────┘     │
│                                                         │           │
│                                                         │           │
│   ┌─────────────────────────────────────────────────────┘           │
│   │                                                                  │
│   │  ┌─────────────────────────────────────────────────────────┐    │
│   │  │                PHASE MCS (3-5 ans)                       │    │
│   │  ├─────────────────────────────────────────────────────────┤    │
│   │  │                                                          │    │
│   │  │  Revues      Gestion       Tests        Gestion          │    │
│   │  │  trimestr.   vulnérab.    périodiques   incidents        │    │
│   │  │     │            │            │             │            │    │
│   │  │     ▼            ▼            ▼             ▼            │    │
│   │  │  ┌──────┐   ┌──────┐    ┌──────┐     ┌──────┐          │    │
│   │  │  │ Q1   │   │Scan  │    │Pentest│     │CERT  │          │    │
│   │  │  │ Q2   │   │hebdo │    │annuel │     │24/7  │          │    │
│   │  │  │ Q3   │   │      │    │       │     │      │          │    │
│   │  │  │ Q4   │   │      │    │       │     │      │          │    │
│   │  │  └──────┘   └──────┘    └──────┘     └──────┘          │    │
│   │  │                                                          │    │
│   │  └──────────────────────────────────────────────────────────┘    │
│   │                                                                  │
│   └──────────────────────────────────────────────────────────────────┘
│                            │
│                            ▼
│                    ┌──────────────┐
│                    │RÉ-HOMOLOGATION│
│                    │  (3-5 ans)    │
│                    └──────────────┘
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### 1.2 Responsabilités MCS

| Rôle | Responsabilités MCS |
|------|---------------------|
| **Autorité d'homologation** | Valide les évolutions majeures, décide des ré-homologations anticipées |
| **RSSI** | Pilote le MCS, coordonne les actions, reporting à l'autorité |
| **MOE / DevOps** | Applique les correctifs, maintient l'infrastructure |
| **SOC / CERT** | Surveillance, détection, réponse aux incidents |
| **DPO** | Veille RGPD, notification violations de données |

---

## 2. Revues de Sécurité Périodiques

### 2.1 Fréquence des Revues par Niveau RGS

| Niveau RGS | Fréquence revue | Participants | Durée |
|------------|-----------------|--------------|-------|
| **RGS*** | Semestrielle | RSSI, MOE | 2h |
| **RGS**** | Trimestrielle | RSSI, MOE, MOA | 4h |
| **RGS***** | Mensuelle | RSSI, MOE, MOA, Autorité | 4h |

### 2.2 Ordre du Jour Type - Revue Trimestrielle

```markdown
# Revue de Sécurité Trimestrielle - [Nom Système]

**Date** : [DATE]
**Participants** : RSSI, MOE, MOA
**Durée** : 4 heures

## 1. Bilan du Trimestre (30 min)

### 1.1 Indicateurs Clés
- [ ] Nombre d'incidents de sécurité : ___
- [ ] Vulnérabilités critiques corrigées : ___ / ___
- [ ] Disponibilité du service : ____%
- [ ] Alertes SIEM traitées : ___

### 1.2 Événements Marquants
- [Liste des incidents majeurs]
- [Changements d'architecture]
- [Nouvelles fonctionnalités déployées]

## 2. État des Vulnérabilités (45 min)

### 2.1 Vulnérabilités Ouvertes
| CVE | CVSS | Composant | Statut | Échéance |
|-----|------|-----------|--------|----------|
| | | | | |

### 2.2 Actions de Remédiation
- [ ] Patches appliqués ce trimestre : ___
- [ ] Patches en attente : ___
- [ ] Dérogations accordées : ___

## 3. Évolution des Risques (30 min)

### 3.1 Nouveaux Risques Identifiés
- [Liste des nouveaux risques]

### 3.2 Risques Résiduels
- [Évolution des risques acceptés]

## 4. Conformité Réglementaire (30 min)

### 4.1 Évolutions RGS/ANSSI
- [Nouvelles recommandations ANSSI]
- [Impact sur le système]

### 4.2 Conformité RGPD
- [ ] Registre des traitements à jour
- [ ] Exercice des droits traités : ___
- [ ] Violations notifiées : ___

## 5. Tests de Sécurité (30 min)

### 5.1 Résultats du Trimestre
- [ ] Scans de vulnérabilités : OK / KO
- [ ] Tests de pénétration : [Date prochain]
- [ ] Exercices de phishing : Taux de clic ____%

### 5.2 Actions Correctives
- [Liste des actions suite aux tests]

## 6. Plan d'Action Q+1 (45 min)

### 6.1 Priorités
1. [Action prioritaire 1]
2. [Action prioritaire 2]
3. [Action prioritaire 3]

### 6.2 Budget Sécurité
- Budget alloué : ___ €
- Budget consommé : ___ €
- Besoins supplémentaires : ___ €

## 7. Décisions (30 min)

- [ ] Décision 1 : ___
- [ ] Décision 2 : ___
- [ ] Prochaine revue : [DATE]

---

**Compte-rendu validé par** : [RSSI]
**Date** : [DATE]
```

### 2.3 Indicateurs de Suivi MCS (KPIs)

| Indicateur | Cible RGS* | Cible RGS** | Cible RGS*** | Fréquence mesure |
|------------|------------|-------------|--------------|------------------|
| **Disponibilité** | 99% | 99.5% | 99.9% | Mensuelle |
| **MTTR vulnérabilités critiques** | <7j | <48h | <24h | Continue |
| **Couverture patches** | >90% | >95% | >99% | Hebdomadaire |
| **Taux résolution alertes** | >80% | >90% | >95% | Mensuelle |
| **Délai détection incidents** | <24h | <4h | <1h | Continue |
| **Taux sensibilisation** | >70% | >85% | >95% | Annuelle |

---

## 3. Gestion des Vulnérabilités

### 3.1 Processus de Gestion des Vulnérabilités

```
┌─────────────┐    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
│ DÉTECTION   │───▶│ ÉVALUATION  │───▶│ PRIORISATION│───▶│ REMÉDIATION │
│             │    │             │    │             │    │             │
│ - Scans     │    │ - CVSS      │    │ - SLA       │    │ - Patch     │
│ - Veille    │    │ - Contexte  │    │ - Criticité │    │ - Workaround│
│ - Pentest   │    │ - Exploit   │    │ - Exposition│    │ - Accept    │
└─────────────┘    └─────────────┘    └─────────────┘    └─────────────┘
       │                  │                  │                  │
       ▼                  ▼                  ▼                  ▼
┌─────────────────────────────────────────────────────────────────────┐
│                     SUIVI ET REPORTING                               │
│  - Dashboard vulnérabilités                                          │
│  - Métriques MTTR                                                    │
│  - Reporting autorité d'homologation                                │
└─────────────────────────────────────────────────────────────────────┘
```

### 3.2 SLA de Correction par Criticité

| Criticité CVSS | Niveau | SLA RGS* | SLA RGS** | SLA RGS*** |
|----------------|--------|----------|-----------|------------|
| **9.0 - 10.0** | Critique | 7 jours | 48 heures | 24 heures |
| **7.0 - 8.9** | Haute | 30 jours | 7 jours | 72 heures |
| **4.0 - 6.9** | Moyenne | 90 jours | 30 jours | 14 jours |
| **0.1 - 3.9** | Faible | 180 jours | 90 jours | 30 jours |

### 3.3 Processus d'Exception (Dérogation)

```yaml
# Template de demande de dérogation
derogation:
  id: "DER-2025-001"
  date_demande: "2025-01-15"
  demandeur: "[Nom]"
  role: "MOE"

  vulnerabilite:
    cve: "CVE-2024-XXXXX"
    cvss: 7.5
    composant: "[Nom composant]"
    description: "[Description de la vulnérabilité]"

  justification:
    raison: "[Pourquoi le patch ne peut pas être appliqué dans les délais]"
    impact_metier: "[Impact sur le service si patch appliqué]"
    mesures_compensatoires:
      - "[Mesure 1 : WAF rule spécifique]"
      - "[Mesure 2 : Isolation réseau]"
      - "[Mesure 3 : Monitoring renforcé]"

  risque_residuel:
    probabilite: "Faible"  # Avec mesures compensatoires
    impact: "Moyen"
    acceptation: "Autorité d'homologation"

  plan_remediation:
    date_cible: "2025-03-01"
    actions:
      - "[Action 1]"
      - "[Action 2]"

  validation:
    rssi:
      decision: "APPROUVE"  # ou REFUSE
      date: "[DATE]"
      commentaire: "[Commentaire]"
    autorite:
      decision: "APPROUVE"  # Requis si dérogation > 30 jours
      date: "[DATE]"
      commentaire: "[Commentaire]"
```

### 3.4 Sources de Veille Vulnérabilités

| Source | Type | Fréquence | URL |
|--------|------|-----------|-----|
| **CERT-FR** | Alertes gouvernementales | Temps réel | https://www.cert.ssi.gouv.fr/ |
| **NVD (NIST)** | Base CVE | Temps réel | https://nvd.nist.gov/ |
| **ANSSI Alertes** | Alertes critiques | Temps réel | https://cyber.gouv.fr/actualites |
| **GitHub Advisory** | Dépendances | Temps réel | https://github.com/advisories |
| **Vendor Security** | Éditeurs | Variable | [URLs éditeurs] |

---

## 4. Tests de Sécurité Périodiques

### 4.1 Planning des Tests par Niveau RGS

| Type de test | RGS* | RGS** | RGS*** |
|--------------|------|-------|--------|
| **Scan de vulnérabilités (infra)** | Mensuel | Hebdomadaire | Quotidien |
| **Scan de vulnérabilités (appli)** | Trimestriel | Mensuel | Hebdomadaire |
| **Scan de dépendances (SCA)** | Hebdomadaire | Quotidien | Chaque build |
| **Pentest externe** | Annuel | Semestriel | Trimestriel |
| **Pentest interne** | Tous les 2 ans | Annuel | Semestriel |
| **Red Team** | Non requis | Tous les 2 ans | Annuel |
| **Exercice de phishing** | Annuel | Semestriel | Trimestriel |
| **Test PRA/PCA** | Annuel | Semestriel | Trimestriel |

### 4.2 Prestataires d'Audit Qualifiés ANSSI

**PASSI (Prestataires d'Audit de la Sécurité des Systèmes d'Information)** :

| Prestataire | Spécialités | Contact |
|-------------|-------------|---------|
| **Wavestone** | Pentest, Red Team, Audit orga | wavestone.com |
| **Orange Cyberdefense** | Pentest, SOC, Forensics | orangecyberdefense.com |
| **Advens** | Pentest, Audit conformité | advens.fr |
| **Synacktiv** | Pentest offensif, R&D | synacktiv.com |
| **Intrinsec** | Pentest, CERT | intrinsec.com |

**Liste officielle** : https://www.lsti-certification.fr/

### 4.3 Template de Suivi des Tests

```yaml
# Registre des tests de sécurité
tests_securite:
  - id: "TEST-2025-001"
    type: "Pentest externe"
    prestataire: "Wavestone"
    date_debut: "2025-01-15"
    date_fin: "2025-01-22"
    perimetre:
      - "Application web production"
      - "API REST"
      - "Infrastructure cloud"
    resultats:
      vulnerabilites_critiques: 0
      vulnerabilites_hautes: 2
      vulnerabilites_moyennes: 5
      vulnerabilites_faibles: 12
    rapport: "docs/audits/pentest-2025-01.pdf"
    plan_remediation: "docs/audits/remediation-2025-01.md"
    statut_remediation: "EN_COURS"  # PLANIFIE | EN_COURS | TERMINE

  - id: "TEST-2025-002"
    type: "Scan vulnérabilités"
    outil: "Trivy + OWASP ZAP"
    date: "2025-01-20"
    automatise: true
    resultats:
      vulnerabilites_critiques: 0
      vulnerabilites_hautes: 1
    actions: "Mise à jour dépendance lodash"
```

---

## 5. Gestion des Incidents de Sécurité

### 5.1 Classification des Incidents

| Niveau | Description | Exemples | Délai réponse |
|--------|-------------|----------|---------------|
| **P1 - Critique** | Compromission confirmée, fuite de données | Ransomware, exfiltration massive | <15 min |
| **P2 - Majeur** | Tentative d'intrusion avancée, indisponibilité | APT détecté, DDoS impactant | <1h |
| **P3 - Significatif** | Incident contenu, impact limité | Compte compromis isolé, malware détecté | <4h |
| **P4 - Mineur** | Événement de sécurité sans impact | Scan de reconnaissance, faux positif | <24h |

### 5.2 Processus de Réponse aux Incidents

```
┌─────────────────────────────────────────────────────────────────────┐
│                  PROCESSUS DE RÉPONSE AUX INCIDENTS                  │
├─────────────────────────────────────────────────────────────────────┤
│                                                                      │
│  1. DÉTECTION        2. QUALIFICATION     3. CONFINEMENT            │
│  ┌──────────┐        ┌──────────┐         ┌──────────┐              │
│  │ Alerte   │───────▶│ Analyse  │────────▶│ Isolation│              │
│  │ SIEM/SOC │        │ Triage   │         │ Préserv. │              │
│  └──────────┘        └──────────┘         └──────────┘              │
│       │                   │                    │                     │
│       │              ┌────┴────┐               │                     │
│       │              │ P1? P2? │               │                     │
│       │              └────┬────┘               │                     │
│       │                   │ Si P1/P2           │                     │
│       │                   ▼                    │                     │
│       │          ┌──────────────┐              │                     │
│       │          │ ESCALADE     │              │                     │
│       │          │ RSSI + Dir.  │              │                     │
│       │          └──────────────┘              │                     │
│       │                                        │                     │
│       ▼                                        ▼                     │
│  4. ÉRADICATION      5. RÉCUPÉRATION      6. POST-MORTEM           │
│  ┌──────────┐        ┌──────────┐         ┌──────────┐              │
│  │ Nettoyage│───────▶│ Restaur. │────────▶│ Analyse  │              │
│  │ Système  │        │ Service  │         │ Leçons   │              │
│  └──────────┘        └──────────┘         └──────────┘              │
│                                                 │                    │
│                                                 ▼                    │
│                                          ┌──────────┐               │
│                                          │ AMÉLIORA-│               │
│                                          │ TIONS    │               │
│                                          └──────────┘               │
│                                                                      │
└─────────────────────────────────────────────────────────────────────┘
```

### 5.3 Obligations de Notification

| Cas | Destinataire | Délai | Base légale |
|-----|--------------|-------|-------------|
| **Violation données personnelles** | CNIL | 72h | RGPD Art. 33 |
| **Violation données personnelles (risque élevé)** | Personnes concernées | Sans délai | RGPD Art. 34 |
| **Incident OIV** | ANSSI | Sans délai | LPM |
| **Incident NIS2 (significatif)** | ANSSI | 24h (alerte précoce) + 72h (rapport) | NIS2 Art. 23 |
| **Incident affectant service public** | Autorité de tutelle | 24h | Circulaire PM |

### 5.4 Template de Fiche Incident

```yaml
# Fiche Incident de Sécurité
incident:
  id: "INC-2025-001"
  titre: "[Titre court de l'incident]"
  classification: "P2"  # P1 | P2 | P3 | P4

  detection:
    date_heure: "2025-01-15T14:32:00Z"
    source: "SIEM - Alerte brute force"
    detecteur: "SOC Niveau 1"

  description:
    resume: "[Description de l'incident en 2-3 phrases]"
    vecteur_attaque: "[Phishing | Exploitation vulnérabilité | Insider | etc.]"
    systemes_impactes:
      - "Serveur web prod-01"
      - "Base de données users"
    donnees_impactees:
      type: "Données personnelles"
      volume: "~1000 enregistrements"
      sensibilite: "DR"

  chronologie:
    - timestamp: "2025-01-15T14:32:00Z"
      action: "Détection alerte SIEM"
      acteur: "SOC"
    - timestamp: "2025-01-15T14:45:00Z"
      action: "Qualification P2, escalade RSSI"
      acteur: "SOC"
    - timestamp: "2025-01-15T15:00:00Z"
      action: "Isolation serveur compromis"
      acteur: "DevOps"
    - timestamp: "2025-01-15T16:30:00Z"
      action: "Analyse forensique initiée"
      acteur: "CERT"

  confinement:
    actions:
      - "Isolation réseau du serveur compromis"
      - "Révocation des sessions utilisateurs"
      - "Blocage IP source"
    efficacite: "Incident contenu, pas de propagation"

  eradication:
    actions:
      - "Réinstallation système depuis image golden"
      - "Rotation des credentials"
      - "Patch de la vulnérabilité exploitée"

  recuperation:
    date_retour_service: "2025-01-15T22:00:00Z"
    duree_indisponibilite: "7h30"
    verification: "Tests de non-régression OK"

  notifications:
    cnil:
      requis: true
      date_notification: "2025-01-16T10:00:00Z"
      reference: "NOTIF-2025-001"
    personnes_concernees:
      requis: true
      date_notification: "2025-01-17T09:00:00Z"
      methode: "Email individuel"

  post_mortem:
    cause_racine: "[Description de la cause racine]"
    lecons_apprises:
      - "[Leçon 1]"
      - "[Leçon 2]"
    actions_amelioration:
      - action: "Mise en place WAF"
        responsable: "DevOps"
        echeance: "2025-02-01"
        statut: "EN_COURS"
      - action: "Formation anti-phishing renforcée"
        responsable: "RH"
        echeance: "2025-02-15"
        statut: "PLANIFIE"

  cloture:
    date: "2025-01-20"
    validateur: "RSSI"
    statut: "CLOS"
```

---

## 6. Gestion des Changements

### 6.1 Classification des Changements

| Type | Description | Validation requise | Impact homologation |
|------|-------------|-------------------|---------------------|
| **Standard** | Patch sécurité, mise à jour mineure | MOE | Aucun |
| **Normal** | Nouvelle fonctionnalité, évolution architecture | RSSI | Mise à jour dossier |
| **Majeur** | Changement de périmètre, nouvelle technologie | Autorité homologation | Ré-homologation possible |
| **Urgence** | Correctif critique (0-day) | RSSI (validation a posteriori) | Aucun |

### 6.2 Critères de Ré-homologation Anticipée

**Une ré-homologation anticipée est requise si** :

- [ ] Changement de niveau RGS cible (ex: RGS* → RGS**)
- [ ] Extension significative du périmètre (>30% nouveaux composants)
- [ ] Changement d'hébergeur ou de localisation des données
- [ ] Incident de sécurité majeur (P1) ayant révélé des failles structurelles
- [ ] Changement de fournisseur d'identité (ex: migration vers FranceConnect)
- [ ] Nouvelles exigences réglementaires (ex: NIS2)
- [ ] Durée depuis dernière homologation > 80% de la validité

### 6.3 Processus de Validation des Changements Majeurs

```yaml
# Template de demande de changement majeur
changement_majeur:
  id: "CHG-2025-001"
  titre: "[Titre du changement]"
  demandeur: "[Nom]"
  date_demande: "2025-01-15"

  description:
    resume: "[Description du changement]"
    justification_metier: "[Pourquoi ce changement est nécessaire]"
    perimetre_impacte:
      - "[Composant 1]"
      - "[Composant 2]"

  analyse_impact:
    securite:
      nouveaux_risques:
        - "[Risque 1]"
        - "[Risque 2]"
      mitigations:
        - "[Mitigation 1]"
        - "[Mitigation 2]"
    conformite:
      impact_rgs: "OUI / NON"
      necessite_rehomologation: "OUI / NON"
      justification: "[Justification]"

  plan_mise_en_oeuvre:
    date_cible: "2025-03-01"
    etapes:
      - "[Étape 1]"
      - "[Étape 2]"
    rollback: "[Procédure de retour arrière]"

  validations:
    rssi:
      avis: "FAVORABLE / DEFAVORABLE / AVEC_RESERVES"
      date: "[DATE]"
      commentaires: "[Commentaires]"
    autorite_homologation:
      decision: "APPROUVE / REFUSE / REHOMOLOGATION_REQUISE"
      date: "[DATE]"
      conditions: "[Conditions éventuelles]"
```

---

## 7. Ré-homologation

### 7.1 Calendrier de Ré-homologation

| Niveau RGS | Durée validité | Rappel T-12 mois | Rappel T-6 mois | Deadline |
|------------|----------------|------------------|-----------------|----------|
| **RGS*** | 5 ans | Alerte planning | Lancement dossier | Ré-homologation |
| **RGS**** | 3 ans | Alerte planning | Lancement dossier | Ré-homologation |
| **RGS***** | 3 ans | Lancement dossier | Commission prévue | Ré-homologation |

### 7.2 Checklist de Ré-homologation

```markdown
# Checklist de Ré-homologation

**Système** : [Nom]
**Homologation actuelle** : [Date] - [Date expiration]
**Ré-homologation cible** : [Date]

## 1. Préparation (T-12 à T-6 mois)

- [ ] Notification à l'autorité d'homologation
- [ ] Planification de la commission
- [ ] Budget alloué (audit, pentest)
- [ ] Ressources identifiées

## 2. Mise à Jour du Dossier (T-6 à T-3 mois)

### 2.1 Documents à mettre à jour
- [ ] Périmètre du système (évolutions depuis dernière homologation)
- [ ] Architecture technique (nouveaux composants)
- [ ] Analyse de risques EBIOS RM (nouveaux scénarios)
- [ ] Mesures de sécurité (nouvelles implémentations)
- [ ] Registre des incidents (bilan)
- [ ] Résultats des audits (pentests, scans)

### 2.2 Points de contrôle
- [ ] Risques résiduels toujours acceptables ?
- [ ] Nouvelles menaces identifiées ?
- [ ] Évolutions réglementaires prises en compte ?
- [ ] Recommandations des audits précédents appliquées ?

## 3. Audits (T-3 à T-1 mois)

- [ ] Pentest réalisé (< 6 mois avant commission)
- [ ] Scan de vulnérabilités à jour
- [ ] Audit de conformité RGS
- [ ] Vulnérabilités critiques/hautes corrigées

## 4. Commission d'Homologation (T-1 mois)

- [ ] Dossier complet transmis aux membres
- [ ] Présentation préparée
- [ ] Questions anticipées

## 5. Décision (T)

- [ ] Décision d'homologation signée
- [ ] Conditions éventuelles documentées
- [ ] Plan MCS mis à jour
- [ ] Prochaine ré-homologation planifiée
```

### 7.3 Contenu du Dossier de Ré-homologation

| Section | Contenu | Source |
|---------|---------|--------|
| **Évolutions depuis dernière homologation** | Liste des changements majeurs | Registre des changements |
| **Bilan des incidents** | Statistiques, incidents majeurs, leçons | Registre des incidents |
| **Bilan des vulnérabilités** | Statistiques, MTTR, dérogations | Dashboard vulnérabilités |
| **Résultats des audits** | Pentest, scans, conformité | Rapports d'audit |
| **État des risques** | Risques actuels vs initiaux | Registre des risques |
| **Conformité réglementaire** | RGS, RGPD, NIS2 | Audit de conformité |
| **Plan MCS mis à jour** | Actions pour la prochaine période | Ce template |

---

## 8. Documentation et Reporting

### 8.1 Documents à Maintenir

| Document | Fréquence MAJ | Responsable | Emplacement |
|----------|---------------|-------------|-------------|
| **Registre des risques** | Continue | RSSI | `docs/security/risk-register.yaml` |
| **Registre des incidents** | Continue | SOC | `docs/security/incidents/` |
| **Registre des vulnérabilités** | Continue | DevSecOps | Outil de suivi |
| **Registre des changements** | Continue | MOE | `docs/security/changes/` |
| **Comptes-rendus de revue** | Trimestriel | RSSI | `docs/security/revues/` |
| **Rapports d'audit** | Après chaque audit | Auditeur | `docs/audits/` |
| **Tableau de bord MCS** | Mensuel | RSSI | Dashboard |

### 8.2 Reporting à l'Autorité d'Homologation

**Fréquence** : Trimestrielle (ou sur incident P1/P2)

```markdown
# Rapport MCS Trimestriel - [Nom Système]

**Période** : Q1 2025
**Destinataire** : Autorité d'homologation
**Rédacteur** : RSSI

## Synthèse Exécutive

| Indicateur | Valeur | Tendance | Cible |
|------------|--------|----------|-------|
| Disponibilité | 99.7% | ↗️ | 99.5% |
| Incidents P1/P2 | 0 | ✅ | 0 |
| Vulnérabilités critiques ouvertes | 0 | ✅ | 0 |
| Conformité RGS | 95% | → | 90% |

**Statut global** : ✅ NOMINAL

## Points d'Attention

- [Point 1 si applicable]
- [Point 2 si applicable]

## Actions Requises

- [ ] [Action requérant validation autorité]

## Prochaine Échéance

- Prochain pentest : [DATE]
- Ré-homologation : [DATE]
```

---

## 9. Checklist MCS Complète

### 9.1 Actions Continues

- [ ] Veille vulnérabilités active (CERT-FR, NVD)
- [ ] Scans de vulnérabilités automatisés
- [ ] Monitoring SIEM opérationnel
- [ ] Backup et tests de restauration
- [ ] Rotation des secrets/credentials

### 9.2 Actions Hebdomadaires

- [ ] Revue des alertes SIEM
- [ ] Application des patches critiques
- [ ] Vérification des backups

### 9.3 Actions Mensuelles

- [ ] Rapport de vulnérabilités
- [ ] Mise à jour du registre des risques
- [ ] Revue des accès et droits

### 9.4 Actions Trimestrielles

- [ ] Revue de sécurité formelle
- [ ] Test de restauration complet
- [ ] Reporting à l'autorité d'homologation
- [ ] Exercice de réponse aux incidents

### 9.5 Actions Annuelles

- [ ] Pentest externe
- [ ] Audit de conformité RGS
- [ ] Sensibilisation sécurité
- [ ] Revue de la politique de sécurité
- [ ] Test PRA/PCA complet

---

## 10. Références

### Documentation ANSSI

- **Guides et publications** : https://cyber.gouv.fr/publications
- **CERT-FR** : https://www.cert.ssi.gouv.fr/
- **Prestataires qualifiés** : https://www.lsti-certification.fr/

### Standards

- **ISO 27001** : Annexe A.12 (Sécurité de l'exploitation)
- **NIST CSF** : Respond, Recover
- **ITIL** : Gestion des incidents, Gestion des changements

---

**Template Version** : 1.0.0
**Last Updated** : 2025-01-15
**Next Review** : 2026-01-15
**Owner** : RSSI
