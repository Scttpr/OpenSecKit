---
title: "Modèle d'exigences d'autorisation - Phase de conception"
template_version: "1.0.0"
constitutional_principle: "III - Security by Design"
ssdlc_phase: "conception"
last_updated: "2025-01-15"
reviewers:
  - security-champion-team
description: "Modèle d'exigences d'autorisation couvrant RBAC, ABAC, permissions niveau ressource et stratégies de contrôle d'accès."
tags:
  - authorization
  - access-control
  - rbac
  - permissions
difficulty: "intermédiaire"
estimated_time: "1-2 heures"
prerequisites:
  - "Exigences d'authentification définies"
  - "Rôles utilisateur identifiés"
related_templates:
  - "authentication-requirements-template-design.md"
  - "owasp-asvs-checklist-design.md"
compliance_frameworks:
  - "OWASP ASVS"
  - "NIST SP 800-162"
---

# Modèle d'exigences d'autorisation - Phase de conception

## 1. Stratégie d'autorisation

**Modèle contrôle d'accès** : ☑ RBAC (basé rôles) ☐ ABAC (basé attributs) ☐ Hybride RBAC + ABAC

**Politique par défaut** : ☑ Refus par défaut (autorisation explicite requise) ☐ Autorisation par défaut

---

## 2. Rôles et permissions

### Définitions rôles

| Rôle | Description | Permissions | Nombre utilisateurs (est.) |
|------|-------------|-------------|---------------------------|
| **Admin** | Accès complet système | Toutes permissions | < 5 |
| **Manager** | Gestion équipe, rapports | users:read, users:update, reports:read, reports:create | 50 |
| **User** | Accès utilisateur standard | profile:read, profile:update, content:read, content:create | 10 000 |
| **Guest** | Accès limité lecture seule | content:read (public uniquement) | Illimité |

### Matrice permissions

| Ressource | Action | Admin | Manager | User | Guest |
|-----------|--------|-------|---------|------|-------|
| Users | Create | ✓ | ✓ | ✗ | ✗ |
| Users | Read All | ✓ | ✓ | ✗ | ✗ |
| Users | Read Own | ✓ | ✓ | ✓ | ✗ |
| Users | Update Own | ✓ | ✓ | ✓ | ✗ |
| Users | Update Any | ✓ | ✓ | ✗ | ✗ |
| Users | Delete | ✓ | ✗ | ✗ | ✗ |
| Content | Create | ✓ | ✓ | ✓ | ✗ |
| Content | Read Public | ✓ | ✓ | ✓ | ✓ |
| Content | Read Private | ✓ | ✓ | Propriétaire uniquement | ✗ |
| Content | Update | ✓ | ✓ | Propriétaire uniquement | ✗ |
| Content | Delete | ✓ | ✓ | Propriétaire uniquement | ✗ |
| Reports | Read | ✓ | ✓ | ✗ | ✗ |
| System Config | Read | ✓ | ✗ | ✗ | ✗ |
| System Config | Update | ✓ | ✗ | ✗ | ✗ |

---

## 3. Application autorisation

### Points d'application

**Où l'autorisation est vérifiée** :
- ☑ Passerelle API (grain grossier : utilisateur authentifié ?)
- ☑ Couche Service (grain fin : utilisateur a permission pour cette action ?)
- ☑ Couche Base de Données (sécurité niveau ligne si applicable)

**Ne jamais se fier à** :
- ☐ Autorisation côté client (facilement contournée)
- ☐ Obfuscation URL ("sécurité par obscurité")

### Flux d'autorisation

```
1. Requête arrive avec cookie session
2. Extraire ID utilisateur de session
3. Charger rôles et permissions utilisateur depuis cache/BD
4. Vérifier permission pour ressource + action demandée
5. Si autorisé : procéder
6. Si non autorisé : retourner 403 Forbidden
7. Logger décision autorisation (succès ou échec)
```

---

## 4. Propriété ressources

### Accès basé propriété

**Ressources avec propriété** :
- Profils utilisateur (propriétaire : utilisateur)
- Éléments contenu (propriétaire : créateur)
- Projets (propriétaire : créateur, partagé avec membres équipe)

**Règles propriété** :
- Propriétaire a accès CRUD complet aux ressources possédées
- Non-propriétaires nécessitent octrois explicites
- Admin peut accéder toutes ressources (avec log audit)

**implémentation** :
```sql
-- Vérifier propriété avant mise à jour
SELECT * FROM content WHERE id = ? AND owner_id = ?
```

### Délégation

**Accès partagé** : ☑ Supporté via octrois explicites

| Ressource | Modèle partage |
|-----------|----------------|
| Content | Propriétaire peut octroyer lecture/écriture à utilisateurs spécifiques |
| Projects | Membres équipe avec rôles définis |

---

## 5. Permissions hiérarchiques

**Hiérarchie rôles** : ☐ Supportée ☑ Non requise

**Considération future** : Hiérarchie Admin > Manager > User > Guest

---

## 6. Contrôle d'accès basé attributs (ABAC)

**Règles ABAC** : ☐ Implémentées ☑ Amélioration future

