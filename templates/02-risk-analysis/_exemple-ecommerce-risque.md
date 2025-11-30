---
title: "Paiement e-commerce - Analyse et notation de risques"
template_version: "1.0.0"
constitutional_principle: ["II"]
ssdlc_phase: "planification"
domain_applicability: "ecommerce"
last_updated: "2025-11-18"
reviewers: ["security-champion-lead", "security-champion-architect"]
related_templates:
  - "../../templates/risk-analysis/risk-scoring-template-planning.md"
  - "../01-threat-modeling/_exemple-ecommerce-stride.md"
tags: ["exemple", "ecommerce", "analyse-risques", "PCI-DSS", "notation"]
scenario_description: "Analyse de risques complète avec matrice de notation pour les menaces de paiement e-commerce"
complexity_level: "intermédiaire"
annotations_included: true
---

# Analyse de risques : flux de paiement e-commerce

**Fonctionnalité** : Paiement de panier d'achat avec traitement de paiement
**Application** : Plateforme e-commerce SecureShop
**Date d'analyse de risques** : 2025-11-18
**Analystes de risques** : Champion de sécurité principal, Product Manager, Responsable technique
**Basé sur le modèle de menaces** : [ecommerce-checkout-threat-model.md](../01-threat-modeling/_exemple-ecommerce-stride.md)

## Résumé exécutif

Cette analyse de risques applique une méthodologie de notation **Criticité × Probabilité × Exposition** aux 16 menaces identifiées dans le modèle de menaces du flux de paiement. En utilisant une échelle de 5 points pour chaque dimension (score total 0-125), nous avons :

**Distribution des risques** :
- **CRITIQUE (80-125)** : 2 menaces - Action immédiate requise
- **ÉLEVÉ (48-79)** : 6 menaces - Doit être traité avant mise en production
- **MOYEN (20-47)** : 5 menaces - Traiter dans le prochain sprint
- **FAIBLE (1-19)** : 3 menaces - Surveiller et suivre

**Top 3 des risques** :
1. **Exposition de données de carte de paiement** (I01) - Score : **100** (5×5×4) - Risque de violation PCI-DSS
2. **Injection SQL dans le traitement de commandes** (T02) - Score : **80** (5×4×4) - Compromission base de données
3. **Manipulation des prix du panier** (T01) - Score : **75** (5×5×3) - Vol de revenus

**Statut d'atténuation** :
- ✅ **14 sur 16 menaces** atténuées à un risque résiduel acceptable (≤20)
- ⏳ **2 menaces** nécessitent des contrôles supplémentaires avant production (D01, R01)

---

## Table des matières

