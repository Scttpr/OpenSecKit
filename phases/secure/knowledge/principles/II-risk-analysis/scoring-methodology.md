---
title: "Modèle de notation de risques - Phase de planification"
template_version: "1.0.0"
constitutional_principle: "II - Risk Analysis"
ssdlc_phase: "planification"
last_updated: "2025-01-15"
reviewers:
  - security-champion-team
  - risk-management-team
description: "Modèle de notation de risques utilisant la méthodologie Criticité × Probabilité × Exposition. Aide les équipes à quantifier et prioriser les risques de sécurité identifiés lors de la modélisation des menaces."
tags:
  - risk-scoring
  - risk-analysis
  - risk-management
  - planning
difficulty: "intermédiaire"
estimated_time: "30-60 minutes"
prerequisites:
  - "Modèle de menaces complété (STRIDE ou Attack Tree)"
  - "Compréhension de la criticité et exposition du système"
related_templates:
  - "../01-threat-modeling/stride-threat-model-template-planning.md"
  - "risk-register-template-all.md"
compliance_frameworks:
  - "NIST RMF"
  - "ISO 27005"
  - "OWASP SAMM"
---

# Modèle de notation de risques - Phase de planification

## Aperçu

### Objectif

Ce modèle fournit une méthodologie structurée pour **quantifier les risques de sécurité** en utilisant un modèle de notation tridimensionnel :

**Score de Risque = Criticité × Probabilité × Exposition**

Où :
- **Criticité** (1-5) : Sévérité de l'impact si la menace se matérialise
- **Probabilité** (1-5) : Vraisemblance que la menace se produise
- **Exposition** (1-5) : Degré d'exposition de la surface d'attaque

**Plage de score total** : 1 (risque minimal) à 125 (risque critique)

### Quand utiliser

- Après avoir complété la modélisation des menaces
- Pendant la phase de planification pour nouvelles fonctionnalités
- Lors de la priorisation du backlog sécurité
- Pour les évaluations de risques de conformité

### Qui devrait utiliser

- Champions de sécurité
- Gestionnaires de risques
- Product owners
- Responsables techniques

---

## 1. Méthodologie de notation de risques

### Score de criticité (impact)

| Score | Niveau | Description | Exemples |
|-------|--------|-------------|----------|
| **5** | Critique | Compromission complète système, violation de données massive, violation réglementaire grave | Exposition complète base de données, arrêt complet service, violation majeure RGPD |
| **4** | Élevé | Perte significative données, perturbation majeure service, pénalités réglementaires | Exposition partielle PII, dégradation service clé, violation SOC 2 |
| **3** | Moyen | Exposition limitée données, impact modéré service, préoccupations conformité | Fuite limitée données utilisateur, panne temporaire, lacune conformité mineure |
| **2** | Faible | Divulgation mineure d'informations, impact service mineur | Fuite messages d'erreur, ralentissement bref, exposition données faible valeur |
| **1** | Minimal | Impact négligeable, problèmes cosmétiques | Données non sensibles, pas d'impact service, problème UX mineur |

### Score de probabilité (vraisemblance)

| Score | Niveau | Description | Indicateurs |
|-------|--------|-------------|-----------|
| **5** | Très élevée | Menace activement exploitée ou trivialement exploitable | Exploits connus disponibles, attaque zero-click, vulnérabilité courante, aucune défense |
| **4** | Élevée | Menace très susceptible de se produire | Facile à exploiter, contrôles faibles, motivation attaquant élevée, schémas d'attaque connus |
| **3** | Moyenne | Menace pourrait raisonnablement se produire | Difficulté modérée, certains contrôles en place, méthodes d'attaque documentées |
| **2** | Faible | Menace improbable mais possible | Difficile à exploiter, contrôles forts, nécessite accès interne ou compétences avancées |
| **1** | Très faible | Menace hautement improbable | Théorique seulement, multiples contrôles forts, nécessite ressources état-nation |

### Score d'exposition (surface d'attaque)

| Score | Niveau | Description | Exemples |
|-------|--------|-------------|----------|
| **5** | Entièrement exposé | Accessible publiquement sans authentification | API publique sans auth, endpoints non authentifiés, base de données face Internet |
| **4** | Hautement exposé | Accessible à large population non fiable | Site web public, app client, authentification faible |
| **3** | Modérément exposé | Accessible à utilisateurs externes limités | API partenaire, accès externe authentifié, accès VPN |
| **2** | Minimalement exposé | Réseau interne uniquement, accès restreint | Outils internes, panneaux admin, nécessite VPN + MFA |
| **1** | Non exposé | Air-gapped, accès physique requis | Systèmes offline, accès physique uniquement, réseaux isolés |

