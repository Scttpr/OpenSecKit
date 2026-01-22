# Framework RGPD

Conformité au Règlement Général sur la Protection des Données (UE 2016/679).

## Commande

```bash
/osk-comply rgpd              # Évaluation de conformité (workflow autonome 5 phases)
/osk-comply rgpd --update     # Mise à jour évaluation existante
/osk-comply rgpd --export md  # Export documents de conformité
```

## Workflow v5.0

Le workflow est **autonome** - il progresse automatiquement à travers les 5 phases sans intervention manuelle.

```
┌─────────────────────────────────────────────────────────────┐
│  PHASE 1: INVENTAIRE DES TRAITEMENTS                        │
│  ────────────────────────────────────────────────────────── │
│  • Découverte des activités de traitement                   │
│  • Identification des bases légales (Art. 6)                │
│  • Détermination des exigences AIPD                         │
│  Output: .osk/comply/rgpd/processing-inventory.yaml         │
└─────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│  PHASE 2: AIPD/DPIA (Conditionnelle)                        │
│  ────────────────────────────────────────────────────────── │
│  Méthodologie PIA CNIL en 4 étapes:                         │
│    1. Étude du contexte                                     │
│    2. Évaluation des principes fondamentaux                 │
│    3. Analyse des risques (3 scénarios)                     │
│    4. Validation (avis DPO, plan d'action)                  │
│  Output: .osk/comply/rgpd/aipd/{traitement}.yaml            │
└─────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│  PHASE 3: ÉVALUATION DES CONTRÔLES                          │
│  ────────────────────────────────────────────────────────── │
│  • Évaluation Articles 5-50                                 │
│  • Vérification mécanismes de transfert (SCCs, BCRs)        │
│  • Conformité sous-traitants (Art. 28)                      │
│  • Mesures de sécurité (25 fiches CNIL)                     │
│  Output: .osk/comply/rgpd/control-assessment.yaml           │
└─────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│  PHASE 4: ANALYSE DES ÉCARTS                                │
│  ────────────────────────────────────────────────────────── │
│  • Catégorisation (organisationnel, technique, légal)       │
│  • Matrice de priorité (BLOQUANT, QUICK_WIN, P1-P3)         │
│  • Feuille de route de remédiation                          │
│  Output: .osk/comply/rgpd/gaps-analysis.yaml                │
└─────────────────────────────────────────────────────────────┘
                          │
                          ▼
┌─────────────────────────────────────────────────────────────┐
│  PHASE 5: DOCUMENTATION                                     │
│  ────────────────────────────────────────────────────────── │
│  Documents générés:                                         │
│  • Registre de traitement (Art. 30)                         │
│  • Mesures de sécurité (Art. 32)                            │
│  • Procédure violation (Art. 33-34)                         │
│  • Procédure droits (Art. 12-22)                            │
│  • Politique confidentialité (Art. 13-14)                   │
│  Output: .osk/comply/rgpd/documents/                        │
└─────────────────────────────────────────────────────────────┘
```

## Documents Générés

| Fichier | Article RGPD | Description |
|---------|--------------|-------------|
| `registre-traitement.md` | Art. 30 | Inventaire des traitements |
| `mesures-securite.md` | Art. 32 | Documentation sécurité |
| `aipd-{traitement}.md` | Art. 35 | Analyse d'impact (si requis) |
| `lia-{traitement}.md` | Art. 6(1)(f) | Évaluation intérêt légitime |
| `violation-donnees.md` | Art. 33-34 | Procédure notification 72h |
| `droits-personnes.md` | Art. 12-22 | Procédure droits des personnes |
| `politique-confidentialite.md` | Art. 13-14 | Mentions légales |
| `clause-{sous-traitant}.md` | Art. 28 | Clauses contractuelles |

Tous les fichiers sont générés dans `.osk/comply/rgpd/documents/`.

## AIPD - Quand est-ce obligatoire ?

**Liste obligatoire CNIL** (14 types de traitements) :
- Données de santé à grande échelle
- Surveillance systématique des employés
- Profilage RH
- Identification biométrique
- Données de personnes vulnérables
- Géolocalisation à grande échelle
- Croisement de bases de données
- Technologie innovante

**Critères CEPD** - AIPD si 2+ critères :
- Évaluation/notation
- Décision automatique avec effet juridique
- Surveillance systématique
- Données sensibles
- Grande échelle
- Croisement de données
- Personnes vulnérables
- Technologie innovante
- Exclusion d'un droit

## Base de Connaissances

| Fichier | Contenu |
|---------|---------|
| `knowledge/core/aipd-modeles.md` | Méthodologie PIA CNIL |
| `knowledge/core/aipd-liste-obligatoire.md` | Liste CNIL des AIPD obligatoires |
| `knowledge/core/guide-securite.md` | 25 fiches sécurité CNIL |
| `knowledge/core/guide-sous-traitant.md` | Obligations sous-traitants |
| `knowledge/core/violations-donnees.md` | Notification des violations |
| `knowledge/reference/rgpd-complet.md` | Texte complet RGPD |
| `knowledge/reference/sccs-2021.md` | Clauses Contractuelles Types |

## Références

- [CNIL - Guide RGPD](https://www.cnil.fr/fr/rgpd-de-quoi-parle-t-on)
- [CNIL - Méthodologie PIA](https://www.cnil.fr/fr/ce-quil-faut-savoir-sur-lanalyse-dimpact-relative-la-protection-des-donnees-dpia)
- [CNIL - 25 fiches sécurité](https://www.cnil.fr/fr/la-securite-des-donnees-personnelles)
- [EUR-Lex - Règlement 2016/679](https://eur-lex.europa.eu/legal-content/FR/TXT/?uri=CELEX%3A32016R0679)
