pub mod ingest;
pub mod init;

use crate::args::Commands;
use anyhow::Result;
use reqwest::blocking::Client;

pub fn handle(command: Commands, client: &Client) -> Result<()> {
    match command {
        Commands::Init { force } => init::run(client, force),
        Commands::Ingest { output, path } => ingest::run(&output, true, &path),
    }
}
