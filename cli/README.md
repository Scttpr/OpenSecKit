# 🛡️ OpenSecKit CLI (`osk`)

`osk` est l'outil en ligne de commande (CLI) du framework **OpenSecKit**. Il automatise la mise en place d'une démarche de sécurité continue ("Security as code") en préparant votre environnement pour des agents IA autonomes (comme Claude Code).

Il permet aux développeurs de documenter et de sécuriser leur architecture sans quitter leur terminal.

## ✨ Fonctionnalités Clés

* **🚀 Initialisation "Zero-Config" :** Télécharge automatiquement la dernière version stable des standards de sécurité (`constitution.md`, templates) et des agents IA depuis le dépôt central, directement dans votre projet.
* **🚜 Ingestion Locale Sécurisée :** Transforme votre base de code en un contexte optimisé pour les LLMs. Contrairement aux outils en ligne, `osk ingest` s'exécute en local et respecte strictement votre `.gitignore` (aucun secret ni `node_modules` ne fuite).
* **🤖 "Agent-Ready" :** Configure automatiquement l'environnement pour [Claude Code](https://docs.anthropic.com/en/docs/agents-and-tools/claude-code/overview), créant des commandes slash personnalisées (à venir).
* **🔄 Mises à jour OTA :** Les templates de sécurité évoluent ? Un simple `osk init` met à jour vos standards locaux sans avoir à recompiler l'outil.

## 📦 Installation

### Prérequis
Vous devez avoir installé et authentifié la CLI Claude pour lancer les agents.

### Option 1 : Binaires (Recommandé)

Téléchargez la dernière version pour Windows, macOS ou Linux depuis la page Releases.

### Option 2 : Compilation depuis les sources

Nécessite Rust & Cargo.
```bash
git clone https://github.com/Scttpr/OpenSecKit.git
cd OpenSecKit/cli
cargo install --path .
```

## 🛠️ Guide d'Utilisation
### 1. Initialiser le Kit

Placez-vous à la racine de votre projet de développement.
```bash
osk init
```

Télécharge la Constitution et les Templates dans un dossier .osk/ (à versionner).

Installe les Prompts Agents dans .claude/commands/ (config locale).

Configure le .gitignore pour éviter de polluer le dépôt.

### 2. Préparer le Contexte (Ingest)

Avant de lancer un audit, l'IA a besoin de "lire" votre code. ingest crée un résumé consolidé.

```bash
osk ingest --output context.txt --stats
```

Génère un fichier unique contenant l'arborescence et le contenu des fichiers pertinents.

Ignore automatiquement les binaires, les images et les fichiers exclus par .gitignore.

## 📂 Structure Générée

Après initialisation, votre projet ressemblera à ceci :

```
MonProjet/
├── .osk/                       # [A COMMITTER]
│   ├── constitution.md         # Les règles d'or
│   ├── templates/              # Modèles
│   └── domaines/               # Extensions métier
│
├── .claude/                    # [NON VERSIONNÉ] Cerveau local de l'outil
│   └── commands/
│       └── osk-command.md      # Commande osk pour Claude code
│
├── src/                        # Votre code source existant
├── context.txt                 # Généré par 'osk ingest' (temporaire)
└── osk (exécutable)
```

## 🛡️ Confidentialité & Sécurité

Pourquoi utiliser osk ingest plutôt qu'un service en ligne ?
- Data Privacy : Votre code source ne quitte jamais votre machine lors de l'étape d'ingestion. Il est envoyé directement de votre terminal à l'API de Claude (Anthropic), sans intermédiaire.
- Secret Safety : L'outil utilise la même logique que git pour ignorer les fichiers. Si vous avez exclu .env ou credentials.json dans votre .gitignore, osk ne les lira pas.