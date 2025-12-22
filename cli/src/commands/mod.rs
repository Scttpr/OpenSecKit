pub mod ingest;
pub mod init;

use crate::args::Commands;
use anyhow::Result;
use reqwest::blocking::Client;

pub fn handle(command: Commands, client: &Client) -> Result<()> {
    match command {
        Commands::Init { force, default } => init::run(client, force, default),
        Commands::Ingest { output, path } => ingest::run(&output, true, &path),
    }
}
