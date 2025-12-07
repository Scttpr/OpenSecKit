# Modèles de Journalisation d'Audit

## Aperçu

Ce répertoire contient des modèles pour le **Principe Constitutionnel VI : Journalisation d'Audit** - journalisation complète des événements de sécurité pour la conformité, la réponse aux incidents et l'analyse forensique.

## Modèles disponibles

| Modèle | Description |
|--------|-------------|
| [logging-requirements-template-design.md](logging-requirements-template-design.md) | Quoi journaliser, rétention, protection |
| [log-centralization-requirements.md](log-centralization-requirements.md) | Exigences pour centraliser les logs |
| [siem-selection-guide.md](siem-selection-guide.md) | Guide de sélection d'un SIEM |
| [security-alert-rules-template.md](security-alert-rules-template.md) | Règles de surveillance et alertes |
| [_example-ecommerce-logging.md](_example-ecommerce-logging.md) | Exemple e-commerce |

## Principes Clés

- ✅ **Journaliser les événements de sécurité** : Auth, autorisation, accès données, actions admin
- ❌ **Ne jamais journaliser de données sensibles** : Mots de passe, tokens, numéros de cartes complets
- ✅ **Protéger les journaux** : Stockage immuable, chiffrement, contrôle d'accès
- ✅ **Conserver les journaux** : 90 jours (sécurité), 7 ans (conformité)

---

**Prochaines étapes** : Procéder à la [Gestion des Correctifs](../07-patch-management/).
