pub mod check;
pub mod ingest;
pub mod init;
pub mod scaffold;
pub mod update;
pub mod validate;

use crate::args::{Commands, CheckCommands, ScaffoldCommands, UpdateCommands, ValidateCommands};
use anyhow::Result;
use reqwest::blocking::Client;

pub fn handle(command: Commands, client: &Client) -> Result<()> {
    match command {
        Commands::Init {
            force,
            default,
            agent,
            all_agents,
        } => init::run(client, force, default, agent, all_agents),

        Commands::Ingest { output, path } => ingest::run(&output, true, &path),

        Commands::Check { command, json } => check::run(command, json),

        Commands::Scaffold { command } => scaffold::run(command),

        Commands::Update { command } => update::run(command),

        Commands::Validate { command } => validate::run(command),
    }
}
