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
    /// Initialize a project with OpenSecKit
    Init {
        /// Force reinstall even if files exist
        #[arg(long, short)]
        force: bool,
        /// Non-interactive mode with defaults (for CI/tests)
        #[arg(long, short)]
        default: bool,
        /// Target AI agent (if not specified, interactive selection)
        #[arg(long, short, value_enum)]
        agent: Option<Agent>,
        /// Install configuration for all agents
        #[arg(long)]
        all_agents: bool,
        /// Use local repository resources (for development)
        #[arg(long, short)]
        local: bool,
        /// Path to local OpenSecKit repository (implies --local)
        #[arg(long, value_name = "PATH")]
        local_path: Option<String>,
        /// Specific version to download (e.g., v4.0.0)
        #[arg(long, short = 'V')]
        version: Option<String>,
    },

    /// Export project context
    Ingest {
        #[arg(short, long, default_value = "context.txt")]
        output: String,
        #[arg(short, long, default_value = ".")]
        path: String,
    },

    /// Validate system model
    ///
    /// Validates the system model for schema compliance and consistency.
    /// Checks YAML schema validity, cross-reference integrity, and
    /// index.yaml size constraint (<200 lines).
    #[command(
        long_about = "Validate the system model for schema compliance and consistency.\n\nChecks:\n- YAML schema validity for all section files\n- Cross-reference integrity (all referenced IDs exist)\n- Index.yaml size constraint (<200 lines)\n\nUsed by: /osk-discover-validate"
    )]
    Validate {
        /// Path to system model (default: .osk/system-model/)
        #[arg(short, long)]
        path: Option<String>,
        /// Output as JSON (for AI agents)
        #[arg(long)]
        json: bool,
    },

    /// Scan project files (respects .gitignore)
    ///
    /// Traverses the directory respecting .gitignore and returns the list
    /// of files with hints extracted from ignored patterns.
    /// Used by AI agents for /osk-discover.
    #[command(
        long_about = "Scan project files respecting .gitignore patterns.\n\nOutputs file list with gitignore hints for AI agents to understand project structure.\nUsed by: /osk-discover"
    )]
    Scan {
        /// Path to scan (default: current directory)
        #[arg(short, long)]
        path: Option<String>,
        /// Output as JSON (for AI agents)
        #[arg(long)]
        json: bool,
    },

    /// Generate a component ID from a file path
    ///
    /// Transforms a file path into a normalized component identifier.
    /// Format: {directory}-{name}, lowercase with hyphens.
    /// Ex: src/api/users.rs → api-users
    #[command(
        long_about = "Generate a normalized component ID from a file path.\n\nFormat: {directory}-{name}, lowercase with hyphens.\nExamples:\n  src/api/users.rs → api-users\n  lib/auth/jwt.py → auth-jwt\n\nUsed by: /osk-discover to create consistent component identifiers"
    )]
    Id {
        /// Source file path
        path: String,
        /// Output as JSON (for AI agents)
        #[arg(long)]
        json: bool,
    },

    /// List files changed since the last scan
    ///
    /// Uses git diff to detect files added, modified, or deleted
    /// since the last scan recorded in index.yaml.
    #[command(
        long_about = "List files changed since the last system model scan.\n\nReads last_commit from .osk/system-model/index.yaml and compares with HEAD.\nOutputs changed files with change type (added/modified/deleted).\n\nUsed by: /osk-discover for incremental model updates"
    )]
    Changes {
        /// Reference commit (default: read from index.yaml)
        #[arg(long)]
        since: Option<String>,
        /// Output as JSON (for AI agents)
        #[arg(long)]
        json: bool,
    },
}
