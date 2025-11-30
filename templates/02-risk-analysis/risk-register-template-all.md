---
title: "Modèle de registre de risques - Toutes phases"
template_version: "1.0.0"
constitutional_principle: "II - analyse de risques"
ssdlc_phase: "all"
last_updated: "2025-01-15"
reviewers:
  - security-champion-team
  - risk-management-team
description: "Modèle complet de registre de risques pour suivre tous les risques de sécurité identifiés tout au long du SSDLC. Sert de source unique de vérité pour la gestion des risques."
tags:
  - risk-register
  - risk-tracking
  - risk-management
  - governance
difficulty: "débutant"
estimated_time: "Maintenance continue (15 min hebdomadaire)"
prerequisites:
  - "Modélisation des menaces et notation des risques complétées"
related_templates:
  - "risk-scoring-template-planning.md"
  - "../01-threat-modeling/stride-threat-model-template-planning.md"
compliance_frameworks:
  - "NIST RMF"
  - "ISO 27001"
  - "SOC 2"
  - "PCI-DSS"
---

# Modèle de registre de risques - Toutes phases

## Aperçu

### Objectif

Le **registre de risques** est un document vivant qui suit tous les risques de sécurité identifiés tout au long du cycle de vie du développement logiciel. Il sert de :

- **Source unique de vérité** pour tous les risques de sécurité
- **Tableau de bord** pour les parties prenantes visualisant la posture de risque
- **Système de suivi** pour la progression de l'atténuation des risques
- **Preuve de conformité** pour audits et évaluations

### Quand utiliser

- **Toutes phases** : Maintenir continuellement de la planification à la maintenance
- **Mises à jour hebdomadaires** : Réviser et mettre à jour le statut des risques
- **Planification sprint** : Prioriser le travail d'atténuation des risques
- **Audits de conformité** : Fournir preuve de gestion des risques

### Qui maintient

- **Propriétaire principal** : Champion de sécurité
- **Contributeurs** : Équipes de développement, Product owners, Équipe sécurité
- **Réviseurs** : Direction, Équipe conformité

---

## 1. Vue d'ensemble du registre de risques

### Posture de risque actuelle

**Au** : [Date]

| Priorité | Nombre | Résumé statut |
|----------|--------|---------------|
| **P0 - Critique** (80-125) | 0 | ✅ Aucun en suspens |
| **P1 - Élevé** (49-79) | 2 | ⚠️ 2 en atténuation |
| **P2 - Moyen** (25-48) | 5 | 🔄 3 dans backlog, 2 en atténuation |
| **P3 - Faible** (11-24) | 8 | 📋 Suivis, non bloquants |
| **P4 - Minimal** (1-10) | 12 | 👁️ Surveillance |
| **Total risques actifs** | **27** | |

### Tendance des risques

```
Mois        Critique  Élevé  Moyen   Faible  Minimal
----------- --------- ------ ------- ------- -------
2024-12     1         5      7       6       10
2025-01     0         2      5       8       12
Tendance    ✅ -1     ✅ -3  ✅ -2   ⚠️ +2   ⚠️ +2
```

**Interprétation** : Risque global en baisse. Risques P0-P2 réduits par atténuation. Augmentation P3-P4 due à modélisation menaces nouvelles fonctionnalités (attendue).

---

## 2. Risques actifs

### Risques P0 - Critiques (score 80-125)

| ID risque | Nom risque | Score | Criticité | Probabilité | Exposition | Date identification | Propriétaire | Statut | Date cible | Statut approbation |
|-----------|------------|-------|-----------|-------------|------------|---------------------|--------------|--------|------------|--------------------|
| [Vide - Pas de risques critiques] | | | | | | | | | | |

**Notes** : Tous risques critiques doivent être atténués ou explicitement acceptés par Direction avant mise en production.

---

### Risques P1 - Élevés (score 49-79)

#### Risque R-2025-001 : Injection SQL dans recherche utilisateur

