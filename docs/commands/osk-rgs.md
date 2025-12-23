# /osk-rgs

Conformité RGS - EBIOS RM et homologation.

## Synopsis

```bash
>>> /osk-rgs [renew]
```

## Description

`/osk-rgs` gère la conformité au Référentiel Général de Sécurité pour les administrations françaises.

## Modes

### Initialisation (défaut)

```bash
>>> /osk-rgs
```

Génère la documentation RGS :

- Analyse EBIOS RM
- Dossier d'homologation
- Plan de Maintien en Condition de Sécurité (MCS)

### Ré-homologation

```bash
>>> /osk-rgs renew
```

Génère un rapport de ré-homologation avec l'évolution depuis le dernier audit.

## Fichiers Générés

| Mode | Fichiers |
|------|----------|
| Initialisation | `docs/security/rgs/EBIOS-RM-{systeme}.md` |
| | `docs/security/rgs/DOSSIER-HOMOLOGATION-{systeme}.md` |
| | `docs/security/rgs/MCS-{systeme}.md` |
| Renew | `docs/security/rgs/AUDIT-YYYY-MM-DD.md` |

## Niveaux RGS

| Niveau | Description |
|--------|-------------|
| Standard | Niveau de base |
| Renforcé | Exigences supplémentaires |
| * | Niveau maximum (sensible défense) |

## Principes Couverts

| Principe | Exigence RGS |
|----------|--------------|
| I - Threat Modeling | EBIOS RM |
| II - Risk Analysis | Analyse de risques |
| III - Security by Design | Mesures de sécurité |
| V - Secrets Management | Cryptographie |
| VI - Audit Logging | Traçabilité |

## Prérequis

- `/osk-configure` exécuté
- Domaine RGS activé

## Voir aussi

- [Domaine RGS](../domains/rgs.md)
