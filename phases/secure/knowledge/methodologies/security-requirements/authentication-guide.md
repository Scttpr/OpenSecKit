---
title: "Modèle d'exigences d'authentification - Phase de conception"
template_version: "1.0.0"
constitutional_principle: "III - sécurité par conception"
ssdlc_phase: "conception"
last_updated: "2025-01-15"
reviewers:
  - security-champion-team
description: "Modèle complet d'exigences d'authentification couvrant mots de passe, MFA, gestion de session et SSO. Aide les équipes à concevoir des systèmes d'authentification sécurisés alignés sur les meilleures pratiques modernes."
tags:
  - authentication
  - identity
  - passwords
  - mfa
  - sso
difficulty: "intermédiaire"
estimated_time: "1-2 heures"
prerequisites:
  - "Compréhension des besoins d'authentification utilisateur"
  - "Modèle de menaces complété"
related_templates:
  - "authorization-requirements-template-design.md"
  - "owasp-asvs-checklist-design.md"
compliance_frameworks:
  - "OWASP ASVS"
  - "NIST SP 800-63B"
---

# Modèle d'exigences d'authentification - Phase de conception

## 1. Stratégie d'authentification

### Méthode d'authentification principale

**Méthode sélectionnée** : ☐ Nom d'utilisateur/Mot de passe ☑ Email/Mot de passe ☐ Téléphone/OTP ☐ SSO uniquement ☐ Sans mot de passe

**Justification** : Basée email pour commodité utilisateur, supporte flux réinitialisation mot de passe

### Authentification multi-facteurs (MFA)

**Application MFA** :

- ☑ Optionnelle pour tous les utilisateurs
- ☐ Requise pour tous les utilisateurs
- ☑ Requise pour administrateurs
- ☑ Requise pour opérations sensibles (paiements, suppression compte)

**Méthodes MFA supportées** :

- ☑ TOTP (Time-based One-Time Password) - Google Authenticator, Authy
- ☐ SMS OTP
- ☑ Email OTP
- ☐ Jetons matériels (YubiKey, etc.)
- ☐ Biométrique (empreinte digitale, reconnaissance faciale)
- ☐ Notifications push

---

## 2. Exigences mots de passe

### Politique mots de passe

| Exigence | Valeur | Justification |
|----------|--------|---------------|
| **Longueur minimale** | 12 caractères | Recommandation NIST SP 800-63B |
| **Longueur maximale** | 128 caractères | Support phrases de passe |
| **Règles complexité** | Aucune | Sécurité basée longueur (pas exigence caractères spéciaux) |
| **Historique mots de passe** | 3 mots de passe précédents | Empêcher réutilisation immédiate |
| **Politique expiration** | Pas d'expiration périodique | Forcer changement uniquement lors compromission |
| **Détection violation** | Oui | Vérifier contre HaveIBeenPwned |
| **Coller autorisé** | Oui | Support gestionnaires mots de passe |
| **Bouton afficher/masquer** | Oui | Améliorer utilisabilité |

### Force mot de passe

- ☑ Indicateur force côté client (bibliothèque zxcvbn)
- ☑ Validation côté serveur exigences minimales
- ☑ Bloquer mots de passe courants (liste top 10 000)
- ☑ Bloquer mots de passe correspondant email/nom utilisateur

---

## 3. Stockage mots de passe

### Algorithme hachage

**Sélectionné** : ☑ bcrypt ☐ scrypt ☐ Argon2 ☐ PBKDF2

**Configuration** :

- **Facteur travail** : 12 (coût bcrypt)
- **Salt** : Salt unique par mot de passe (automatique avec bcrypt)
- **Pepper** : Secret additionnel stocké séparément de base de données

**Format stockage** :

```
$2b$12$[salt][hash]
```

**Rotation mot de passe** : Hash recalculé à connexion si facteur travail obsolète

---

## 4. Flux d'authentification

### Processus connexion

```
1. Utilisateur soumet email + mot de passe
2. Vérification rate limiting (5 tentatives par 15 min par compte)
3. Recherche utilisateur par email (temps constant pour éviter énumération)
4. Vérifier mot de passe (comparaison bcrypt)
5. Si MFA activée :
   a. Générer et envoyer OTP
   b. Utilisateur soumet OTP
   c. Vérifier OTP (usage unique, expiration 10 min)
6. Créer session (cookie sécurisé, HttpOnly)
7. Logger événement authentification réussie
8. Rediriger vers application
```

**Gestion échecs connexion** :

- Message d'erreur générique : "Email ou mot de passe invalide"
- Logger tentative échouée avec IP, user agent
- Incrémenter compteur tentatives échouées
- Après 5 échecs en 15 min : requérir CAPTCHA
- Après 10 échecs en 1 heure : verrouillage temporaire compte (15 min)

