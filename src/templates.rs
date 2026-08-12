use anyhow::{Result, bail};

const BUILT_IN_TEMPLATES: &[Template] = &[
    Template {
        name: "code-docs",
        contents: include_str!("../prompts/code-docs.md"),
    },
    Template {
        name: "default",
        contents: include_str!("../prompts/default.md"),
    },
    Template {
        name: "documentation",
        contents: include_str!("../prompts/documentation.md"),
    },
    Template {
        name: "incident-report",
        contents: include_str!("../prompts/incident-report.md"),
    },
    Template {
        name: "summarize",
        contents: include_str!("../prompts/summarize.md"),
    },
];

#[derive(Debug, Clone, Copy)]
pub struct Template {
    pub name: &'static str,
    pub contents: &'static str,
}

pub fn list() -> Result<Vec<Template>> {
    Ok(BUILT_IN_TEMPLATES.to_vec())
}

pub fn load(name: &str) -> Result<String> {
    let templates = list()?;

    let Some(template) = templates.iter().find(|template| template.name == name) else {
        let available = templates
            .iter()
            .map(|template| template.name)
            .collect::<Vec<_>>()
            .join(", ");

        bail!(
            "Template '{name}' was not found. Available templates: {}",
            if available.is_empty() {
                "none"
            } else {
                &available
            }
        );
    };

    Ok(template.contents.to_string())
}

#[cfg(test)]
mod tests {
    use super::{list, load};

    #[test]
    fn built_in_templates_are_discoverable() {
        let templates = list().unwrap();
        let names = templates
            .iter()
            .map(|template| template.name)
            .collect::<Vec<_>>();

        assert_eq!(
            names,
            vec![
                "code-docs",
                "default",
                "documentation",
                "incident-report",
                "summarize"
            ]
        );
    }

    #[test]
    fn missing_template_error_lists_available_templates() {
        let error = load("missing").unwrap_err().to_string();

        assert!(error.contains("Template 'missing' was not found"));
        assert!(error.contains("default"));
        assert!(error.contains("incident-report"));
    }
}
