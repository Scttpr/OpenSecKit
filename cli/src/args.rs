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

    /// Créer des structures de fichiers
    Scaffold {
        #[command(subcommand)]
        command: ScaffoldCommands,
        /// Afficher le résultat en JSON (pour agents AI)
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

/// Sous-commandes pour osk scaffold
#[derive(Subcommand)]
pub enum ScaffoldCommands {
    /// Créer la structure d'une nouvelle feature
    Feature {
        /// Nom de la feature
        name: String,
        /// Ne pas créer de branche git
        #[arg(long)]
        no_branch: bool,
    },
    /// Créer un rapport d'incident
    Incident {
        /// Description de l'incident
        description: String,
        /// Sévérité (CRITIQUE, IMPORTANT, MODERE, MINEUR)
        #[arg(long, default_value = "IMPORTANT")]
        severity: String,
    },
    /// Créer la structure RGPD
    Rgpd,
    /// Créer la structure RGS
    Rgs {
        /// Nom du système
        system: String,
    },
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
}