---

## 5. Gestion de session

### Jeton session

| Attribut | Valeur |
|----------|--------|
| **Génération jeton** | Aléatoire cryptographiquement sécurisé (128-bit) |
| **Stockage jeton** | Côté serveur (Redis) avec données session |
| **Attributs cookie** | HttpOnly, Secure, SameSite=Lax |
| **Nom cookie** | `__Host-session` (empêche attaques sous-domaine) |
| **Timeout inactivité** | 30 minutes |
| **Timeout absolu** | 24 heures (ré-authentification requise) |

### Cycle de vie session

**Création session** :

1. Lors authentification réussie
2. Ancienne session invalidée (prévention fixation session)
3. Nouvel ID session généré
4. Données session stockées côté serveur

**Validation session** (à chaque requête) :

1. Extraire ID session du cookie
2. Rechercher session dans Redis
3. Valider : non expirée, IP correspond (optionnel), user agent correspond (empreinte)
4. Mettre à jour horodatage dernière activité

**Terminaison session** :

- Déconnexion explicite : supprimer session de Redis
- Timeout inactivité : nettoyage automatique
- Timeout absolu : forcer ré-authentification
- Changement mot de passe : invalider toutes sessions
- Initié admin : permettre révocation session

---

## 6. Récupération compte

### Flux réinitialisation mot de passe

```
1. Utilisateur demande réinitialisation mot de passe (email)
2. Générer jeton réinitialisation sécurisé (256-bit aléatoire)
3. Stocker hash jeton en base avec expiration (1 heure)
4. Envoyer lien réinitialisation par email
5. Utilisateur clique lien dans 1 heure
6. Vérifier validité jeton
7. Permettre saisie nouveau mot de passe (avec exigences force)
8. Invalider toutes sessions existantes
9. Logger événement réinitialisation mot de passe
10. Envoyer email confirmation
```

**Contrôles sécurité** :

