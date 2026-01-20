---
title: "Checklist sécurité OWASP ASVS - Phase de conception"
template_version: "1.0.0"
constitutional_principle: "III - Security by Design"
ssdlc_phase: "conception"
last_updated: "2025-01-15"
reviewers:
  - security-champion-team
  - architecture-team
description: "Checklist OWASP Application Security Verification Standard (ASVS) pour phase conception. Aide équipes identifier et documenter contrôles sécurité requis selon niveau risque application."
tags:
  - owasp-asvs
  - security-requirements
  - security-controls
  - design
difficulty: "intermédiaire"
estimated_time: "2-3 heures"
prerequisites:
  - "Modèle menaces et analyse risques complétés"
  - "Compréhension architecture application"
related_templates:
  - "authentication-requirements-template-design.md"
  - "authorization-requirements-template-design.md"
  - "encryption-requirements-template-design.md"
compliance_frameworks:
  - "OWASP ASVS 4.0"
  - "PCI-DSS"
  - "ISO 27001"
---

# Checklist sécurité OWASP ASVS - Phase de conception

## Aperçu

### Objectif

L'OWASP Application Security Verification Standard (ASVS) fournit cadre complet pour exigences sécurité. Cette checklist aide équipes à :
- Identifier contrôles sécurité requis selon niveau risque application
- Documenter exigences sécurité pendant phase conception
- Assurer couverture sécurité complète
- Préparer tests vérification sécurité

### Niveaux ASVS

| Niveau | Description | Cas d'usage |
|--------|-------------|-------------|
| **Niveau 1** | Sécurité basique | Applications faible risque, outils internes |
| **Niveau 2** | Sécurité standard | Plupart applications web, risque modéré |
| **Niveau 3** | Sécurité avancée | Applications haut risque (finance, santé), infrastructure critique |

**Sélectionner niveau application** : ☐ Niveau 1 / ☑ Niveau 2 / ☐ Niveau 3

---

## 1. Architecture, conception et modélisation menaces (V1)

### V1.1 SDLC sécurisé

| ID | Exigence | L1 | L2 | L3 | Statut | Notes implémentation |
|----|----------|----|----|----|----|---------------------|
| 1.1.1 | SDLC sécurisé avec activités sécurité toutes phases | | ✓ | ✓ | ☑ | Utilisation SSDLC Toolkit |
| 1.1.2 | Modélisation menaces pour changements conception significatifs | | ✓ | ✓ | ☑ | Modèle menaces STRIDE complété |
| 1.1.3 | Documentation architecture sécurité | | ✓ | ✓ | ☐ | Planifié Sprint 2 |
| 1.1.4 | Inventaire composants avec versions et licences | | ✓ | ✓ | ☐ | Outil SCA planifié |
| 1.1.5 | Définition et application exigences sécurité | | ✓ | ✓ | ☑ | Cette checklist |
| 1.1.6 | Directives codage sécurisé | | ✓ | ✓ | ☐ | Planifié Sprint 3 |

### V1.2 Architecture authentification

| ID | Exigence | L1 | L2 | L3 | Statut | Notes implémentation |
|----|----------|----|----|----|----|---------------------|
| 1.2.1 | Utiliser mécanismes authentification éprouvés industrie | ✓ | ✓ | ✓ | ☑ | OAuth 2.0 + OIDC |
| 1.2.2 | Contrôles authentification appliqués côté serveur | ✓ | ✓ | ✓ | ☑ | Validation session côté serveur |
| 1.2.3 | Contrôle authentification unique vérifié | | ✓ | ✓ | ☑ | Service Auth centralisé |
| 1.2.4 | Stockage mots de passe sécurisé avec algorithmes modernes | ✓ | ✓ | ✓ | ☑ | bcrypt facteur travail 12 |

### V1.4 Architecture contrôle d'accès

