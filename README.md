# OpenSecKit

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Constitution](https://img.shields.io/badge/Constitution-v2.0.0-blue.svg)](constitution.md)
[![Templates](https://img.shields.io/badge/Templates-40-brightgreen.svg)](templates/)
[![Claude Code](https://img.shields.io/badge/Claude_Code-Optimal-orange.svg)](cli/)

**Security as Code, AI-Ready.** Framework de sécurité logicielle conçu pour être utilisé par des agents IA et des humains. Basé sur 7 principes constitutionnels et des templates actionnables.

> **Note :** OpenSecKit est optimisé pour [Claude Code](https://claude.com/claude-code). Les autres implémentations (Gemini, usage manuel) sont en **beta**.

---

## Pourquoi OpenSecKit ?

**Sécurisez votre code avec des agents IA** : Utilisez Claude Code ou Gemini pour auditer, spécifier et corriger votre code en suivant les standards de sécurité de l'industrie.

**Ou utilisez-le manuellement** : 40 templates Markdown prêts à l'emploi, organisés selon 7 principes constitutionnels (SSDLC).

**CLI incluse** : `osk` détecte votre stack, ingère votre code localement et vous donne accès à des commandes expertes.

**Support LLM** :

- **Claude Code** (recommandé, support complet)
- **Gemini** (beta)
- **Usage manuel** (beta)

Agnostique du langage de programmation.

---

## Ce que vous trouverez dans ce repo

### 📜 La Constitution (7 principes fondamentaux)

👉 **[constitution.md](constitution.md)** - Les 7 principes non négociables pour développer des logiciels sécurisés :

1. Modélisation des menaces
2. Analyse de risques continue
3. Sécurité dès la conception
4. Tests de sécurité
5. Gestion rigoureuse des secrets
6. Traçabilité et auditabilité
7. Patch management proactif

### 🗂️ Templates organisés par principe

| Principe | Dossier | Templates |
|----------|---------|-----------|
| **I. Modélisation des menaces** | [templates/01-threat-modeling/](templates/01-threat-modeling/) | STRIDE, attack trees, data flow diagrams |
| **II. Analyse des risques** | [templates/02-risk-analysis/](templates/02-risk-analysis/) | Scoring, registre des risques, matrices |
| **III. Sécurité dès la conception** | [templates/03-security-requirements/](templates/03-security-requirements/) | OWASP ASVS, auth, chiffrement |
| **IV. Tests de sécurité** | [templates/04-security-testing/](templates/04-security-testing/) | SAST, DAST, SCA, tests de régression |
| **V. Gestion des secrets** | [templates/05-secrets-management/](templates/05-secrets-management/) | Vault, rotation, détection |
| **VI. Journalisation d'audit** | [templates/06-audit-logging/](templates/06-audit-logging/) | Logs structurés, SIEM, alertes |
| **VII. Patch management** | [templates/07-patch-management/](templates/07-patch-management/) | SLA correctifs, scan dépendances |

**Chaque dossier contient** :

- `README.md` - Quand et comment utiliser ce principe
- Templates spécifiques (Markdown + frontmatter YAML)
- Exemples réels préfixés `_example-`

### 🌍 3 Domaines de conformité sectoriels

Pour répondre aux exigences réglementaires spécifiques :

- **[domaines/rgpd/](domaines/rgpd/)** - Conformité RGPD (protection des données personnelles UE) : DPIA, breach notification, DPO
- **[domaines/nis2/](domaines/nis2/)** - Directive NIS2 (infrastructures critiques UE) : gestion des risques, reporting d'incidents
- **[domaines/gouvernement-rgs/](domaines/gouvernement-rgs/)** - RGS (gouvernement français) : homologation, EBIOS, cryptographie ANSSI

### 🤖 Prompts pour agents IA

Workflows pré-conçus pour agents LLM (dans `prompts/`) :

- **`osk-audit.md`** - Audit de sécurité complet aligné sur la constitution
- **`osk-specify.md`** - Génération de specs de sécurité pour User Stories
- **`osk-context.md`** - Extraction de l'ADN technique du projet
- **`osk-domain.md`** - Assistant de conformité sectorielle (RGPD, NIS2, RGS)
- **`osk-assess.md`** - Analyse de risques et maintenance documentaire (Threat Models, AIPD)
- **`osk-incident.md`** - Assistant de gestion de crise et réponse à incident

### ⚙️ CLI (`osk`)

👉 **[cli/README.md](cli/README.md)** - Outil en ligne de commande pour automatiser les audits et specs :

- Détection automatique de la stack technique
- Sélection intelligente des fichiers pertinents (économise tokens)
- Commandes : `init`, `audit`, `spec`, `domaine`, `context`, `ingest`
- Support Claude et Gemini
- Intégration optionnelle avec Claude Code (slash commands)

---

## Trois façons d'utiliser OpenSecKit

### 1. Avec Claude Code (recommandé - support complet)

**Intégration optimale**

```bash
# Installer la CLI OpenSecKit
git clone https://github.com/Scttpr/OpenSecKit
cd OpenSecKit/cli
cargo install --path .

# Initialiser (choisir Claude et activer l'intégration)
cd mon-projet/
osk init

# Lancer Claude Code
claude

# Dans l'interface Claude Code
/osk-audit                    # Audit de sécurité
/osk-spec "description"       # Specs de sécurité
/osk-domain rgpd              # Conformité RGPD
```

**Avantage clé** : Claude Code peut modifier vos fichiers directement. Support complet et testé.

### 2. Avec Gemini (beta)

```bash
# Initialiser (choisir Gemini)
osk init

# Exécuter des commandes
export GEMINI_API_KEY="votre-clé"
osk audit
osk spec "En tant qu'admin, je veux bannir un utilisateur"
osk domaine rgpd
```

**Note :** Support en beta. Pas de modification de fichiers automatique.

### 3. Usage manuel (beta)

```bash
# Export du contexte pour usage offline
osk ingest

# Génération de la mémoire projet
osk context

# Copier-coller le contexte dans ChatGPT, Claude web, etc.
```

**Avantage clé** : Fonctionne avec n'importe quel LLM (ChatGPT, etc.) via copier-coller.

### 4. Sans CLI (templates uniquement)

```bash
# 1. Lire la constitution
cat constitution.md

# 2. Choisir un principe (ex: threat modeling)
cd templates/01-threat-modeling/

# 3. Consulter l'exemple
cat _example-ecommerce-stride.md

# 4. Copier le template
cp stride-threat-model-template-planning.md ~/mon-projet/docs/threat-model.md

# 5. Remplir selon votre contexte
vim ~/mon-projet/docs/threat-model.md
```

**Avantage clé** : Contrôle total, pas de dépendance à des outils externes, fonctionne offline.

---

## Structure du repo

```
OpenSecKit/
├── constitution.md           # Les 7 principes fondamentaux
├── QUICK-START.md           # Démarrage en 10 minutes
├── FAQ.md                   # Questions fréquentes
│
├── templates/               # Templates organisés par principe (27 templates)
│   ├── 01-threat-modeling/  # Modélisation des menaces (STRIDE, etc.)
│   ├── 02-risk-analysis/    # Analyse de risques (scoring, matrices)
│   ├── 03-security-requirements/  # Exigences de sécurité (OWASP ASVS)
│   ├── 04-security-testing/ # Tests de sécurité (SAST, DAST, SCA)
│   ├── 05-secrets-management/  # Gestion des secrets (Vault, rotation)
│   ├── 06-audit-logging/    # Journalisation (logs structurés, SIEM)
│   └── 07-patch-management/ # Patch management (SLA, dépendances)
│
├── domaines/                # Extensions sectorielles (13 templates)
│   ├── rgpd/                # Conformité RGPD (UE)
│   ├── nis2/                # Directive NIS2 (infrastructures critiques)
│   └── gouvernement-rgs/    # RGS (gouvernement français)
│
├── prompts/                 # Prompts pour agents IA
│   ├── osk-audit.md         # Audit de sécurité complet
│   ├── osk-specify.md       # Génération de specs
│   ├── osk-context.md       # Extraction d'ADN technique
│   └── osk-domain.md        # Conformité sectorielle
│
└── cli/                     # CLI OpenSecKit
    └── README.md            # Documentation de la CLI
```

---

## Démarrage rapide

**Je veux un audit de sécurité maintenant**

```bash
# Installer et configurer
git clone https://github.com/Scttpr/OpenSecKit
cd OpenSecKit/cli
cargo install --path .

cd mon-projet/
export CLAUDE_API_KEY="votre-clé"  # ou GEMINI_API_KEY
osk init
osk audit
```

**Je veux comprendre les principes d'abord**

👉 **[QUICK-START.md](QUICK-START.md)** - Guide en 10 minutes avec exemple e-commerce

👉 **[constitution.md](constitution.md)** - Les 7 principes expliqués en détail

**Je veux explorer les templates**

```bash
# Voir tous les templates disponibles
ls -R templates/

# Exemple : modélisation des menaces
cat templates/01-threat-modeling/_example-ecommerce-stride.md

# Exemple : gestion des secrets
cat templates/05-secrets-management/_example-ecommerce-secrets.md
```

---

## Documentation complète

| Document | Contenu |
|----------|---------|
| [QUICK-START.md](QUICK-START.md) | Guide pratique en 10 minutes avec exemple e-commerce |
| [constitution.md](constitution.md) | Les 7 principes constitutionnels expliqués en détail |
| [FAQ.md](FAQ.md) | Questions fréquentes (agents IA, templates, outils, conformité) |
| [cli/README.md](cli/README.md) | Documentation de la CLI `osk` |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Comment contribuer au projet |

---

## Contribuer

Vous avez créé un template utile ? Vous voulez ajouter un domaine de conformité ? Partagez-le !

1. Forkez le référentiel
2. Créez votre template dans `templates/{principe}/` ou `domaines/{secteur}/`
3. Ajoutez un exemple `_example-*.md`
4. Soumettez une Pull Request

👉 **[CONTRIBUTING.md](CONTRIBUTING.md)** pour les détails

---

## Licence

[MIT License](LICENSE) - Librement utilisable, modifiable et partageable.

---

## Support

- **Discussions** : [GitHub Discussions](https://github.com/Scttpr/OpenSecKit/issues)
- **Bugs** : [GitHub Issues](https://github.com/Scttpr/OpenSecKit/issues)
