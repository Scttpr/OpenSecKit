---
title: "Exigences d'Authentification FranceConnect RGS"
template_version: "1.0.0"
domain: "government-rgs"
constitutional_principles:
  - "III - Security by Design"
  - "IV - Defense in Depth"
  - "II - Risk Analysis"
regulatory_references:
  - "RGS v2.0 - Annexe B2 (Authentification)"
  - "Règlement eIDAS (UE) 910/2014 - Niveau Substantiel"
  - "Documentation FranceConnect v3.3"
  - "OpenID Connect Core 1.0"
  - "OAuth 2.0 (RFC 6749)"
ssdlc_phase: "III - Conception"
difficulty: "avancé"
estimated_time: "40-60 heures (intégration initiale), 8-12 heures (revue annuelle)"
---

# Exigences d'Authentification FranceConnect RGS

## Objectif

Ce template fournit les **exigences techniques d'implémentation** pour intégrer **FranceConnect**, le service de fédération d'identité national français, dans les applications destinées aux utilisateurs ou citoyens français. FranceConnect permet l'authentification **Single Sign-On (SSO)** en utilisant les identifiants gouvernementaux existants (Impots.gouv.fr, Ameli.fr, La Poste, etc.).

**Contexte Réglementaire** :
- **RGS v2.0 Annexe B2** : Les mécanismes d'authentification des systèmes gouvernementaux français doivent respecter les niveaux de sécurité RGS
- **Règlement eIDAS (UE) 910/2014** : FranceConnect fournit une authentification de **Niveau Substantiel** (niveau d'assurance eIDAS)
- **Article 9 du RGAA** : Les sites web du secteur public français doivent fournir une authentification accessible

**Alignement Constitutionnel** :
- **Principe III (Security by Design)** : L'authentification est un contrôle de sécurité fondamental
- **Principe IV (Defense in Depth)** : OAuth 2.0 + OpenID Connect fournit plusieurs couches de sécurité (flux code d'autorisation, paramètre state, PKCE)
- **Principe II (Risk Analysis)** : Niveaux de vérification d'identité alignés sur la sensibilité des données

**Quand utiliser FranceConnect** :
- ✅ Services Gouvernement-Citoyen (G2C) nécessitant une vérification d'identité
- ✅ Applications traitant des données administratives françaises sensibles (fiscales, santé, prestations sociales)
- ✅ Services nécessitant une authentification eIDAS Niveau Substantiel ou Élevé
- ❌ Applications B2B (utiliser d'autres fournisseurs OAuth ou fédération SAML)
- ❌ Services anonymes (FranceConnect nécessite la divulgation de l'identité)

---

## 1. Architecture FranceConnect

### 1.1 Flux d'Authentification (OpenID Connect Authorization Code Flow)

```mermaid
sequenceDiagram
    participant User as Citoyen/Utilisateur
    participant App as Votre Application
    participant FC as FranceConnect
    participant IdP as Fournisseur d'Identité<br/>(Impots.gouv.fr, Ameli, etc.)

    User->>App: 1. Clic "Se connecter avec FranceConnect"
    App->>FC: 2. Requête d'autorisation<br/>(client_id, redirect_uri, scope, state, nonce)
    FC->>User: 3. Affichage écran de sélection IdP
    User->>IdP: 4. Sélection IdP (ex: Impots.gouv.fr)
    IdP->>User: 5. Authentification (identifiant + mot de passe + 2FA)
    IdP->>FC: 6. Assertion d'authentification
    FC->>App: 7. Code d'autorisation (via redirection)
    App->>FC: 8. Requête de jeton (code + client_secret)
    FC->>App: 9. ID Token + Access Token
    App->>FC: 10. Requête UserInfo (access_token)
    FC->>App: 11. Données d'identité utilisateur (given_name, family_name, birthdate, etc.)
    App->>User: 12. Session établie
```

### 1.2 Niveaux de Service FranceConnect

FranceConnect propose deux niveaux de service :

| Niveau de Service | Cas d'Usage | Données d'Identité Disponibles | Niveau eIDAS | Complexité d'Intégration |
|-------------------|-------------|-------------------------------|--------------|-------------------------|
| **FranceConnect** | Services publics généraux | Identité basique (nom, date de naissance, genre, lieu de naissance) | **Substantiel** | ⭐⭐ Modérée |
| **FranceConnect+** | Services haute sécurité (bancaire, notaires) | Vérification d'identité renforcée + claims avancés | **Élevé** | ⭐⭐⭐ Avancée |

**Ce template couvre** : FranceConnect (niveau Substantiel) - l'intégration la plus courante.

Pour **FranceConnect+**, des exigences supplémentaires s'appliquent (vérification biométrique, détection de fraude, etc.) - consulter la documentation DINUM.

---

## 2. Prérequis et Inscription

### 2.1 Critères d'Éligibilité

Pour intégrer FranceConnect, votre organisation doit être :

✅ **Entité Éligible** :
- Administration publique française (ministères, collectivités locales, agences publiques)
- Prestataire de services du secteur privé agissant pour le compte de l'administration publique (via convention de délégation)
- Association ou entreprise privée assurant une mission de service public (avec justification)

❌ **Non Éligible** :
- Applications B2C purement commerciales
- Entités non françaises sans délégation de service public français

### 2.2 Processus d'Inscription du Fournisseur de Services

**Étape 1 : Demande d'Accès** (délai de traitement 4-8 semaines)
1. Soumettre la demande sur : https://franceconnect.gouv.fr/partenaires
2. Fournir :
   - Numéro SIRET de l'organisation
   - Description du service (justification d'intérêt public)
   - Finalités du traitement des données (base légale Article 6 RGPD)
   - Nombre estimé d'utilisateurs
   - Coordonnées du contact technique

**Étape 2 : Revue de Minimisation des Données** (approbation DINUM)
- La DINUM (Direction Interministérielle du Numérique) examine vos scopes d'identité demandés
- **Principe de Minimisation des Données** : Ne demander que les attributs d'identité nécessaires à votre service
- Exemple de rejet : Demander `birthdate` pour un service qui n'a besoin que d'une vérification d'âge

**Étape 3 : Réception des Identifiants**
- **Client ID** : Identifiant unique pour votre application (ex : `abc123def456`)
- **Client Secret** : Secret partagé pour l'échange de jetons (stocker dans **HSM** ou **gestionnaire de secrets**)
- **Accès aux Environnements** :
  - **Environnement d'Intégration** : https://fcp.integ01.dev-franceconnect.fr (tests)
  - **Environnement de Production** : https://app.franceconnect.gouv.fr (live)

**Étape 4 : Configuration des URLs de Callback**
- Enregistrer les URIs de redirection autorisées (correspondance exacte requise, pas de wildcards)
- Exemple : `https://example.gouv.fr/auth/franceconnect/callback`
- Utiliser **HTTPS uniquement** (HTTP non autorisé en production)

---

## 3. Exigences Techniques d'Implémentation

### 3.1 Configuration OpenID Connect

**Point de Découverte** (auto-configuration OpenID Connect) :
```bash
# Environnement d'intégration
https://fcp.integ01.dev-franceconnect.fr/api/v1/.well-known/openid-configuration

# Environnement de production
https://app.franceconnect.gouv.fr/api/v1/.well-known/openid-configuration
```

**Points de Terminaison Clés** (depuis le document de découverte) :
| Point de Terminaison | URL | Objectif |
|---------------------|-----|----------|
| **Autorisation** | `/api/v1/authorize` | Initier le flux d'authentification |
| **Jeton** | `/api/v1/token` | Échanger le code d'autorisation contre des jetons |
| **UserInfo** | `/api/v1/userinfo` | Récupérer les claims d'identité utilisateur |
| **Déconnexion** | `/api/v1/logout` | Initier la déconnexion FranceConnect |
| **JWKS** | `/api/v1/jwks` | Clés publiques pour la vérification de signature de l'ID Token |

### 3.2 Requête d'Autorisation (Étape 2 du Flux)

**Format de Requête** :
```http
GET /api/v1/authorize?
  response_type=code&
  client_id=VOTRE_CLIENT_ID&
  redirect_uri=https%3A%2F%2Fexample.gouv.fr%2Fauth%2Ffranceconnect%2Fcallback&
  scope=openid%20profile%20email%20birth&
  state=VALEUR_STATE_ALEATOIRE&
  nonce=VALEUR_NONCE_ALEATOIRE&
  acr_values=eidas1 HTTP/1.1
Host: app.franceconnect.gouv.fr
```

**Paramètres Requis** :

| Paramètre | Valeur | Exigence de Sécurité |
|-----------|--------|---------------------|
| `response_type` | `code` | ✅ Utiliser le **Flux Code d'Autorisation** (pas le flux implicite) |
| `client_id` | Votre client ID enregistré | ✅ Correspond à l'identifiant émis par la DINUM |
| `redirect_uri` | URL de callback exacte | ✅ **Correspondance exacte** avec l'URI enregistrée (sensible à la casse, pas de wildcards) |
| `scope` | `openid` + scopes additionnels | ✅ **Minimisation des données** : Ne demander que les claims nécessaires |
| `state` | Chaîne aléatoire cryptographique (≥128 bits) | 🔒 **Protection CSRF** - valider au callback |
| `nonce` | Chaîne aléatoire cryptographique (≥128 bits) | 🔒 **Protection contre les attaques par rejeu** - valider dans l'ID Token |
| `acr_values` | `eidas1` (Substantiel) ou `eidas2` (Faible) | ✅ Demander le niveau eIDAS minimum (défaut : `eidas1`) |

**Définitions des Scopes** :

| Scope | Claims Inclus | Cas d'Usage | Notes de Minimisation |
|-------|---------------|-------------|----------------------|
| `openid` | `sub` (identifiant unique) | **Obligatoire** - Toujours inclure | ID utilisateur pseudonyme (change par fournisseur de service) |
| `profile` | `family_name`, `given_name`, `preferred_username` | Nom d'affichage utilisateur | Ne pas demander si le service est anonyme |
| `email` | `email` | Contacter l'utilisateur | Seulement si le service envoie des emails |
| `birth` | `birthdate`, `birthplace`, `birthcountry` | Vérification d'âge, preuve d'identité | Seulement si légalement requis |
| `gender` | `gender` | Rapports statistiques | Éviter sauf nécessité (sensibilité RGPD) |
| `address` | `address` (JSON structuré) | Expédition, facturation | Rarement nécessaire - utiliser une API postale |

### 3.3 Échange de Jeton (Étapes 8-9 du Flux)

**Format de Requête** :
```http
POST /api/v1/token HTTP/1.1
Host: app.franceconnect.gouv.fr
Content-Type: application/x-www-form-urlencoded
Authorization: Basic BASE64(client_id:client_secret)

grant_type=authorization_code&
code=CODE_AUTORISATION_DU_CALLBACK&
redirect_uri=https%3A%2F%2Fexample.gouv.fr%2Fauth%2Ffranceconnect%2Fcallback
```

**Méthodes d'Authentification** (choisir une) :

| Méthode | Niveau de Sécurité | Cas d'Usage |
|---------|-------------------|-------------|
| **HTTP Basic Auth** (recommandé) | ⭐⭐⭐ Élevé | `Authorization: Basic BASE64(client_id:client_secret)` |
| **Corps POST** | ⭐⭐ Moyen | Inclure `client_id` et `client_secret` dans le corps de la requête (déprécié) |

**Réponse** :
```json
{
  "access_token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9...",
  "token_type": "Bearer",
  "expires_in": 3600,
  "id_token": "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Validation de l'ID Token** (JWT - RFC 7519) :

🔒 **ÉTAPE DE SÉCURITÉ CRITIQUE** - Valider TOUS les éléments suivants :

1. **Vérification de Signature** :
   ```python
   import jwt
   from jwt import PyJWKClient

   # Récupérer JWKS depuis https://app.franceconnect.gouv.fr/api/v1/jwks
   jwks_client = PyJWKClient("https://app.franceconnect.gouv.fr/api/v1/jwks")
   signing_key = jwks_client.get_signing_key_from_jwt(id_token)

   # Vérifier signature + claims
   decoded = jwt.decode(
       id_token,
       signing_key.key,
       algorithms=["RS256"],  # FranceConnect utilise RS256
       audience=VOTRE_CLIENT_ID,
       issuer="https://app.franceconnect.gouv.fr"
   )
   ```

2. **Claim Émetteur (`iss`)** :
   ```python
   assert decoded["iss"] == "https://app.franceconnect.gouv.fr"
   ```

3. **Claim Audience (`aud`)** :
   ```python
   assert decoded["aud"] == VOTRE_CLIENT_ID
   ```

4. **Claim Expiration (`exp`)** :
   ```python
   import time
   assert decoded["exp"] > time.time()  # Jeton non expiré
   ```

5. **Validation du Nonce** (protection contre les attaques par rejeu) :
   ```python
   # Comparer avec le nonce stocké en session lors de la requête d'autorisation
   assert decoded["nonce"] == session["oauth_nonce"]
   ```

6. **ACR (Authentication Context Class Reference)** :
   ```python
   assert decoded["acr"] in ["eidas1", "eidas2", "eidas3"]  # Vérifier le niveau eIDAS
   ```

### 3.4 Requête UserInfo (Étapes 10-11 du Flux)

**Requête** :
```http
GET /api/v1/userinfo HTTP/1.1
Host: app.franceconnect.gouv.fr
Authorization: Bearer ACCESS_TOKEN_DE_LA_REPONSE_JETON
```

**Réponse** :
```json
{
  "sub": "1234567890abcdef1234567890abcdef12345678",
  "given_name": "Jean",
  "family_name": "Dupont",
  "preferred_username": "Jean DUPONT",
  "email": "jean.dupont@example.fr",
  "birthdate": "1985-03-15",
  "birthplace": "75015",
  "birthcountry": "99100",
  "gender": "male"
}
```

**Définitions des Claims** :

| Claim | Format | Exemple | Notes |
|-------|--------|---------|-------|
| `sub` | Chaîne (64 caractères hex) | `1234...cdef` | **Pseudonyme** - unique par fournisseur de service |
| `given_name` | Chaîne | `Jean` | Prénom (encodé UTF-8) |
| `family_name` | Chaîne | `Dupont` | Nom de famille (encodé UTF-8) |
| `email` | Email | `jean.dupont@example.fr` | **Non vérifié** par FranceConnect (fourni par l'IdP) |
| `birthdate` | ISO 8601 (AAAA-MM-JJ) | `1985-03-15` | Date de naissance |
| `birthplace` | Code INSEE | `75015` | Code commune français (5 chiffres) |
| `birthcountry` | Code COG | `99100` | Pays de naissance |
| `gender` | Enum | `male`, `female` | Genre (catégorie spéciale RGPD - minimiser l'usage) |

**Exigences de Stockage des Données** :
- 🔒 Stocker `sub` (identifiant unique) dans votre base de données utilisateurs
- ⚠️ **Ne PAS stocker `email`** sauf si explicitement nécessaire (Article 5.1.c RGPD - minimisation)
- 🔒 Si stockage de données d'identité, chiffrer au repos (RGS Annexe B4 - Confidentialité)

---

## 4. Implémentation de la Déconnexion

### 4.1 Flux de Déconnexion Unique

FranceConnect supporte la **Déconnexion Initiée par le RP** (OpenID Connect Session Management) :

**Requête de Déconnexion** :
```http
GET /api/v1/logout?
  id_token_hint=ID_TOKEN_DE_LA_CONNEXION&
  state=STATE_ALEATOIRE&
  post_logout_redirect_uri=https%3A%2F%2Fexample.gouv.fr%2Flogout-callback HTTP/1.1
Host: app.franceconnect.gouv.fr
```

**Paramètres** :
| Paramètre | Requis | Description |
|-----------|--------|-------------|
| `id_token_hint` | ✅ Oui | L'`id_token` reçu lors de la connexion |
| `state` | ✅ Oui | Jeton de protection CSRF (valider au callback) |
| `post_logout_redirect_uri` | ✅ Oui | URI de redirection enregistrée (doit être pré-enregistrée auprès de la DINUM) |

**Important** :
- ⚠️ **Toujours détruire la session locale AVANT de rediriger** vers la déconnexion FranceConnect
- ⚠️ **Ne PAS compter uniquement sur la déconnexion FranceConnect** - votre application doit invalider sa propre session
- ✅ Stocker `id_token` en session lors de la connexion (nécessaire pour la déconnexion)

---

## 5. Exigences de Sécurité (RGS Annexe B2)

### 5.1 Exigences Cryptographiques

| Composant | Exigence | Justification |
|-----------|----------|---------------|
| **HTTPS/TLS** | TLS 1.2+ avec Forward Secrecy | RGS Annexe B4 - Confidentialité |
| **Suites de Chiffrement** | ECDHE-RSA-AES256-GCM-SHA384 ou supérieur | Recommandations TLS ANSSI |
| **Certificat** | Certificat valide d'une AC approuvée ANSSI | Ancres de confiance PKI gouvernement français |
| **Stockage Client Secret** | HSM ou gestionnaire de secrets chiffré (Vault, AWS Secrets Manager) | Prévenir le vol d'identifiants |
| **Génération State/Nonce** | Aléatoire cryptographique sécurisé (≥128 bits) | `os.urandom(16)` ou `/dev/urandom` |

### 5.2 Protection CSRF et Attaques par Rejeu

**Vecteur d'Attaque 1 : CSRF sur le Callback d'Autorisation**
- **Menace** : L'attaquant trompe l'utilisateur pour qu'il complète le flux OAuth avec le compte de l'attaquant
- **Mitigation** : Valider le paramètre `state`
  ```python
  # Au callback
  if request.args.get("state") != session.pop("oauth_state"):
      abort(403, "Paramètre state invalide - attaque CSRF potentielle")
  ```

**Vecteur d'Attaque 2 : Rejeu de l'ID Token**
- **Menace** : L'attaquant réutilise un ID Token volé pour usurper l'identité de l'utilisateur
- **Mitigation** : Valider le claim `nonce`
  ```python
  if decoded_id_token["nonce"] != session.pop("oauth_nonce"):
      abort(403, "Nonce invalide - attaque par rejeu potentielle")
  ```

### 5.3 Gestion des Sessions

**Exigences de Timeout de Session** (RGS Annexe B2) :

| Type de Session | Timeout Inactivité | Timeout Absolu | Justification |
|-----------------|-------------------|----------------|---------------|
| **Sensibilité Faible** | 30 minutes | 8 heures | Information publique générale |
| **Sensibilité Moyenne** | 15 minutes | 4 heures | Accès données personnelles (défaut FranceConnect) |
| **Sensibilité Élevée** | 5 minutes | 1 heure | Transactions financières, données de santé |

---

## 6. Exigences d'Accessibilité (RGAA)

### 6.1 Design du Bouton FranceConnect

**Directives de Marque Officielles** : https://partenaires.franceconnect.gouv.fr/kit-communication

**Implémentation Requise du Bouton** :

```html
<a href="/auth/franceconnect"
   class="franceconnect-button"
   role="button"
   aria-label="Se connecter avec FranceConnect - Service d'authentification de l'État">
  <img src="/assets/franceconnect-logo.svg"
       alt="FranceConnect"
       width="200"
       height="60" />
</a>
```

**Exigences d'Accessibilité (RGAA 4.1)** :
- ✅ **Ratio de contraste du bouton** : 4.5:1 minimum (WCAG AA)
- ✅ **Navigation clavier** : Le bouton doit être accessible via la touche Tab
- ✅ **Lecteur d'écran** : Utiliser `aria-label` pour décrire l'objectif du bouton
- ✅ **Indicateur de focus** : Contour de focus visible (2px solid, contraste élevé)
- ✅ **Cible tactile mobile** : Minimum 44x44 pixels CSS (WCAG 2.5.5)

---

## 7. Conformité RGPD

### 7.1 Base Légale pour l'Authentification FranceConnect

**Base Légale Article 6.1 RGPD** :

Pour les fournisseurs de services du **secteur public** :
- ✅ **Article 6.1.e** : Traitement nécessaire à l'exécution d'une mission effectuée dans l'intérêt public

Pour les fournisseurs de services du **secteur privé** (service public délégué) :
- ✅ **Article 6.1.b** : Traitement nécessaire à l'exécution d'un contrat
- ✅ **Article 6.1.c** : Traitement nécessaire au respect d'une obligation légale

### 7.2 Accord de Traitement des Données (DPA)

**Conditions de Traitement des Données FranceConnect** :
- La DINUM agit en tant que **sous-traitant** (Article 28 RGPD)
- Votre organisation est le **responsable du traitement**
- Accord de Traitement des Données : https://partenaires.franceconnect.gouv.fr/cgu

**Termes Clés** :
- **Conservation des Données** : Les données d'identité ne sont **pas stockées** par FranceConnect (passage uniquement)
- **Sous-traitants Ultérieurs** : Les Fournisseurs d'Identité (Impots.gouv.fr, Ameli, etc.) agissent comme sous-traitants ultérieurs
- **Transferts de Données** : Tout traitement a lieu au sein de l'**UE** (infrastructure hébergée en France)

### 7.3 Exigences de Mention d'Information

**Obligations de Transparence Article 13 RGPD** :

Votre mention d'information doit informer les utilisateurs :

```markdown
### Authentification via FranceConnect

Nous utilisons **FranceConnect** pour vous authentifier de manière sécurisée.

**Données collectées** :
- Nom et prénom
- Date de naissance
- Adresse e-mail
- [Autres données selon les scopes demandés]

**Finalité** : Authentification de votre identité pour accéder au service [nom du service].

**Base légale** : Exécution d'une mission d'intérêt public (Article 6.1.e du RGPD).

**Destinataires** : Les données sont transmises par FranceConnect (DINUM) et ne sont pas stockées par ce service.

**Durée de conservation** : Les données d'identité sont conservées pendant la durée de votre utilisation du service + [X ans] pour conformité légale.

**Droits** : Vous disposez d'un droit d'accès, de rectification et de suppression de vos données. Contactez [dpo@example.gouv.fr].
```

---

## 8. Tests et Validation

### 8.1 Tests en Environnement d'Intégration

**Comptes de Test** : La DINUM fournit des comptes de test pour les tests d'intégration

| Compte de Test | Fournisseur d'Identité | Cas d'Usage |
|----------------|----------------------|-------------|
| **test.franceconnect@rie.gouv.fr** | IdP Générique | Test de flux basique |
| **test.impots@dgfip.finances.gouv.fr** | Simulateur Impots.gouv.fr | Intégration service fiscal |

**Checklist de Test** :

- [ ] **Requête d'Autorisation** : Vérifier la redirection vers l'écran de sélection IdP FranceConnect
- [ ] **Sélection IdP** : Tester plusieurs IdPs (Impots, Ameli, La Poste)
- [ ] **Authentification** : Compléter le flux de connexion avec les identifiants de test
- [ ] **Échange de Jeton** : Vérifier la signature de l'ID Token, validation des claims
- [ ] **Requête UserInfo** : Récupérer les données d'identité utilisateur
- [ ] **Établissement de Session** : L'utilisateur est connecté à votre application
- [ ] **Validation State** : Tenter une attaque CSRF (manipuler le paramètre state) → Doit échouer
- [ ] **Validation Nonce** : Tenter une attaque par rejeu (réutiliser l'ID Token) → Doit échouer
- [ ] **Déconnexion** : Vérifier que la session SSO FranceConnect est détruite
- [ ] **Gestion des Erreurs** : Tester les erreurs réseau, jetons invalides, sessions expirées
- [ ] **Accessibilité** : Tester la navigation clavier, compatibilité lecteur d'écran

### 8.2 Supervision en Production

**Métriques Clés** :

| Métrique | Cible | Seuil d'Alerte | Outil de Supervision |
|----------|-------|----------------|---------------------|
| **Taux de Succès Authentification** | >98% | <95% | Logs applicatifs |
| **Latence Requête d'Autorisation** | <500ms | >2s | APM (Datadog, New Relic) |
| **Latence Échange de Jeton** | <1s | >5s | APM |
| **Disponibilité FranceConnect** | SLA 99.5% | Indisponibilité >5 min | Supervision uptime |

---

## 9. Exemple d'Implémentation (Python/Flask)

### 9.1 Exemple Complet Fonctionnel

```python
import os
import secrets
import time
from flask import Flask, redirect, request, session, abort
import jwt
from jwt import PyJWKClient

app = Flask(__name__)
app.secret_key = os.environ["FLASK_SECRET_KEY"]

# Configuration FranceConnect
FC_CLIENT_ID = os.environ["FC_CLIENT_ID"]
FC_CLIENT_SECRET = os.environ["FC_CLIENT_SECRET"]
FC_AUTHORIZATION_ENDPOINT = "https://app.franceconnect.gouv.fr/api/v1/authorize"
FC_TOKEN_ENDPOINT = "https://app.franceconnect.gouv.fr/api/v1/token"
FC_USERINFO_ENDPOINT = "https://app.franceconnect.gouv.fr/api/v1/userinfo"
FC_LOGOUT_ENDPOINT = "https://app.franceconnect.gouv.fr/api/v1/logout"
FC_JWKS_URL = "https://app.franceconnect.gouv.fr/api/v1/jwks"
REDIRECT_URI = "https://example.gouv.fr/auth/franceconnect/callback"

@app.route("/login/franceconnect")
def login_franceconnect():
    """Initier l'authentification FranceConnect"""
    state = secrets.token_urlsafe(32)
    nonce = secrets.token_urlsafe(32)

    session["oauth_state"] = state
    session["oauth_nonce"] = nonce
    session["oauth_state_expiry"] = time.time() + 600

    auth_url = (
        f"{FC_AUTHORIZATION_ENDPOINT}?"
        f"response_type=code&"
        f"client_id={FC_CLIENT_ID}&"
        f"redirect_uri={REDIRECT_URI}&"
        f"scope=openid profile email birth&"
        f"state={state}&"
        f"nonce={nonce}&"
        f"acr_values=eidas1"
    )

    return redirect(auth_url)

@app.route("/auth/franceconnect/callback")
def franceconnect_callback():
    """Gérer le callback d'autorisation FranceConnect"""
    state = request.args.get("state")
    if not state or state != session.pop("oauth_state", None):
        abort(403, "Paramètre state invalide")

    if time.time() > session.get("oauth_state_expiry", 0):
        abort(403, "Paramètre state expiré")

    code = request.args.get("code")
    if not code:
        abort(400, "Code d'autorisation manquant")

    import requests
    from requests.auth import HTTPBasicAuth

    token_response = requests.post(
        FC_TOKEN_ENDPOINT,
        data={
            "grant_type": "authorization_code",
            "code": code,
            "redirect_uri": REDIRECT_URI
        },
        auth=HTTPBasicAuth(FC_CLIENT_ID, FC_CLIENT_SECRET)
    )

    if token_response.status_code != 200:
        abort(500, "Échec de l'échange de jeton")

    tokens = token_response.json()
    id_token = tokens["id_token"]
    access_token = tokens["access_token"]

    jwks_client = PyJWKClient(FC_JWKS_URL)
    signing_key = jwks_client.get_signing_key_from_jwt(id_token)

    try:
        decoded = jwt.decode(
            id_token,
            signing_key.key,
            algorithms=["RS256"],
            audience=FC_CLIENT_ID,
            issuer="https://app.franceconnect.gouv.fr"
        )
    except jwt.InvalidTokenError as e:
        abort(403, f"ID Token invalide : {str(e)}")

    if decoded["nonce"] != session.pop("oauth_nonce", None):
        abort(403, "Nonce invalide")

    userinfo_response = requests.get(
        FC_USERINFO_ENDPOINT,
        headers={"Authorization": f"Bearer {access_token}"}
    )

    if userinfo_response.status_code != 200:
        abort(500, "Échec de la requête UserInfo")

    user_data = userinfo_response.json()

    session["user_id"] = user_data["sub"]
    session["user_name"] = f"{user_data['given_name']} {user_data['family_name']}"
    session["id_token"] = id_token
    session["last_activity"] = time.time()

    return redirect("/dashboard")

@app.route("/logout")
def logout():
    """Déconnecter l'utilisateur et détruire la session FranceConnect"""
    id_token = session.get("id_token")
    session.clear()

    if id_token:
        state = secrets.token_urlsafe(32)
        logout_url = (
            f"{FC_LOGOUT_ENDPOINT}?"
            f"id_token_hint={id_token}&"
            f"state={state}&"
            f"post_logout_redirect_uri=https://example.gouv.fr/logout-callback"
        )
        return redirect(logout_url)

    return redirect("/")

if __name__ == "__main__":
    app.run(ssl_context="adhoc")
```

---

## 10. Dépannage des Problèmes Courants

### 10.1 Erreurs d'Intégration

| Erreur | Cause | Solution |
|--------|-------|----------|
| **invalid_client** | client_id ou client_secret incorrect | Vérifier les identifiants de l'inscription DINUM |
| **invalid_redirect_uri** | URI de redirection non enregistrée ou ne correspond pas exactement | Vérifier la correspondance exacte (sensible à la casse, slash final, protocole) |
| **invalid_scope** | Scope demandé non autorisé par la DINUM | Ne demander que les scopes approuvés (vérifier l'email d'approbation DINUM) |
| **invalid_grant** | Code d'autorisation expiré ou déjà utilisé | Les codes expirent après 60 secondes ; assurer l'application à usage unique |
| **signature_verification_failed** | Signature de l'ID Token invalide | Vérifier avec le bon point JWKS, vérifier le décalage d'horloge (<5 min) |

---

## 11. Checklist de Conformité

### 11.1 Validation Pré-Production

- [ ] **Inscription** : Approbation DINUM reçue, identifiants émis
- [ ] **HTTPS** : TLS 1.2+ avec certificat valide d'une AC approuvée
- [ ] **Minimisation des Données** : Seuls les scopes nécessaires au service sont demandés
- [ ] **Protection CSRF** : Paramètre state validé au callback
- [ ] **Protection Rejeu** : Nonce validé dans l'ID Token
- [ ] **Validation ID Token** : Signature, émetteur, audience, expiration tous vérifiés
- [ ] **Gestion des Sessions** : Timeout inactivité (15 min), timeout absolu (4 heures)
- [ ] **Déconnexion** : Déconnexion initiée par le RP implémentée
- [ ] **Gestion des Erreurs** : Messages d'erreur conviviaux, pas de données sensibles dans les logs
- [ ] **Accessibilité** : Conformité RGAA 4.1 (navigation clavier, lecteur d'écran, contraste)
- [ ] **Mention d'Information** : Obligations de transparence Article 13 RGPD respectées
- [ ] **Supervision** : Taux de succès authentification, métriques de latence suivis

### 11.2 Matrice de Conformité RGS Annexe B2

| Exigence RGS | Implémentation | Preuve |
|--------------|----------------|--------|
| **B2.1 - Vérification d'Identité** | FranceConnect fournit eIDAS Niveau Substantiel | Attestation DINUM |
| **B2.2 - Gestion des Identifiants** | client_secret OAuth 2.0 stocké dans HSM | Logs de rotation des secrets |
| **B2.3 - Protocole d'Authentification** | Flux OpenID Connect Authorization Code | Revue de code, tests d'intégration |
| **B2.4 - Gestion des Sessions** | Timeout inactivité 15 minutes, jetons de session sécurisés | Configuration des sessions |
| **B2.5 - Authentification Multi-Facteur** | Héritée de l'IdP (Impots.gouv.fr fournit 2FA) | Documentation FranceConnect |

---

## 12. Alignement sur les Principes Constitutionnels

| Principe | Comment ce Template le Supporte |
|----------|--------------------------------|
| **III - Security by Design** | OAuth 2.0 + OpenID Connect fournit une architecture d'authentification sécurisée par défaut ; les paramètres state/nonce préviennent les attaques courantes (CSRF, rejeu) |
| **IV - Defense in Depth** | Multiples couches de sécurité : chiffrement TLS, vérification de signature ID Token, validation state, validation nonce, timeouts de session |
| **II - Risk Analysis** | L'authentification eIDAS Niveau Substantiel fournit une assurance d'identité proportionnelle à la sensibilité des données (RGS Annexe B2) |

---

## Contrôle de Version

**Version du Template** : 1.0.0
**Dernière Mise à Jour** : 2025-01-19
**Prochaine Date de Revue** : 2026-01-19
**Propriétaire** : Architecte Sécurité
**Approuvé Par** : RSSI

**Journal des Modifications** :
- **v1.0.0** (2025-01-19) : Template initial basé sur FranceConnect v3.3, RGS v2.0 Annexe B2, Règlement eIDAS, OpenID Connect Core 1.0

**Références** :
- Documentation Partenaires FranceConnect : https://partenaires.franceconnect.gouv.fr
- RGS v2.0 : https://www.ssi.gouv.fr/rgs
- OpenID Connect Core 1.0 : https://openid.net/specs/openid-connect-core-1_0.html
- Règlement eIDAS : https://eur-lex.europa.eu/eli/reg/2014/910/oj
