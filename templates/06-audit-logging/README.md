# Modèles de Journalisation d'Audit

## Aperçu

Ce répertoire contient des modèles pour le **Principe Constitutionnel VI : Journalisation d'Audit** - journalisation complète des événements de sécurité pour la conformité, la réponse aux incidents et l'analyse forensique.

## Modèles disponibles

| Modèle | Description |
|--------|-------------|
| [modele-exigences-journalisation-conception.md](logging-requirements-template-design.md) | Quoi journaliser, rétention, protection |
| [siem-integration-guide.md](siem-integration-guide.md) | Intégration SIEM (Splunk, ELK) |
| [security-alert-rules-template.md](security-alert-rules-template.md) | Règles de surveillance et alertes |

## Principes Clés

- ✅ **Journaliser les événements de sécurité** : Auth, autorisation, accès données, actions admin
- ❌ **Ne jamais journaliser de données sensibles** : Mots de passe, tokens, numéros de cartes complets
- ✅ **Protéger les journaux** : Stockage immuable, chiffrement, contrôle d'accès
- ✅ **Conserver les journaux** : 90 jours (sécurité), 7 ans (conformité)

---

**Prochaines étapes** : Procéder à la [Gestion des Correctifs](../07-patch-management/).
