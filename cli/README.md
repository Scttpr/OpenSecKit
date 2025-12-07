# CLI OpenSecKit (`osk`)

Outil en ligne de commande pour automatiser les audits de sécurité et
la génération de spécifications avec Claude ou Gemini.

> **Recommandation :** Pour une expérience optimale, utilisez avec [Claude Code](https://claude.com/claude-code).
> Les autres providers (Gemini, usage manuel) sont en **beta**.

## Installation

### Prérequis

- **Rust et Cargo** installés
- Une clé API Claude (`CLAUDE_API_KEY`) ou Gemini (`GEMINI_API_KEY`)

### Depuis les sources

```bash
git clone https://github.com/Scttpr/OpenSecKit
cd OpenSecKit/cli
cargo install --path .
```

Vérifiez l'installation :

```bash
osk --version
```

---

## Démarrage rapide

### 1. Configuration initiale

```bash
cd mon-projet/
osk init
```

La commande `init` va :

1. Vous demander quel LLM utiliser (Claude ou Gemini)
2. Détecter votre stack technique automatiquement
3. Télécharger les templates et prompts depuis GitHub
4. Créer `.osk/config.toml` avec votre configuration
5. Optionnellement générer les slash commands pour Claude Code

**Variables d'environnement requises :**

```bash
# Pour Claude
export CLAUDE_API_KEY="votre-clé"

# Pour Gemini
export GEMINI_API_KEY="votre-clé"
```

### 2. Lancer un audit de sécurité

```bash
osk audit
```

Cette commande :

- Sélectionne intelligemment les fichiers pertinents
- Les envoie avec le prompt d'audit à l'API
- Affiche un rapport de conformité aux 7 principes
- Sauvegarde le résultat dans `.osk/memory/`

### 3. Générer des specs de sécurité

```bash
osk spec "En tant qu'admin, je veux bannir un utilisateur"
```

Génère les spécifications de sécurité pour cette user story.

---

## Commandes disponibles

### `osk init`

Initialise OpenSecKit dans votre projet.

```bash
osk init          # Configuration interactive
osk init --force  # Écrase les fichiers existants
```

**Ce qui est créé :**

```plaintext
.osk/
├── config.toml       # Configuration (provider, modèle, stack)
├── registry.toml     # Mapping des commandes vers les prompts
├── constitution.md   # Les 7 principes
├── templates/        # Templates de sécurité
├── domaines/         # Templates RGPD, NIS2, RGS
├── prompts/          # Cache des prompts téléchargés
└── memory/           # Historique des exécutions

docs/
└── context/
    └── meta.md       # Mémoire du projet (généré automatiquement)

.claude/              # Si intégration Claude Code activée
└── commands/         # Slash commands (.md)
```

**Détection de stack :**

La CLI détecte automatiquement :

- Langages (via fichiers de config : `package.json`, `Cargo.toml`,
`pyproject.toml`, etc.)
- Frameworks (React, Next.js, Express, Django, etc.)
- Outils (Docker, Kubernetes, etc.)

Vous pouvez modifier la stack détectée lors de l'initialisation.

### `osk ingest`

Génère un fichier `context.txt` contenant l'arborescence et le contenu de
votre code.

```bash
osk ingest                      # Crée context.txt
osk ingest -o audit-context.txt # Sortie personnalisée
osk ingest -p ./src             # Cible un dossier spécifique
```

**Filtrage automatique :**

- Respecte `.gitignore`
- Exclut les binaires (images, PDF, `.exe`, `.so`, etc.)
- Exclut `.git/` et `.osk/`
- Détecte les binaires par extension et inspection du contenu

**Utilisation :** Utile pour copier-coller manuellement dans ChatGPT ou
pour usage offline.

### `osk audit`

Lance un audit de sécurité complet basé sur les 7 principes constitutionnels.

```bash
osk audit                    # Utilise le provider configuré
osk audit --provider gemini  # Force l'usage de Gemini
```

**Ce qui se passe :**

1. **Sélection intelligente** : L'outil demande à l'LLM de lister les fichiers
pertinents (max 25)
2. **Lecture locale** : Seuls ces fichiers sont lus localement
3. **Envoi à l'API** : Le contenu est envoyé avec le prompt d'audit
4. **Génération du rapport** : L'LLM produit un audit structuré
5. **Sauvegarde** : Le résultat est affiché et sauvegardé dans `.osk/memory/`

**Exemple de rapport généré :**

```markdown
# Audit de sécurité

## Score de conformité : 65/100

## Top 3 Risques Critiques

1. **Injection SQL** (Score 20/25) - `src/db/users.js:42`
2. **Secrets exposés** (Score 18/25) - Historique git
3. **Pas d'auth sur /admin** (Score 15/25) - `routes/admin.js`

## Analyse par principe

| Principe | Statut | Preuves | Recommandations |
|----------|--------|---------|-----------------|
| I. Menaces | 🔴 | Pas de threat-model.md | Créer modélisation |

```bash
`osk spec <user_story>`
```

Génère les spécifications de sécurité pour une fonctionnalité.

```bash
osk spec "Feature d'export CSV des données utilisateur"
```

**Sortie attendue :**

- Analyse des menaces STRIDE
- Exigences d'authentification/autorisation
- Schémas de validation requis
- Logs de sécurité à implémenter
- Tests de sécurité à écrire

### `osk domaine <nom>`

Analyse de conformité pour un domaine réglementaire.

```bash
osk domaine rgpd  # Conformité RGPD
osk domaine nis2  # Directive NIS2
osk domaine rgs   # RGS (gouvernement français)
```

Remplit automatiquement les templates de conformité du domaine avec
des preuves tirées de votre code.

### `osk context`

Génère ou met à jour `docs/context/meta.md` (mémoire du projet).

```bash
osk context
```

Ce fichier contient :

- Stack technique détaillée
- Principes de sécurité détectés dans le code
- Conventions de code observées
- Bibliothèques utilisées pour l'auth, la validation, etc.

**Usage :** Ce fichier est automatiquement injecté dans les prompts
pour personnaliser les analyses.

---

## Intégration avec Claude Code (recommandé)

**L'expérience optimale pour OpenSecKit.**

Si vous avez activé l'intégration lors de `osk init`, les prompts
sont copiés dans `.claude/commands/`.

**Utilisation :**

```bash
# Lancer Claude Code
claude

# Dans l'interface Claude Code
/osk-audit              # Lance un audit
/osk-spec "description" # Génère des specs
/osk-domain rgpd        # Conformité RGPD
/osk-context            # Met à jour meta.md
```

**Avantages vs autres providers :**

- ✅ **Modification automatique** : Claude Code peut modifier vos
fichiers directement
- ✅ **Support complet** : Toutes les fonctionnalités testées et optimisées
- ✅ **Workflow fluide** : Slash commands intégrées dans l'interface
- ✅ **Itération rapide** : Audit → Correction → Vérification dans la même
session

**Autres providers (Gemini, CLI standalone) :** Support en beta, modification
manuelle requise.

---

## Configuration (`.osk/config.toml`)

Fichier généré par `osk init` :

```toml
[agent]
provider = "claude"  # ou "gemini"
model = "claude-sonnet-4.5"
temperature = 0.2

[project]
name = "MonProjet"
stack = "Node.js, React, PostgreSQL, Docker"

[memory]
enabled = true
path = ".osk/memory"
```

**Modifier la configuration :**

```bash
vim .osk/config.toml
```

**Changer de provider :**

```bash
# Éditer config.toml
provider = "gemini"
model = "gemini-2.0-flash-exp"

# Ou utiliser --provider à chaque commande
osk audit --provider gemini
```

---

## Sélection intelligente de fichiers

Au lieu d'envoyer tout votre code à l'API, `osk` utilise un système de
sélection en deux passes :

**1. Première passe (sélection) :**

- Génère l'arborescence complète du projet
- Demande à l'LLM de lister les 25 fichiers les plus pertinents
- Ne coûte que quelques centimes (petite requête)

**2. Deuxième passe (analyse) :**

- Lit localement uniquement les fichiers sélectionnés
- Envoie le contenu avec le prompt d'analyse
- Économise tokens et coûts

**Fallback :** Si la sélection échoue, utilise `context.txt` s'il existe.

---

## Gestion de la mémoire

Par défaut, `osk` sauvegarde l'historique des interactions dans `.osk/memory/`.

**Contenu :**

- Commande exécutée
- Réponse de l'LLM
- Timestamp

**Désactiver la mémoire :**

```toml
[memory]
enabled = false
```

**Note :** Cette mémoire n'est PAS partagée avec Claude Code (qui a sa
propre session).

---

## Confidentialité et sécurité

**Ce qui ne quitte jamais votre machine :**

- Fichiers listés dans `.gitignore`
- Secrets (`.env`, `credentials.json`, etc.)
- Binaires
- Dossiers `.git/` et `.osk/`

**Ce qui est envoyé à l'API :**

- Uniquement les fichiers sélectionnés par la première passe
- Le prompt de commande
- Votre stack technique (configurée dans `config.toml`)

**Inspection :** Utilisez `osk ingest` pour voir exactement ce qui
serait envoyé.

---

## Dépannage

### `Config introuvable`

```bash
# Vous n'avez pas initialisé le projet
osk init
```

### `Variable CLAUDE_API_KEY manquante`

```bash
# Exporter la clé API
export CLAUDE_API_KEY="votre-clé"

# Ou la mettre dans .env
echo 'CLAUDE_API_KEY=votre-clé' >> .env
```

### `Sélection auto échouée. Fallback.`

La première passe a échoué (quota API, réseau). La CLI utilise
`context.txt` si disponible.

**Solution :** Générez `context.txt` d'abord :

```bash
osk ingest
osk audit
```

### Templates pas à jour

```bash
# Forcer le téléchargement
osk init --force
```

---

## Développement

### Compiler depuis les sources

```bash
cd cli/
cargo build --release
./target/release/osk --version
```

### Tests

```bash
cargo test
```

---

## Limitations

- **Taille de code :** Même avec sélection intelligente, les très gros projets
(>10k fichiers) peuvent poser problème
- **Langages supportés pour détection :** Rust, Node.js, Python, Go, Java,
PHP, Ruby (extensible)
- **Providers LLM :**
  - **Claude Code** : Support complet (recommandé)
  - **Gemini** : Beta (pas de modification automatique de fichiers)
  - **Autres** : Via export manuel (`osk ingest`)

---

## Comparaison des approches

| Critère | Claude Code | CLI (Gemini) | Manuel (templates) |
|---------|-------------|--------------|-------------------|
| Support | ✅ Complet | ⚠️ Beta | ✅ Stable |
| Vitesse | 5 min | 5 min | 30-60 min |
| Sélection fichiers | Automatique | Automatique | Manuelle |
| Modification fichiers | ✅ Automatique | ❌ Manuelle | ✅ Manuelle |
| Personnalisation | Via `meta.md` | Via `meta.md` | Totale |
| Offline | Cache local | Cache local | 100% |
| Coût | API calls | API calls | Gratuit |

---

## Support

- **Issues :** [GitHub Issues](https://github.com/Scttpr/OpenSecKit/issues)
- **Discussions :**
[GitHub Discussions](https://github.com/Scttpr/OpenSecKit/issues)
