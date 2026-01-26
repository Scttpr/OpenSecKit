# Modèles d'exigences de sécurité

## Aperçu

Ce répertoire contient des modèles pour le **Principe constitutionnel III : sécurité dès la conception** - l'intégration de contrôles de sécurité dans l'architecture et la conception du système avant le début de l'implémentation.

## Pourquoi des exigences de sécurité ?

Les exigences de sécurité vous aident à :

- **Concevoir la sécurité** dès le départ (pas l'ajouter après coup)
- **Traduire les menaces en contrôles** (modèle de menaces → exigences)
- **Communiquer les besoins de sécurité** aux développeurs et architectes
- **Vérifier l'implémentation de la sécurité** (exigences → cas de tests)
- **Démontrer la conformité** avec les cadres de sécurité

## Modèles disponibles

| Modèle | Phase | Difficulté | Temps | Description |
|--------|-------|------------|-------|-------------|
| [`asvs-guide.md`](asvs-guide.md) | Conception | Intermédiaire | 2-3 heures | Guide OWASP ASVS pour identifier les contrôles de sécurité requis |
| [`authentication-guide.md`](authentication-guide.md) | Conception | Intermédiaire | 1-2 heures | Politiques de mots de passe, MFA, gestion de sessions, exigences SSO |
| [`authorization-guide.md`](authorization-guide.md) | Conception | Intermédiaire | 1-2 heures | RBAC, permissions, contrôle d'accès, prévention IDOR |
| [`encryption-guide.md`](encryption-guide.md) | Conception | Avancé | 1-2 heures | Données au repos, en transit, gestion des clés, algorithmes cryptographiques |

## Quand utiliser ces modèles

### Phase de conception

- Utiliser le **Guide OWASP ASVS** pour identifier tous les contrôles de sécurité requis
- Utiliser le **Guide d'Authentification** pour concevoir la connexion, MFA, gestion de sessions
- Utiliser le **Guide d'Autorisation** pour concevoir les rôles, permissions, contrôle d'accès
- Utiliser le **Guide de Chiffrement** pour concevoir la protection des données et gestion des clés

### Workflow recommandé

```
1. Compléter la modélisation des menaces (Principe I)
   ↓
2. Noter les risques (Principe II)
   ↓
3. Identifier les contrôles requis (Guide OWASP ASVS)
   ↓
4. Concevoir le système d'authentification
   ↓
5. Concevoir le système d'autorisation
   ↓
6. Concevoir la stratégie de chiffrement
   ↓
7. Documenter les exigences de sécurité
   ↓
8. Créer les cas de tests (Principe IV)
```

## Démarrage rapide

### Première définition d'exigences de sécurité ?

**Commencez ici** : [`asvs-guide.md`](asvs-guide.md)

1. **Déterminer le niveau ASVS** : Niveau 1 (basique), Niveau 2 (standard), ou Niveau 3 (haute sécurité)
2. **Réviser la checklist ASVS** : Parcourir chaque catégorie d'exigences
3. **Marquer les exigences applicables** : Cocher celles qui s'appliquent à votre application
4. **Approfondir** : Utiliser les guides spécialisés (auth, authz, chiffrement) pour les détails
5. **Documenter les décisions** : Capturer la justification pour chaque exigence

### Besoin d'exigences spécifiques ?

**Authentification** : [`authentication-guide.md`](authentication-guide.md)

- Politiques de mots de passe, force, stockage (bcrypt)
- Authentification multi-facteurs (TOTP, SMS)
- Gestion de sessions (timeouts, cookies)
- Intégration SSO (OAuth, SAML)

**Autorisation** : [`authorization-guide.md`](authorization-guide.md)

- Définitions de rôles (admin, utilisateur, invité)
- Matrice de permissions (qui peut faire quoi)
- RBAC vs ABAC
- Prévention IDOR

**Chiffrement** : [`encryption-guide.md`](encryption-guide.md)

- Configuration TLS (TLS 1.3, suites de chiffrement)
- Chiffrement de base de données (au repos)
- Gestion des clés (rotation, stockage)
- Algorithmes approuvés (AES-256, RSA-4096)

## Intégration avec les autres principes

Les exigences de sécurité font le lien entre conception et implémentation :

**Entrées** :

- **Principe I (modélisation des menaces)** : Les menaces déterminent quels contrôles sont nécessaires
- **Principe II (analyse de risques)** : Les risques prioritaires exigent des contrôles plus forts

**Sorties** :

- **Principe IV (tests de sécurité)** : Les exigences deviennent des cas de tests
- **Phase d'implémentation** : Les exigences guident l'implémentation par les développeurs
- **Revue de Code** : Vérifier que les exigences sont correctement implémentées

## Conformité constitutionnelle

D'après la constitution SSDLC, le Principe III exige :

- **Contrôles de sécurité documentés** en phase de conception (cette checklist)
- **Contrôles adressant les menaces identifiées** (lien avec le modèle de menaces)
- **Défense en profondeur** (multiples couches de sécurité)
- **Principe du moindre privilège** (accès minimal requis)
- **Sécurité par défaut** (refus par défaut, échec sécurisé)

## Modèles courants d'exigences de sécurité

### Authentification

- Mot de passe : 12+ caractères, hachage bcrypt, détection de violations
- MFA : Obligatoire pour admins, optionnel pour utilisateurs
- Session : Cookies HttpOnly, timeout inactif 30min, timeout absolu
- SSO : OAuth 2.0 + OIDC

### Autorisation

- Modèle : RBAC (contrôle d'accès basé sur les rôles)
- Par défaut : Refus par défaut (autorisation explicite requise)
- Application : Côté serveur, couche service
- Prévention : Vérifications IDOR, validation de propriété

### Chiffrement

- En Transit : TLS 1.3, chiffrements forts uniquement, HSTS
- Au Repos : AES-256-GCM, TDE base de données, S3 SSE-KMS
- Clés : Vault/KMS, chiffrement d'enveloppe, rotation (90 jours)
- Algorithmes : Approuvés NIST uniquement (pas de crypto personnalisé)

### Validation des entrées

- Stratégie : liste blanche (validation positive)
- Sanitisation : encodage de sortie contextuel
- Prévention d'injection : requêtes paramétrées, pas d'eval()

### Journalisation

- Événements : Authentification, autorisation, accès données, actions admin
- Pas de PII : Nettoyer les données sensibles des journaux
- Intégrité : Stockage de journaux immuable (SIEM)
- Rétention : 90 jours minimum

## Mapping de conformité

Ces modèles aident à satisfaire :

- **OWASP ASVS 4.0** : Application Security Verification Standard
- **NIST SP 800-53** : Contrôles de sécurité et Confidentialité
- **PCI-DSS** : Exigences 6, 8 (Développement Sécurisé, Authentification)
- **HIPAA** : § 164.308(a)(4) Contrôles d'Accès, § 164.312 Garanties Techniques
- **ISO 27001** : Annexe A.9 Contrôle d'Accès, A.10 Cryptographie

## Outils et ressources

### Outils d'exigences de sécurité

- **OWASP ASVS** : <https://owasp.org/www-project-application-security-verification-standard/>
- **NIST SP 800-53** : <https://csrc.nist.gov/publications/detail/sp/800-53/rev-5/final>
- **BSIMM** : <https://www.bsimm.com/>

### Ressources externes

- [OWASP Cheat Sheet Series](https://cheatsheetseries.owasp.org/)
- [NIST Cybersecurity Framework](https://www.nist.gov/cyberframework)
- [CWE Top 25](https://cwe.mitre.org/top25/)

## Contribuer

Vous avez des améliorations pour les modèles d'exigences de sécurité ? Voir les [directives de contribution](../../CONTRIBUTING.txt).

Contributions précieuses :

- Modèles d'exigences spécifiques aux secteurs (santé, finance)
- Intégration avec les outils de tests de sécurité
- Exemples d'exigences pour technologies émergentes (IA/ML, blockchain)

---

**Besoin d'aide ?** Ouvrir une [Discussion GitHub](https://github.com/Scttpr/OpenSecKit/issues) avec le tag `03-security-requirements`.

**Prochaines étapes** : Après avoir défini les exigences de sécurité, procéder aux [tests de sécurité](../IV-security-testing/) pour créer des cas de tests vérifiant l'implémentation.
