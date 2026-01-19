# /osk-discover validate

Valide la complétude et cohérence du system-model, avec résolution optionnelle des gaps.

## Synopsis

```
/osk-discover validate [--resolve]
```

## Description

Vérifie que le system-model est complet et cohérent avant de passer aux phases Comply ou Secure. Identifie les gaps bloquants et recommande des actions.

Avec `--resolve`, guide l'utilisateur à travers les gaps identifiés pour les résoudre interactivement.

## Options

| Option | Description |
|--------|-------------|
| `--resolve` | Après validation, résoudre les gaps interactivement |

## Prérequis

- System-model existant (`.osk/system-model/index.yaml`)

## CLI Utilisées

| Commande | Usage |
|----------|-------|
| `osk validate system-model --json` | Validation YAML et références |

## Vérifications

### Complétude
- [ ] Toutes les sections requises présentes
- [ ] Champs obligatoires remplis
- [ ] Données PII identifiées
- [ ] Trust boundaries définies

### Cohérence
- [ ] Références croisées valides (IDs existent)
- [ ] Pas de composants orphelins
- [ ] Data flows cohérents

### Contraintes
- [ ] `index.yaml` < 200 lignes
- [ ] YAML valide syntaxiquement

## Output

Rapport de validation avec statut par section :
- ✅ Ready - Prêt pour comply/secure
- ⚠️ Warnings - Utilisable mais améliorable
- ❌ Blocking - Gaps à résoudre d'abord

## Exemple (Validation)

```
>>> /osk-discover validate

🔍 Validating system model...

COMPLETENESS CHECK
==================
✅ index.yaml (187 lines)
✅ business.yaml
✅ architecture.yaml
⚠️ data.yaml - Missing: retention policy for USER-PROFILE
✅ actors.yaml
✅ boundaries.yaml
❌ integrations.yaml - Missing: DPA status for Stripe
✅ controls.yaml
⚠️ gaps.yaml - 3 unresolved gaps

CONSISTENCY CHECK
=================
✅ All component references valid
✅ All data category references valid
⚠️ COMP-LEGACY referenced but marked as deleted

SUMMARY
=======
Status: ⚠️ WARNINGS (usable with caveats)

Blocking issues: 1
  → integrations.yaml: DPA status required for compliance

Warnings: 3
  → data.yaml: retention policy recommended
  → gaps.yaml: 3 gaps should be resolved
  → COMP-LEGACY: remove stale reference

Run /osk-discover validate --resolve to fix gaps interactively.
```

## Exemple (With Resolution)

```
>>> /osk-discover validate --resolve

🔍 Validating system model...
[... validation output ...]

🔍 Gap Resolution
=================

Found 4 gaps requiring attention:

CRITICAL (blocks compliance):
  ⛔ GAP-001: Data retention policy unknown
     Category: data | Action: USER_INPUT_REQUIRED

HIGH (security risk):
  🔴 GAP-002: DPA status for Stripe integration unknown
     Category: integration | Action: USER_INPUT_REQUIRED

Options:
[A]ll - Resolve all gaps in order of severity
[C]ritical - Only resolve critical gaps
[S]elect - Choose specific gaps to resolve

Choice: A

───────────────────────────────────────
GAP-001: Data retention policy unknown
───────────────────────────────────────
Category: data
Affected: USER-PROFILE, PAYMENT-HISTORY
Why it matters: Required for RGPD compliance (Art. 5)

? What is your data retention policy for user profiles?
  [1] 1 year after account deletion
  [2] 3 years after last activity
  [3] 7 years (legal requirement)
  [4] Custom...

Choice: 2

✓ GAP-001 resolved → data.yaml updated

...

✅ Resolution complete
   - 4 gaps resolved
   - 0 gaps remaining
   - System model ready for compliance assessment
```

## Voir aussi

- [`/osk-discover`](osk-discover.md) - Build/update le system-model
- [`/osk-comply rgpd`](osk-comply-rgpd.md) - Évaluation RGPD (requiert validation)
