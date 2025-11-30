---
title: "Paiement e-commerce - Modèle de menaces STRIDE"
template_version: "1.0.0"
constitutional_principle: ["I"]
ssdlc_phase: "planification"
domain_applicability: "ecommerce"
last_updated: "2025-11-18"
reviewers: ["security-champion-lead", "security-champion-architect"]
related_templates:
  - "../../templates/01-threat-modeling/stride-threat-model-template-planning.md"
  - "../02-risk-analysis/_exemple-ecommerce-risque.md"
tags: ["exemple", "ecommerce", "paiement", "STRIDE", "PCI-DSS"]
scenario_description: "Modèle de menaces STRIDE complet pour un flux de paiement e-commerce gérant le traitement des paiements, l'authentification utilisateur et l'exécution des commandes"
complexity_level: "intermédiaire"
annotations_included: true
---

# Modèle de menaces : Flux de paiement e-commerce

**Fonctionnalité** : Paiement du panier avec traitement des paiements
**Application** : Plateforme e-commerce SecureShop
**Propriétaire** : Équipe produit - Squad paiement
**Date de modélisation** : 2025-11-18
**Participants** : Architecte sécurité, Lead backend, Lead frontend, Product manager
**Statut de revue** : Approuvé par le champion sécurité

## Résumé exécutif

Ce modèle de menaces analyse les risques de sécurité dans le flux de paiement de SecureShop, une plateforme e-commerce traitant les paiements par carte bancaire. Le flux de paiement gère des données de paiement sensibles (périmètre PCI-DSS), l'authentification utilisateur et l'exécution des commandes. En utilisant la méthodologie STRIDE, nous avons identifié **23 menaces distinctes** à travers 6 catégories, avec **8 menaces hautement prioritaires** nécessitant une atténuation immédiate avant la mise en production.

**Résultats clés** :
- **Critique** : Exposition des données de paiement pendant la transmission (S03, I01)
- **Critique** : Injection SQL dans le traitement des commandes (T02)
- **Élevé** : Vulnérabilités de détournement de session (S02)
- **Élevé** : Autorisation insuffisante sur les endpoints de paiement (E02)

**Prochaines étapes** : Voir [Analyse de risques](../02-risk-analysis/_exemple-ecommerce-risque.md) pour le plan de remédiation priorisé.

---

## 1. Vue d'ensemble du système

### 1.1 Description de la fonctionnalité

Le flux de paiement permet aux clients de :
1. Consulter le contenu du panier (articles, quantités, sous-total)
2. Saisir l'adresse de livraison
3. Sélectionner le mode de livraison
4. Saisir les informations de paiement (carte bancaire via Stripe)
5. Consulter le récapitulatif de la commande
6. Confirmer et soumettre la commande
7. Recevoir la confirmation de commande

### 1.2 Diagramme d'architecture

```
┌─────────────┐          ┌──────────────┐          ┌─────────────┐
│ Navigateur  │  HTTPS   │ Serveur web  │   API    │   Backend   │
│ (React SPA) │◄────────►│   (Nginx)    │◄────────►│  (Node.js)  │
└─────────────┘          └──────────────┘          └─────────────┘
                                                           │
                                                           ▼
                         ┌──────────────┐          ┌─────────────┐
                         │    Stripe    │          │ PostgreSQL  │
                         │  API paiement│          │  (BDD comm.)│
                         └──────────────┘          └─────────────┘
                                │                         │
                                └─────────────────────────┘
                                  (Données de paiement tokenisées)
```

### 1.3 Flux de données

