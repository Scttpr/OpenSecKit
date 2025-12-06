use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "osk")]
#[command(version)]
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
        #[arg(long)]
        stats: bool,
        #[arg(short, long, default_value = ".")]
        path: String,
    },
    Run {
        #[arg(index = 1, num_args = 1..)]
        keywords: Vec<String>,

        #[arg(short, long)]
        input: Option<String>,

        #[arg(long)]
        provider: Option<String>,
    },
}
