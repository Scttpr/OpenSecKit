pub mod ingest;
pub mod init;
pub mod run;

use crate::args::Commands;
use anyhow::Result;
use reqwest::blocking::Client;

pub fn handle(command: Commands, client: &Client) -> Result<()> {
    match command {
        Commands::Init { force } => init::run(client, force),

        Commands::Ingest {
            output,
            stats,
            path,
        } => ingest::run(&output, stats, &path),

        Commands::Run {
            keywords,
            input,
            provider,
        } => run::exec(client, keywords, input, provider),
    }
}