| ID | Exigence | L1 | L2 | L3 | Statut | Notes implémentation |
|----|----------|----|----|----|----|---------------------|
| 1.4.1 | Points application fiables | ✓ | ✓ | ✓ | ☑ | Passerelle API + Couche Service |
| 1.4.2 | Contrôle accès basé attributs ou fonctionnalités | | ✓ | ✓ | ☑ | RBAC avec attributs |
| 1.4.3 | Principe moindre privilège | | ✓ | ✓ | ☐ | Conception en cours |
| 1.4.4 | Contrôle accès niveau microservice/fonction | | | ✓ | ☐ | Amélioration future |
| 1.4.5 | Refus par défaut pour contrôle accès | | ✓ | ✓ | ☑ | Autorisation explicite requise |

---

## 2. Authentification (V2)

### V2.1 Sécurité mots de passe

| ID | Exigence | L1 | L2 | L3 | Statut | Notes implémentation |
|----|----------|----|----|----|----|---------------------|
| 2.1.1 | Mots de passe 12+ caractères ou phrases de passe 64+ | ✓ | ✓ | ✓ | ☑ | Min 12, max 128 caractères |
| 2.1.2 | Mots de passe 64+ caractères supportés | ✓ | ✓ | ✓ | ☑ | Max 128 caractères |
| 2.1.3 | Pas de troncation mot de passe | ✓ | ✓ | ✓ | ☑ | Mot de passe complet stocké (haché) |
| 2.1.4 | Caractères Unicode autorisés mots de passe | ✓ | ✓ | ✓ | ☑ | Support UTF-8 |
| 2.1.5 | Utilisateurs peuvent changer mot de passe | ✓ | ✓ | ✓ | ☑ | Endpoint /account/password |
| 2.1.6 | Changement mot de passe nécessite mot de passe actuel | ✓ | ✓ | ✓ | ☑ | Implémenté |
| 2.1.7 | Mots de passe vérifiés contre bases violations | | ✓ | ✓ | ☐ | Planifié : API HaveIBeenPwned |
| 2.1.8 | Indicateur force mot de passe | | ✓ | ✓ | ☐ | Planifié pour UI |
| 2.1.9 | Pas de règles composition mot de passe | | ✓ | ✓ | ☑ | Basé longueur uniquement |
| 2.1.10 | Pas de changements mots de passe périodiques | | ✓ | ✓ | ☑ | Uniquement lors compromission |
| 2.1.11 | Fonctionnalité coller champs mot de passe | ✓ | ✓ | ✓ | ☑ | Activée |
| 2.1.12 | Option afficher/masquer mot de passe | ✓ | ✓ | ✓ | ☑ | Basculer visibilité |

### V2.2 Sécurité authentificateur général

| ID | Exigence | L1 | L2 | L3 | Statut | Notes implémentation |
|----|----------|----|----|----|----|---------------------|
| 2.2.1 | Contrôles anti-automation | ✓ | ✓ | ✓ | ☐ | Rate limiting planifié |
| 2.2.2 | MFA pour fonctions admin | | ✓ | ✓ | ☐ | Planifié T1 |
| 2.2.3 | MFA pour tous utilisateurs | | | ✓ | ☐ | Futur (L3 uniquement) |
| 2.2.4 | Récupération compte pas plus faible qu'auth primaire | | ✓ | ✓ | ☐ | Email + questions sécurité |
| 2.2.5 | Enregistrement appareil OTP/MFA nécessite auth | | | ✓ | ☐ | Futur |
| 2.2.6 | Vérifier inscription appareil OTP | | | ✓ | ☐ | Futur |

### V2.7 Vérificateur hors bande

| ID | Exigence | L1 | L2 | L3 | Statut | Notes implémentation |
|----|----------|----|----|----|----|---------------------|
| 2.7.1 | OTP valide usage unique uniquement | | ✓ | ✓ | ☑ | Jetons usage unique Redis |
| 2.7.2 | Générateurs OTP secret seed | | ✓ | ✓ | ☐ | TOTP planifié |
| 2.7.3 | Algorithmes cryptographiques approuvés OTP | | ✓ | ✓ | ☐ | HOTP/TOTP (RFC 6238) |

---

## 3. Gestion de session (V3)

### V3.1 Sécurité fondamentale gestion session

