# Guide de contribution à OpenSecKit

Merci de votre intérêt pour contribuer à OpenSecKit ! Ce guide vous aidera à démarrer.

---

## 🎯 Comment contribuer

### Types de contributions

Nous acceptons plusieurs types de contributions :

1. **Nouveaux templates** : Ajout de templates pour les principes existants
2. **Exemples** : Nouveaux cas d'usage concrets (fintech, healthcare, etc.)
3. **Domaines sectoriels** : Extensions pour compliance (PCI-DSS, HIPAA, etc.)
4. **Corrections** : Bugs, typos, liens cassés
5. **Améliorations** : Documentation, clarifications, meilleures pratiques
6. **Traductions** : Internationalisation (EN, ES, etc.)

---

## 🚀 Démarrage rapide

### 1. Fork et clone

```bash
# Fork le repository sur GitHub
# Puis clone votre fork
git clone https://github.com/VOTRE-USERNAME/OpenSecKit.git
cd OpenSecKit

# Ajouter le remote upstream
git remote add upstream https://github.com/ORIGINAL-ORG/OpenSecKit.git
```

### 2. Créer une branche

```bash
git checkout -b feature/nom-de-votre-feature
```

### 3. Faire vos modifications

Suivez les conventions décrites ci-dessous.

### 4. Tester localement

```bash
# Vérifier les liens
./scripts/check-links.sh

# Vérifier le frontmatter YAML
./scripts/validate-frontmatter.sh
```

### 5. Commit et push

```bash
git add .
git commit -m "feat: description de votre contribution"
git push origin feature/nom-de-votre-feature
```

### 6. Créer une Pull Request

Ouvrez une PR sur GitHub avec :
- Titre clair et descriptif
- Description détaillée des changements
- Référence aux issues liées (si applicable)

---

## 📝 Conventions

### Structure des fichiers

#### Templates

Les templates doivent suivre cette structure :

```
templates/XX-nom-principe/
├── README.md                    # Vue d'ensemble du principe
├── nom-template.md              # Template générique
├── nom-guide.md                 # Guide d'implémentation
└── _example-projet-nom.md       # Exemple concret
```

#### Nommage des fichiers

- **Minuscules et tirets** : `threat-model-template.md`
- **Suffixes explicites** :
  - `-template.md` : Document à remplir
  - `-guide.md` : Instructions pas à pas
  - `-policy.md` : Politique/procédure
- **Préfixe exemples** : `_example-{projet}-{sujet}.md`

#### Frontmatter YAML

Tous les fichiers markdown doivent avoir un frontmatter YAML :

```yaml
---
title: "Titre du document"
constitutional_principle: "I - Threat modeling"
ssdlc_phase: "planning"  # planning | design | implementation | all
tags: ["tag1", "tag2"]
---
```

**Champs optionnels** :
```yaml
template_version: "1.0.0"
difficulty: "beginner"        # beginner | intermediate | advanced
estimated_time: "30 minutes"
project: "ShopSecure"         # Pour les exemples
compliance: ["RGPD", "NIS2"]  # Pour les domaines
```

### Style d'écriture

- **Ton** : Professionnel mais accessible
- **Langue principale** : Français (traductions EN/ES bienvenues)
- **Capitalisation** : Seulement en début de phrase ou noms propres
- **Exemples** : Toujours concrets et fonctionnels
- **Code** : Commenté et testé

### Commits

