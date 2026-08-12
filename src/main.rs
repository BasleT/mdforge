use anyhow::Result;
use clap::Parser;
use mdforge::{
    cli::{Cli, Commands},
    generator, input, output,
    providers::ollama::OllamaProvider,
    templates,
};

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if matches!(&cli.command, Commands::Templates) {
        print_templates()?;
        return Ok(());
    }

    let (input_text, default_output) = match &cli.command {
        Commands::Prompt { text } => (text.clone(), output::default_for_prompt()),

        Commands::File { path } => {
            let contents = input::read_file(path)?;
            let output_path = output::default_for_file(path);

            (contents, output_path)
        }

        Commands::Templates => unreachable!(),
    };

    if input_text.trim().is_empty() {
        anyhow::bail!("Input is empty. Provide prompt text or a non-empty text file.");
    }

    let template = templates::load(&cli.template)?;

    println!();
    println!("mdforge");
    println!("  model     {}", cli.model);
    println!("  template  {}", cli.template);
    println!("  strict    {}", if cli.strict { "on" } else { "off" });
    println!();
    println!("Forging Markdown...");

    let provider = OllamaProvider::default();
    let markdown = generator::generate_with_template(
        &provider,
        &cli.model,
        template,
        cli.instruction.as_deref(),
        &input_text,
        cli.strict,
    )
    .await?;

    let output_path = cli.output.unwrap_or(default_output);

    output::write(&output_path, &markdown)?;

    println!();
    println!("Created {}", output_path.display());

    Ok(())
}

fn print_templates() -> Result<()> {
    let templates = templates::list()?;

    println!();
    println!("Available templates");
    println!();

    if templates.is_empty() {
        println!("  No templates found.");
        return Ok(());
    }

    for template in templates {
        println!("  {}", template.name);
    }

    Ok(())
}