- ☑ Jeton réinitialisation usage unique
- ☑ Expiration 1 heure
- ☑ Rate limit : 3 demandes réinitialisation par heure par compte
- ☑ Message succès générique (pas d'énumération utilisateur)
- ☑ Confirmation email après réinitialisation réussie
- ☑ Logger toutes tentatives réinitialisation mot de passe

### Questions récupération compte (non recommandé)

**Statut** : ☐ Implémenté ☑ Non Implémenté

**Justification** : Questions sécurité intrinsèquement faibles (devinables, ingénierie sociale). Utilisation récupération par email uniquement.

---

## 7. Authentification unique (SSO)

### Protocole SSO

**Sélectionné** : ☐ SAML 2.0 ☑ OAuth 2.0 + OIDC ☐ CAS ☐ Non applicable

**Fournisseurs supportés** :

- ☑ Google OAuth
- ☑ GitHub OAuth
- ☐ Microsoft Azure AD
- ☐ Okta
- ☐ Auth0

### Implémentation SSO

**Flux OAuth 2.0** : Authorization Code Flow avec PKCE

**Configuration** :

- **URI redirection** : `https://app.example.com/auth/callback`
- **Scopes** : `openid`, `email`, `profile`
- **Validation ID token** : Signature, émetteur, audience, expiration
- **Liaison compte** : Lier identité SSO au compte existant via email

**Création compte via SSO** :

1. Utilisateur s'authentifie avec fournisseur OAuth
2. Vérifier ID token
3. Vérifier si email existe en base
4. Si oui : lier identité OAuth, connecter
5. Si non : créer nouveau compte avec email d'OAuth, lier identité
6. Pas de mot de passe requis pour comptes SSO uniquement

---

## 8. Protection anti-automation & force brute

### Rate limiting

| Portée | Limite | Fenêtre | Action si dépassement |
|--------|--------|---------|----------------------|
| Par IP | 10 tentatives connexion | 5 minutes | Bloquer 15 minutes |
| Par compte | 5 tentatives connexion | 15 minutes | Requérir CAPTCHA |
| Par compte | 10 tentatives connexion | 1 heure | Verrouiller compte 15 minutes |
| Réinitialisation mot de passe | 3 demandes | 1 heure par compte | Bloquer demandes ultérieures |

### CAPTCHA

**Fournisseur** : ☑ Google reCAPTCHA v3 ☐ hCaptcha ☐ Personnalisé

**Conditions déclenchement** :

- Après 5 tentatives connexion échouées pour compte
- Sur demande réinitialisation mot de passe
- Sur enregistrement compte

**Vérification** :

- Vérification côté serveur réponse CAPTCHA
- Seuil score : 0.5 (reCAPTCHA v3)

---

## 9. Prévention énumération comptes

**Stratégies** :

- ☑ Messages d'erreur génériques ("Email ou mot de passe invalide" pour les deux)
- ☑ Recherche utilisateur temps constant (prévenir attaques timing)
- ☑ Temps réponse identique pour utilisateurs existants/non-existants
- ☑ Message succès générique pour réinitialisation mot de passe ("Si compte existe, email envoyé")
- ☑ Pas de distinction entre "email introuvable" et "mauvais mot de passe"

**Compromis** : UX légèrement réduite (impossible de savoir si email enregistré), sécurité significativement améliorée

---

## 10. Transmission sécurisée

### Exigences TLS

- ☑ HTTPS requis pour tous endpoints authentification
- ☑ HTTP Strict Transport Security (HSTS) activé
  - `Strict-Transport-Security: max-age=31536000; includeSubDomains; preload`
- ☑ TLS 1.3 préféré, TLS 1.2 minimum
- ☑ Suites cipher fortes uniquement (ECDHE-RSA-AES256-GCM-SHA384, etc.)
- ☑ Validation certificat (pas auto-signés en production)

### En-têtes additionnels

- `Content-Security-Policy` : Prévenir XSS
- `X-Frame-Options: DENY` : Prévenir clickjacking
- `X-Content-Type-Options: nosniff` : Prévenir MIME sniffing

---

## 11. Journalisation et surveillance

### Événements à logger

| Événement | Niveau log | Champs à logger |
|-----------|------------|-----------------|
| Connexion réussie | INFO | ID utilisateur, email, horodatage, IP, user agent |
| Connexion échouée | WARNING | Email (tenté), horodatage, IP, user agent, raison |
| Verrouillage compte | WARNING | Email, horodatage, IP, durée verrouillage |
| Demande réinitialisation mot de passe | INFO | Email, horodatage, IP |
| Réinitialisation mot de passe complétée | INFO | ID utilisateur, horodatage, IP |
| MFA activée/désactivée | INFO | ID utilisateur, horodatage, IP |
| Session créée | DEBUG | ID session (hash), ID utilisateur, horodatage |
| Session invalidée | DEBUG | ID session (hash), ID utilisateur, horodatage, raison |

**Ne jamais logger** : Mots de passe (texte clair ou hachés), jetons session, codes OTP

### Alertes

- ☑ Alerte sur multiples connexions échouées depuis même IP (attaque potentielle)
- ☑ Alerte sur verrouillages compte dépassant seuil
- ☑ Alerte sur pics réinitialisation mot de passe (credential stuffing potentiel)

---

## 12. Conformité et standards

### Alignement avec standards

- ☑ **OWASP ASVS** : Exigences V2 authentification
- ☑ **NIST SP 800-63B** : Directives identité numérique (authentification et gestion cycle de vie)
- ☑ **PCI-DSS** : Exigence 8 (identifier et authentifier accès)
- ☐ **HIPAA** : § 164.312(a)(2)(i) identification utilisateur unique
- ☐ **RGPD** : Article 32 sécurité du traitement

### Piste d'audit

- ☑ Tous événements authentification loggés avec horodatages
- ☑ Logs conservés minimum 90 jours
- ☑ Protection intégrité logs (stockage immuable ou SIEM)

---

## 13. Liste de vérification implémentation

### Développement

- [ ] Implémenter hachage mot de passe avec bcrypt (facteur travail 12)
- [ ] Ajouter détection violation mot de passe (API HaveIBeenPwned)
- [ ] Implémenter rate limiting par IP et par compte
- [ ] Ajouter intégration CAPTCHA (reCAPTCHA v3)
- [ ] Implémenter gestion session sécurisée (Redis, cookies HttpOnly)
- [ ] Ajouter support MFA (TOTP)
- [ ] Implémenter flux réinitialisation mot de passe avec jetons sécurisés
- [ ] Ajouter intégration SSO OAuth 2.0 (Google, GitHub)
- [ ] Implémenter recherche utilisateur temps constant
- [ ] Ajouter journalisation authentification

### Revue sécurité

- [ ] Test intrusion flux authentification
- [ ] Vérifier stockage mot de passe (jamais texte clair)
- [ ] Tester efficacité rate limiting
- [ ] Vérifier prévention énumération compte
- [ ] Tester timeout session et timeout absolu
- [ ] Vérifier application MFA pour comptes admin
- [ ] Tester expiration jeton réinitialisation mot de passe
- [ ] Vérifier configuration TLS et HSTS

---

## Ressources liées

- [Checklist OWASP ASVS](owasp-asvs-checklist-design.md)
- [Exigences autorisation](authorization-requirements-template-design.md)
- [NIST SP 800-63B](https://pages.nist.gov/800-63-3/sp800-63b.html)
- [OWASP Authentication Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authentication_Cheat_Sheet.html)
