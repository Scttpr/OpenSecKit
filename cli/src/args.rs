use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "osk")]
#[command(author = "Scttpr <https://github.com/Scttpr/OpenSecKit>")]
#[command(version)]
#[command(about = "🛡️  Assistant de Sécurité Agile", long_about = None)]
#[command(long_about = "
OpenSecKit (osk) est le pont entre votre code et votre conformité sécurité.

Il permet de :
1. 📥 Initialiser les standards de sécurité (.osk) et les agents IA (.claude)
2. 🚜 Transformer votre base de code en contexte digeste pour les LLMs (Ingest)
3. 🤖 Automatiser la documentation de sécurité via Claude Code

Utilisez 'osk help <commande>' pour plus de détails sur une commande spécifique.")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init {
        #[arg(long, short, help = "Écrase les fichiers locaux existants")]
        force: bool,
    },

    Ingest {
        #[arg(short, long, default_value = "codebase_dump.txt", help = "Fichier de destination")]
        output: String,
        #[arg(long, help = "Affiche le nombre de caractères et l'estimation de tokens")]
        stats: bool,
    },
}