use anyhow::Result;
use reqwest::header::HeaderMap;
use serde_json::{json, Value};
use super::common::LlmDriver;

pub struct GeminiDriver;

impl LlmDriver for GeminiDriver {
    fn get_url(&self, model: &str, api_key: &str) -> String {
        format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{model}:generateContent?key={api_key}"
        )
    }

    fn get_headers(&self, _api_key: &str) -> Result<HeaderMap> {
        Ok(HeaderMap::new())
    }

    fn build_body(&self, _model: &str, system: &str, user: &str) -> Value {
        json!({
            "systemInstruction": { "parts": [{ "text": system }] },
            "contents": [{ "parts": [{ "text": user }] }]
        })
    }

    fn extract_response<'a>(&self, json: &'a Value) -> Option<&'a str> {
        json["candidates"][0]["content"]["parts"][0]["text"].as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_gemini_body_builder() {
        let driver = GeminiDriver;
        let body = driver.build_body("gemini-pro", "Sys", "User");
        
        assert_eq!(body["systemInstruction"]["parts"][0]["text"], "Sys");
        assert_eq!(body["contents"][0]["parts"][0]["text"], "User");
    }

    #[test]
    fn test_gemini_response_extraction() {
        let driver = GeminiDriver;
        let fake_response = json!({
            "candidates": [{
                "content": {
                    "parts": [{ "text": "Résultat du test" }]
                }
            }]
        });

        let result = driver.extract_response(&fake_response);
        assert_eq!(result, Some("Résultat du test"));
    }
}