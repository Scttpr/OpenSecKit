---
title: "Guide de sélection d'un SIEM"
template_version: "1.0.0"
constitutional_principle: "VI - Journalisation et audit"
ssdlc_phase: "design"
difficulty: "intermediate"
estimated_time: "2-3 hours"
tags: ["siem", "security-monitoring", "selection", "requirements"]
---

# Guide de sélection d'un SIEM

## Objectif

Aider à choisir la solution SIEM (Security Information and Event Management) adaptée aux besoins, budget et contraintes de l'organisation.

**Principe** : On ne peut pas protéger ce qu'on ne surveille pas. Un SIEM transforme des millions de logs en alertes actionnables.

---

## Qu'est-ce qu'un SIEM ?

Un SIEM est une plateforme qui :

1. **Collecte** les logs de sécurité de toutes les sources
2. **Normalise** les données dans un format commun
3. **Corrèle** les événements pour détecter des patterns suspects
4. **Alerte** en temps réel sur les incidents potentiels
5. **Archive** pour conformité et analyse forensique

### SIEM vs Log Management

| Capacité | Log Management | SIEM |
|----------|----------------|------|
| Collecte de logs | ✅ | ✅ |
| Recherche | ✅ | ✅ |
| Visualisation | ✅ | ✅ |
| **Corrélation d'événements** | ❌ | ✅ |
| **Détection de menaces** | ❌ | ✅ |
| **Alerting sécurité** | Basique | ✅ Avancé |
| **Compliance reporting** | ❌ | ✅ |
| **Threat intelligence** | ❌ | ✅ |

**Quand un simple log management suffit** :

- Petite application
- Pas d'exigences de sécurité critiques
- Budget très limité

**Quand un SIEM est nécessaire** :

- Données sensibles (PII, paiements, santé)
- Exigences de conformité (PCI-DSS, HIPAA, SOC 2)
- Menaces ciblées (finance, gouvernement)

---

## Fonctionnalités essentielles d'un SIEM

### 1. Collecte et normalisation

**Exigences** :

- [ ] Support de multiples sources (syslog, API, agents)
- [ ] Parsing automatique de formats courants
- [ ] Normalisation en schéma commun
- [ ] Enrichissement (geo-IP, threat intel)

**Pourquoi c'est important** : Sans normalisation, impossible de corréler des événements de sources différentes (ex: login échoué Windows + accès refusé AWS).

---

### 2. Corrélation d'événements

**Exigences** :

- [ ] Moteur de règles de corrélation
- [ ] Détection de séquences d'événements
- [ ] Agrégation par utilisateur/IP/ressource
- [ ] Fenêtres temporelles configurables

**Exemple de corrélation** :

```
Événement 1: Login échoué (user: alice, IP: 1.2.3.4)
Événement 2: Login échoué (user: bob, IP: 1.2.3.4)
Événement 3: Login échoué (user: charlie, IP: 1.2.3.4)
→ ALERTE: Brute force attack from 1.2.3.4
```

---

### 3. Détection de menaces

**Approches** :

- **Basée sur règles** : Patterns connus (brute force, port scanning)
- **Basée sur anomalies** : Écarts statistiques (ML/AI)
- **Threat intelligence** : IoCs externes (malware IPs, domains)

**Exigences** :

- [ ] Bibliothèque de règles pré-configurées
- [ ] Possibilité de créer des règles personnalisées
- [ ] Détection d'anomalies comportementales
- [ ] Intégration threat intelligence (MISP, feeds)

---

### 4. Alerting et notification

**Exigences** :

