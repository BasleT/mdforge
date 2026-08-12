pub mod ollama;

use anyhow::Result;
use std::future::Future;
use std::pin::Pin;

pub type ProviderFuture<'a> = Pin<Box<dyn Future<Output = Result<String>> + Send + 'a>>;

/// A provider-agnostic request for Markdown generation.
#[derive(Debug, Clone, Copy)]
pub struct GenerateRequest<'a> {
    pub model: &'a str,
    pub prompt: &'a str,
}

/// Boundary implemented by local or future model providers.
pub trait LlmProvider {
    fn generate<'a>(&'a self, request: GenerateRequest<'a>) -> ProviderFuture<'a>;
}
