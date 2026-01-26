# Modèles de Journalisation d'Audit

## Aperçu

Ce répertoire contient des modèles pour le **Principe Constitutionnel VI : Journalisation d'Audit** - journalisation complète des événements de sécurité pour la conformité, la réponse aux incidents et l'analyse forensique.

## Modèles disponibles

| Modèle | Description |
|--------|-------------|
| [`logging-strategy.md`](logging-strategy.md) | Quoi journaliser, rétention, protection |
| [`centralization-guide.md`](centralization-guide.md) | Exigences pour centraliser les logs |
| [`siem-integration.md`](siem-integration.md) | Guide d'intégration SIEM |
| [`alert-rules-guide.md`](alert-rules-guide.md) | Règles de surveillance et alertes |

## Principes Clés

- **Journaliser les événements de sécurité** : Auth, autorisation, accès données, actions admin
- **Ne jamais journaliser de données sensibles** : Mots de passe, tokens, numéros de cartes complets
- **Protéger les journaux** : Stockage immuable, chiffrement, contrôle d'accès
- **Conserver les journaux** : 90 jours (sécurité), 7 ans (conformité)

---

**Prochaines étapes** : Procéder à la [Gestion des Correctifs](../VII-patch-management/).