| Champ | Valeur |
|-------|--------|
| **ID risque** | R-2025-001 |
| **Nom risque** | Vulnérabilité injection SQL dans endpoint recherche utilisateur |
| **Score risque** | 60 (Criticité : 5, Probabilité : 3, Exposition : 4) |
| **Priorité** | P1 - Élevé |
| **Source menace** | Modèle menaces STRIDE - T01 |
| **Date identification** | 2025-01-10 |
| **Identifié par** | Champion sécurité (modélisation menaces) |
| **Composants affectés** | Service utilisateur, API recherche |
| **Description** | Endpoint recherche utilisateur construit requêtes SQL par concaténation chaînes, vulnérable aux attaques injection SQL. Attaquant pourrait extraire base de données utilisateur complète. |
| **Impact** | Compromission complète base de données, violation RGPD, dommage réputation |
| **Vecteur d'attaque** | Entrée malveillante dans paramètre recherche contourne validation entrée |
| **Contrôles existants** | Validation entrée basique (insuffisante), compte base de données lecture seule |
| **Stratégie traitement** | ☑ Atténuer / ☐ Accepter / ☐ Transférer / ☐ Éviter |
| **Plan atténuation** | 1. Revue de code (Sprint 1)<br>2. Implémenter requêtes paramétrées (Sprint 1-2)<br>3. Déployer SAST dans CI/CD (Sprint 2)<br>4. Test d'intrusion (Sprint 3) |
| **Propriétaire** | Équipe ingénierie (Responsable : Alice) |
| **Statut** | 🔄 En cours (60% complet) |
| **Cible complétion** | 2025-02-15 |
| **Risque résiduel** | 12 (Criticité : 5, Probabilité : 1, Exposition : 2.4) → P3 Faible |
| **Approbation requise** | Champion sécurité + Manager |
| **Approuvé par** | [Champion sécurité : Bob, 2025-01-12]<br>[Manager : Carol, 2025-01-12] |
| **Dernière MAJ** | 2025-01-18 |
| **Notes** | Revue de code complétée, 80% requêtes refactorisées en paramétrées. Outil SAST sélectionné (Semgrep). |

---

#### Risque R-2025-002 : DDoS sur endpoint authentification

| Champ | Valeur |
|-------|--------|
| **ID risque** | R-2025-002 |
| **Nom risque** | Déni de service distribué sur endpoint authentification |
| **Score risque** | 60 (Criticité : 4, Probabilité : 3, Exposition : 5) |
| **Priorité** | P1 - Élevé |
| **Source menace** | Modèle menaces STRIDE - D01 |
| **Date identification** | 2025-01-10 |
| **Identifié par** | Champion sécurité |
| **Composants affectés** | Service auth, Passerelle API |
| **Description** | Endpoint authentification manque rate limiting suffisant et protection DDoS. Attaquant pourrait inonder endpoint empêchant connexions utilisateurs légitimes. |
| **Impact** | Indisponibilité service, perte revenus, insatisfaction clients |
| **Vecteur d'attaque** | Botnet distribué inondant endpoint /auth/login |
| **Contrôles existants** | Protection DDoS basique fournisseur cloud, rate limiting par IP (faible) |
| **Stratégie traitement** | ☑ Atténuer / ☐ Accepter / ☐ Transférer / ☐ Éviter |
| **Plan atténuation** | 1. Déployer CDN avec atténuation DDoS (Sprint 2)<br>2. Implémenter rate limiting avancé (basé compte) (Sprint 2)<br>3. Ajouter CAPTCHA pour échecs répétés (Sprint 3)<br>4. Test de charge pour vérifier (Sprint 3) |
| **Propriétaire** | Équipe plateforme (Responsable : Dave) |
| **Statut** | 📋 Planifié (début Sprint 2) |
| **Cible complétion** | 2025-03-01 |
| **Risque résiduel** | 20 (Criticité : 4, Probabilité : 2, Exposition : 2.5) → P3 Faible |
| **Approbation requise** | Champion sécurité + Manager |
| **Approuvé par** | [Champion sécurité : Bob, 2025-01-12]<br>[Manager : Carol, 2025-01-12] |
| **Dernière MAJ** | 2025-01-18 |
| **Notes** | Fournisseur CDN sélectionné (Cloudflare). implémentation début 2025-01-20. |

---

### Risques P2 - Moyens (score 25-48)