| ID | Exigence | L1 | L2 | L3 | Statut | Notes implémentation |
|----|----------|----|----|----|----|---------------------|
| 3.1.1 | IDs session réutilisables pas dans URLs | ✓ | ✓ | ✓ | ☑ | Sessions basées cookie |
| 3.1.2 | Déconnexion termine session | ✓ | ✓ | ✓ | ☑ | Session invalidée |
| 3.1.3 | Timeout session ou avertissement déconnexion | ✓ | ✓ | ✓ | ☐ | Timeout inactivité 30 min planifié |

### V3.2 Liaison session

| ID | Exigence | L1 | L2 | L3 | Statut | Notes implémentation |
|----|----------|----|----|----|----|---------------------|
| 3.2.1 | Nouvelle session lors authentification | ✓ | ✓ | ✓ | ☑ | Régénération session |
| 3.2.2 | Jetons session entropie 64+ bits | ✓ | ✓ | ✓ | ☑ | 128-bit aléatoire sécurisé |
| 3.2.3 | Jetons session dans cookies | ✓ | ✓ | ✓ | ☑ | HttpOnly, Secure, SameSite |
| 3.2.4 | Jetons session CSPRNG | | ✓ | ✓ | ☑ | crypto.randomBytes() |

### V3.3 Timeout session

| ID | Exigence | L1 | L2 | L3 | Statut | Notes implémentation |
|----|----------|----|----|----|----|---------------------|
| 3.3.1 | Déconnexion invalide toutes sessions | | | ✓ | ☐ | Session unique actuellement |
| 3.3.2 | Ré-auth pour transactions sensibles | | ✓ | ✓ | ☐ | Planifié paiements |
| 3.3.3 | Timeout inactivité 30 minutes ou moins | | ✓ | ✓ | ☐ | implémentation 30 min |
| 3.3.4 | Timeout absolu 12 heures ou moins | | | ✓ | ☐ | implémentation 24h (nécessite réduction) |

---

## 4. Contrôle d'accès (V4)

### V4.1 Conception générale contrôle d'accès

| ID | Exigence | L1 | L2 | L3 | Statut | Notes implémentation |
|----|----------|----|----|----|----|---------------------|
| 4.1.1 | Points application fiables | ✓ | ✓ | ✓ | ☑ | Autorisation côté serveur |
| 4.1.2 | Contrôle accès basé attributs | | ✓ | ✓ | ☑ | RBAC avec attributs utilisateur |
| 4.1.3 | Principe moindre privilège | | ✓ | ✓ | ☐ | Refus par défaut implémenté |
| 4.1.4 | Surcharge contrôle accès par transaction | | | ✓ | ☐ | Non requis |
| 4.1.5 | Refus par défaut | ✓ | ✓ | ✓ | ☑ | Permissions explicites requises |

### V4.2 Contrôle d'accès niveau opération

| ID | Exigence | L1 | L2 | L3 | Statut | Notes implémentation |
|----|----------|----|----|----|----|---------------------|
| 4.2.1 | Données sensibles et APIs protégées | ✓ | ✓ | ✓ | ☑ | Authentification requise |
| 4.2.2 | Pas de références objet directes non sécurisées | ✓ | ✓ | ✓ | ☐ | implémentation références UUID |

### V4.3 Autres considérations contrôle d'accès

| ID | Exigence | L1 | L2 | L3 | Statut | Notes implémentation |
|----|----------|----|----|----|----|---------------------|
| 4.3.1 | Fonctions admin ségrégées | ✓ | ✓ | ✓ | ☑ | Portail admin séparé |
| 4.3.2 | Listage répertoire désactivé | ✓ | ✓ | ✓ | ☑ | Config Nginx |
| 4.3.3 | Métadonnées contrôle accès non manipulables | | ✓ | ✓ | ☑ | Rôles côté serveur |

---

## 5. Validation, assainissement et encodage (V5)

### V5.1 Validation entrée

