# /osk-rgpd

Conformité RGPD - Registre des traitements et DPIA.

## Synopsis

```bash
>>> /osk-rgpd [audit]
```

## Description

`/osk-rgpd` gère la conformité au Règlement Général sur la Protection des Données.

## Modes

### Configuration (défaut)

```bash
>>> /osk-rgpd
```

Génère la documentation RGPD :

- Registre des traitements (Art. 30)
- DPIA global
- Procédure de violation
- Politique de conservation

### Audit

```bash
>>> /osk-rgpd audit
```

Effectue un audit de conformité et génère un rapport daté.

## Fichiers Générés

| Mode | Fichiers |
|------|----------|
| Configuration | `docs/security/rgpd/registre-traitements.md` |
| | `docs/security/rgpd/dpia-global.md` |
| | `docs/security/rgpd/procedure-violation.md` |
| | `docs/security/rgpd/politique-conservation.md` |
| Audit | `docs/security/rgpd/AUDIT-YYYY-MM-DD.md` |

## Principes Couverts

| Principe | Article RGPD |
|----------|--------------|
| II - Risk Analysis | Art. 35 (DPIA) |
| III - Security by Design | Art. 25 (Privacy by Design) |
| VI - Audit Logging | Art. 33 (Notification) |

## Prérequis

- `/osk-configure` exécuté
- `.osk/config.toml` présent
- Domaine RGPD activé

## Voir aussi

- [Domaine RGPD](../domains/rgpd.md)
