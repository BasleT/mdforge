# mdforge

[![CI](https://github.com/BasleT/mdforge/actions/workflows/ci.yml/badge.svg)](https://github.com/BasleT/mdforge/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

Local-first Markdown generation for developers using Rust and Ollama.

mdforge is a focused CLI that transforms prompts and local text files into structured Markdown using a locally running Ollama model. It is designed for messy notes, logs, source files, configuration, JSON, and rough ideas that should become useful documentation without sending source material to a hosted LLM API.

## Why mdforge

Developers often have private material that is almost documentation: scratch notes, incident fragments, CLI source code, config files, TODOs, or JSON payloads. Cloud AI tools can format that material well, but uploading private source or operational notes is not always acceptable.

mdforge keeps the workflow local-first. It composes a clear layered prompt, sends it to Ollama on your machine, cleans up the Markdown result, and writes a predictable output file.

## Features

- Local-first AI workflow through Ollama
- Prompt-to-Markdown generation
- File-to-Markdown generation for UTF-8 text files
- Built-in reusable transformation templates
- Strict source-grounding mode
- Custom model selection with `--model`
- Predictable output paths with `--output`
- Rust CLI built with clap
- No cloud account required for generation

## Demo

Input:

```text
server died at 3am
database connection timeout
restarted api
need to investigate connection pool
probably add better logging
```

Command:

```text
mdforge file examples/incident-notes.txt --template incident-report --strict
```

Example output:

```markdown
# Incident Report

## Summary
Server died at 3am. Database connection timeout occurred. The API was restarted.

## Actions Taken
- Restarted api

## Follow-Up Actions
- Need to investigate connection pool
- Probably add better logging
```

Exact output depends on the local model. Strict mode tells mdforge to avoid unsupported operational facts, but generated Markdown should still be reviewed when accuracy matters.

## Installation

Requirements:

- Rust and Cargo
- Ollama running locally
- A compatible local Ollama model

Install and run Ollama from the official Ollama project, then pull a model:

```text
ollama pull qwen2.5-coder:14b
```

The default development model is `qwen2.5-coder:14b`, but you can use another installed Ollama model with `--model`.

Build mdforge:

```text
git clone https://github.com/BasleT/mdforge.git
cd mdforge
cargo build --release
```

Run from the repository:

```text
cargo run -- prompt "Turn this idea into useful Markdown"
```

Or run the built binary:

```text
target/release/mdforge prompt "Turn this idea into useful Markdown"
```

## Usage

```text
mdforge prompt <TEXT>
mdforge file <PATH>
mdforge templates
```

Global options:

```text
--model <MODEL>        Ollama model to use
-t, --template <NAME>  Markdown transformation template
-o, --output <PATH>    Custom output path
-i, --instruction <TEXT>
                       Additional instruction for the transformation
--strict               Enable stricter source-grounding rules
```

Examples:

```text
mdforge prompt "Explain this architecture"
mdforge prompt "Summarize this idea" --template summarize
mdforge file examples/messy-notes.txt
mdforge file examples/incident-notes.txt --template incident-report --strict
mdforge file examples/small-cli.rs --template code-docs --strict
mdforge file examples/sample.json --output output/config-docs.md
```

## Templates

Built-in templates:

- `default`: general-purpose transformation into clean structured Markdown
- `summarize`: concise summarization that preserves important facts and action items
- `documentation`: technical documentation from supplied material
- `code-docs`: conservative source-code documentation
- `incident-report`: source-grounded incident report formatting

List templates:

```text
mdforge templates
```

## Strict Mode

Strict mode injects a source-grounding policy into the composed prompt:

```text
mdforge file examples/incident-notes.txt --template incident-report --strict
```

It is intended for code documentation, incident reports, technical records, meeting notes, and operational documentation. It tells the model to use only information supported by the source and to avoid converting uncertainty into fact.

LLM output can still be imperfect. Review generated Markdown when factual accuracy matters.

## Architecture

```text
Global mdforge policy
        +
Strictness policy
        +
Selected template
        +
User instruction
        +
Source content
        |
        v
Prompt composer
        |
        v
Ollama provider
        |
        v
Markdown cleanup
        |
        v
Output writer
```

Ollama is isolated behind a provider boundary so future providers can be added without rewriting the CLI flow. v0.1.0 intentionally supports Ollama only.

## Privacy

With the Ollama provider, source material is sent to the locally running Ollama service instead of a hosted LLM API. Your local Ollama setup, installed models, and machine configuration still determine where data is processed and stored. Review your Ollama configuration before using mdforge with sensitive material.

See [SECURITY.md](SECURITY.md) for data handling notes and vulnerability reporting guidance.

## Development

```text
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
```

Useful smoke checks:

```text
cargo run -- templates
cargo run -- prompt "Summarize this local-first Markdown CLI"
cargo run -- file examples/incident-notes.txt --template incident-report --strict
```

## Roadmap

Future ideas, not current v0.1.0 capabilities:

- Multiple input files
- Custom user templates
- Config file
- Streaming output
- Additional local or OpenAI-compatible providers
- Optional grounding verification
- Directory processing

## Repository

GitHub: <https://github.com/BasleT/mdforge>

Repository description:

```text
Local-first Rust CLI for turning prompts and text files into structured Markdown with Ollama.
```

## License

MIT
