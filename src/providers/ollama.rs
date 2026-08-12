use super::{GenerateRequest, LlmProvider, ProviderFuture};
use anyhow::{Context, Result, bail};
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::time::Duration;

const DEFAULT_OLLAMA_URL: &str = "http://localhost:11434/api/generate";
const REQUEST_TIMEOUT: Duration = Duration::from_secs(120);

#[derive(Debug, Clone)]
pub struct OllamaProvider {
    endpoint: String,
    client: Client,
}

impl Default for OllamaProvider {
    fn default() -> Self {
        Self::new(DEFAULT_OLLAMA_URL)
    }
}

impl OllamaProvider {
    pub fn new(endpoint: impl Into<String>) -> Self {
        Self {
            endpoint: endpoint.into(),
            client: Client::new(),
        }
    }

    async fn generate_inner(&self, request: GenerateRequest<'_>) -> Result<String> {
        let response = self
            .client
            .post(&self.endpoint)
            .timeout(REQUEST_TIMEOUT)
            .json(&OllamaGenerateRequest {
                model: request.model,
                prompt: request.prompt,
                stream: false,
            })
            .send()
            .await
            .context("Could not connect to Ollama. Start Ollama and try again.")?;

        let status = response.status();
        if !status.is_success() {
            let body = response
                .text()
                .await
                .unwrap_or_else(|_| "Ollama did not return an error body.".to_string());

            if status == StatusCode::NOT_FOUND || body.to_lowercase().contains("model") {
                bail!(
                    "Ollama could not use model '{}'. Pull it with `ollama pull {}` or choose another model with `--model`.\nOllama response: {}",
                    request.model,
                    request.model,
                    body.trim()
                );
            }

            bail!(
                "Ollama returned HTTP {} from {}.\nOllama response: {}",
                status,
                self.endpoint,
                body.trim()
            );
        }

        let body: OllamaGenerateResponse = response
            .json()
            .await
            .context("Could not parse Ollama response as JSON")?;

        Ok(body.response)
    }
}

impl LlmProvider for OllamaProvider {
    fn generate<'a>(&'a self, request: GenerateRequest<'a>) -> ProviderFuture<'a> {
        Box::pin(async move { self.generate_inner(request).await })
    }
}

#[derive(Debug, Serialize)]
struct OllamaGenerateRequest<'a> {
    model: &'a str,
    prompt: &'a str,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct OllamaGenerateResponse {
    response: String,
}