| ID risque | Nom risque | Score | Propriétaire | Statut | Date cible | Notes |
|-----------|------------|-------|--------------|--------|------------|-------|
| R-2025-003 | Logs audit manquants pour actions admin | 48 | Équipe sécurité | 🔄 En cours | 2025-02-28 | Framework journalisation sélectionné |
| R-2025-004 | Politique mot de passe faible | 36 | Équipe produit | 📋 Backlog | 2025-03-31 | Nécessite design UX |
| R-2025-005 | Attributs cookie non sécurisés | 32 | Ingénierie | ✅ Atténué | 2025-01-20 | Flags SameSite, Secure ajoutés |
| R-2025-006 | Messages erreur verbeux | 30 | Ingénierie | 🔄 En cours | 2025-02-15 | Erreurs génériques implémentées |
| R-2025-007 | Headers sécurité manquants | 27 | DevOps | 📋 Backlog | 2025-02-28 | CSP, HSTS, X-Frame-Options |

**Entrées détaillées** : [Voir section 3 pour détails complets sur chaque risque P2]

---

### Risques P3 - Faibles (score 11-24)

| ID risque | Nom risque | Score | Propriétaire | Statut | Date cible |
|-----------|------------|-------|--------------|--------|------------|
| R-2025-008 | Timeout session trop long (24h) | 24 | Équipe produit | 📋 Planifié | T2 2025 |
| R-2025-009 | Suites cipher TLS obsolètes | 20 | DevOps | 📋 Planifié | T1 2025 |
| R-2025-010 | Validation entrée manquante champs profil | 18 | Ingénierie | 📋 Backlog | T2 2025 |
| [R-2025-011 à R-2025-015] | [Risques faibles additionnels] | | | | |

---

### Risques P4 - Minimaux (score 1-10)

| ID risque | Nom risque | Score | Statut | Notes |
|-----------|------------|-------|--------|-------|
| R-2025-016 | Divulgation information commentaires HTML | 8 | 👁️ Surveillance | Revue pendant nettoyage code |
| R-2025-017 | IDs de session prévisibles (entropie faible) | 6 | 👁️ Surveillance | Utilise RNG cryptographiquement sécurisé |
| [R-2025-018 à R-2025-027] | [Risques minimaux additionnels] | | | |

---

## 3. Risques atténués (clos)

### Récemment atténués (derniers 30 jours)

| ID risque | Nom risque | Score original | Date atténuation | Risque résiduel | Vérification |
|-----------|------------|----------------|------------------|-----------------|--------------|
| R-2024-045 | Application HTTPS manquante | 80 | 2025-01-05 | 8 | Vérifié par analyse sécurité |
| R-2024-048 | Clés API en dur dans code | 75 | 2025-01-08 | 5 | Analyse Gitleaks propre |
| R-2025-005 | Attributs cookie non sécurisés | 32 | 2025-01-20 | 6 | Test intrusion vérifié |

**Total risques atténués ce mois** : 3

---

## 4. Risques acceptés

### Risques acceptés avec justification

| ID risque | Nom risque | Score | Date acceptation | Accepté par | Justification | Date re-revue |
|-----------|------------|-------|------------------|-------------|---------------|---------------|
| R-2024-012 | Panneau admin accessible via VPN uniquement (pas MFA) | 36 | 2024-12-15 | Directeur ingénierie | Panneau admin nécessite VPN (jeton matériel) + mot de passe. MFA ajoute sécurité additionnelle minimale vu exigence jeton matériel. Rapport coût-bénéfice ne justifie pas implémentation MFA actuellement. | 2025-06-15 |

**Note** : Risques acceptés doivent être re-revus tous les 6 mois ou lorsque circonstances changent.

---

## 5. Métriques de gestion des risques

### Vélocité des risques

**Temps moyen d'atténuation** (P0-P2) :
- P0 (Critique) : Cible < 7 jours | Réel : N/A (0 risques critiques)
- P1 (Élevé) : Cible < 30 jours | Réel : 22 jours moyenne
- P2 (Moyen) : Cible < 90 jours | Réel : 45 jours moyenne

### Découverte de risques

**Nouveaux risques identifiés** :
- Ce sprint : 5 nouveaux risques (3× P3, 2× P4)
- Ce trimestre : 18 nouveaux risques
- Source : Modélisation menaces (12), Test d'intrusion (4), Revue de code (2)

### Statut conformité

