# Modèles de tests de sécurité

## Aperçu

Ce répertoire contient des modèles pour le **Principe Constitutionnel IV : tests de sécurité** - tests automatisés et manuels pour vérifier que les contrôles de sécurité sont correctement implémentés et que les vulnérabilités sont identifiées avant le déploiement.

## Modèles disponibles

| Modèle | Phase | Description |
|--------|-------|-------------|
| [guide-integration-sast-implementation.md](sast-integration-guide-implementation.md) | implémentation | tests de sécurité Statiques d'Application dans CI/CD |
| [dast-integration-guide-implementation.md](dast-integration-guide-implementation.md) | Tests | tests de sécurité Dynamiques d'Application |
| [sca-dependency-scanning.md](sca-dependency-scanning.md) | Toutes Phases | Analyse de Composition Logicielle pour dépendances |
| [security-regression-tests-template.md](security-regression-tests-template.md) | Tests | Périmètre et rapport de tests d'intrusion manuels |

## Démarrage rapide

**Implémenter dans cet ordre** :
1. **SCA** (plus facile, meilleur ROI) - Détecter les dépendances vulnérables
2. **SAST** (effort moyen) - Trouver les vulnérabilités du code tôt
3. **DAST** (nécessite application en cours d'exécution) - Tester l'application en direct
4. **Tests d'Intrusion** (manuel, pré-production) - Validation par expert

---

**Prochaines étapes** : Après les tests de sécurité, procéder à la [Gestion des Secrets](../05-secrets-management/) et la [Journalisation d'Audit](../06-audit-logging/).