**Attributs potentiels** :
- Localisation utilisateur (géolocalisation IP)
- Heure du jour (restreindre accès heures bureau)
- Type appareil (mobile vs desktop)
- Niveau classification données

---

## 7. Autorisation API

### Endpoints REST API

| Endpoint | Méthode | Permission requise | Notes |
|----------|---------|-------------------|-------|
| `/api/users` | GET | `users:read` ou admin | Lister tous utilisateurs |
| `/api/users/{id}` | GET | `users:read` ou propriétaire ou admin | Obtenir détails utilisateur |
| `/api/users/{id}` | PUT | Propriétaire ou `users:update` ou admin | Mettre à jour utilisateur |
| `/api/users/{id}` | DELETE | Admin uniquement | Supprimer utilisateur |
| `/api/content` | POST | `content:create` | Créer contenu |
| `/api/content/{id}` | GET | Public : tous, Privé : propriétaire ou admin | Obtenir contenu |
| `/api/content/{id}` | PUT | Propriétaire ou admin | Mettre à jour contenu |
| `/api/content/{id}` | DELETE | Propriétaire ou admin | Supprimer contenu |
| `/api/admin/*` | ANY | Admin uniquement | Endpoints admin |

### En-têtes autorisation

**Basé session** :
```
Cookie: __Host-session=<session-token>
```

**Clé API (pour comptes service)** :
```
Authorization: Bearer <api-key>
```

---

## 8. Prévention référence objet directe non sécurisée (IDOR)

**Stratégies** :
- ☑ Utiliser UUID au lieu IDs séquentiels pour ressources
- ☑ Toujours vérifier propriété/permissions avant retourner ressource
- ☑ Ne jamais faire confiance IDs fournis client sans validation

**Exemple code vulnérable** :
```javascript
// MAUVAIS : Pas de vérification autorisation
app.get('/api/users/:id', (req, res) => {
  const user = db.getUserById(req.params.id);
  return res.json(user); // Retourne n'importe quel utilisateur !
});
```

**Exemple code sécurisé** :
```javascript
// BON : Autorisation appliquée
app.get('/api/users/:id', (req, res) => {
  const requestedUserId = req.params.id;
  const currentUserId = req.session.userId;

  // Vérifier si utilisateur demande ses propres données, ou est admin
  if (requestedUserId !== currentUserId && !req.user.isAdmin) {
    return res.status(403).json({ error: 'Forbidden' });
  }

  const user = db.getUserById(requestedUserId);
  return res.json(user);
});
```

---

## 9. Prévention escalade privilèges

**Escalade privilèges horizontale** (accéder données autres utilisateurs) :
- ☑ Vérifier ID utilisateur correspond propriétaire ressource
- ☑ Valider tous IDs fournis par utilisateur

**Escalade privilèges verticale** (obtenir privilèges supérieurs) :
- ☑ Changements rôle nécessitent approbation admin (pas self-service)
- ☑ Changements permission loggés et audités
- ☑ Opérations sensibles nécessitent ré-authentification

---

## 10. Contrôles d'accès admin

**Accès panneau admin** :
- ☑ Authentification séparée (requérir MFA)
- ☑ Liste blanche IP (VPN ou réseau bureau uniquement)
- ☑ Journalisation audit pour toutes actions admin

**Permissions admin** :
- Peut impersonner utilisateurs (pour support, avec log audit)
- Peut modifier toutes données (avec log audit)
- Peut octroyer/révoquer permissions

**Responsabilité admin** :
- Toutes actions admin loggées avec ID utilisateur admin
- Revue périodique logs accès admin
- Recertification annuelle accès admin

---

## 11. Journalisation et surveillance

### Événements à logger

| Événement | Champs |
|-----------|--------|
| Succès autorisation | ID utilisateur, ressource, action, horodatage |
| Échec autorisation | ID utilisateur, ressource tentée, action, horodatage, raison |
| Changement permission | ID utilisateur, utilisateur modifié, anciennes permissions, nouvelles permissions, ID admin, horodatage |
| Impersonnification admin | ID admin, ID utilisateur impersonné, durée, horodatage |

**Détection anomalies** :
- Alerte sur schémas d'accès inhabituels (utilisateur accédant nombreuses ressources rapidement)
- Alerte sur échecs autorisation répétés
- Alerte sur tentatives escalade permission

---

## 12. Liste de vérification implémentation

- [ ] Définir matrice rôles et permissions
- [ ] Implémenter logique attribution rôles
- [ ] Appliquer autorisation couche service (pas seulement client)
- [ ] Utiliser UUID pour identifiants ressources
- [ ] Valider propriété pour tous accès ressource
- [ ] Implémenter politique refus par défaut
- [ ] Ajouter journalisation autorisation
- [ ] Tester vulnérabilités IDOR (test intrusion)
- [ ] Tester scénarios escalade privilèges
- [ ] Documenter modèle autorisation dans docs architecture

---

## Ressources liées

- [Exigences authentification](authentication-requirements-template-design.md)
- [OWASP ASVS V4](https://github.com/OWASP/ASVS/blob/master/4.0/en/0x12-V4-Access-Control.md)
- [OWASP Authorization Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Authorization_Cheat_Sheet.html)