- [ ] Tous risques P0 atténués ou approuvés par Direction
- [x] Tous risques P1 atténués ou approuvés par Champion sécurité + Manager
- [x] Tous risques P2 suivis avec plan atténuation
- [x] Registre risques revu hebdomadairement
- [x] Revue risques trimestrielle complétée

---

## 6. Modèle entrée risque détaillée

Utiliser ce modèle pour ajouter nouveaux risques :

### Risque R-YYYY-NNN : [Nom risque]

| Champ | Valeur |
|-------|--------|
| **ID risque** | R-[ANNÉE]-[NNN] |
| **Nom risque** | [Nom descriptif] |
| **Score risque** | [1-125] (Criticité : [ ], Probabilité : [ ], Exposition : [ ]) |
| **Priorité** | [P0/P1/P2/P3/P4] |
| **Source menace** | [ID STRIDE, Attack Tree, Revue de code, etc.] |
| **Date identification** | [AAAA-MM-JJ] |
| **Identifié par** | [Personne/Équipe] |
| **Composants affectés** | [Lister composants] |
| **Description** | [Description complète du risque] |
| **Impact** | [Que se passe-t-il si risque se matérialise ?] |
| **Vecteur d'attaque** | [Comment attaquant pourrait exploiter ceci ?] |
| **Contrôles existants** | [Quelles défenses actuellement en place ?] |
| **Stratégie traitement** | ☐ Atténuer / ☐ Accepter / ☐ Transférer / ☐ Éviter |
| **Plan atténuation** | [Plan atténuation étape par étape] |
| **Propriétaire** | [Équipe/Personne responsable] |
| **Statut** | [📋 Planifié / 🔄 En cours / ✅ Atténué / ⏸️ En attente / 👁️ Surveillance] |
| **Cible complétion** | [AAAA-MM-JJ] |
| **Risque résiduel** | [Score attendu après atténuation] |
| **Approbation requise** | [Oui/Non, par qui ?] |
| **Approuvé par** | [Nom, Date] |
| **Dernière MAJ** | [AAAA-MM-JJ] |
| **Notes** | [Contexte additionnel] |

---

## 7. Revue et maintenance

### Liste vérification revue hebdomadaire

- [ ] Mettre à jour statut tous risques P0-P2
- [ ] Ajouter risques nouvellement identifiés
- [ ] Clore risques atténués (vérifier atténuation d'abord)
- [ ] Mettre à jour scores risques si circonstances changées
- [ ] Réviser risques en retard et escalader si nécessaire
- [ ] Mettre à jour tableau bord tendance risques
- [ ] Partager mises à jour avec parties prenantes

### Revue approfondie trimestrielle

- [ ] Re-noter tous risques actifs
- [ ] Réviser risques acceptés (toujours acceptables ?)
- [ ] Analyser tendances risques
- [ ] Mettre à jour processus gestion risques si nécessaire
- [ ] Présenter posture risque à direction
- [ ] Vérifier exigences conformité satisfaites

---

## 8. Intégration avec SSDLC

### Phase planification
- **Entrée** : Modèles menaces (STRIDE, Attack Trees)
- **Sortie** : Backlog risques priorisé

### Phase conception
- **Entrée** : Décisions conception impactant risque
- **Sortie** : Scores risques mis à jour, nouveaux risques architecturaux

### Phase implémentation
- **Entrée** : Résultats revue de code, résultats SAST
- **Sortie** : Nouveaux risques identifiés, mises à jour statut risques

### Phase tests
- **Entrée** : DAST, résultats tests d'intrusion
- **Sortie** : Vérification risques atténués, nouveaux risques

### Phase déploiement
- **Entrée** : Résultats analyse infrastructure
- **Sortie** : Risques liés configuration

### Phase maintenance
- **Entrée** : Divulgations CVE, incidents sécurité
- **Sortie** : Nouveaux risques, risques existants re-notés

---

## Ressources liées

- [Modèle de notation de risques](risk-scoring-template-planning.md)
- [Modèle de menaces STRIDE](../01-threat-modeling/stride-threat-model-template-planning.md)
- [Exigences de sécurité](../03-security-requirements/)
- [NIST SP 800-30 - Évaluation des risques](https://csrc.nist.gov/publications/detail/sp/800-30/rev-1/final)
- [ISO 27001 Annexe A.5.7 - Renseignement sur les menaces](https://www.iso.org/standard/27001)