---

## 2. Calcul de risque

### Formule

```
Score de risque = Criticité × Probabilité × Exposition
```

**Plage de risque total** : 1 à 125

### Seuils de risque

| Score de risque | Priorité | Action requise | Approbation requise |
|-----------------|----------|----------------|---------------------|
| **80-125** | P0 - Critique | Atténuation immédiate, bloquer mise en production | Approbation direction/exécutif pour accepter |
| **49-79** | P1 - Élevé | Atténuation requise avant mise en production | Approbation champion sécurité + manager |
| **25-48** | P2 - Moyen | Atténuation dans backlog sprint | Approbation champion sécurité |
| **11-24** | P3 - Faible | Atténuation planifiée, non bloquante | Approbation chef d'équipe |
| **1-10** | P4 - Minimal | Surveiller, traiter opportunément | Pas d'approbation nécessaire |

---

## 3. Feuille de notation de risques

### Menace 1 : [Nom de la menace du modèle de menaces]

**ID menace** : [ex : T01 de l'analyse STRIDE]

**Description menace** : [Brève description de la menace]

**Source menace** : [Catégorie STRIDE, nœud Attack Tree, ou autre]

#### Notation

**Score de criticité** : [ ] / 5
- **Justification** : [Expliquer l'impact si cette menace se matérialise]
- **Actifs affectés** : [Données, systèmes, utilisateurs impactés]
- **Impact business** : [Perte de revenus, réputation, conformité, etc.]

**Score de probabilité** : [ ] / 5
- **Justification** : [Expliquer la vraisemblance d'occurrence]
- **Complexité d'attaque** : [Facile/Moyenne/Difficile]
- **Motivation attaquant** : [Élevée/Moyenne/Faible]
- **Contrôles existants** : [Quelles défenses sont en place ?]

**Score d'exposition** : [ ] / 5
- **Justification** : [Expliquer l'exposition de la surface d'attaque]
- **Accessibilité** : [Qui peut atteindre ce vecteur d'attaque ?]
- **Authentification requise** : [Oui/Non, force]
- **Localisation réseau** : [Face internet, interne, isolé]

#### Calcul

```
Score de risque = [Criticité] × [Probabilité] × [Exposition]
                = [__] × [__] × [__]
                = [____] / 125
```

**Priorité** : [P0/P1/P2/P3/P4]

#### Actions requises

- **Approbation requise** : [Oui/Non, par qui ?]
- **Atténuation requise** : [Immédiate/Avant mise en production/Backlog/Surveiller]
- **Propriétaire** : [Équipe ou personne responsable]
- **Échéance** : [Date]

---

### Menace 2 : [Nom de la menace]

**ID menace** : [ID]

**Description menace** : [Description]

**Source menace** : [Source]

#### Notation

**Score de criticité** : [ ] / 5
- **Justification** : [Expliquer]

**Score de probabilité** : [ ] / 5
- **Justification** : [Expliquer]

**Score d'exposition** : [ ] / 5
- **Justification** : [Expliquer]

#### Calcul

```
Score de risque = [__] × [__] × [__] = [____] / 125
```

**Priorité** : [P0/P1/P2/P3/P4]

---

### [Continuer pour toutes les menaces identifiées]

---

## 4. Tableau de bord résumé des risques

| ID menace | Nom menace | Criticité | Probabilité | Exposition | **Score risque** | Priorité | Statut |
|-----------|------------|-----------|-------------|------------|------------------|----------|--------|
| T01 | Injection SQL | 5 | 3 | 4 | **60** | P1 - Élevé | En atténuation |
| T02 | Détournement session | 4 | 4 | 5 | **80** | P0 - Critique | Nécessite approbation |
| T03 | Attaque DDoS | 4 | 3 | 5 | **60** | P1 - Élevé | En atténuation |
| T04 | [Menace] | [ ] | [ ] | [ ] | **[ ]** | [ ] | [ ] |

### Distribution des risques

```
P0 (Critique) : 1 menace(s)  █
P1 (Élevé) :    2 menace(s)  ██
P2 (Moyen) :    0 menace(s)
P3 (Faible) :   0 menace(s)
P4 (Minimal) :  0 menace(s)
```

---

## 5. Exigences d'approbation

### Menaces nécessitant approbation direction (score ≥ 80)

| ID menace | Score risque | Justification business pour accepter le risque |
|-----------|--------------|-----------------------------------------------|
| T02 | 80 | Risque de détournement session atténué par application MFA (T1 2025), rate limiting, et timeout session. Risque résiduel acceptable pour lancement avec feuille de route atténuation. |

**Approbation** :
- [ ] Champion sécurité : [Nom] - [Date]
- [ ] Manager : [Nom] - [Date]
- [ ] Directeur/Exécutif : [Nom] - [Date]

---

### Menaces nécessitant approbation champion sécurité (score 49-79, 25-48, ou 11-24)

| ID menace | Score risque | Plan atténuation | Date cible |
|-----------|--------------|------------------|------------|
| T01 | 60 | Implémenter requêtes paramétrées, revue code, SAST | 2025-02-15 |
| T03 | 60 | Déployer CDN avec protection DDoS, rate limiting | 2025-03-01 |

**Approbation** :
- [ ] Champion sécurité 1 : [Nom] - [Date]
- [ ] Champion sécurité 2 : [Nom] - [Date]

---

## 6. Décisions de traitement des risques

Pour chaque risque noté, documenter l'approche de traitement :

### Menace T01 : Injection SQL (score risque : 60, P1 - Élevé)

**Stratégie de traitement** : ☑ Atténuer / ☐ Accepter / ☐ Transférer / ☐ Éviter

**Plan d'atténuation** :
1. **Immédiat** : Revue de code de toutes les requêtes base de données
2. **Sprint +1** : Implémenter requêtes paramétrées dans tout le code
3. **Sprint +2** : Déployer outil SAST dans pipeline CI/CD
4. **Sprint +3** : Effectuer test d'intrusion pour injection SQL

**Risque résiduel après atténuation** : 12 (Criticité : 5, Probabilité : 1, Exposition : 2.4)
- **Nouvelle priorité** : P3 - Faible
- **Justification** : Avec requêtes paramétrées et SAST, probabilité chute à très faible

**Propriétaire** : Équipe ingénierie
**Cible de complétion** : 2025-02-15
**Vérification** : Test d'intrusion + analyse SAST propre

---

### Menace T02 : Détournement session (score risque : 80, P0 - Critique)

**Stratégie de traitement** : ☑ Atténuer / ☐ Accepter / ☐ Transférer / ☐ Éviter

**Plan d'atténuation** :
1. **Immédiat** : Appliquer HTTPS avec HSTS
2. **Sprint +1** : Implémenter cookies HttpOnly, Secure, SameSite
3. **Sprint +2** : Ajouter jetons CSRF à toutes requêtes modifiant état
4. **T1 2025** : Imposer MFA pour tous utilisateurs

**Risque résiduel après atténuation** : 20 (Criticité : 4, Probabilité : 2, Exposition : 2.5)
- **Nouvelle priorité** : P3 - Faible
- **Justification** : MFA + CSRF + cookies sécurisés réduisent significativement probabilité succès attaque

**Propriétaire** : Équipe sécurité
**Cible de complétion** : 2025-03-31 (déploiement MFA)
**Vérification** : Audit sécurité + test d'intrusion

---

## 7. Liste de vérification

- [ ] Toutes les menaces du modèle de menaces notées
- [ ] Criticité, Probabilité, Exposition justifiées pour chaque menace
- [ ] Scores de risque calculés correctement
- [ ] Menaces priorisées (P0-P4)
- [ ] Approbation obtenue pour risques Critiques (≥80)
- [ ] Approbation obtenue pour risques Élevés (49-79)
- [ ] Plans d'atténuation documentés pour risques P0-P2
- [ ] Risque résiduel calculé pour menaces atténuées
- [ ] Propriétaires et échéances assignés
- [ ] Registre de risques mis à jour (voir risk-register-template-all.md)

---

## 8. Revue et mises à jour

### Historique des revues

| Version | Date | Réviseur | Modifications |
|---------|------|----------|---------------|
| 1.0 | 2025-01-15 | [Nom] | Notation risques initiale |

### Déclencheurs de re-notation

Re-noter les risques lorsque :
- [ ] Nouvelles menaces identifiées
- [ ] Atténuations implémentées (calculer risque résiduel)
- [ ] Paysage d'attaque change (nouveaux exploits découverts)
- [ ] Exposition système change (modifications architecture)
- [ ] Revue de risques trimestrielle

---

## Ressources liées

- [Modèle de menaces STRIDE](../01-threat-modeling/stride-threat-model-template-planning.md)
- [Modèle de registre de risques](risk-register-template-all.md)
- [NIST SP 800-30 - Guide d'évaluation des risques](https://csrc.nist.gov/publications/detail/sp/800-30/rev-1/final)
- [ISO 27005 - Gestion des risques de sécurité de l'information](https://www.iso.org/standard/75281.html)
- [Méthodologie OWASP de notation des risques](https://owasp.org/www-community/OWASP_Risk_Rating_Methodology)