**Étape 1 : Consultation du panier**
- Utilisateur → Frontend : Demande du contenu du panier
- Frontend → Backend : GET /api/cart (JWT dans l'en-tête Authorization)
- Backend → Base de données : Requête des articles du panier pour l'utilisateur
- Base de données → Backend → Frontend → Utilisateur : Données du panier (IDs produits, quantités, prix)

**Étape 2 : Adresse de livraison**
- Utilisateur → Frontend : Saisie de l'adresse (nom, rue, ville, code postal, pays)
- Frontend : Validation côté client (format, complétude)
- Frontend → Backend : POST /api/checkout/shipping (données d'adresse + JWT)
- Backend : Validation côté serveur, création de l'enregistrement de livraison
- Backend → Base de données : INSERT shipping_addresses

**Étape 3 : Informations de paiement**
- Utilisateur → Frontend : Saisie des détails de carte (numéro, expiration, CVV)
- Frontend → Stripe.js : Tokenisation des données de carte (conformité PCI-DSS SAQ-A - aucune donnée de carte ne touche nos serveurs)
- Stripe → Frontend : Retour du jeton de paiement (tok_abc123)
- Frontend → Backend : POST /api/checkout/payment (jeton uniquement + JWT)
- Backend → API Stripe : Création de l'intention de paiement avec le jeton
- Stripe → Backend : Confirmation du statut de l'intention de paiement

**Étape 4 : Confirmation de commande**
- Backend → Base de données : Création de l'enregistrement de commande (user_id, articles, total, livraison, payment_intent_id)
- Backend → Frontend : Confirmation de commande (order_id, total, livraison estimée)
- Frontend → Utilisateur : Affichage de la page de confirmation

### 1.4 Stack technologique

- **Frontend** : React 18.2, Stripe.js 3.x, Axios
- **Backend** : Node.js 20.x, Express 4.18, ORM Sequelize
- **Base de données** : PostgreSQL 15.x
- **Passerelle de paiement** : API Stripe v2023-10-16
- **Authentification** : JWT (jetons d'accès), cookies HTTP-only (jetons de rafraîchissement)
- **Serveur web** : Nginx 1.24 (TLS 1.3, reverse proxy)

---

## 2. Actifs

### 2.1 Actifs de données

| ID actif | Nom de l'actif | Sensibilité | PII | PCI-DSS | Description |
|----------|----------------|-------------|-----|---------|-------------|
| D01 | Données de carte de paiement | **CRITIQUE** | Oui | Oui | Numéro de carte, expiration, CVV (tokenisé via Stripe) |
| D02 | Identifiants utilisateur | **CRITIQUE** | Oui | Non | Email, mot de passe haché, jetons JWT |
| D03 | Adresses de livraison | **ÉLEVÉ** | Oui | Non | Nom, adresse, code postal, téléphone |
| D04 | Historique des commandes | **MOYEN** | Oui | Non | IDs de commande, articles achetés, totaux, horodatages |
| D05 | Données du panier | **FAIBLE** | Non | Non | IDs de produits, quantités (temporaire) |
| D06 | Jetons de paiement | **CRITIQUE** | Non | Oui | Jetons de paiement Stripe (usage unique) |
| D07 | Données de session | **ÉLEVÉ** | Oui | Non | Jetons d'accès JWT, jetons de rafraîchissement, IDs de session |

### 2.2 Actifs fonctionnels

| ID actif | Nom de l'actif | Criticité | Description |
|----------|----------------|-----------|-------------|
| F01 | Traitement des paiements | **CRITIQUE** | Intégration API Stripe pour débiter les cartes |
| F02 | Création de commande | **ÉLEVÉ** | Service backend créant les enregistrements de commande |
| F03 | Gestion des stocks | **ÉLEVÉ** | Validation et réservation des stocks |
| F04 | Authentification utilisateur | **CRITIQUE** | Système d'authentification basé sur JWT |
| F05 | Calcul de livraison | **MOYEN** | Logique de calcul des frais de port |
| F06 | Notifications email | **MOYEN** | Emails de confirmation de commande |

### 2.3 Actifs d'infrastructure

| ID actif | Nom de l'actif | Criticité | Description |
|----------|----------------|-----------|-------------|
| I01 | Base de données PostgreSQL | **CRITIQUE** | Stocke toutes les données applicatives |
| I02 | Serveurs API backend | **CRITIQUE** | Serveurs d'application Node.js |
| I03 | Reverse proxy Nginx | **ÉLEVÉ** | Terminaison TLS, répartition de charge |
| I04 | Clés API Stripe | **CRITIQUE** | Clés secrètes pour le traitement des paiements |

---

## 3. Frontières de confiance

### 3.1 Définitions des frontières

1. **Utilisateur ↔ Frontend (navigateur)**
   - Niveau de confiance : Non fiable
   - Communication : HTTPS (TLS 1.3)
   - Contrôles : CSP, SRI, CORS

2. **Frontend ↔ API backend**
   - Niveau de confiance : Semi-fiable (utilisateurs authentifiés)
   - Communication : HTTPS avec authentification JWT
   - Contrôles : Validation JWT, rate limiting, validation des entrées

3. **API backend ↔ Base de données**
   - Niveau de confiance : Fiable (réseau interne)
   - Communication : Protocole PostgreSQL (chiffré)
   - Contrôles : Requêtes paramétrées, utilisateur BDD à moindre privilège

4. **API backend ↔ API Stripe**
   - Niveau de confiance : Semi-fiable (service tiers)
   - Communication : HTTPS avec authentification par clé API
   - Contrôles : Rotation des clés API, validation de signature webhook

### 3.2 Points d'entrée

| Point d'entrée | Frontière de confiance | Authentification | Autorisation |
|----------------|------------------------|------------------|--------------|
| GET /api/cart | Frontend → Backend | JWT requis | L'utilisateur ne peut accéder qu'à son propre panier |
| POST /api/checkout/shipping | Frontend → Backend | JWT requis | L'utilisateur ne peut mettre à jour que sa propre livraison |
| POST /api/checkout/payment | Frontend → Backend | JWT requis | L'utilisateur ne peut payer que sa propre commande |
| POST /api/orders | Frontend → Backend | JWT requis | L'utilisateur ne peut créer que des commandes pour son propre panier |
| POST /webhooks/stripe | Stripe → Backend | Signature webhook | N/A (vérifié via signature) |

---

## 4. Analyse des menaces STRIDE

### 4.1 Spoofing (usurpation d'identité) (S)

#### S01 : Usurpation d'utilisateur via vol de jeton JWT

**Description** : Un attaquant vole un jeton d'accès JWT valide (par ex. via XSS, interception réseau) et usurpe l'identité d'un utilisateur légitime.

**Actifs menacés** : D02 (Identifiants utilisateur), D07 (Données de session), F04 (Authentification)

**Scénario d'attaque** :
1. L'attaquant injecte du JavaScript malveillant via une vulnérabilité XSS stockée
2. Le script exfiltre le jeton d'accès JWT depuis localStorage
3. L'attaquant rejoue le jeton dans l'en-tête Authorization pour accéder au panier de la victime et passer des commandes

**Impact** : L'attaquant peut voir le panier de la victime, modifier l'adresse de livraison, passer des commandes non autorisées

**Vraisemblance** : Moyenne (nécessite une vulnérabilité XSS)

**Contre-mesures** :
- ✅ **IMPLÉMENTÉ** : Jetons JWT stockés en mémoire uniquement (pas dans localStorage)
- ✅ **IMPLÉMENTÉ** : Cookies HTTP-only pour les jetons de rafraîchissement
- ✅ **IMPLÉMENTÉ** : Expiration courte des jetons (15 minutes pour les jetons d'accès)
- ✅ **IMPLÉMENTÉ** : En-têtes CSP pour prévenir XSS
- ⏳ **PLANIFIÉ** : Lier JWT à l'empreinte de l'appareil (hash IP + User-Agent)

**Risque résiduel** : Faible (après contre-mesures planifiées)

---

#### S02 : Détournement de session via interception réseau

**Description** : Un attaquant intercepte le trafic HTTP pour voler les jetons de session en transit.

**Actifs menacés** : D07 (Données de session), F04 (Authentification)

**Scénario d'attaque** :
1. L'utilisateur se connecte à un Wi-Fi public (café, aéroport)
2. L'attaquant effectue une attaque MITM (ARP spoofing, AP malveillant)
3. L'attaquant dégrade HTTPS vers HTTP ou exploite une configuration TLS faible
4. L'attaquant capture le jeton JWT depuis les requêtes interceptées

**Impact** : Prise de contrôle complète du compte, achats non autorisés

**Vraisemblance** : Faible (HTTPS appliqué, mais mauvaises configurations possibles)

**Contre-mesures** :
- ✅ **IMPLÉMENTÉ** : TLS 1.3 uniquement (config Nginx : `ssl_protocols TLSv1.3;`)
- ✅ **IMPLÉMENTÉ** : En-tête HSTS (max-age=31536000, includeSubDomains, preload)
- ✅ **IMPLÉMENTÉ** : Certificate pinning (backend → Stripe)
- ❌ **MANQUANT** : Certificate pinning (frontend → backend) - limitation navigateur
- ✅ **IMPLÉMENTÉ** : Flags de cookie sécurisés (Secure, HttpOnly, SameSite=Strict)

**Risque résiduel** : Très faible

---

#### S03 : Altération de données de paiement via manipulation de jeton Stripe

**Description** : Un attaquant tente de manipuler le jeton de paiement Stripe pour débiter un montant différent ou une carte différente.

**Actifs menacés** : D01 (Données de carte de paiement), D06 (Jetons de paiement), F01 (Traitement des paiements)

**Scénario d'attaque** :
1. Un utilisateur légitime tokenise sa carte via Stripe.js (tok_abc123 pour 100€)
2. L'attaquant intercepte la requête POST /api/checkout/payment
3. L'attaquant modifie le jeton de paiement ou le montant en transit
4. Le backend traite le paiement manipulé

**Impact** : Débits incorrects, fraude au paiement, perte financière

**Vraisemblance** : Très faible (les jetons Stripe sont à usage unique et liés au montant)

**Contre-mesures** :
- ✅ **IMPLÉMENTÉ** : Intentions de paiement Stripe (montant verrouillé côté serveur, pas fourni par le client)
- ✅ **IMPLÉMENTÉ** : Le backend recalcule le total de la commande depuis le panier (ignore le montant fourni par le client)
- ✅ **IMPLÉMENTÉ** : TLS pour toutes les requêtes de paiement
- ✅ **IMPLÉMENTÉ** : Validation webhook Stripe (vérification de signature)

**Risque résiduel** : Très faible

---

### 4.2 Tampering (altération) (T)

#### T01 : Manipulation du prix du panier

**Description** : Un attaquant modifie les prix des articles du panier dans le frontend pour acheter des articles à des prix inférieurs.

**Actifs menacés** : D05 (Données du panier), F02 (Création de commande)

**Scénario d'attaque** :
1. L'utilisateur ajoute un article à 200€ au panier
2. L'attaquant ouvre les DevTools du navigateur, modifie l'état du panier (mutation d'état React)
3. Le frontend affiche un prix de 10€
4. L'attaquant procède au paiement
5. Le backend utilise le prix fourni par le client au lieu du prix en base de données

**Impact** : Perte financière, vol de revenus, stock vendu en dessous du coût

**Vraisemblance** : Élevée (attaque web courante)

**Contre-mesures** :
- ✅ **IMPLÉMENTÉ** : Le backend recalcule tous les prix depuis la base de données produits (ne fait jamais confiance aux prix du client)
- ✅ **IMPLÉMENTÉ** : Validation du panier au paiement : `SELECT price FROM products WHERE id = ?`
- ✅ **IMPLÉMENTÉ** : Validation du total de la commande avant traitement du paiement
- ⏳ **PLANIFIÉ** : Journalisation d'audit des différences de prix (frontend vs backend)

**Risque résiduel** : Très faible (si la validation backend fonctionne correctement)

---

#### T02 : Injection SQL dans le traitement des commandes

**Description** : Un attaquant injecte des commandes SQL via les entrées utilisateur (adresse de livraison, IDs de produits) pour manipuler la base de données.

**Actifs menacés** : I01 (Base de données), D04 (Historique des commandes), D03 (Adresses de livraison)

**Scénario d'attaque** :
1. L'attaquant saisit une chaîne malveillante dans le champ d'adresse : `'; DROP TABLE orders; --`
2. Le backend concatène l'entrée utilisateur dans une requête SQL (code vulnérable)
3. La base de données exécute le SQL malveillant, supprimant la table orders

**Impact** : Perte de données, corruption de base de données, accès non autorisé aux données

**Vraisemblance** : Moyenne (si les requêtes paramétrées ne sont pas utilisées systématiquement)

**Contre-mesures** :
- ✅ **IMPLÉMENTÉ** : ORM Sequelize avec requêtes paramétrées (tous les accès BDD)
- ✅ **IMPLÉMENTÉ** : Validation des entrées (liste blanche des caractères autorisés pour les champs d'adresse)
- ✅ **IMPLÉMENTÉ** : Utilisateur BDD à moindre privilège (pas de permissions DROP)
- ✅ **IMPLÉMENTÉ** : Réplicas en lecture de la base de données (séparation des accès lecture/écriture)

**Risque résiduel** : Très faible

---

#### T03 : Modification du montant de paiement avant débit Stripe

**Description** : Condition de concurrence où l'attaquant modifie le total de la commande entre la validation et le traitement du paiement.

**Actifs menacés** : F01 (Traitement des paiements), D06 (Jetons de paiement)

**Scénario d'attaque** :
1. L'utilisateur initie le paiement avec une commande de 100€
2. Le backend valide le total (100€) et crée l'intention de paiement
3. L'attaquant envoie une requête concurrente pour modifier le panier (ajoute un article à 500€)
4. Le backend débite 100€ mais exécute une commande de 600€

**Impact** : Perte financière, débits incorrects

**Vraisemblance** : Faible (nécessite un timing précis)

**Contre-mesures** :
- ✅ **IMPLÉMENTÉ** : Transactions de base de données (BEGIN ... COMMIT)
- ✅ **IMPLÉMENTÉ** : Verrouillage du panier pendant le paiement (SELECT FOR UPDATE)
- ✅ **IMPLÉMENTÉ** : Recalcul du total de commande immédiatement avant paiement
- ⏳ **PLANIFIÉ** : Clés d'idempotence pour les requêtes de paiement

**Risque résiduel** : Très faible

---

### 4.3 Repudiation (répudiation) (R)

#### R01 : L'utilisateur nie avoir passé la commande

**Description** : Un utilisateur passe une commande, reçoit les biens, puis prétend ne pas avoir autorisé l'achat.

**Actifs menacés** : D04 (Historique des commandes), F02 (Création de commande)

**Scénario d'attaque** :
1. L'utilisateur complète le paiement, la commande est exécutée et expédiée
2. L'utilisateur contacte le support en prétendant que son compte a été piraté
3. L'utilisateur demande un remboursement sans retourner les biens

**Impact** : Perte financière (chargebacks), fraude

**Vraisemblance** : Moyenne (la fraude aux chargebacks est courante)

**Contre-mesures** :
- ✅ **IMPLÉMENTÉ** : Logs d'audit pour tous les événements de création de commande (horodatage, IP, User-Agent)
- ✅ **IMPLÉMENTÉ** : Email de confirmation envoyé à l'email enregistré de l'utilisateur
- ✅ **IMPLÉMENTÉ** : SMS/2FA pour les commandes de valeur élevée (>500€)
- ⏳ **PLANIFIÉ** : Empreinte digitale de l'appareil (suivi des appareils connus)
- ⏳ **PLANIFIÉ** : Vérification de l'adresse de livraison (comparaison avec les commandes précédentes)

**Risque résiduel** : Moyen (les chargebacks restent possibles, mais la piste d'audit soutient l'investigation)

---

#### R02 : L'administrateur nie les modifications de configuration

**Description** : Un admin modifie la configuration de la passerelle de paiement (par ex. change les clés API Stripe) et nie sa responsabilité.

**Actifs menacés** : I04 (Clés API Stripe), F01 (Traitement des paiements)

**Scénario d'attaque** :
1. Un admin malveillant remplace les clés Stripe de production par les clés de son compte personnel
2. Les paiements sont routés vers le compte Stripe de l'attaquant au lieu du compte de l'entreprise
3. L'admin nie avoir effectué les modifications lorsqu'elles sont découvertes

**Impact** : Vol de paiements, perte financière, violations de conformité

**Vraisemblance** : Faible (nécessite un initié malveillant)

**Contre-mesures** :
- ✅ **IMPLÉMENTÉ** : Logs d'audit des actions admin (horodatage, utilisateur, IP, action)
- ✅ **IMPLÉMENTÉ** : Processus de gestion des changements (approbation à 2 personnes pour les modifications de config)
- ✅ **IMPLÉMENTÉ** : Contrôle de version pour la configuration (historique Git)
- ⏳ **PLANIFIÉ** : Alertes sur les modifications de clés API Stripe (notification Slack)

**Risque résiduel** : Faible

---

### 4.4 Information Disclosure (divulgation d'informations) (I)

#### I01 : Exposition des données de carte de paiement dans le navigateur

**Description** : Les données sensibles de carte de paiement (PAN, CVV) stockées dans la mémoire du navigateur ou transmises au backend.

**Actifs menacés** : D01 (Données de carte de paiement)

**Scénario d'attaque** :
1. L'utilisateur saisit les détails de carte dans le formulaire frontend
2. Le code frontend vulnérable stocke les données de carte dans localStorage ou les envoie au backend
3. L'attaquant exploite une XSS pour exfiltrer les données de carte
4. Violation de conformité PCI-DSS (périmètre SAQ-D au lieu de SAQ-A)

**Impact** : Violation PCI-DSS, amendes (5 000€-100 000€/mois), violation de données de carte

**Vraisemblance** : Élevée (si Stripe.js n'est pas utilisé correctement)

**Contre-mesures** :
- ✅ **IMPLÉMENTÉ** : Tokenisation Stripe.js (les données de carte ne touchent JAMAIS nos serveurs)
- ✅ **IMPLÉMENTÉ** : Conformité PCI-DSS SAQ-A (validée via Stripe)
- ✅ **IMPLÉMENTÉ** : En-têtes CSP bloquant les scripts inline
- ✅ **IMPLÉMENTÉ** : Pas de données de carte dans les logs, bases de données ou code applicatif

**Risque résiduel** : Très faible

---

#### I02 : Exposition des données utilisateur via réponses API non sécurisées

**Description** : Les réponses API incluent des données sensibles (emails, numéros de téléphone, adresses) d'autres utilisateurs.

**Actifs menacés** : D03 (Adresses de livraison), D04 (Historique des commandes)

**Scénario d'attaque** :
1. L'attaquant appelle GET /api/orders/123
2. Le backend ne vérifie pas l'autorisation (la commande appartient à un autre utilisateur)
3. L'API retourne les détails de commande incluant l'adresse de livraison et les articles
4. L'attaquant itère les IDs de commande pour récolter les données utilisateur

**Impact** : Violation de PII, violations RGPD, perte de confidentialité utilisateur

**Vraisemblance** : Moyenne (les vulnérabilités IDOR sont courantes)

**Contre-mesures** :
- ✅ **IMPLÉMENTÉ** : Vérifications d'autorisation sur tous les endpoints (user_id = JWT.user_id)
- ✅ **IMPLÉMENTÉ** : Requêtes de base de données limitées à l'utilisateur authentifié
- ✅ **IMPLÉMENTÉ** : Tests unitaires pour l'autorisation (cas de test négatifs)
- ⏳ **PLANIFIÉ** : Tests de sécurité API (détection automatisée d'IDOR)

**Risque résiduel** : Faible

---

#### I03 : Divulgation d'adresse de livraison via attaque temporelle

**Description** : Un attaquant infère des adresses de livraison valides en mesurant les temps de réponse de l'API.

**Actifs menacés** : D03 (Adresses de livraison)

**Scénario d'attaque** :
1. L'attaquant soumet une requête de paiement avec une adresse devinée
2. Les adresses valides déclenchent le service de vérification d'adresse (réponse plus lente)
3. Les adresses invalides échouent rapidement (réponse plus rapide)
4. L'attaquant utilise la différence de timing pour énumérer les adresses valides

**Impact** : Énumération d'adresses, violation de confidentialité

**Vraisemblance** : Très faible (nécessite un attaquant sophistiqué)

**Contre-mesures** :
- ⏳ **PLANIFIÉ** : Validation d'adresse à temps constant (ajouter un délai au chemin rapide)
- ✅ **IMPLÉMENTÉ** : Rate limiting (max 10 requêtes/minute par IP)

**Risque résiduel** : Très faible

---

### 4.5 Denial of Service (déni de service) (D)

#### D01 : Épuisement du traitement des paiements

**Description** : Un attaquant inonde l'endpoint de paiement de requêtes pour épuiser le quota de l'API Stripe ou les ressources backend.

**Actifs menacés** : F01 (Traitement des paiements), I02 (Serveurs backend)

**Scénario d'attaque** :
1. L'attaquant crée un réseau de bots (ou utilise des comptes compromis)
2. Le bot soumet des milliers de requêtes de paiement à POST /api/checkout/payment
3. Les limites de taux de l'API Stripe sont déclenchées (100 requêtes/seconde)
4. Les utilisateurs légitimes ne peuvent pas compléter le paiement

**Impact** : Perte de revenus, désabonnement client, violations de SLA

**Vraisemblance** : Moyenne (les attaques DDoS sont courantes pour l'e-commerce)

**Contre-mesures** :
- ✅ **IMPLÉMENTÉ** : Rate limiting (5 tentatives de paiement/minute par utilisateur)
- ✅ **IMPLÉMENTÉ** : CAPTCHA pour comportement suspect (plusieurs paiements échoués)
- ⏳ **PLANIFIÉ** : WAF avec protection DDoS (Cloudflare)
- ⏳ **PLANIFIÉ** : Mise en cache des intentions de paiement (réutiliser l'intention si requête dupliquée)

**Risque résiduel** : Moyen (la protection DDoS nécessite un CDN)

---

#### D02 : Épuisement du pool de connexions base de données

**Description** : Un attaquant inonde l'API de requêtes pour épuiser le pool de connexions à la base de données.

**Actifs menacés** : I01 (Base de données), I02 (Serveurs backend)

**Scénario d'attaque** :
1. L'attaquant envoie des requêtes rapides à GET /api/cart
2. Chaque requête ouvre une connexion à la base de données
3. Le pool de connexions (max 20 connexions) est épuisé
4. Les requêtes légitimes échouent avec des erreurs "connection timeout"

**Impact** : Panne de service, impact client

**Vraisemblance** : Moyenne

**Contre-mesures** :
- ✅ **IMPLÉMENTÉ** : Pooling de connexions (max 20 connexions, file d'attente de débordement)
- ✅ **IMPLÉMENTÉ** : Timeout de requête (30 secondes max)
- ✅ **IMPLÉMENTÉ** : Rate limiting (100 requêtes/minute par IP)
- ⏳ **PLANIFIÉ** : Réplicas en lecture de la base de données (distribuer la charge de lecture)

**Risque résiduel** : Faible

---

### 4.6 Elevation of Privilege (élévation de privilèges) (E)

#### E01 : Accès admin non autorisé via manipulation du rôle JWT

**Description** : Un attaquant modifie le payload JWT pour escalader les privilèges d'utilisateur à admin.

**Actifs menacés** : F04 (Authentification), I01 (Base de données)

**Scénario d'attaque** :
1. L'attaquant obtient un JWT utilisateur valide : `{ "user_id": 123, "role": "user" }`
2. L'attaquant décode le JWT, modifie le rôle : `{ "user_id": 123, "role": "admin" }`
3. L'attaquant réencode le JWT avec la même signature (si la clé HMAC a fuité)
4. Le backend accepte le jeton modifié, accorde les privilèges admin

**Impact** : Compromission complète du système, violation de données, fraude financière

**Vraisemblance** : Faible (nécessite une fuite de clé secrète ou confusion d'algorithme)

**Contre-mesures** :
- ✅ **IMPLÉMENTÉ** : JWT signé avec RS256 (asymétrique, pas HS256)
- ✅ **IMPLÉMENTÉ** : Clé privée stockée dans variable d'environnement sécurisée (pas dans le code)
- ✅ **IMPLÉMENTÉ** : Validation du rôle sur chaque endpoint admin (pas seulement le claim JWT)
- ✅ **IMPLÉMENTÉ** : Vérification de l'algorithme (rejet des jetons non signés)

**Risque résiduel** : Très faible

---

#### E02 : Modification de paiement non autorisée via IDOR

**Description** : Un attaquant modifie le moyen de paiement pour la commande d'un autre utilisateur.

**Actifs menacés** : F01 (Traitement des paiements), D06 (Jetons de paiement)

**Scénario d'attaque** :
1. L'utilisateur A crée une commande (order_id=100)
2. L'attaquant (utilisateur B) devine l'ID de commande
3. L'attaquant appelle PUT /api/orders/100/payment avec son propre jeton Stripe
4. Le backend ne vérifie pas la propriété de la commande
5. La commande de l'utilisateur A est débitée sur la carte de l'attaquant (ou vice versa)

**Impact** : Fraude au paiement, débits non autorisés

**Vraisemblance** : Élevée (si l'autorisation manque)

**Contre-mesures** :
- ✅ **IMPLÉMENTÉ** : Validation de propriété de commande (user_id = JWT.user_id)
- ✅ **IMPLÉMENTÉ** : Intention de paiement verrouillée après création (order_id immuable)
- ✅ **IMPLÉMENTÉ** : Métadonnées d'intention de paiement Stripe (inclut user_id pour vérification)

**Risque résiduel** : Très faible

---

#### E03 : Accès au panneau admin via traversée de chemin

**Description** : Un attaquant accède au tableau de bord admin via manipulation d'URL.

**Actifs menacés** : F04 (Authentification), I01 (Base de données)

**Scénario d'attaque** :
1. L'attaquant découvre le panneau admin à /admin/dashboard
2. L'attaquant essaie la traversée de chemin : /admin/../api/users
3. Le routage backend contourne le middleware d'authentification en raison d'une mauvaise configuration
4. L'attaquant accède à l'API de gestion utilisateur sans authentification

**Impact** : Accès admin non autorisé, violation de données utilisateur

**Vraisemblance** : Faible (nécessite une mauvaise configuration de routage)

**Contre-mesures** :
- ✅ **IMPLÉMENTÉ** : Middleware d'authentification appliqué globalement (toutes les routes /api/*)
- ✅ **IMPLÉMENTÉ** : Normalisation de chemin (prévention de la traversée ../)
- ✅ **IMPLÉMENTÉ** : Routes admin isolées dans l'espace de noms /admin avec middleware séparé
- ✅ **IMPLÉMENTÉ** : Vérifications de contrôle d'accès basé sur les rôles (RBAC) sur chaque route admin

**Risque résiduel** : Très faible

---

## 5. Tableau récapitulatif des menaces

| ID | Catégorie | Menace | Vraisemblance | Impact | Risque | Contre-mesures | Risque résiduel |
|----|----------|--------|---------------|--------|--------|----------------|-----------------|
| S01 | Spoofing | Vol de jeton JWT | Moyenne | Élevé | **ÉLEVÉ** | CSP, expiration courte, liaison appareil | Faible |
| S02 | Spoofing | Détournement de session | Faible | Élevé | Moyen | TLS 1.3, HSTS, cookies sécurisés | Très faible |
| S03 | Spoofing | Manipulation jeton paiement | Très faible | Critique | Moyen | Intentions de paiement Stripe | Très faible |
| T01 | Tampering | Manipulation prix panier | Élevée | Élevé | **ÉLEVÉ** | Recalcul prix backend | Très faible |
| T02 | Tampering | Injection SQL | Moyenne | Critique | **ÉLEVÉ** | Requêtes paramétrées | Très faible |
| T03 | Tampering | Condition de concurrence paiement | Faible | Élevé | Moyen | Transactions BDD, verrouillage | Très faible |
| R01 | Repudiation | Déni de commande | Moyenne | Moyen | Moyen | Logs d'audit, confirmation email | Moyen |
| R02 | Repudiation | Déni modification config | Faible | Élevé | Moyen | Logs d'audit admin | Faible |
| I01 | Information Disclosure | Exposition données carte | Élevée | Critique | **CRITIQUE** | Tokenisation Stripe.js | Très faible |
| I02 | Information Disclosure | Exposition données utilisateur (IDOR) | Moyenne | Élevé | **ÉLEVÉ** | Vérifications d'autorisation | Faible |
| I03 | Information Disclosure | Énumération adresse | Très faible | Faible | Très faible | Validation temps constant | Très faible |
| D01 | Denial of Service | Inondation paiement | Moyenne | Élevé | **ÉLEVÉ** | Rate limiting, CAPTCHA | Moyen |
| D02 | Denial of Service | Épuisement pool connexions | Moyenne | Élevé | **ÉLEVÉ** | Pooling connexions, rate limiting | Faible |
| E01 | Elevation of Privilege | Manipulation rôle JWT | Faible | Critique | **ÉLEVÉ** | Signature RS256, validation rôle | Très faible |
| E02 | Elevation of Privilege | IDOR paiement | Élevée | Élevé | **ÉLEVÉ** | Validation propriété | Très faible |
| E03 | Elevation of Privilege | Accès admin traversée chemin | Faible | Critique | **ÉLEVÉ** | Normalisation chemin, RBAC | Très faible |

**Niveaux de risque** :
- **CRITIQUE** : Action immédiate requise, bloque le déploiement en production
- **ÉLEVÉ** : Doit être traité avant la mise en production
- **MOYEN** : À traiter dans le sprint suivant
- **FAIBLE** : Backlog, surveiller les changements

**Nombre de menaces** : 16 menaces analysées (6 de plus identifiées dans l'évaluation complète)
**Menaces élevées/critiques** : 8
**Atténuées à faible/très faible** : 14 (88%)

---

## 6. Résumé des atténuations

### 6.1 Contrôles implémentés

✅ **Authentification et autorisation** :
- JWT avec RS256 (signature asymétrique)
- Expiration courte des jetons (15 minutes)
- Validation de propriété sur toutes les ressources spécifiques à l'utilisateur
- Contrôle d'accès basé sur les rôles (RBAC)

✅ **Protection des données** :
- TLS 1.3 pour toutes les communications
- Tokenisation Stripe.js (PCI-DSS SAQ-A)
- Requêtes paramétrées (prévention injection SQL)
- Flags de cookie sécurisés (HttpOnly, Secure, SameSite)

✅ **Validation des entrées** :
- Recalcul des prix backend (ne jamais faire confiance au client)
- Validation par liste blanche pour les adresses
- Validation frontend + backend

✅ **Audit et surveillance** :
- Logs d'audit pour toutes les opérations sensibles
- Confirmations email pour les commandes

### 6.2 Contrôles planifiés (Sprint 2)

⏳ **Améliorations du rate limiting** :
- WAF avec protection DDoS (Cloudflare)
- Mise en cache des intentions de paiement (idempotence)

⏳ **Surveillance avancée** :
- Empreinte digitale de l'appareil
- Service de vérification d'adresse
- Alertes de différence de prix

⏳ **Tests de sécurité** :
- Détection automatisée d'IDOR
- Tests d'intrusion

### 6.3 Contrôles différés (Backlog)

📋 **Fonctionnalités avancées** :
- Analytique comportementale (détection de fraude)
- Notation du risque client
- Apprentissage automatique pour détection d'anomalies

---

## 7. Exigences de test

### 7.1 Cas de test de sécurité

Pour chaque menace, les cas de test correspondants doivent passer :

| ID menace | Cas de test | Résultat attendu |
|-----------|-------------|------------------|
| S01 | Rejouer JWT volé après expiration | 401 Non autorisé |
| T01 | Modifier prix panier dans navigateur, payer | Backend utilise prix BDD, pas prix client |
| T02 | Soumettre payload injection SQL dans adresse | Entrée rejetée, pas d'exécution SQL |
| I02 | Demander commande d'un autre utilisateur | 403 Interdit |
| E02 | Modifier paiement pour commande autre utilisateur | 403 Interdit |

Voir [Guide de tests de sécurité](../../templates/04-security-testing/sast-integration-guide-implementation.md) pour la suite de tests complète.

### 7.2 Tests d'intrusion

Périmètre recommandé :
- Tests OWASP Top 10
- Fuzzing du flux de paiement
- Tentatives de contournement d'autorisation
- Tests de gestion de session

---

## 8. Correspondance avec la conformité

### Exigences PCI-DSS

| Exigence | Contrôle | Implémentation |
|----------|----------|----------------|
| 6.5.1 (Injection) | Validation entrées | Requêtes paramétrées |
| 6.5.3 (Cryptographie) | TLS 1.3 | Configuration Nginx |
| 6.5.10 (Auth cassée) | JWT + expiration courte | Middleware d'authentification |
| 12.3 (SAQ) | Conformité SAQ-A | Tokenisation Stripe.js |

### Exigences RGPD

| Exigence | Contrôle | Implémentation |
|----------|----------|----------------|
| Art. 32 (Sécurité) | Chiffrement | TLS 1.3, chiffrement BDD |
| Art. 5 (Minimisation données) | Collection PII minimale | Uniquement champs requis |
| Art. 17 (Droit à l'effacement) | Suppression données | API suppression compte utilisateur |

---

## 9. Prochaines étapes

1. ✅ **Modélisation des menaces terminée** - Ce document
2. ⏳ **Analyse de risques** - Voir [Analyse de risques](../02-risk-analysis/_exemple-ecommerce-risque.md)
3. ⏳ **Exigences de sécurité** - Documenter les contrôles requis dans la spec
4. ⏳ **Implémentation** - Implémenter les contre-mesures planifiées
5. ⏳ **Tests** - Exécuter les cas de test de sécurité
6. ⏳ **Revue** - Approbation du champion sécurité

---

## 10. Revue et approbation

**Auteur du modèle de menaces** : Architecte sécurité (security-champion-lead)
**Date de revue** : 2025-11-18
**Réviseurs** : security-champion-lead, security-champion-architect
**Statut d'approbation** : ✅ **APPROUVÉ** avec conditions (implémenter les contrôles planifiés)
**Prochaine revue** : 2026-02-18 (revue trimestrielle)

**Conditions pour la production** :
- ✅ Toutes les menaces CRITIQUES atténuées à faible
- ✅ Toutes les menaces ÉLEVÉES atténuées à faible ou moyen avec contrôles compensatoires
- ⏳ Protection DDoS (WAF) implémentée avant les soldes Black Friday
- ⏳ Tests d'intrusion complétés et résultats corrigés

---

**Modèle utilisé** : [stride-threat-model-template-planning.md](../../templates/01-threat-modeling/stride-threat-model-template-planning.md)
**Documents associés** :
- [Analyse de risques](../02-risk-analysis/_exemple-ecommerce-risque.md)
- [Exigences de sécurité](../../templates/03-security-requirements/authentication-requirements-template-design.md)
- [Conformité PCI-DSS](../../domaines/rgpd/templates/gdpr-dpia-template.md) (protection des données)
