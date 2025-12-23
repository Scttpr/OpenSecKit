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

impl Agent {
    pub fn display_name(&self) -> &'static str {
        match self {
            Agent::ClaudeCode => "Claude Code",
            Agent::Copilot => "GitHub Copilot",
            Agent::Cursor => "Cursor",
            Agent::Gemini => "Gemini",
        }
    }

    pub fn all() -> Vec<Agent> {
        vec![Agent::ClaudeCode, Agent::Copilot, Agent::Cursor, Agent::Gemini]
    }
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

    Ingest {
        #[arg(short, long, default_value = "context.txt")]
        output: String,
        #[arg(short, long, default_value = ".")]
        path: String,
    },
}
