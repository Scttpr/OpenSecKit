use anyhow::Result;
use reqwest::header::HeaderMap;
use serde_json::{json, Value};
use super::common::LlmDriver;

pub struct ClaudeDriver;

impl LlmDriver for ClaudeDriver {
    fn get_url(&self, _model: &str, _api_key: &str) -> String {
        "https://api.anthropic.com/v1/messages".to_string()
    }

    fn get_headers(&self, api_key: &str) -> Result<HeaderMap> {
        let mut headers = HeaderMap::new();
        headers.insert("x-api-key", api_key.parse()?);
        headers.insert("anthropic-version", "2023-06-01".parse()?);
        headers.insert("content-type", "application/json".parse()?);
        Ok(headers)
    }

    fn build_body(&self, model: &str, system: &str, user: &str) -> Value {
        json!({
            "model": model,
            "max_tokens": 4096,
            "system": system,
            "messages": [{ "role": "user", "content": user }]
        })
    }

    fn extract_response<'a>(&self, json: &'a Value) -> Option<&'a str> {
        json["content"][0]["text"].as_str()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_claude_headers() {
        let driver = ClaudeDriver;
        let headers = driver.get_headers("sk-123").unwrap();
        assert_eq!(headers.get("x-api-key").unwrap(), "sk-123");
        assert!(headers.contains_key("anthropic-version"));
    }

    #[test]
    fn test_claude_body() {
        let driver = ClaudeDriver;
        let body = driver.build_body("claude-3", "Sys", "User");
        assert_eq!(body["system"], "Sys");
        assert_eq!(body["messages"][0]["content"], "User");
    }
}