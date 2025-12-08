use clap::{Parser, Subcommand};

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
        #[arg(long, short)]
        force: bool,
    },

    Ingest {
        #[arg(short, long, default_value = "context.txt")]
        output: String,
        #[arg(short, long, default_value = ".")]
        path: String,
    },

    Audit {
        #[arg(long)]
        provider: Option<String>,
    },

    Spec {
        user_story: String,
        #[arg(long)]
        provider: Option<String>,
    },

    Domaine {
        nom: String,
        #[arg(long)]
        provider: Option<String>,
    },

    Context {
        #[arg(long)]
        provider: Option<String>,
    },

    Assess {
        subject: String,
        #[arg(long)]
        provider: Option<String>,
    },

    Incident {
        description: String,
        #[arg(long)]
        provider: Option<String>,
    },
}
