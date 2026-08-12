const GLOBAL_CONTEXT: &str = r#"You are mdforge, a local Markdown transformation pipeline.

You convert supplied source material into clear, useful Markdown while preserving the source's meaning.
Use only the selected template, user instruction, and source input below.
Return only the finished Markdown document."#;

const STRICT_MODE_POLICY: &str = r#"SOURCE-STRICT MODE

Use only information explicitly supported by the supplied source.

Do not introduce unsupported:

- facts
- identifiers
- timestamps
- causes
- models
- commands
- values
- examples
- affected systems
- people
- impact
- recommendations

Do not fill missing information using common conventions or assumptions.
Do not convert suspected information into confirmed information.
If information is unavailable, either omit it or explicitly state that it is not present in the source.
Keep wording close to the source when precision matters."#;

const DEFAULT_USER_INSTRUCTION: &str = "Transform the source input into Markdown.";

/// The structured inputs mdforge flattens into a single model prompt.
#[derive(Debug, Clone)]
pub struct PromptRequest {
    pub template: String,
    pub instruction: String,
    pub source: String,
    pub strict: bool,
}

impl PromptRequest {
    pub fn new(
        template: String,
        instruction: Option<String>,
        source: String,
        strict: bool,
    ) -> Self {
        Self {
            template,
            instruction: instruction.unwrap_or_else(|| DEFAULT_USER_INSTRUCTION.to_string()),
            source,
            strict,
        }
    }
}

pub fn compose(request: &PromptRequest) -> String {
    let strict_policy = if request.strict {
        format!("\n\n--- STRICT MODE POLICY ---\n\n{}", STRICT_MODE_POLICY)
    } else {
        String::new()
    };

    format!(
        r#"{global_context}{strict_policy}

--- SELECTED TEMPLATE ---

{template}

--- USER INSTRUCTION ---

{instruction}

--- SOURCE INPUT ---

{source}

--- END SOURCE INPUT ---

Return only the finished Markdown document.
"#,
        global_context = GLOBAL_CONTEXT,
        strict_policy = strict_policy,
        template = request.template.trim(),
        instruction = request.instruction.trim(),
        source = request.source.trim()
    )
}

#[cfg(test)]
mod tests {
    use super::{PromptRequest, compose};

    #[test]
    fn compose_layers_prompt_in_order_without_strict_policy() {
        let prompt = compose(&PromptRequest::new(
            "Template rules".to_string(),
            Some("User instruction".to_string()),
            "Source input".to_string(),
            false,
        ));

        assert!(prompt.contains("You are mdforge"));
        assert!(prompt.contains("--- SELECTED TEMPLATE ---\n\nTemplate rules"));
        assert!(prompt.contains("--- USER INSTRUCTION ---\n\nUser instruction"));
        assert!(prompt.contains("--- SOURCE INPUT ---\n\nSource input"));
        assert!(!prompt.contains("--- STRICT MODE POLICY ---"));

        let template_index = prompt.find("--- SELECTED TEMPLATE ---").unwrap();
        let instruction_index = prompt.find("--- USER INSTRUCTION ---").unwrap();
        let source_index = prompt.find("--- SOURCE INPUT ---").unwrap();

        assert!(template_index < instruction_index);
        assert!(instruction_index < source_index);
    }

    #[test]
    fn compose_includes_strict_policy_when_requested() {
        let prompt = compose(&PromptRequest::new(
            "Template rules".to_string(),
            None,
            "Source input".to_string(),
            true,
        ));

        assert!(prompt.contains("--- STRICT MODE POLICY ---"));
        assert!(prompt.contains("SOURCE-STRICT MODE"));
        assert!(prompt.contains("Do not introduce unsupported"));
        assert!(
            prompt
                .contains("--- USER INSTRUCTION ---\n\nTransform the source input into Markdown.")
        );
    }

    #[test]
    fn compose_orders_strict_policy_before_template() {
        let prompt = compose(&PromptRequest::new(
            "Template rules".to_string(),
            Some("User instruction".to_string()),
            "Source input".to_string(),
            true,
        ));

        let global_index = prompt.find("You are mdforge").unwrap();
        let strict_index = prompt.find("--- STRICT MODE POLICY ---").unwrap();
        let template_index = prompt.find("--- SELECTED TEMPLATE ---").unwrap();
        let instruction_index = prompt.find("--- USER INSTRUCTION ---").unwrap();
        let source_index = prompt.find("--- SOURCE INPUT ---").unwrap();

        assert!(global_index < strict_index);
        assert!(strict_index < template_index);
        assert!(template_index < instruction_index);
        assert!(instruction_index < source_index);
    }
}
