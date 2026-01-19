pub mod changes;
pub mod check;
pub mod id;
pub mod ingest;
pub mod init;
pub mod scan;
pub mod update;
pub mod validate;

use crate::args::Commands;
use anyhow::Result;
use reqwest::blocking::Client;

pub fn handle(command: Commands, client: &Client) -> Result<()> {
    match command {
        Commands::Init {
            force,
            default,
            agent,
            all_agents,
            local,
            local_path,
            version,
        } => init::run(client, force, default, agent, all_agents, local, local_path, version),

        Commands::Ingest { output, path } => ingest::run(&output, true, &path),

        Commands::Check { command, json } => check::run(command, json),

        Commands::Update { command, json } => update::run(command, json),

        Commands::Validate { command, json } => validate::run(command, json),

        Commands::Scan { path, json } => scan::run(path.as_deref(), json),

        Commands::Id { path, json } => id::run(&path, json),

        Commands::Changes { since, json } => changes::run(since.as_deref(), json),
    }
}
