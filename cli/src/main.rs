#![warn(clippy::all, clippy::pedantic)]

mod args;
mod commands;
mod config;
mod github;
mod memory;
mod prompts;
mod providers;
mod stack;

use anyhow::Result;
use clap::Parser;
use reqwest::blocking::Client;

fn main() -> Result<()> {
    dotenv::dotenv().ok();

    let cli = args::Cli::parse();
    let client = Client::builder().user_agent(config::USER_AGENT).build()?;

    commands::handle(cli.command, &client)?;
    Ok(())
}
