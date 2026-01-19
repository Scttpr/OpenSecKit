use clap::{Parser, Subcommand, ValueEnum};

/// Agent AI cible pour l'installation
#[derive(Clone, Copy, Debug, PartialEq, ValueEnum)]
pub enum Agent {
    /// Claude Code (slash commands dans .claude/commands/)
    ClaudeCode,
    /// GitHub Copilot (instructions dans .github/copilot-instructions.md)
    Copilot,
    /// Cursor AI (règles dans .cursor/rules/)
    Cursor,
    /// Google Gemini (instructions dans .gemini/)
    Gemini,
}

#[derive(Parser)]
#[command(name = "osk")]
#[command(version, about = "OpenSecKit - Security as Code CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialiser un projet avec OpenSecKit
    Init {
        /// Forcer la réinstallation même si les fichiers existent
        #[arg(long, short)]
        force: bool,
        /// Mode non-interactif avec valeurs par défaut (pour CI/tests)
        #[arg(long, short)]
        default: bool,
        /// Agent AI cible (si non spécifié, sélection interactive)
        #[arg(long, short, value_enum)]
        agent: Option<Agent>,
        /// Installer la configuration pour tous les agents
        #[arg(long)]
        all_agents: bool,
        /// Utiliser les ressources locales du dépôt (pour développement)
        #[arg(long, short)]
        local: bool,
        /// Chemin vers le dépôt OpenSecKit local (implique --local)
        #[arg(long, value_name = "PATH")]
        local_path: Option<String>,
        /// Version spécifique à télécharger (ex: v3.2.0)
        #[arg(long, short = 'V')]
        version: Option<String>,
    },

    /// Exporter le contexte du projet
    Ingest {
        #[arg(short, long, default_value = "context.txt")]
        output: String,
        #[arg(short, long, default_value = ".")]
        path: String,
    },

    /// Vérifier les prérequis pour une commande
    Check {
        #[command(subcommand)]
        command: CheckCommands,
        /// Afficher le résultat en JSON
        #[arg(long)]
        json: bool,
    },

    /// Mettre à jour les fichiers mécaniquement
    Update {
        #[command(subcommand)]
        command: UpdateCommands,
        /// Afficher le résultat en JSON (pour agents AI)
        #[arg(long)]
        json: bool,
    },

    /// Valider la cohérence des fichiers
    Validate {
        #[command(subcommand)]
        command: ValidateCommands,
        /// Afficher le résultat en JSON (pour agents AI)
        #[arg(long)]
        json: bool,
    },

    /// Scanner les fichiers du projet (respecte .gitignore)
    ///
    /// Parcourt le répertoire en respectant .gitignore et retourne la liste
    /// des fichiers avec des indices extraits des patterns ignorés.
    /// Utilisé par les agents AI pour /osk-discover init.
    #[command(long_about = "Scan project files respecting .gitignore patterns.\n\nOutputs file list with gitignore hints for AI agents to understand project structure.\nUsed by: /osk-discover init, /osk-discover update")]
    Scan {
        /// Chemin à scanner (défaut: répertoire courant)
        #[arg(short, long)]
        path: Option<String>,
        /// Afficher le résultat en JSON (pour agents AI)
        #[arg(long)]
        json: bool,
    },

    /// Générer un ID de composant à partir d'un chemin de fichier
    ///
    /// Transforme un chemin de fichier en identifiant de composant normalisé.
    /// Format: {directory}-{name}, minuscules avec tirets.
    /// Ex: src/api/users.rs → api-users
    #[command(long_about = "Generate a normalized component ID from a file path.\n\nFormat: {directory}-{name}, lowercase with hyphens.\nExamples:\n  src/api/users.rs → api-users\n  lib/auth/jwt.py → auth-jwt\n\nUsed by: /osk-discover init to create consistent component identifiers")]
    Id {
        /// Chemin du fichier source
        path: String,
        /// Afficher le résultat en JSON (pour agents AI)
        #[arg(long)]
        json: bool,
    },

    /// Lister les fichiers modifiés depuis le dernier scan
    ///
    /// Utilise git diff pour détecter les fichiers ajoutés, modifiés ou
    /// supprimés depuis le dernier scan enregistré dans index.yaml.
    #[command(long_about = "List files changed since the last system model scan.\n\nReads last_commit from .osk/system-model/index.yaml and compares with HEAD.\nOutputs changed files with change type (added/modified/deleted).\n\nUsed by: /osk-discover update for incremental model updates")]
    Changes {
        /// Commit de référence (défaut: lu depuis index.yaml)
        #[arg(long)]
        since: Option<String>,
        /// Afficher le résultat en JSON (pour agents AI)
        #[arg(long)]
        json: bool,
    },
}

/// Sous-commandes pour osk check
#[derive(Subcommand)]
pub enum CheckCommands {
    /// Vérifier les prérequis pour /osk-configure
    Configure,
    /// Vérifier les prérequis pour /osk-analyze
    Analyze,
    /// Vérifier les prérequis pour /osk-specify
    Specify {
        /// Nom de la feature
        feature: String,
    },
    /// Vérifier les prérequis pour /osk-harden
    Harden {
        /// Nom de la feature
        feature: String,
    },
    /// Vérifier les prérequis pour /osk-plan
    Plan {
        /// Nom de la feature
        feature: String,
    },
    /// Vérifier les prérequis pour /osk-tasks
    Tasks {
        /// Nom de la feature
        feature: String,
    },
    /// Vérifier les prérequis pour /osk-implement
    Implement {
        /// Nom de la feature
        feature: String,
    },
    /// Vérifier les prérequis pour /osk-dashboard
    Dashboard,
}

/// Sous-commandes pour osk update
#[derive(Subcommand)]
pub enum UpdateCommands {
    /// Recalculer les statistiques du risk-register
    Stats,
    /// Mettre à jour le statut d'une tâche
    Task {
        /// ID de la tâche (ex: T001)
        id: String,
        /// Marquer comme terminée
        #[arg(long)]
        done: bool,
    },
    /// Mettre à jour le statut d'un risque
    Risk {
        /// ID du risque (ex: RISK-AUTH-001)
        id: String,
        /// Nouveau statut
        #[arg(long)]
        status: String,
    },
    /// Régénérer le dashboard
    Dashboard,
}

/// Sous-commandes pour osk validate
#[derive(Subcommand)]
pub enum ValidateCommands {
    /// Valider la syntaxe des fichiers YAML
    Yaml,
    /// Vérifier les dépendances des tâches
    Deps {
        /// Nom de la feature
        feature: String,
    },
    /// Vérifier la complétude du workflow
    Workflow {
        /// Nom de la feature
        feature: String,
    },
    /// Valider le system model (.osk/system-model/)
    ///
    /// Vérifie la cohérence du modèle système: schéma YAML, références croisées,
    /// et contrainte de taille de index.yaml (<200 lignes).
    #[command(long_about = "Validate the system model for schema compliance and consistency.\n\nChecks:\n- YAML schema validity for all 9 section files\n- Cross-reference integrity (all referenced IDs exist)\n- Index.yaml size constraint (<200 lines)\n\nUsed by: /osk-discover validate, /osk-discover init (final step)")]
    SystemModel {
        /// Chemin vers le system model (défaut: .osk/system-model/)
        #[arg(short, long)]
        path: Option<String>,
    },
}
