# OpenSecKit

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Constitution](https://img.shields.io/badge/Constitution-v1.0.0-blue.svg)](constitution.md)
[![Templates](https://img.shields.io/badge/Templates-25+-brightgreen.svg)](templates/)

**Sécurisez vos projets en suivant 7 principes constitutionnels**. OpenSecKit fournit des templates prêts à l'emploi pour implémenter un cycle de développement logiciel sécurisé (SSDLC).

---

## 🚀 Démarrage

### Nouveau sur OpenSecKit ?

👉 **[QUICK-START.md](QUICK-START.md)** - Démarrez en 10 minutes avec un exemple concret

### Vous voulez comprendre les principes ?

👉 **[constitution.md](constitution.md)** - Les 7 principes de sécurité expliqués

### Vous avez des questions ?

👉 **[FAQ.md](FAQ.md)** - Réponses aux questions les plus fréquentes

---

## 📚 Templates par principe

Les templates sont organisés selon les 7 principes constitutionnels :

| Principe | Dossier | Description |
|----------|---------|-------------|
| **I. Modélisation des menaces** | [templates/01-threat-modeling/](templates/01-threat-modeling/) | STRIDE, attack trees, data flow diagrams |
| **II. Analyse des risques** | [templates/02-risk-analysis/](templates/02-risk-analysis/) | Scoring, registre des risques, matrices |
| **III. Sécurité dès la conception** | [templates/03-security-requirements/](templates/03-security-requirements/) | OWASP ASVS, authentification, chiffrement |
| **IV. Tests de sécurité continus** | [templates/04-security-testing/](templates/04-security-testing/) | SAST, DAST, SCA, tests de régression |
| **V. Gestion des secrets** | [templates/05-secrets-management/](templates/05-secrets-management/) | Vault, rotation, détection de secrets |
| **VI. Journalisation d'audit** | [templates/06-audit-logging/](templates/06-audit-logging/) | Logs structurés, SIEM, alertes |
| **VII. Patch management** | [templates/07-patch-management/](templates/07-patch-management/) | SLA correctifs, scan de dépendances |

**Chaque dossier contient** :
- `README.md` - Quand et comment utiliser ce principe
- Templates spécifiques avec frontmatter YAML
- Exemples préfixés `_example-`

---

## 🌍 Extensions sectorielles

Pour des exigences spécifiques à votre secteur :

- **[domaines/rgpd/](domaines/rgpd/)** - Conformité RGPD (protection des données personnelles UE)
- **[domaines/nis2/](domaines/nis2/)** - Directive NIS2 (infrastructures critiques UE)
- **[domaines/gouvernement-rgs/](domaines/gouvernement-rgs/)** - Référentiel Général de Sécurité (gouvernement français)

---

## 🎯 Workflow typique

```bash
# 1. Lire le guide de démarrage rapide
cat QUICK-START.md

# 2. Comprendre les principes
cat constitution.md

# 3. Choisir un principe (ex: modélisation des menaces)
cd templates/01-threat-modeling/

# 4. Lire le README du principe
cat README.md

# 5. Consulter l'exemple
cat _example-ecommerce-stride.md

# 6. Copier le template dans votre projet
cp stride-threat-model-template-planning.md ~/mon-projet/docs/threat-model.md

# 7. Adapter à votre contexte
# Remplir les sections selon votre application
```

---

## 📖 Documentation complète

| Document | Contenu |
|----------|---------|
| [QUICK-START.md](QUICK-START.md) | Guide pratique en 10 minutes avec exemple e-commerce |
| [constitution.md](constitution.md) | Les 7 principes constitutionnels expliqués |
| [FAQ.md](FAQ.md) | Questions fréquentes (démarrage, templates, validation, outils) |
| [CHANGELOG.md](CHANGELOG.md) | Historique des modifications |
| [.github/CONTRIBUTING.md](.github/CONTRIBUTING.md) | Comment contribuer au projet |
| [.github/SECURITY.md](.github/SECURITY.md) | Politique de signalement des vulnérabilités |

---

## 🤝 Contribuer

Vous avez créé un template utile ? Partagez-le !

1. Forkez le référentiel
2. Créez votre template dans `templates/{principe}/`
3. Ajoutez un exemple `_example-*.md`
4. Soumettez une Pull Request

👉 **[.github/CONTRIBUTING.md](.github/CONTRIBUTING.md)** pour les détails

---

## 📜 Licence

[MIT License](LICENSE) - Librement utilisable, modifiable et partageable.

---

## 📞 Support

- **Discussions** : [GitHub Discussions](https://github.com/YourOrg/OpenSecKit/discussions)
- **Bugs** : [GitHub Issues](https://github.com/YourOrg/OpenSecKit/issues)
- **Sécurité** : [.github/SECURITY.md](.github/SECURITY.md)