1. [Méthodologie de notation de risques](#méthodologie-de-notation-de-risques)
2. [Registre de risques](#registre-de-risques)
3. [Calculs de score de risque](#calculs-de-score-de-risque)
4. [Plan d'atténuation priorisé](#plan-datténuation-priorisé)
5. [Évaluation du risque résiduel](#évaluation-du-risque-résiduel)
6. [Décisions d'acceptation de risques](#décisions-dacceptation-de-risques)
7. [Surveillance et revue](#surveillance-et-revue)

---

## 1. Méthodologie de notation de risques

### 1.1 Formule de notation

```
Score de risque = Criticité × Probabilité × Exposition
```

**Plage** : 0 (pas de risque) à 125 (risque maximal)

### 1.2 Dimension de criticité

**Mesure** : Impact sur le business, les utilisateurs ou la conformité si la menace est exploitée.

| Score | Niveau | Description | Exemples |
|-------|--------|-------------|----------|
| 5 | Critique | Faillite d'entreprise, amendes massives, responsabilité juridique grave | Violation PCI-DSS (amendes 100K$+), violation de données de paiement |
| 4 | Élevé | Perte financière significative, violations de conformité, dommage à la marque | Vol de revenus, violations RGPD, panne prolongée |
| 3 | Moyen | Impact financier modéré, inconvénient utilisateur, préjudice réputationnel | Dégradation de service, fuite de données mineure, tentatives de fraude |
| 2 | Faible | Perte financière mineure, inconvénient temporaire | Litige de commande individuelle, bref temps d'arrêt |
| 1 | Minimal | Impact négligeable | Erreurs de journalisation, problèmes UI cosmétiques |

### 1.3 Dimension de probabilité

**Mesure** : Vraisemblance d'exploitation de la menace (complexité d'attaque, motivation de l'attaquant, contrôles existants).

| Score | Niveau | Description | Facteurs |
|-------|--------|-------------|---------|
| 5 | Très élevée | Très susceptible de se produire dans 1 mois | Trivial à exploiter, vulnérabilité publiquement connue, motivation attaquant élevée |
| 4 | Élevée | Susceptible de se produire dans 3 mois | Facile à exploiter, compétences attaquant modérées requises, vecteur d'attaque commun |
| 3 | Moyenne | Possible de se produire dans 1 an | Complexité modérée, outils spécialisés requis, motivation attaquant plus faible |
| 2 | Faible | Improbable dans 1 an | Complexité élevée, vecteur d'attaque rare, motivation attaquant faible |
| 1 | Très faible | Rare ou théorique | Complexité extrême, pas d'exploit connu, nécessite accès interne |

### 1.4 Dimension d'exposition

**Mesure** : Nombre de vecteurs d'attaque, utilisateurs affectés et taille de la surface d'attaque.

| Score | Niveau | Description | Portée |
|-------|--------|-------------|-------|
| 5 | Maximale | Face Internet, millions d'utilisateurs, multiples points d'entrée | API publique, pas d'authentification requise |
| 4 | Élevée | Face Internet, milliers d'utilisateurs, accès authentifié | Endpoints authentifiés, large base d'utilisateurs |
| 3 | Moyenne | Accès externe limité, centaines d'utilisateurs | Protégé par VPN, rôles utilisateur spécifiques |
| 2 | Faible | Systèmes internes, dizaines d'utilisateurs | Réseau interne uniquement, accès admin |
| 1 | Minimale | Systèmes isolés, peu d'utilisateurs | Air-gapped, admin unique |

### 1.5 Seuils de niveau de risque

| Score de risque | Niveau de risque | Action requise | Niveau d'approbation |
|------------|------------|-----------------|----------------|
| 80-125 | **CRITIQUE** | Atténuation immédiate, bloque production | Approbation direction |
| 48-79 | **ÉLEVÉ** | Atténuer avant mise en production | Approbation champion sécurité |
| 20-47 | **MOYEN** | Atténuer dans le prochain sprint | Approbation chef d'équipe |
| 1-19 | **FAIBLE** | Surveiller et suivre | Documentation uniquement |

---

## 2. Registre de risques

### 2.1 Registre de risques complet

| ID risque | Menace | Catégorie | Risque inhérent | Risque résiduel | Statut | Propriétaire |
|---------|--------|----------|---------------|---------------|--------|-------|
| I01 | Exposition de données de carte de paiement | Divulgation d'informations | 100 (CRITIQUE) | 5 (FAIBLE) | ✅ Atténué | Architecte sécurité |
| T02 | Injection SQL | Altération | 80 (CRITIQUE) | 8 (FAIBLE) | ✅ Atténué | Responsable backend |
| T01 | Manipulation des prix du panier | Altération | 75 (ÉLEVÉ) | 6 (FAIBLE) | ✅ Atténué | Responsable backend |
| E02 | IDOR de paiement | Élévation de privilèges | 72 (ÉLEVÉ) | 6 (FAIBLE) | ✅ Atténué | Responsable backend |
| I02 | IDOR de données utilisateur | Divulgation d'informations | 64 (ÉLEVÉ) | 12 (FAIBLE) | ✅ Atténué | Responsable backend |
| D01 | DDoS de flood de paiement | Déni de service | 60 (ÉLEVÉ) | 30 (MOYEN) | ⏳ Partiel | Responsable DevOps |
| E01 | Manipulation de rôle JWT | Élévation de privilèges | 60 (ÉLEVÉ) | 6 (FAIBLE) | ✅ Atténué | Architecte sécurité |
| S01 | Vol de jeton JWT | Usurpation | 48 (ÉLEVÉ) | 12 (FAIBLE) | ✅ Atténué | Architecte sécurité |
| D02 | Épuisement du pool de connexions | Déni de service | 48 (ÉLEVÉ) | 12 (FAIBLE) | ✅ Atténué | Responsable backend |
| R01 | Répudiation de commande | Répudiation | 36 (MOYEN) | 24 (MOYEN) | ⏳ Partiel | Product Manager |
| T03 | Race condition de paiement | Altération | 32 (MOYEN) | 6 (FAIBLE) | ✅ Atténué | Responsable backend |
| S02 | Détournement de session | Usurpation | 30 (MOYEN) | 3 (FAIBLE) | ✅ Atténué | Architecte sécurité |
| R02 | Répudiation de changement de config | Répudiation | 24 (MOYEN) | 12 (FAIBLE) | ✅ Atténué | Responsable DevOps |
| S03 | Manipulation de jeton Stripe | Usurpation | 20 (MOYEN) | 2 (FAIBLE) | ✅ Atténué | Responsable backend |
| E03 | Accès admin par path traversal | Élévation de privilèges | 18 (FAIBLE) | 2 (FAIBLE) | ✅ Atténué | Responsable backend |
| I03 | Attaque temporelle d'adresse | Divulgation d'informations | 10 (FAIBLE) | 5 (FAIBLE) | ✅ Atténué | Responsable backend |

**Résumé** :
- **Total des risques** : 16
- **Critique** : 2 (inhérent) → 0 (résiduel)
- **Élevé** : 6 (inhérent) → 1 (résiduel, atténuation partielle)
- **Moyen** : 5 (inhérent) → 2 (résiduel, atténuation partielle)
- **Faible** : 3 (inhérent) → 13 (résiduel)

---

## 3. Calculs de score de risque

### 3.1 Risques CRITIQUES (Score 80-125)

#### Risque I01 : exposition de données de carte de paiement dans le navigateur

**Description de la menace** : Données de carte de paiement sensibles (PAN, CVV) stockées en mémoire du navigateur ou transmises au backend, violant les exigences PCI-DSS SAQ-A.

**Score de risque inhérent** : **100**

**Détail de la notation** :
- **Criticité** : 5 (Critique) - Violation PCI-DSS = amendes 5 000$-100 000$/mois, violation de données de carte
- **Probabilité** : 5 (Très élevée) - Vulnérabilité courante si Stripe.js pas utilisé correctement
- **Exposition** : 4 (Élevée) - Tous les utilisateurs de paiement (milliers quotidiennement)
- **Calcul** : 5 × 5 × 4 = **100**

**Contre-mesures implémentées** :
1. ✅ Tokenisation Stripe.js (données de carte ne touchent JAMAIS nos serveurs)
2. ✅ Validation de conformité PCI-DSS SAQ-A via Stripe
3. ✅ En-têtes CSP empêchant scripts inline et XSS
4. ✅ Pas de données de carte dans journaux, bases de données ou code application
5. ✅ Audit de code frontend (vérifié que données de carte pas stockées dans localStorage/sessionStorage)

**Score de risque résiduel** : **5**
- **Criticité** : 5 (inchangé - impact toujours critique si contre-mesures échouent)
- **Probabilité** : 1 (Très faible - multiples couches de défense)
- **Exposition** : 1 (Minimale - données tokenisées uniquement)
- **Calcul** : 5 × 1 × 1 = **5** (FAIBLE)

**Validation** :
- ✅ Audit de sécurité complété (2025-11-15)
- ✅ Intégration Stripe revue par champion sécurité
- ✅ Questionnaire PCI-DSS SAQ-A soumis et approuvé

---

#### Risque T02 : injection SQL dans le traitement de commandes

**Description de la menace** : Un attaquant injecte des commandes SQL via entrée utilisateur (adresse de livraison, IDs produit) pour manipuler ou exfiltrer des données de la base.

**Score de risque inhérent** : **80**

**Détail de la notation** :
- **Criticité** : 5 (Critique) - Compromission complète base de données, vol de données, perte de données
- **Probabilité** : 4 (Élevée) - Vulnérabilité web courante si requêtes paramétrées pas utilisées
- **Exposition** : 4 (Élevée) - Tous les champs de formulaire de paiement (adresse, sélection produit)
- **Calcul** : 5 × 4 × 4 = **80**

**Contre-mesures implémentées** :
1. ✅ ORM Sequelize avec requêtes paramétrées (100% des accès base de données)
2. ✅ Validation d'entrée (whitelist caractères autorisés pour champs adresse)
3. ✅ Utilisateur base de données à privilèges minimaux (SELECT, INSERT, UPDATE uniquement - pas de DROP, DELETE sur tables production)
4. ✅ Réplicas de lecture base de données (séparation des permissions lecture/écriture)
5. ✅ Règles WAF bloquant patterns d'injection SQL (Cloudflare)

**Score de risque résiduel** : **8**
- **Criticité** : 5 (inchangé)
- **Probabilité** : 1 (Très faible - ORM empêche construction SQL directe)
- **Exposition** : 2 (Faible - WAF + validation d'entrée limitent surface d'attaque)
- **Calcul** : 5 × 1 × 2 = **10**, réduit à **8** avec confiance défense en profondeur

**Validation** :
- ✅ Audit de code (grep pour requêtes SQL brutes - AUCUNE trouvée)
- ✅ Scan SAST (CodeQL) - 0 découverte d'injection SQL
- ⏳ Prévu : Test d'intrusion (planifié pour 2025-12-01)

---

### 3.2 Risques ÉLEVÉS (Score 48-79)

#### Risque T01 : manipulation des prix du panier

**Description de la menace** : Un attaquant modifie les prix des articles du panier dans le frontend pour acheter des articles à des prix frauduleux.

**Score de risque inhérent** : **75**

**Détail de la notation** :
- **Criticité** : 5 (Critique) - Vol de revenus direct, faillite potentielle si généralisé
- **Probabilité** : 5 (Très élevée) - Attaque triviale (DevTools navigateur)
- **Exposition** : 3 (Moyenne) - Tous les utilisateurs authentifiés
- **Calcul** : 5 × 5 × 3 = **75**

**Contre-mesures implémentées** :
1. ✅ Le backend recalcule tous les prix depuis la base de données produit (ignore les prix fournis par le client)
2. ✅ Validation du panier à chaque étape de paiement : `SELECT price FROM products WHERE id = ?`
3. ✅ Validation du total de commande avant traitement du paiement
4. ✅ Journalisation des écarts de prix (frontend vs backend) - alertes sur anomalies
5. ✅ Tests unitaires pour validation de prix (cas de test négatifs : prix altérés rejetés)

**Score de risque résiduel** : **6**
- **Criticité** : 5 (inchangé)
- **Probabilité** : 1 (Très faible - contrôles backend infaillibles)
- **Exposition** : 1 (Minimale - attaque n'a aucun effet)
- **Calcul** : 5 × 1 × 1 = **5**, ajusté à **6** avec confiance piste d'audit

**Validation** :
- ✅ Test d'intrusion : Tentative de manipulation de prix rejetée (2025-11-16)
- ✅ Tests unitaires réussis (100% couverture pour logique validation prix)

---

#### Risque E02 : modification de paiement non autorisée (IDOR)

**Description de la menace** : Un attaquant modifie le moyen de paiement d'une commande d'un autre utilisateur via devinette d'ID de commande.

**Score de risque inhérent** : **72**

**Détail de la notation** :
- **Criticité** : 4 (Élevée) - Fraude de paiement, charges non autorisées, rétrofacturations
- **Probabilité** : 4 (Élevée) - Vulnérabilités IDOR courantes
- **Exposition** : 5 (Maximale) - IDs de commande séquentiels rendent devinette triviale
- **Calcul** : 4 × 4 × 5 = **80**, réduit à **72** (IDs de commande sont des UUIDs, pas séquentiels)

**Calcul réel** (avec atténuation UUID) :
- **Criticité** : 4
- **Probabilité** : 3 (Moyenne - plus difficile de deviner UUIDs)
- **Exposition** : 3 (Moyenne - nécessite authentification)
- **Calcul** : 4 × 3 × 3 = **36** → Ajusté à **72** pour risque inhérent avant atténuations

**Contre-mesures implémentées** :
1. ✅ Validation de propriété de commande : `WHERE order_id = ? AND user_id = JWT.user_id`
2. ✅ UUIDs pour IDs de commande (pas d'entiers séquentiels)
3. ✅ Intent de paiement verrouillé après création (order_id immuable dans métadonnées Stripe)
4. ✅ Tests d'autorisation (cas négatifs : accès aux commandes d'autres utilisateurs → 403)

**Score de risque résiduel** : **6**
- **Criticité** : 4 (inchangé)
- **Probabilité** : 1 (Très faible - vérifications de propriété empêchent l'attaque)
- **Exposition** : 1 (Minimale - UUIDs rendent devinette impraticable)
- **Calcul** : 4 × 1 × 1 = **4**, ajusté à **6** avec confiance

---

#### Risque I02 : exposition de données utilisateur via IDOR

**Description de la menace** : Les réponses API incluent des données sensibles (emails, adresses) d'autres utilisateurs en raison de vérifications d'autorisation manquantes.

**Score de risque inhérent** : **64**

**Détail de la notation** :
- **Criticité** : 4 (Élevée) - Violation RGPD, violation PII, amende €20M ou 4% du chiffre d'affaires
- **Probabilité** : 4 (Élevée) - IDOR est OWASP Top 10 (A01:2021)
- **Exposition** : 4 (Élevée) - Tous les endpoints API retournant des données utilisateur
- **Calcul** : 4 × 4 × 4 = **64**

**Contre-mesures implémentées** :
1. ✅ Vérifications d'autorisation sur TOUS les endpoints : `WHERE user_id = JWT.user_id`
2. ✅ Requêtes base de données scopées à l'utilisateur authentifié (jamais de SELECT global)
3. ✅ Tests unitaires pour autorisation (cas de test négatifs)
4. ✅ Tests de sécurité API (Burp Suite - aucune découverte IDOR)

**Score de risque résiduel** : **12**
- **Criticité** : 4 (inchangé)
- **Probabilité** : 1 (Très faible - pattern d'autorisation cohérent)
- **Exposition** : 3 (Moyenne - nécessite authentification pour tenter)
- **Calcul** : 4 × 1 × 3 = **12** (FAIBLE)

---

#### Risque D01 : DDoS de traitement de paiement

**Description de la menace** : Un attaquant inonde l'endpoint de paiement pour épuiser le quota API Stripe ou les ressources backend.

**Score de risque inhérent** : **60**

**Détail de la notation** :
- **Criticité** : 4 (Élevée) - Perte de revenus pendant Black Friday/Cyber Monday
- **Probabilité** : 3 (Moyenne) - Attaques DDoS courantes pour e-commerce
- **Exposition** : 5 (Maximale) - Endpoint public, limitation de débit minimale initialement
- **Calcul** : 4 × 3 × 5 = **60**

**Contre-mesures implémentées** :
1. ✅ Limitation de débit (5 tentatives de paiement/minute par utilisateur)
2. ✅ CAPTCHA pour comportement suspect (3+ paiements échoués)
3. ⏳ **PRÉVU** : WAF avec protection DDoS (Cloudflare - prévu pour 2025-12-10)
4. ⏳ **PRÉVU** : Mise en cache d'intent de paiement (réutilisation d'intent si requête dupliquée)

**Score de risque résiduel** : **30** (MOYEN - atténuation partielle)
- **Criticité** : 4 (inchangé)
- **Probabilité** : 2 (Faible - limitation de débit réduit l'efficacité)
- **Exposition** : 4 (Élevée - toujours face Internet)
- **Calcul** : 4 × 2 × 4 = **32**, ajusté à **30**

**Critères d'acceptation** :
- ⏳ Déployer WAF avant Black Friday (24 novembre 2025)
- ⏳ Test de charge : 1 000 req/s sans dégradation

**Propriétaire du risque** : Responsable DevOps
**Échéance d'atténuation** : 2025-12-10

---

#### Risque E01 : manipulation de rôle JWT

**Score de risque inhérent** : **60**
**Score de risque résiduel** : **6** (FAIBLE)

**Détail de la notation** :
- **Criticité** : 5 (Critique) - Compromission complète du système
- **Probabilité** : 3 (Moyenne - nécessite fuite de clé secrète)
- **Exposition** : 4 (Élevée - tous les endpoints authentifiés)
- **Calcul** : 5 × 3 × 4 = **60**

**Contre-mesures** :
- ✅ JWT signé avec RS256 (asymétrique, pas HS256)
- ✅ Clé privée dans variable d'environnement sécurisée (pas codée en dur)
- ✅ Validation de rôle sur chaque endpoint admin (pas juste claim JWT)
- ✅ Vérification d'algorithme (rejet jetons non signés)

**Résiduel** : 5 × 1 × 1 = **5**, ajusté à **6**

---

#### Risque S01 : vol de jeton JWT via XSS

**Score de risque inhérent** : **48**
**Score de risque résiduel** : **12** (FAIBLE)

**Détail de la notation** :
- **Criticité** : 4 (Élevée) - Prise de contrôle de compte
- **Probabilité** : 3 (Moyenne - vulnérabilités XSS courantes)
- **Exposition** : 4 (Élevée - affecte tous les utilisateurs)
- **Calcul** : 4 × 3 × 4 = **48**

**Contre-mesures** :
- ✅ JWT en mémoire uniquement (pas localStorage)
- ✅ Cookies HTTP-only pour jetons de rafraîchissement
- ✅ Expiration courte du jeton (15 minutes)
- ✅ En-têtes CSP empêchant XSS
- ⏳ Empreinte digitale d'appareil (prévu)

**Résiduel** : 4 × 1 × 3 = **12** (FAIBLE)

---

#### Risque D02 : épuisement du pool de connexions base de données

**Score de risque inhérent** : **48**
**Score de risque résiduel** : **12** (FAIBLE)

**Détail de la notation** :
- **Criticité** : 4 (Élevée) - Panne de service
- **Probabilité** : 3 (Moyenne)
- **Exposition** : 4 (Élevée)
- **Calcul** : 4 × 3 × 4 = **48**

**Contre-mesures** :
- ✅ Pool de connexions (max 20, débordement en file d'attente)
- ✅ Timeout de requête (30s)
- ✅ Limitation de débit (100 req/min par IP)
- ⏳ Réplicas de lecture base de données (prévu)

**Résiduel** : 4 × 1 × 3 = **12** (FAIBLE)

---

### 3.3 Risques MOYENS (Score 20-47)

#### Risque R01 : répudiation de commande (utilisateur nie avoir passé commande)

**Score de risque inhérent** : **36**
**Score de risque résiduel** : **24** (MOYEN - atténuation partielle)

**Détail de la notation** :
- **Criticité** : 3 (Moyenne) - Rétrofacturations, pertes dues à la fraude
- **Probabilité** : 3 (Moyenne - fraude de rétrofacturation courante)
- **Exposition** : 4 (Élevée - toutes les commandes)
- **Calcul** : 3 × 3 × 4 = **36**

**Contre-mesures implémentées** :
- ✅ Journaux d'audit (timestamp, IP, User-Agent)
- ✅ Confirmation par email
- ✅ SMS/2FA pour commandes de valeur élevée (>500$)
- ⏳ **PRÉVU** : Empreinte digitale d'appareil
- ⏳ **PRÉVU** : Service de vérification d'adresse

**Score de risque résiduel** : **24**
- **Criticité** : 3 (inchangé)
- **Probabilité** : 2 (Faible - piste d'audit dissuade la fraude)
- **Exposition** : 4 (Élevée - toujours toutes les commandes)
- **Calcul** : 3 × 2 × 4 = **24** (MOYEN)

**Acceptation** : Risque accepté pour MVP. Contrôles supplémentaires prévus pour v1.1 (empreinte digitale d'appareil).

**Propriétaire du risque** : Product Manager

---

#### Risque T03 : race condition de paiement

**Score de risque inhérent** : **32**
**Score de risque résiduel** : **6** (FAIBLE)

**Notation** : 4 × 2 × 4 = **32** → 4 × 1 × 1 = **4** (FAIBLE après transactions DB)

---

#### Risque S02 : détournement de session via MITM

**Score de risque inhérent** : **30**
**Score de risque résiduel** : **3** (FAIBLE)

**Notation** : 3 × 2 × 5 = **30** → 3 × 1 × 1 = **3** (FAIBLE après TLS 1.3, HSTS)

---

#### Risque R02 : répudiation de changement de config

**Score de risque inhérent** : **24**
**Score de risque résiduel** : **12** (FAIBLE)

**Notation** : 4 × 2 × 3 = **24** → 4 × 1 × 3 = **12** (FAIBLE après journaux d'audit admin)

---

#### Risque S03 : manipulation de jeton Stripe

**Score de risque inhérent** : **20**
**Score de risque résiduel** : **2** (FAIBLE)

**Notation** : 4 × 1 × 5 = **20** → 4 × 1 × 1 = **4**, ajusté à **2** (intents de paiement Stripe)

---

### 3.4 Risques FAIBLES (Score 1-19)

#### Risque E03 : accès admin par path traversal

**Score de risque inhérent** : **18**
**Score de risque résiduel** : **2** (FAIBLE)

**Notation** : 3 × 2 × 3 = **18** → 3 × 1 × 1 = **3**, ajusté à **2**

---

#### Risque I03 : énumération d'adresse via attaque temporelle

**Score de risque inhérent** : **10**
**Score de risque résiduel** : **5** (FAIBLE)

**Notation** : 2 × 1 × 5 = **10** → 2 × 1 × 1 = **2**, ajusté à **5**

---

## 4. Plan d'atténuation priorisé

### 4.1 Blocages pré-production (risque résiduel CRITIQUE/ÉLEVÉ)

| Priorité | ID risque | Menace | Risque résiduel | Action requise | Propriétaire | Échéance |
|----------|---------|--------|---------------|-----------------|-------|----------|
| 1 | D01 | DDoS de flood de paiement | 30 (MOYEN) | Déployer WAF (Cloudflare) | Responsable DevOps | 2025-12-10 |
| 2 | R01 | Répudiation de commande | 24 (MOYEN) | Implémenter empreinte digitale d'appareil | Responsable backend | 2026-01-15 |

**Note** : Les deux sont à risque résiduel MOYEN, mais critiques pour la préparation à la production.

### 4.2 Améliorations post-production (Sprint 2)

| ID risque | Menace | Action | Propriétaire | Sprint |
|---------|--------|--------|-------|--------|
| D01 | DDoS de paiement | Mise en cache d'intent de paiement (idempotence) | Responsable backend | Sprint 2 |
| R01 | Répudiation de commande | Service de vérification d'adresse | Responsable backend | Sprint 2 |
| S01 | Vol de JWT | Empreinte digitale d'appareil | Architecte sécurité | Sprint 2 |

---

## 5. Évaluation du risque résiduel

### 5.1 Carte de chaleur des risques résiduels

```
                    EXPOSITION
                1   2   3   4   5
              ┌───┬───┬───┬───┬───┐
          5   │   │   │   │I01│   │  CRITICITÉ
          4   │   │   │   │   │   │
          3   │   │   │   │   │   │
          2   │   │   │   │   │   │
          1   │I03│E03│S01│D01│   │
P         0   │   │E01│I02│R01│   │
R             │   │S02│D02│   │   │
O             │   │T01│   │   │   │
B             │   │T02│   │   │   │
A             │   │T03│   │   │   │
B             │   │E02│   │   │   │
I             └───┴───┴───┴───┴───┘
L
I
T
É
```

**Interprétation** :
- La plupart des risques migrent vers le quadrant **FAIBLE** (en bas à gauche)
- **2 risques** restent dans **MOYEN** (D01, R01) - nécessitent contrôles supplémentaires
- **Aucun risque** dans zones ÉLEVÉ ou CRITIQUE après atténuation

---

## 6. Décisions d'acceptation de risques

### 6.1 Risques acceptés (résiduel MOYEN)

#### Risque accepté : D01 (DDoS de paiement) - Score résiduel 30

**Justification** :
- Déploiement WAF planifié pour 2025-12-10 (avant trafic Black Friday)
- Limitation de débit actuelle fournit protection de base pour lancement MVP
- Impact financier : Perte de revenus max estimée 10 000$ pour panne d'1 heure (acceptable pour MVP)

**Critères d'acceptation** :
- ✅ Limitation de débit active (5 req/min par utilisateur)
- ✅ CAPTCHA implémenté
- ✅ Alertes de surveillance configurées (latence > 5s déclenche alerte)

**Approuvé par** : CTO (2025-11-18)

**Conditions** :
- WAF doit être déployé avant 2025-12-10
- Si déployé avant lancement MVP, le risque devient obsolète

---

#### Risque accepté : R01 (répudiation de commande) - Score résiduel 24

**Justification** :
- Fraude de rétrofacturation est risque standard de l'industrie (0,5-1% des transactions)
- Piste d'audit fournit preuve solide pour résolution de litiges
- Amélioration empreinte digitale d'appareil prévue pour v1.1

**Critères d'acceptation** :
- ✅ Confirmation par email envoyée (utilisateur ne peut pas prétendre aucune notification)
- ✅ SMS/2FA pour commandes >500$ (réduit fraude haute valeur)
- ✅ Journaux d'audit capturent IP, timestamp, User-Agent

**Approuvé par** : CFO (2025-11-18)

**Impact financier** : Rétrofacturations estimées 5 000$/mois (0,5% de 1M$ revenus mensuels) - acceptable

---

## 7. Surveillance et revue

### 7.1 Métriques de surveillance des risques

| ID risque | Métrique de surveillance | Seuil d'alerte | Tableau de bord |
|---------|-------------------|-----------------|-----------|
| D01 | Latence endpoint paiement | >5s moyenne | Datadog |
| D01 | Taux d'échec paiement | >10% | Tableau de bord Stripe |
| R01 | Nombre de rétrofacturations | >50/mois | Tableau de bord finances |
| R01 | Taux de rétrofacturation | >1% des transactions | Tableau de bord finances |
| T01 | Journaux d'écart de prix | >10/jour | Journaux application |
| I02 | Nombre 403 Forbidden | >100/heure (tentatives IDOR potentielles) | Journaux WAF |

### 7.2 Calendrier de revue des risques

- **Hebdomadaire** : Revue du tableau de bord des métriques de risque par champion sécurité
- **Mensuelle** : Revue des tendances de rétrofacturation par équipe produit (R01)
- **Trimestrielle** : Réévaluation complète des risques (actualisation modèle de menaces + analyse de risques)
- **Annuelle** : Test d'intrusion externe et audit de conformité

### 7.3 Événements déclencheurs pour réévaluation des risques

L'analyse de risques sera mise à jour si :

1. **Nouvelle menace identifiée** (ex : vulnérabilité zero-day dans Stripe.js)
2. **Changement d'architecture** (ex : migration de Stripe vers un autre processeur de paiement)
3. **Changement réglementaire** (ex : nouvelle version PCI-DSS)
4. **Incident de sécurité** (ex : tentative réelle d'injection SQL détectée)
5. **Changement business** (ex : augmentation de trafic x10 nécessite réévaluation DDoS)

---

## 8. Correspondance de conformité

### 8.1 Couverture des risques PCI-DSS

| Exigence | Risques traités | Atténuations |
|-------------|-----------------|-------------|
| 6.5.1 (Failles d'injection) | T02 (Injection SQL) | Requêtes paramétrées, validation d'entrée |
| 6.5.3 (Cryptographie non sécurisée) | S02 (Détournement de session) | TLS 1.3, HSTS |
| 6.5.10 (Authentification cassée) | S01 (Vol JWT), E01 (Manipulation de rôle) | Signatures RS256, expiration courte, validation de rôle |
| 12.10 (Réponse aux incidents) | R01 (Répudiation de commande), R02 (Répudiation de changement de config) | Journaux d'audit, alertes de surveillance |

### 8.2 Couverture des risques RGPD

| Article | Risques traités | Atténuations |
|---------|-----------------|-------------|
| Art. 32 (Sécurité du traitement) | I01 (Données de carte), I02 (IDOR données utilisateur) | Chiffrement (TLS), contrôles d'accès |
| Art. 5 (Minimisation des données) | I02 (Exposition données utilisateur) | Scopes d'autorisation, réponses API minimales |
| Art. 30 (Registres de traitement) | R01, R02 (Répudiation) | Journaux d'audit, confirmations par email |

---

## 9. Validation de l'analyse de risques

### 9.1 Revue par les pairs

- **Revu par** : Champion de sécurité principal, Architecte champion de sécurité
- **Date de revue** : 2025-11-18
- **Commentaires de revue** :
  - ✅ Méthodologie de notation cohérente avec le cadre constitutionnel de risques
  - ✅ Évaluations de risque résiduel validées par tests de sécurité
  - ✅ Décisions d'acceptation de risque appropriées pour portée MVP
  - ⏳ Recommandation de réévaluation trimestrielle après lancement en production

### 9.2 Approbation des parties prenantes

| Partie prenante | Rôle | Statut d'approbation | Date |
|-------------|------|-----------------|------|
| CTO | Approbation technique | ✅ Approuvé (avec conditions) | 2025-11-18 |
| CFO | Acceptation des risques financiers | ✅ Approuvé | 2025-11-18 |
| Champion de sécurité principal | Validation de sécurité | ✅ Approuvé | 2025-11-18 |
| Product Manager | Évaluation d'impact business | ✅ Approuvé | 2025-11-18 |

**Conditions pour mise en production** :
1. ✅ Tous les risques inhérents CRITIQUES et ÉLEVÉS atténués à résiduel FAIBLE ou MOYEN
2. ⏳ Déploiement WAF complété avant 2025-12-10
3. ⏳ Test d'intrusion complété sans découverte ÉLEVÉE

---

## 10. Prochaines étapes

1. ✅ **Analyse de risques complète** - Ce document
2. ⏳ **Implémenter atténuations prévues** - Déploiement WAF, empreinte digitale d'appareil
3. ⏳ **Test d'intrusion** - Valider hypothèses de risque résiduel (planifié 2025-12-01)
4. ⏳ **Déploiement en production** - Après validation test d'intrusion
5. ⏳ **Surveillance post-lancement** - Suivre tableau de bord des métriques de risque

---

## Annexe A : référence de notation de risques

### A.1 Carte de référence rapide

```
Score de risque = Criticité × Probabilité × Exposition

CRITIQUE : 80-125 → Action immédiate, bloque production
ÉLEVÉ :    48-79  → Atténuer avant production
MOYEN :    20-47  → Atténuer dans le prochain sprint
FAIBLE :   1-19   → Surveiller et suivre
```

### A.2 Feuille de calcul de notation

Pour chaque menace :

1. **Identifier la menace** depuis le modèle de menaces
2. **Évaluer la criticité** (1-5) : Quel est l'impact business ?
3. **Évaluer la probabilité** (1-5) : Quelle est la vraisemblance d'exploitation ?
4. **Évaluer l'exposition** (1-5) : Combien d'utilisateurs/systèmes affectés ?
5. **Calculer le score** : C × P × E
6. **Déterminer le niveau de risque** : Mapper le score aux seuils
7. **Identifier les contre-mesures** : Contrôles existants + prévus
8. **Recalculer le risque résiduel** : Ajuster P et E après atténuations

---

**Modèle utilisé** : [risk-scoring-template-planning.md](../../templates/02-risk-analysis/risk-scoring-template-planning.md)
**Documents liés** :
- [Modèle de menaces](../01-threat-modeling/_exemple-ecommerce-stride.md)
- [Exigences de sécurité](../../templates/03-security-requirements/authentication-requirements-template-design.md)
