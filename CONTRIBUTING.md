# Contributing

Thanks for taking an interest in mdforge.

mdforge is intentionally small: a local-first Rust CLI that transforms prompts and individual UTF-8 text files into Markdown through Ollama. Contributions should preserve that focus.

## Local Setup

```text
git clone https://github.com/BasleT/mdforge.git
cd mdforge
cargo test
```

mdforge uses Rust 1.85+.

Ollama is only required for manual generation checks. Unit and smoke tests do not require Ollama.

## Quality Gate

Run these before opening a pull request:

```text
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo package --allow-dirty
```

## Scope Guidelines

Good v0.x contributions:

- CLI polish
- Better errors
- Template improvements
- Prompt composition tests
- Markdown cleanup improvements
- Documentation and examples
- Provider boundary improvements that keep Ollama working

Out of scope for the current release line unless discussed first:

- Hosted cloud LLM providers
- Web UI
- Directory ingestion
- Binary document parsing
- PDF/OCR support
- Databases, embeddings, vector search, or RAG

## Privacy

Do not include private notes, employer-specific logs, credentials, internal hostnames, or proprietary source in issues, tests, examples, screenshots, or pull requests.
