use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "mdforge",
    version,
    about = "Forge structured Markdown from prompts and local files using local AI.",
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Ollama model to use
    #[arg(long, global = true, default_value = "qwen2.5-coder:14b")]
    pub model: String,

    /// Markdown transformation template
    #[arg(short, long, global = true, default_value = "default")]
    pub template: String,

    /// Custom output path
    #[arg(short, long, global = true)]
    pub output: Option<PathBuf>,

    /// Additional instruction for the Markdown transformation
    #[arg(short, long, global = true)]
    pub instruction: Option<String>,

    /// Enable stricter source-grounding rules
    #[arg(long, global = true)]
    pub strict: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Transform raw text into Markdown
    Prompt {
        /// Text to transform
        text: String,
    },

    /// Transform a local file into Markdown
    File {
        /// File to transform
        path: PathBuf,
    },

    /// List available Markdown templates
    Templates,
}

#[cfg(test)]
mod tests {
    use super::{Cli, Commands};
    use clap::Parser;
    use std::path::{Path, PathBuf};

    #[test]
    fn parses_global_strict_flag_before_subcommand() {
        let cli = Cli::try_parse_from(["mdforge", "--strict", "prompt", "hello"]).unwrap();

        assert!(cli.strict);
        assert!(matches!(
            cli.command,
            Commands::Prompt { text } if text == "hello"
        ));
    }

    #[test]
    fn parses_global_strict_flag_after_subcommand() {
        let cli = Cli::try_parse_from(["mdforge", "file", "input.txt", "--strict"]).unwrap();

        assert!(cli.strict);
        assert!(matches!(
            cli.command,
            Commands::File { path } if path == Path::new("input.txt")
        ));
    }

    #[test]
    fn parses_global_instruction() {
        let cli = Cli::try_parse_from([
            "mdforge",
            "--instruction",
            "Make this terse",
            "prompt",
            "hello",
        ])
        .unwrap();

        assert_eq!(cli.instruction.as_deref(), Some("Make this terse"));
    }

    #[test]
    fn parses_templates_command() {
        let cli = Cli::try_parse_from(["mdforge", "templates"]).unwrap();

        assert!(matches!(cli.command, Commands::Templates));
    }

    #[test]
    fn parses_global_model_template_and_output() {
        let cli = Cli::try_parse_from([
            "mdforge",
            "--model",
            "llama3.1:8b",
            "--template",
            "summarize",
            "--output",
            "summary.md",
            "prompt",
            "hello",
        ])
        .unwrap();

        assert_eq!(cli.model, "llama3.1:8b");
        assert_eq!(cli.template, "summarize");
        assert_eq!(cli.output, Some(PathBuf::from("summary.md")));
    }
}
