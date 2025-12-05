pub mod init;
pub mod ingest; // Nouveau module

use crate::args::Commands;
use reqwest::blocking::Client;
use anyhow::Result;

pub fn handle(command: Commands, client: &Client) -> Result<()> {
    match command {
        Commands::Init { force } => init::run(client, force),
        Commands::Ingest { output, stats } => ingest::run(&output, stats),
    }
}