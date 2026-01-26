# Gestion des secrets

## Aperçu

Ce répertoire contient des modèles pour le **Principe constitutionnel V : gestion rigoureuse des secrets** - l'élimination totale des secrets en clair du code source et leur gestion centralisée sécurisée.

## Pourquoi la gestion des secrets ?

La gestion des secrets vous protège contre :

- **Fuites de données** : 73% des fuites impliquent des secrets exposés (GitHub Security Report 2024)
- **Compromission de comptes** : clés API exposées = accès non autorisé
- **Violations réglementaires** : exposition de secrets = non-conformité RGPD, PCI-DSS
- **Risque humain** : les développeurs commitent accidentellement des secrets

**Principe absolu** : Aucun secret (mot de passe, clé API, certificat, token) ne doit jamais se trouver dans le code source.

## Templates disponibles

| Template | Phase | Difficulté | Temps | Description |
|----------|-------|------------|-------|-------------|
| [`vault-guide.md`](vault-guide.md) | Implémentation | Intermédiaire | 2-3 heures | Guide d'intégration d'un gestionnaire de secrets (Vault, Key Vault, Secrets Manager) |
| [`rotation-guide.md`](rotation-guide.md) | Planification | Débutant | 1 heure | Politique de rotation automatique des secrets critiques |
| [`detection-guide.md`](detection-guide.md) | Implémentation | Débutant | 30 min | Configuration de la détection pré-commit (gitleaks, trufflehog) |

## Quand utiliser ces templates

### Phase de planification

- Définir la **politique de rotation** dès la conception du projet
- Identifier les **types de secrets** à gérer (API keys, certificats, tokens, mots de passe)
- Choisir le **gestionnaire de secrets** adapté au contexte

### Phase d'implémentation

- Intégrer le **gestionnaire de secrets** dans le code (voir vault-guide.md)
- Configurer la **détection pré-commit** pour bloquer les commits contenant des secrets
- Migrer les secrets existants vers le gestionnaire

### Phase de maintenance

- Automatiser la **rotation des secrets** selon la politique définie
- Auditer les **accès aux secrets** (qui a accédé à quoi et quand)
- Révoquer immédiatement les secrets compromis

## Workflow recommandé

```
1. Définir la politique de rotation
   ↓
2. Choisir le gestionnaire de secrets
   (Vault, Azure Key Vault, AWS Secrets Manager)
   ↓
3. Configurer la détection pré-commit
   (gitleaks, trufflehog)
   ↓
4. Intégrer le gestionnaire dans le code
   ↓
5. Migrer les secrets existants
   ↓
6. Automatiser la rotation
   ↓
7. Configurer les alertes d'accès
```

## Démarrage rapide

### Première fois ?

**Commencez ici** : [`detection-guide.md`](detection-guide.md)

1. **Installer gitleaks** : empêche les commits contenant des secrets
2. **Scanner le code existant** : détecter les secrets déjà présents
3. **Migrer vers un gestionnaire** : suivre vault-guide.md
4. **Définir la politique** : utiliser rotation-guide.md

### Vous avez déjà un gestionnaire de secrets ?

**Passez à** : [`rotation-guide.md`](rotation-guide.md)

1. **Documenter la politique** de rotation
2. **Automatiser la rotation** des secrets critiques
3. **Configurer les audits** d'accès
4. **Former l'équipe** aux bonnes pratiques

## Choisir le bon template

### Utiliser vault-guide.md quand

- Vous démarrez un nouveau projet
- Vous devez migrer depuis des secrets en clair
- Vous voulez centraliser la gestion des secrets
- Vous devez respecter des exigences de conformité strictes

### Utiliser rotation-guide.md quand

- Vous avez déjà un gestionnaire de secrets
- Vous devez définir les règles de rotation
- Vous préparez un audit de sécurité
- Vous voulez automatiser la rotation

### Utiliser detection-guide.md quand

- Vous voulez bloquer les secrets dans les commits (obligatoire pour tous)
- Vous devez scanner le code existant
- Vous intégrez un nouveau développeur
- Vous voulez ajouter une protection CI/CD

## Règles absolues

### Ne JAMAIS faire