| ID | Exigence | L1 | L2 | L3 | Statut | Notes implémentation |
|----|----------|----|----|----|----|---------------------|
| 5.1.1 | Validation positive (liste autorisée) | ✓ | ✓ | ✓ | ☐ | implémentation validateurs |
| 5.1.2 | Données structurées fortement typées | ✓ | ✓ | ✓ | ☑ | Schémas TypeScript/Pydantic |
| 5.1.3 | Validation URL avant traitement | ✓ | ✓ | ✓ | ☐ | Bibliothèque analyse URL |
| 5.1.4 | Validation upload fichier | ✓ | ✓ | ✓ | ☐ | Type MIME + magic bytes |
| 5.1.5 | Pas eval() ou réflexion code sur données non fiables | ✓ | ✓ | ✓ | ☑ | Règles linting appliquées |

### V5.2 Assainissement et sandboxing

| ID | Exigence | L1 | L2 | L3 | Statut | Notes implémentation |
|----|----------|----|----|----|----|---------------------|
| 5.2.1 | Assainissement HTML | ✓ | ✓ | ✓ | ☐ | Bibliothèque DOMPurify |
| 5.2.2 | Assainissement éditeur HTML | | ✓ | ✓ | ☐ | Non applicable (pas d'éditeur) |
| 5.2.3 | Encodage sortie sensible contexte XSS | ✓ | ✓ | ✓ | ☑ | Auto-échappement React |
| 5.2.4 | Paramétrisation requêtes base données | ✓ | ✓ | ✓ | ☑ | ORM avec requêtes paramétrées |

### V5.3 Encodage sortie et prévention injection

| ID | Exigence | L1 | L2 | L3 | Statut | Notes implémentation |
|----|----------|----|----|----|----|---------------------|
| 5.3.1 | Encodage sortie sensible contexte | ✓ | ✓ | ✓ | ☑ | Échappement moteur template |
| 5.3.2 | Protection injection template | ✓ | ✓ | ✓ | ☑ | Templates sandboxés |
| 5.3.3 | Prévention injection SQL | ✓ | ✓ | ✓ | ☑ | Requêtes paramétrées uniquement |
| 5.3.4 | Prévention injection commande OS | ✓ | ✓ | ✓ | ☑ | Pas exécution shell entrée utilisateur |
| 5.3.5 | Prévention XSS | ✓ | ✓ | ✓ | ☑ | CSP + encodage sortie |
| 5.3.6 | Prévention injection XML | ✓ | ✓ | ✓ | ☐ | Non applicable (pas XML) |
| 5.3.7 | Prévention injection JSON | ✓ | ✓ | ✓ | ☑ | Échappement JSON.stringify |

---

## 6. Cryptographie (V6)

### V6.2 Algorithmes

| ID | Exigence | L1 | L2 | L3 | Statut | Notes implémentation |
|----|----------|----|----|----|----|---------------------|
| 6.2.1 | Algorithmes cryptographiques éprouvés industrie | ✓ | ✓ | ✓ | ☑ | AES-256-GCM, RSA-4096, SHA-256 |
| 6.2.2 | Pas modes non sécurisés (ECB, etc.) | ✓ | ✓ | ✓ | ☑ | Mode GCM pour AES |
| 6.2.3 | Nombres aléatoires sécurisés (CSPRNG) | ✓ | ✓ | ✓ | ☑ | crypto.randomBytes() |
| 6.2.4 | Pas crypto déprécié/personnalisé | ✓ | ✓ | ✓ | ☑ | Bibliothèques standard uniquement |
| 6.2.5 | Nombres aléatoires non sécurisés pas pour sécurité | ✓ | ✓ | ✓ | ☑ | Math.random() évité |
| 6.2.6 | Nonces, IVs uniques par chiffrement | | ✓ | ✓ | ☑ | Générés par opération |

### V6.3 Valeurs aléatoires

| ID | Exigence | L1 | L2 | L3 | Statut | Notes implémentation |
|----|----------|----|----|----|----|---------------------|
| 6.3.1 | CSPRNG pour secrets | ✓ | ✓ | ✓ | ☑ | crypto.randomBytes() |
| 6.3.2 | UUID aléatoires utilisent CSPRNG approuvé | | ✓ | ✓ | ☑ | uuid.v4() |

---

## 7. Gestion erreurs et journalisation (V7)

### V7.1 Contenu logs

| ID | Exigence | L1 | L2 | L3 | Statut | Notes implémentation |
|----|----------|----|----|----|----|---------------------|
| 7.1.1 | Pas de données sensibles dans logs | ✓ | ✓ | ✓ | ☐ | Nettoyage logs planifié |
| 7.1.2 | Événements sécurité loggés | | ✓ | ✓ | ☐ | Auth, authz, validation entrée |
| 7.1.3 | Transactions haute valeur loggées | | ✓ | ✓ | ☐ | Opérations paiement |
| 7.1.4 | Contrôles intégrité log | | | ✓ | ☐ | Signature logs planifiée |

### V7.2 Traitement logs

| ID | Exigence | L1 | L2 | L3 | Statut | Notes implémentation |
|----|----------|----|----|----|----|---------------------|
| 7.2.1 | Prévention injection log | | ✓ | ✓ | ☐ | Journalisation structurée (JSON) |
| 7.2.2 | Protection logs anti-altération | | | ✓ | ☐ | SIEM avec vérifications intégrité |

### V7.3 Protection logs

| ID | Exigence | L1 | L2 | L3 | Statut | Notes implémentation |
|----|----------|----|----|----|----|---------------------|
| 7.3.1 | Logs encodés pour visionneur | | ✓ | ✓ | ☑ | Logs structurés JSON |
| 7.3.2 | Contrôles accès sur logs | | ✓ | ✓ | ☑ | Accès admin uniquement |
| 7.3.3 | Logs audit protégés | | | ✓ | ☐ | Stockage immuable planifié |
| 7.3.4 | Synchronisation temporelle | | ✓ | ✓ | ☑ | NTP configuré |

### V7.4 Gestion erreurs

| ID | Exigence | L1 | L2 | L3 | Statut | Notes implémentation |
|----|----------|----|----|----|----|---------------------|
| 7.4.1 | Messages erreur génériques aux utilisateurs | ✓ | ✓ | ✓ | ☐ | implémentation messages génériques |
| 7.4.2 | Gestion exceptions dans tout code | ✓ | ✓ | ✓ | ☑ | Blocs try-catch |
| 7.4.3 | Gestionnaire erreur "dernier recours" sécurisé | | ✓ | ✓ | ☑ | Gestionnaire erreur global |

---

## 8. Protection données (V8)

### V8.1 Protection générale données

| ID | Exigence | L1 | L2 | L3 | Statut | Notes implémentation |
|----|----------|----|----|----|----|---------------------|
| 8.1.1 | Données classifiées par sensibilité | | ✓ | ✓ | ☐ | Inventaire données en cours |
| 8.1.2 | Protection proportionnelle à classification | | ✓ | ✓ | ☐ | Exigences chiffrement définies |
| 8.1.3 | Règles rétention données appliquées | | ✓ | ✓ | ☐ | Politique rétention 7 ans |
| 8.1.4 | Suppression données vérifiée | | | ✓ | ☐ | Suppression soft actuellement |

### V8.2 Protection données côté client

| ID | Exigence | L1 | L2 | L3 | Statut | Notes implémentation |
|----|----------|----|----|----|----|---------------------|
| 8.2.1 | Données sensibles pas en cache | ✓ | ✓ | ✓ | ☐ | En-têtes Cache-Control |
| 8.2.2 | Données sensibles retirées stockage client | ✓ | ✓ | ✓ | ☑ | Pas localStorage données sensibles |
| 8.2.3 | Chiffrement stockage côté client (mobile) | | ✓ | ✓ | ☐ | Non applicable (app web) |

### V8.3 Données privées sensibles

| ID | Exigence | L1 | L2 | L3 | Statut | Notes implémentation |
|----|----------|----|----|----|----|---------------------|
| 8.3.1 | Données sensibles pas loggées | ✓ | ✓ | ✓ | ☐ | Nettoyage logs pour PII |
| 8.3.2 | Données sensibles pas dans URLs | ✓ | ✓ | ✓ | ☑ | Requêtes POST données sensibles |
| 8.3.3 | Données sensibles pas paramètres GET | ✓ | ✓ | ✓ | ☑ | POST/headers uniquement |
| 8.3.4 | Données sensibles masquées UI | ✓ | ✓ | ✓ | ☐ | Masquage cartes crédit, mots de passe |
| 8.3.5 | Mémoire effacée après usage données sensibles | | ✓ | ✓ | ☐ | Mise à zéro variables planifiée |
| 8.3.6 | Mots de passe obscurcis dumps mémoire | | | ✓ | ☐ | Amélioration future |
| 8.3.7 | Clés chiffrement protégées en mémoire | | | ✓ | ☐ | HSM pour production |
| 8.3.8 | Clés sensibles tournées régulièrement | | | ✓ | ☐ | Rotation 90 jours planifiée |

---

## 9. Communications (V9)

### V9.1 Sécurité communication client

| ID | Exigence | L1 | L2 | L3 | Statut | Notes implémentation |
|----|----------|----|----|----|----|---------------------|
| 9.1.1 | TLS pour toutes connexions | ✓ | ✓ | ✓ | ☑ | HTTPS appliqué |
| 9.1.2 | TLS 1.2+ uniquement | ✓ | ✓ | ✓ | ☑ | TLS 1.3 préféré |
| 9.1.3 | Suites cipher fortes uniquement | ✓ | ✓ | ✓ | ☑ | Config cipher moderne |

### V9.2 Sécurité communication serveur

| ID | Exigence | L1 | L2 | L3 | Statut | Notes implémentation |
|----|----------|----|----|----|----|---------------------|
| 9.2.1 | Certificats TLS fiables | ✓ | ✓ | ✓ | ☑ | Let's Encrypt |
| 9.2.2 | Vérification certificat TLS | | ✓ | ✓ | ☑ | Validation certificat activée |
| 9.2.3 | TLS mutuel pour microservices | | | ✓ | ☐ | mTLS planifié service mesh |
| 9.2.4 | Vérification révocation certificat | | | ✓ | ☐ | OCSP stapling planifié |

---

## Résumé

### Statut conformité

| Chapitre ASVS | Total exigences | Satisfaites | Planifiées | Non applicable | Conformité % |
|---------------|-----------------|------------|------------|----------------|--------------|
| V1 - Architecture | 15 | 9 | 5 | 1 | 60% |
| V2 - Authentification | 23 | 14 | 7 | 2 | 61% |
| V3 - Gestion Session | 9 | 5 | 4 | 0 | 56% |
| V4 - Contrôle Accès | 8 | 5 | 2 | 1 | 63% |
| V5 - Validation | 17 | 10 | 6 | 1 | 59% |
| V6 - Cryptographie | 9 | 8 | 0 | 1 | 89% |
| V7 - Erreurs & Logs | 11 | 5 | 6 | 0 | 45% |
| V8 - Protection Données | 13 | 3 | 9 | 1 | 23% |
| V9 - Communications | 8 | 6 | 2 | 0 | 75% |
| **TOTAL** | **113** | **65** | **41** | **7** | **58%** |

**Cible pour mise en production** : 90% conformité exigences niveau 2

---

## Actions

### Backlog sprint

| Priorité | Exigence | Propriétaire | Sprint |
|----------|----------|--------------|--------|
| P0 | V5.3.3 - Prévention injection SQL (vérifier toutes requêtes) | Ingénierie | Actuel |
| P0 | V7.4.1 - Messages erreur génériques | Ingénierie | Actuel |
| P1 | V2.2.1 - Rate limiting | Plateforme | Sprint +1 |
| P1 | V3.3.3 - Timeout inactivité 30 min | Ingénierie | Sprint +1 |
| P1 | V8.1.1 - Classification données | sécurité | Sprint +2 |
| P1 | V7.1.2 - Journalisation événements sécurité | Ingénierie | Sprint +2 |

---

## Ressources liées

- [OWASP ASVS 4.0](https://owasp.org/www-project-application-security-verification-standard/)
- [Exigences authentification](authentication-requirements-template-design.md)
- [Exigences autorisation](authorization-requirements-template-design.md)
- [Exigences chiffrement](encryption-requirements-template-design.md)
