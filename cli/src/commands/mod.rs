pub mod ingest;
pub mod init;
pub mod run;

use crate::args::Commands;
use anyhow::Result;
use reqwest::blocking::Client;

pub fn handle(command: Commands, client: &Client) -> Result<()> {
    match command {
        Commands::Init { force } => init::run(client, force),
        Commands::Ingest { output, path } => ingest::run(&output, true, &path),

        Commands::Audit { provider } => run::exec_specific(client, "audit", None, provider),
        Commands::Spec {
            user_story,
            provider,
        } => run::exec_specific(client, "spec", Some(user_story), provider),

        Commands::Domaine { nom, provider } => {
            run::exec_specific(client, "domain", Some(nom), provider)
        }

        Commands::Context { provider } => run::exec_specific(client, "context", None, provider),
        Commands::Assess { subject, provider } => {
            run::exec_specific(client, "assess", Some(subject), provider)
        }
        Commands::Incident {
            description,
            provider,
        } => run::exec_specific(client, "incident", Some(description), provider),
    }
}
