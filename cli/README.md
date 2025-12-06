# 🛡️ OpenSecKit CLI (`osk`)

> **Security as Code & AI-Ready Environment**

`osk` est l'outil en ligne de commande (CLI) du framework **OpenSecKit**. Il automatise la sécurisation de votre code et prépare votre environnement de développement pour les agents IA (comme Claude Code) ou les assistants LLM classiques, tout en garantissant la confidentialité des données.

## ✨ Pourquoi utiliser `osk` ?

* **🚀 Initialisation Intelligente :** Détecte automatiquement votre stack technique (Rust, Node, Python, Docker, K8s...) et configure le contexte pour des réponses ultra-pertinentes.
* **📜 Registre de Commandes Dynamique :** Plus besoin de prompter manuellement. Utilisez des commandes expertes (`audit`, `spec`, `fix`) téléchargées et mises à jour à la volée depuis le dépôt central.
* **🤖 Agnostique & Multi-Provider :** Support natif de **Claude** (Anthropic) et **Gemini** (Google). Changez de "cerveau" selon vos besoins ou la disponibilité des API.
* **🔒 Confidentialité Maximale :** L'ingestion du code se fait **localement**. Vos secrets, fichiers d'environnement et dossiers ignorés par git ne sont jamais lus ni envoyés.
* **⚡ Network-First Caching :** L'outil cherche toujours la dernière version des règles de sécurité en ligne, mais utilise un cache local robuste pour fonctionner hors-ligne.

---

## 📦 Installation

### Prérequis

* **Rust & Cargo** (pour la compilation)
* Une clé API valide pour le fournisseur choisi (Anthropic ou Google).

> **⚠️ Important : Distinction avec Claude Code**
> Même si vous utilisez l'agent CLI officiel `claude` (authentifié via sa propre procédure de login), vous **devez** définir la variable d'environnement `CLAUDE_API_KEY` pour utiliser les commandes `osk`.
> `osk` est un client indépendant qui effectue ses propres appels API sécurisés pour les audits et le mode chatbot.

### Depuis les sources

Clonez le dépôt Git officiel, placez-vous dans le dossier du CLI et lancez l'installation via Cargo. Une fois l'opération terminée, vérifiez l'installation en affichant la version de l'outil.

---

## 🛠️ Guide de Démarrage Rapide

### 1. Initialiser le projet (`init`)

Placez-vous à la racine de votre projet et lancez la commande d'initialisation.

L'outil va :

1. Télécharger le **registre des commandes**.
2. Détecter votre stack technique (ex: "Rust, Docker").
3. Créer le dossier `.osk/` contenant la configuration.
4. **Si activé :** Générer les fichiers de configuration pour l'agent `claude` dans `.claude/commands`.

### 2. Digérer le code (`ingest`)

Avant de poser une question, consolidez votre base de code dans un fichier de contexte optimisé via la commande d'ingestion.

* Crée un fichier `context.txt` (ignoré par git).
* Filtre automatiquement les fichiers binaires (images, PDF, exécutables) pour ne pas polluer le contexte.
* Affiche une estimation des tokens.

---

## 🚀 Utilisation avec Claude Code (Agent Officiel)

Si vous avez activé l'intégration Claude Code lors de l'initialisation, `osk` a généré des "Slash Commands" personnalisées pour l'agent officiel d'Anthropic.

1. **Prérequis :** Installez l'agent officiel (si ce n'est pas déjà fait) :
    `npm install -g @anthropic-ai/claude-code`
2. **Lancer l'agent :** Lancez simplement la commande `claude` dans votre terminal.
3. **Utiliser les commandes OSK :**
    À l'intérieur de l'interface Claude Code, tapez `/` pour voir les commandes disponibles. Vous verrez apparaître les commandes générées par `osk` :
    * `/audit` : Lance un audit de sécurité complet.
    * `/spec ma feature en langage naturel` : Génère des spécifications de sécurité.

> **Avantage Clé :** Contrairement à `osk run` (qui est un chatbot en lecture seule), l'agent **Claude Code** a la capacité de **modifier vos fichiers** directement pour appliquer les corrections de sécurité suggérées par les prompts `osk`.

---

## 🤖 Utilisation Standard (`osk run`)

Si vous n'utilisez pas l'agent Claude Code, ou si vous utilisez Gemini, utilisez la commande `osk run`. Elle possède deux modes :

#### A. Mode Commande (Expert)

Utilisez des workflows de sécurité pré-définis. Si vous devez fournir des données spécifiques (une User Story, un fichier), utilisez **impérativement** le flag d'input (`-i` ou `--input`).

Exemples d'utilisation :

* **Audit de sécurité :** `osk run audit menaces`
* **Audit spécifique :** `osk run audit design`
* **Spécifications :** `osk run spec -i "En tant qu'admin, je veux pouvoir bannir un utilisateur"`

#### B. Mode Libre (Chatbot)

Posez simplement une question à l'IA sur votre code en passant votre question comme argument direct :
`osk run "Pourquoi ma fonction de login est-elle lente ?"`

---

## ⚙️ Configuration

### Variables d'environnement

L'outil récupère les clés API nécessaires (Claude ou Gemini) directement depuis vos variables d'environnement système. Assurez-vous qu'elles sont définies dans votre shell.

### Fichier de Configuration

Un fichier de configuration TOML est généré dans le dossier `.osk`. Il permet de définir :

* L'agent (fournisseur, modèle, température).
* Le projet (nom, stack technique injectée dans le System Prompt).
* La mémoire (activation et chemin).

### Registre des Commandes

Un fichier registre mappe vos commandes courtes vers des URLs de prompts Markdown. Pour le mettre à jour vers la dernière version du dépôt, vous pouvez forcer une nouvelle initialisation.

---

## 📂 Structure du Projet

L'outil ajoute une structure spécifique à votre projet :

* **.osk/** : Dossier de configuration principal.
  * **config.toml** : Configuration locale.
  * **registry.toml** : Mapping des commandes.
  * **prompts/** : Cache des prompts téléchargés.
  * **memory/** : Contient l'historique des conversations.
* **.claude/** : Configuration pour l'agent officiel (si activé).
  * **commands/** : Contient les fichiers `.md` utilisés par la commande `claude`.
* **context.txt** : Le fichier de contexte généré (ignoré par git).

---

## 🛡️ Architecture & Confidentialité

### Ingestion Locale

L'outil utilise la librairie Rust `ignore` (base de `ripgrep`) pour scanner votre disque localement. Le fichier de contexte ne quitte jamais votre machine sauf lors d'une exécution explicite vers l'API du fournisseur choisi.

### Caching Intelligent

Lorsque vous lancez une commande, l'outil tente de télécharger la dernière version du prompt depuis GitHub.

* **Succès réseau :** Le prompt est exécuté et mis en cache localement.
* **Échec réseau :** L'outil utilise silencieusement la version en cache.

### Gestion de la Mémoire

Une mémoire glissante (conservant les dernières interactions) est stockée localement pour donner du contexte à l'IA.

> **Note :** Cette mémoire est exclusive à la CLI `osk`. L'agent officiel "Claude Code" gère sa propre session indépendamment.

---
