#![warn(clippy::all)]

mod agents;
mod args;
mod commands;
mod config;
mod github;
mod registry;
mod utils;

use anyhow::Result;
use clap::Parser;
use reqwest::blocking::Client;

fn main() -> Result<()> {
    dotenvy::dotenv().ok();

    let cli = args::Cli::parse();
    let client = Client::builder().user_agent(config::USER_AGENT).build()?;

    commands::handle(cli.command, &client)?;
    Ok(())
}
