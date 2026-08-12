use anyhow::{Result, bail};

pub fn clean_markdown(markdown: &str) -> Result<String> {
    let trimmed = markdown.trim();

    if trimmed.is_empty() {
        bail!(
            "The model returned an empty response. Try a different model or a more specific input."
        );
    }

    let cleaned = strip_outer_markdown_fence(trimmed)
        .unwrap_or(trimmed)
        .trim();

    if cleaned.is_empty() {
        bail!("The model response only contained an empty Markdown code fence.");
    }

    Ok(cleaned.to_string())
}

fn strip_outer_markdown_fence(markdown: &str) -> Option<&str> {
    let mut lines = markdown.lines();
    let first = lines.next()?.trim();

    if !matches!(first, "```" | "```markdown" | "```md") {
        return None;
    }

    let last_line_start = markdown.rfind("\n```")?;
    let after_last = &markdown[last_line_start + 4..];

    if !after_last.trim().is_empty() {
        return None;
    }

    let content_start = markdown.find('\n')? + 1;
    Some(&markdown[content_start..last_line_start])
}

#[cfg(test)]
mod tests {
    use super::clean_markdown;

    #[test]
    fn rejects_empty_model_response() {
        assert!(clean_markdown(" \n\t ").is_err());
    }

    #[test]
    fn removes_outer_markdown_fence() {
        let cleaned = clean_markdown("```markdown\n# Title\n\nText\n```").unwrap();

        assert_eq!(cleaned, "# Title\n\nText");
    }

    #[test]
    fn preserves_internal_code_fences() {
        let markdown = "# Title\n\n```rust\nfn main() {}\n```\n";
        let cleaned = clean_markdown(markdown).unwrap();

        assert_eq!(cleaned, "# Title\n\n```rust\nfn main() {}\n```");
    }
}
