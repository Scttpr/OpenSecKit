use super::LlmProvider;
use anyhow::{bail, Context, Result};
use reqwest::blocking::Client;
use reqwest::header::HeaderMap;
use reqwest::StatusCode;
use serde_json::Value;

pub trait LlmDriver {
    fn get_url(&self, model: &str, api_key: &str) -> String;
    fn get_headers(&self, api_key: &str) -> Result<HeaderMap>;
    fn build_body(&self, model: &str, system: &str, user: &str) -> Value;
    fn extract_response<'a>(&self, json: &'a Value) -> Option<&'a str>;
}

pub struct GenericProvider<D: LlmDriver> {
    api_key: String,
    model: String,
    client: Client,
    driver: D,
}

impl<D: LlmDriver> GenericProvider<D> {
    pub fn new(client: Client, api_key: &str, model: &str, driver: D) -> Self {
        Self {
            api_key: api_key.to_string(),
            model: model.to_string(),
            client,
            driver,
        }
    }

    fn parse_provider_error(&self, body: &str) -> Option<String> {
        let json: Value = serde_json::from_str(body).ok()?;

        if let Some(msg) = json.get("error").and_then(|e| e.get("message")) {
            return Some(msg.as_str().unwrap_or_default().to_string());
        }

        if let Some(msg) = json.get("message") {
            return Some(msg.as_str().unwrap_or_default().to_string());
        }

        None
    }
}

impl<D: LlmDriver> LlmProvider for GenericProvider<D> {
    fn complete(&self, system_prompt: &str, user_input: &str) -> Result<String> {
        let url = self.driver.get_url(&self.model, &self.api_key);
        let headers = self.driver.get_headers(&self.api_key)?;
        let body = self
            .driver
            .build_body(&self.model, system_prompt, user_input);

        let resp = self
            .client
            .post(&url)
            .headers(headers)
            .json(&body)
            .send()
            .context("Erreur réseau : Impossible de contacter le fournisseur d'IA")?;

        if !resp.status().is_success() {
            let status = resp.status();
            let raw_body = resp.text().unwrap_or_default();

            let api_message = self
                .parse_provider_error(&raw_body)
                .unwrap_or_else(|| "Aucun détail fourni par l'API".to_string());

            let advice = match status {
                StatusCode::UNAUTHORIZED => {
                    "⛔ Clé API invalide. Vérifiez votre variable d'environnement."
                }
                StatusCode::FORBIDDEN => {
                    "🚫 Accès interdit. Modèle non autorisé ou compte inactif."
                }
                StatusCode::TOO_MANY_REQUESTS => "⏳ Quota dépassé (Rate Limit).",
                StatusCode::INTERNAL_SERVER_ERROR
                | StatusCode::BAD_GATEWAY
                | StatusCode::SERVICE_UNAVAILABLE => "🔥 Erreur côté serveur (Provider).",
                StatusCode::BAD_REQUEST => "❌ Requête invalide (Contexte trop long ?).",
                _ => "Erreur inconnue.",
            };

            bail!(
                "\n❌ ECHEC APPEL IA ({})\n👉 Conseil : {}\n🛑 Message API : {}\n",
                status,
                advice,
                api_message
            );
        }

        let json: Value = resp.json()?;
        match self.driver.extract_response(&json) {
            Some(t) => Ok(t.to_string()),
            None => bail!("Réponse illisible (Structure JSON inattendue): {}", json),
        }
    }
}