- [ ] Alertes en temps réel (< 1 minute)
- [ ] Sévérité configurable (Critical, High, Medium, Low)
- [ ] Routage intelligent (email, Slack, PagerDuty, ticket)
- [ ] Dédoublonnage (éviter spam d'alertes identiques)
- [ ] Escalade automatique si non traité

---

### 5. Investigation et forensique

**Exigences** :

- [ ] Recherche rapide sur grands volumes (TB de logs)
- [ ] Timeline reconstruction
- [ ] Export de preuves (CSV, PDF)
- [ ] Annotation et commentaires d'incidents
- [ ] Intégration avec outils IR (incident response)

---

### 6. Compliance et reporting

**Exigences** :

- [ ] Rapports de conformité pré-configurés (PCI-DSS, HIPAA)
- [ ] Génération de rapports programmée
- [ ] Audit trail des actions dans le SIEM
- [ ] Rétention long terme (7 ans)
- [ ] Immuabilité des logs archivés

---

## Comparatif des approches

### Open Source

**Exemples** : ELK Stack, Wazuh, Graylog, OpenSearch

**Avantages** :

- ✅ Gratuit (licence)
- ✅ Contrôle total
- ✅ Personnalisable à l'infini
- ✅ Pas de vendor lock-in

**Inconvénients** :

- ❌ Expertise technique requise
- ❌ Temps de setup et maintenance
- ❌ Pas de support officiel (sauf si payant)
- ❌ Corrélation moins avancée out-of-the-box

**Coût réel** :

- Infrastructure : $500-5000/mois
- Temps équipe : 20-40% FTE
- Formation : $2000-5000/an

**Quand choisir** :

- Équipe technique compétente
- Budget limité
- Exigences de souveraineté des données

---

### Commercial On-Premise

**Exemples** : Splunk Enterprise, IBM QRadar, LogRhythm

**Avantages** :

- ✅ Fonctionnalités avancées (corrélation, ML)
- ✅ Support professionnel
- ✅ Interface intuitive
- ✅ Marketplace d'intégrations

**Inconvénients** :

- ❌ Très coûteux ($10k-100k+/an)
- ❌ Complexe à déployer
- ❌ Ressources infrastructure importantes
- ❌ Pricing opaque

**Coût réel** :

- Licences : $10k-100k/an
- Infrastructure : $2k-10k/mois
- Formation : $5k-20k/an

**Quand choisir** :

- Grande entreprise
- Budget conséquent
- Besoin de support et garanties

---

### SaaS (Cloud)

**Exemples** : Datadog Security, Sumo Logic, Azure Sentinel, Google Chronicle

**Avantages** :

- ✅ Déploiement rapide (jours vs mois)
- ✅ Pas de maintenance infrastructure
- ✅ Scaling automatique
- ✅ Intégrations cloud natives
- ✅ Mises à jour continues

**Inconvénients** :

- ❌ Coût élevé à grande échelle
- ❌ Données hébergées hors organisation
- ❌ Dépendance au fournisseur
- ❌ Personnalisation limitée

**Coût réel** :

- $100-5000/mois selon volume
- Pas de coût infrastructure
- Pas de coût maintenance

**Quand choisir** :

- Infrastructure cloud
- Équipe réduite
- Besoin de rapidité
- Pas d'exigences de souveraineté strictes

---

## Matrice de décision

### Par volume de logs

| Volume/jour | Open Source | Commercial | SaaS |
|-------------|-------------|------------|------|
| < 5 GB | ⭐⭐⭐ | ⭐ | ⭐⭐⭐ |
| 5-50 GB | ⭐⭐⭐ | ⭐⭐ | ⭐⭐ |
| 50-500 GB | ⭐⭐ | ⭐⭐⭐ | ⭐ |
| > 500 GB | ⭐⭐⭐ | ⭐⭐⭐ | ⭐ |

---

### Par taille d'équipe sécurité

| Équipe | Open Source | Commercial | SaaS |
|--------|-------------|------------|------|
| Pas d'équipe dédiée | ⭐ | ⭐ | ⭐⭐⭐ |
| 1-2 personnes | ⭐⭐ | ⭐⭐ | ⭐⭐⭐ |
| 3-10 personnes | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ |
| > 10 personnes | ⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐ |

---

### Par budget annuel

| Budget | Recommandation |
|--------|----------------|
| < $5k/an | Open source (ELK, Wazuh) + infra minimale |
| $5k-50k/an | SaaS (Datadog, Sumo Logic) ou ELK managé |
| $50k-200k/an | Commercial (Splunk) ou ELK optimisé |
| > $200k/an | Commercial premium ou multi-SIEM |

---

## Critères de sélection détaillés

### 1. Fonctionnels

- [ ] Support des sources de logs nécessaires
- [ ] Règles de détection préconfigurées (MITRE ATT&CK)
- [ ] Corrélation cross-source
- [ ] Détection d'anomalies (ML)
- [ ] Threat intelligence intégrée
- [ ] Rapports de conformité (PCI, HIPAA, SOC 2)
- [ ] Interface intuitive
- [ ] API complète pour automatisation

### 2. Techniques

- [ ] Performance (requêtes < 2s sur 30 jours de données)
- [ ] Scalabilité (supportera croissance 2-3 ans)
- [ ] Disponibilité (SLA > 99.5%)
- [ ] Rétention (90 jours hot, 7 ans cold)
- [ ] Chiffrement (at rest + in transit)
- [ ] Authentification (SSO, MFA)
- [ ] Contrôle d'accès granulaire (RBAC)

### 3. Opérationnels

- [ ] Facilité de déploiement (< 1 mois)
- [ ] Maintenance requise (< 20% FTE)
- [ ] Support disponible (24/7 si critique)
- [ ] Documentation complète
- [ ] Communauté active
- [ ] Feuille de route produit

### 4. Financiers

- [ ] Coût initial (setup, licences)
- [ ] Coût récurrent (licences, infra, support)
- [ ] Modèle de pricing (par GB, par host, flat)
- [ ] Coûts cachés (formation, consultants)
- [ ] ROI estimé

---

## Processus de sélection recommandé

### Phase 1 : Définition des besoins (1 semaine)

1. **Inventaire** :
   - [ ] Sources de logs à intégrer
   - [ ] Volume actuel et projeté (3 ans)
   - [ ] Exigences de conformité

2. **Use cases** :
   - [ ] Détections prioritaires (top 5)
   - [ ] Workflows d'investigation
   - [ ] Rapports nécessaires

3. **Contraintes** :
   - [ ] Budget disponible
   - [ ] Équipe (taille, compétences)
   - [ ] Infrastructure existante

---

### Phase 2 : Shortlist (1 semaine)

1. **Présélection** :
   - Éliminer les solutions manifestement inadaptées
   - Retenir 3-5 candidats

2. **Demandes d'information** :
   - RFI (Request for Information) aux éditeurs
   - Vérifier conformité aux exigences

---

### Phase 3 : POC (Proof of Concept) (2-4 semaines)

1. **Tests fonctionnels** :
   - [ ] Intégrer 2-3 sources représentatives
   - [ ] Configurer 5-10 règles de détection
   - [ ] Créer 3-5 dashboards
   - [ ] Simuler des incidents
   - [ ] Mesurer performance

2. **Tests opérationnels** :
   - [ ] Facilité de configuration
   - [ ] Temps de réponse support
   - [ ] Qualité documentation

3. **Évaluation** :
   - Scorecard de comparaison
   - Feedback équipe
   - Analyse coût/bénéfice

---

### Phase 4 : Décision (1 semaine)

1. **Synthèse** :
   - Tableau comparatif final
   - Recommandation motivée

2. **Validation** :
   - Approbation direction
   - Négociation contrat
   - Planning déploiement

---

## Checklist de validation finale

Avant de signer :

- [ ] POC réussi avec données réelles
- [ ] Performance validée (volume × 3 pour marge)
- [ ] Budget validé (incluant coûts cachés)
- [ ] Équipe formée (1-2 jours minimum)
- [ ] Roadmap de déploiement claire
- [ ] Support adapté aux besoins
- [ ] Contrat relu par juridique
- [ ] Plan de migration (si remplacement)
- [ ] Mesures de succès définies

---

## Anti-patterns à éviter

❌ **Choisir sur le prix uniquement** : Solution inadaptée coûtera plus cher
❌ **Pas de POC** : Mauvaises surprises post-achat
❌ **Ignorer les coûts cachés** : Budget explosé
❌ **Négliger l'équipe** : Solution trop complexe non utilisée
❌ **Viser trop haut** : Sur-ingénierie pour les besoins réels
❌ **Sous-estimer la croissance** : Re-migration dans 2 ans

---

## Métriques de succès post-déploiement

À 3 mois :

- [ ] 80%+ des sources intégrées
- [ ] 10+ règles de détection actives
- [ ] < 10% faux positifs
- [ ] Équipe autonome sur opérations courantes

À 6 mois :

- [ ] 100% des sources intégrées
- [ ] 1+ incident détecté et résolu via SIEM
- [ ] MTTD (Mean Time To Detect) < 15 minutes
- [ ] Rapports de conformité automatisés

À 12 mois :

- [ ] ROI positif démontré
- [ ] Intégration SOAR (orchestration)
- [ ] Threat hunting actif
- [ ] Amélioration continue des règles

---

## Ressources associées

- [Exigences de centralisation des logs](log-centralization-requirements.md)
- [Exigences de journalisation](logging-requirements-template-design.md)
- [Template règles d'alerte](security-alert-rules-template.md)