Nous suivons [Conventional Commits](https://www.conventionalcommits.org/) :

- `feat:` Nouvelle fonctionnalité
- `fix:` Correction de bug
- `docs:` Documentation uniquement
- `style:` Formatage (pas de changement de code)
- `refactor:` Refactoring
- `test:` Ajout de tests
- `chore:` Tâches de maintenance

**Exemples** :
```bash
git commit -m "feat: add DAST integration guide for ZAP"
git commit -m "fix: correct broken links in risk-analysis README"
git commit -m "docs: improve quick-start guide clarity"
```

---

## 🎨 Créer un nouveau template

### Checklist

- [ ] Lire [ARCHITECTURE.md](ARCHITECTURE.md) pour comprendre la structure
- [ ] Identifier le principe constitutionnel (I-VII)
- [ ] Créer le fichier avec le bon nommage
- [ ] Ajouter le frontmatter YAML complet
- [ ] Inclure :
  - [ ] Objectif clair
  - [ ] Instructions pas à pas
  - [ ] Checklist de validation
  - [ ] Liens vers ressources externes
- [ ] Créer un exemple concret (`_example-*.md`)
- [ ] Mettre à jour le README du principe
- [ ] Tester les liens : `./scripts/check-links.sh`
- [ ] Valider le YAML : `./scripts/validate-frontmatter.sh`

### Structure type d'un template

```markdown
---
title: "Template : Nom du template"
constitutional_principle: "X - Nom du principe"
ssdlc_phase: "planning"
tags: ["template", "principe-x"]
---

# Nom du template

## Objectif

[Pourquoi ce template existe, quel problème il résout]

## Prérequis

- Prérequis 1
- Prérequis 2

## Instructions

### Étape 1 : [Nom de l'étape]

[Instructions détaillées]

### Étape 2 : [Nom de l'étape]

[Instructions détaillées]

## Checklist de validation

- [ ] Critère 1
- [ ] Critère 2

## Validation constitutionnelle

**Conformité au Principe X** :
- [ ] Critère spécifique au principe

## Ressources

- [Ressource externe 1](URL)
- [Ressource externe 2](URL)
```

---

## 🌍 Créer un exemple concret

Les exemples doivent être :
- **Réalistes** : Basés sur des cas d'usage réels
- **Complets** : Code fonctionnel, pas de pseudo-code
- **Commentés** : Explications claires
- **Cohérents** : Suivre un projet fil rouge (ex: ShopSecure)

### Projets fil rouge existants

- **ShopSecure** (e-commerce) : Projet principal
- **FinSecure** (fintech) : Banque en ligne (V2)
- **HealthVault** (healthcare) : Dossier patient (V2)

### Structure type d'un exemple

```markdown
---
title: "Exemple : [Nom] pour [Projet]"
template_version: "1.0.0"
constitutional_principle: "X - Nom"
project: "ShopSecure"
ssdlc_phase: "implementation"
tags: ["example", "ecommerce", "sujet"]
---

# Exemple : [Nom] pour [Projet]

**Projet** : ShopSecure - Plateforme e-commerce B2C
**Contexte** : [Description du contexte]
**Stack** : [Technologies utilisées]

---

## 1. [Section 1]

[Code + Explications]

## 2. [Section 2]

[Code + Explications]

## Métriques de succès

| Métrique | Cible | Actuel | Statut |
|----------|-------|--------|--------|
| [Métrique 1] | [Valeur] | [Valeur] | ✅/⚠️/❌ |

---

**Prochaine étape** : [Lien vers le principe suivant]
```

---

## 🏢 Créer un domaine sectoriel

Pour ajouter un nouveau domaine de conformité (ex: PCI-DSS, HIPAA) :

### Structure

```
domaines/
└── nom-domaine/
    ├── README.md
    ├── templates/
    │   ├── template-1.md
    │   └── template-2.md
    └── exemples/
        └── exemple-1.md
```

### Contenu du README

Le README du domaine doit expliquer :
- Contexte réglementaire
- Qui est concerné
- Exigences principales
- Mapping avec les 7 principes constitutionnels
- Templates spécifiques au domaine

---

## 🧪 Tests et validation

### Avant de soumettre une PR

```bash
# 1. Vérifier les liens
./scripts/check-links.sh

# 2. Valider le frontmatter YAML
./scripts/validate-frontmatter.sh

# 3. Vérifier le formatage markdown (optionnel)
npx markdownlint-cli2 "**/*.md"
```

### CI/CD

Les GitHub Actions vérifieront automatiquement :
- Liens markdown
- Frontmatter YAML
- Formatage (bientôt)

---

## 🔍 Processus de revue

### Ce que nous vérifions

1. **Conformité** : Respect des conventions (nommage, frontmatter, structure)
2. **Qualité** : Clarté, exactitude, complétude
3. **Liens** : Tous les liens fonctionnent
4. **Exemples** : Code testé et fonctionnel
5. **Documentation** : Instructions claires et suffisantes

### Temps de revue

- **Simple** (typo, lien cassé) : 1-2 jours
- **Moyen** (nouveau template) : 3-5 jours
- **Complex** (nouveau domaine) : 1-2 semaines

---

## 💡 Idées de contributions

### Priorité haute

- [ ] Traduction anglaise des templates principaux
- [ ] Exemples alternatifs (fintech, healthcare)
- [ ] Templates manquants dans domaines/ (NIS2, RGPD, RGS)

### Priorité moyenne

- [ ] Guides transversaux (incident response, security champions)
- [ ] Templates spécialisés (cloud, mobile, API)
- [ ] Amélioration des exemples existants

### Priorité basse

- [ ] Traduction espagnole
- [ ] Nouveaux domaines (PCI-DSS, HIPAA, SOC2)
- [ ] Intégrations avec outils externes

---

## 📞 Besoin d'aide ?

- **Questions** : [GitHub Discussions](https://github.com/ORG/OpenSecKit/discussions)
- **Bugs** : [GitHub Issues](https://github.com/ORG/OpenSecKit/issues)
- **Clarifications** : Ouvrir une issue avec le tag `question`

---

## 🙏 Reconnaissance

Tous les contributeurs sont listés dans [CONTRIBUTORS.md](CONTRIBUTORS.md).

Les contributions significatives peuvent être reconnues par :
- Mention dans CHANGELOG.md
- Badge "Contributor" GitHub
- Mention dans les releases

---

## 📜 Licence

En contribuant, vous acceptez que vos contributions soient sous [licence MIT](LICENSE).

---

**Merci de contribuer à rendre le développement sécurisé accessible à tous !** 🎉