- Commiter un secret en clair dans le code (`.env`, `config.yml`, variables hardcodées)
- Partager des secrets par email, Slack, ou tout autre canal non sécurisé
- Utiliser le même secret en dev, staging et production
- Stocker des secrets dans des variables d'environnement publiques
- Laisser des secrets dans l'historique git (même après suppression)

### TOUJOURS faire

- Utiliser un gestionnaire de secrets dédié (Vault, Key Vault, Secrets Manager)
- Activer la détection pré-commit (gitleaks, trufflehog)
- Faire tourner les secrets critiques tous les 90 jours maximum
- Logger tous les accès aux secrets (qui, quoi, quand)
- Révoquer immédiatement un secret suspect

## Gestionnaires de secrets recommandés

### Open source

- **HashiCorp Vault** : solution complète, self-hosted
- **Mozilla SOPS** : chiffrement de fichiers de configuration
- **Doppler** : freemium, facile à démarrer

### Cloud providers

- **Azure Key Vault** : intégré Azure, excellent pour .NET
- **AWS Secrets Manager** : intégré AWS, rotation automatique
- **GCP Secret Manager** : intégré Google Cloud

### CI/CD

- **GitHub Secrets** : pour les workflows GitHub Actions
- **GitLab CI Variables** : pour les pipelines GitLab
- **CircleCI Contexts** : pour CircleCI

## Outils de détection

### Scan pré-commit

- **gitleaks** : rapide, nombreuses règles prédéfinies
- **trufflehog** : détection d'entropie élevée (secrets générés)
- **detect-secrets** : baseline pour ignorer les faux positifs

### Scan en CI/CD

- **GitGuardian** : SaaS, détection temps réel
- **Spectral** : open source, intégration CI/CD
- **gitleaks en CI** : gratuit, efficace

## Intégration avec les autres principes

La gestion des secrets alimente :

- **Principe VI (journalisation d'audit)** : tous les accès aux secrets doivent être loggés
- **Principe III (sécurité dès la conception)** : secrets gérés dès le début du projet
- **Principe IV (tests de sécurité)** : scan de secrets dans le pipeline CI/CD
- **Principe VII (patch management)** : rotation = patch des credentials

## Conformité

Ces templates aident à satisfaire les exigences de :

- **OWASP Top 10** : A07:2021 – Identification and Authentication Failures
- **PCI-DSS** : Requirement 8.2 - Authentication credentials must be protected
- **RGPD** : Article 32 - Security of processing
- **ISO 27001** : A.9.4 - Secret authentication information of users
- **SOC 2** : CC6.1 - Logical and physical access controls

## Métriques de succès

Pour mesurer l'efficacité de la gestion des secrets :

- **Taux de conformité** : 100% des commits passent le scan pré-commit
- **Temps de rotation** : < 90 jours pour tous les secrets critiques
- **Détection de fuites** : 0 secret exposé publiquement
- **Temps de révocation** : < 1h après détection d'une compromission
- **Couverture d'audit** : 100% des accès aux secrets loggés

## Ressources externes

### Documentation officielle

- [HashiCorp Vault Best Practices](https://learn.hashicorp.com/vault)
- [AWS Secrets Manager User Guide](https://docs.aws.amazon.com/secretsmanager/)
- [Azure Key Vault Documentation](https://docs.microsoft.com/azure/key-vault/)

### Guides et articles

- [OWASP Secret Management Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Secrets_Management_Cheat_Sheet.html)
- [GitHub Security Best Practices](https://docs.github.com/en/code-security/secret-scanning)

## Contribuer

Vous avez des améliorations pour ces templates ? Voir les [directives de contribution](../../CONTRIBUTING.txt).

Contributions courantes :

- Guides d'intégration pour d'autres langages/frameworks
- Scripts d'automatisation de rotation
- Exemples de migration depuis secrets en clair
- Configurations de détection avancées

---

**Besoin d'aide ?** Ouvrir une [Discussion GitHub](https://github.com/Scttpr/OpenSecKit/issues) avec le tag `secrets-management`.

**Prochaines étapes** : Après la gestion des secrets, passer à la [journalisation d'audit](../VI-audit-logging/) pour tracer les accès.
