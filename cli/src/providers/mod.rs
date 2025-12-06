pub mod claude;
pub mod common;
pub mod gemini;

use anyhow::Result;
use common::GenericProvider;
use reqwest::blocking::Client;

pub trait LlmProvider {
    fn complete(&self, system_prompt: &str, user_input: &str) -> Result<String>;
}

pub fn get_provider(
    client: Client,
    provider_name: &str,
    model: &str,
    api_key: &str,
) -> Result<Box<dyn LlmProvider>> {
    match provider_name {
        "gemini" => Ok(Box::new(GenericProvider::new(
            client, // On passe le client
            api_key,
            model,
            gemini::GeminiDriver,
        ))),
        "claude" => Ok(Box::new(GenericProvider::new(
            client, // On passe le client
            api_key,
            model,
            claude::ClaudeDriver,
        ))),
        _ => anyhow::bail!("Provider non supporté : {provider_name}"),
    }
}
