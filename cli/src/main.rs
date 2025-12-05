mod args;
mod config;
mod github;
mod commands;

use clap::Parser;
use reqwest::blocking::Client;
use anyhow::Result;

fn main() -> Result<()> {
    let cli = args::Cli::parse();

    let client = Client::builder()
        .user_agent(config::USER_AGENT)
        .build()?;

    commands::handle(cli.command, &client)?;

    Ok(())
}
