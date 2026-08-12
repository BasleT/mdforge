use crate::{
    prompt,
    providers::{GenerateRequest, LlmProvider},
    templates, validation,
};
use anyhow::Result;

pub async fn generate<P: LlmProvider>(
    provider: &P,
    model: &str,
    template_name: &str,
    instruction: Option<&str>,
    input: &str,
    strict: bool,
) -> Result<String> {
    let template = templates::load(template_name)?;

    generate_with_template(provider, model, template, instruction, input, strict).await
}

pub async fn generate_with_template<P: LlmProvider>(
    provider: &P,
    model: &str,
    template: String,
    instruction: Option<&str>,
    input: &str,
    strict: bool,
) -> Result<String> {
    let prompt = prompt::compose(&prompt::PromptRequest::new(
        template,
        instruction.map(str::to_string),
        input.to_string(),
        strict,
    ));

    let markdown = provider
        .generate(GenerateRequest {
            model,
            prompt: &prompt,
        })
        .await?;

    validation::clean_markdown(&markdown)
}

#[cfg(test)]
mod tests {
    use super::generate_with_template;
    use crate::providers::{GenerateRequest, LlmProvider, ProviderFuture};
    use anyhow::Result;

    struct MockProvider;

    impl LlmProvider for MockProvider {
        fn generate<'a>(&'a self, request: GenerateRequest<'a>) -> ProviderFuture<'a> {
            Box::pin(async move {
                assert_eq!(request.model, "test-model");
                assert!(request.prompt.contains("--- SELECTED TEMPLATE ---"));
                Ok("```markdown\n# Cleaned\n```".to_string())
            })
        }
    }

    #[tokio::test]
    async fn generate_composes_prompt_and_cleans_provider_output() -> Result<()> {
        let markdown = generate_with_template(
            &MockProvider,
            "test-model",
            "Template".to_string(),
            Some("Make it short."),
            "source",
            false,
        )
        .await?;

        assert_eq!(markdown, "# Cleaned");

        Ok(())
    }
}
