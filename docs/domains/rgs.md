# RGS

Référentiel Général de Sécurité (France).

## Vue d'ensemble

Le RGS définit les règles de sécurité pour les systèmes d'information des administrations françaises.

## Niveaux

| Niveau | Description | Usage |
|--------|-------------|-------|
| Standard | Niveau de base | Téléservices classiques |
| Renforcé | Exigences supplémentaires | Données sensibles |
| * | Maximum | Sensible défense |

## Commandes

### Initialisation

```bash
>>> /osk-rgs
```

Génère :

- `docs/security/rgs/EBIOS-RM-{systeme}.md`
- `docs/security/rgs/DOSSIER-HOMOLOGATION-{systeme}.md`
- `docs/security/rgs/MCS-{systeme}.md`

### Ré-homologation

```bash
>>> /osk-rgs renew
```

Génère un rapport de ré-homologation.

## EBIOS RM

Méthode d'analyse de risques de l'ANSSI :

### Ateliers

| Atelier | Objectif |
|---------|----------|
| 1 | Cadrage et socle de sécurité |
| 2 | Sources de risque |
| 3 | Scénarios stratégiques |
| 4 | Scénarios opérationnels |
| 5 | Traitement du risque |

## Homologation

Le dossier d'homologation contient :

1. **Stratégie d'homologation** - Périmètre et planning
2. **Analyse de risques** - EBIOS RM
3. **PSSI** - Politique de sécurité
4. **Plan de traitement** - Mesures de sécurité
5. **Décision d'homologation** - Validation autorité

## MCS - Maintien en Condition de Sécurité

| Activité | Fréquence |
|----------|-----------|
| Veille vulnérabilités | Continue |
| Patch management | Selon SLA |
| Audits de sécurité | Annuel |
| Tests d'intrusion | Annuel |
| Revue des droits | Trimestriel |

## Exigences Cryptographiques

Selon le référentiel ANSSI :

| Usage | Algorithme | Taille clé |
|-------|------------|------------|
| Chiffrement symétrique | AES | 256 bits |
| Hash | SHA-256+ | - |
| Signature | RSA | 3072+ bits |
| Échange de clés | ECDH | P-256+ |

## Voir aussi

- [Commande /osk-rgs](../commands/osk-rgs.md)
- [ANSSI - RGS](https://www.ssi.gouv.fr/entreprise/reglementation/confiance-numerique/le-referentiel-general-de-securite-rgs/)
- [ANSSI - EBIOS RM](https://www.ssi.gouv.fr/guide/la-methode-ebios-risk-manager-le-guide/)
