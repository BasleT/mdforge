# Project Title

mdforge

## One-Line Description

A local-first Rust CLI that turns prompts and text files into structured Markdown with Ollama.

## Short Project Description

mdforge is a developer tool for transforming messy notes, logs, JSON, config files, and source snippets into useful Markdown without uploading private material to cloud AI services. It uses a layered prompt pipeline, built-in Markdown templates, and local Ollama inference to keep the workflow focused and inspectable.

## Full Case-Study Description

I built mdforge to solve a recurring documentation problem: useful developer context often starts as rough notes, source files, logs, or incident fragments that are not ready to publish or share. Existing AI writing tools can help, but they often require sending private material to hosted services.

mdforge approaches the problem as a local-first Rust CLI. The command layer handles prompt input, file input, template selection, output paths, model selection, and strict mode. The generation layer builds a structured prompt from global mdforge policy, optional source-grounding policy, the selected template, user instruction, and source content. That prompt is sent through an isolated Ollama provider boundary, then the response is cleaned before writing Markdown to disk.

The architecture keeps v0.1.0 intentionally small. It does not include cloud providers, a web UI, directory ingestion, or a plugin system. The main tradeoff is that source grounding is prompt-enforced rather than formally verified, so generated output still needs human review when accuracy matters. The code is structured so a future grounding verification pass or additional provider can be added without rewriting the CLI.

## Technical Highlights

- Rust CLI built with clap and async Ollama HTTP calls.
- Layered prompt composer with explicit global policy, strict policy, template, instruction, and source sections.
- Built-in Markdown templates embedded from readable files.
- Ollama isolated behind a provider boundary.
- Strict mode for source-grounded transformations such as incident reports and code documentation.
- Lightweight Markdown response cleanup that removes accidental outer fences while preserving internal code blocks.
- Unit and executable smoke tests for CLI parsing, template discovery, prompt composition, input/output behavior, provider boundaries, and cleanup.
- CI for formatting, clippy with warnings denied, and tests.

## Tech Stack

- Rust
- clap
- tokio
- reqwest
- serde
- anyhow
- Ollama
- GitHub Actions

## Links

- GitHub: https://github.com/BasleT/mdforge

## Suggested Screenshots

- `mdforge --help`
- Messy source notes in `examples/incident-notes.txt`
- Generated Markdown incident report
- `--strict` incident report command and output
- Passing `cargo test` or CI output

## Resume Bullet

Built mdforge, a local-first Rust CLI that transforms prompts and text files into structured Markdown through Ollama using layered prompt composition, built-in templates, strict source-grounding mode, and tested provider/cleanup boundaries.

## Interview Talking Points

- Why v0.1.0 keeps Ollama as the only provider while still isolating provider code.
- How layered prompt composition improves testability and reduces prompt drift.
- Why strict mode is prompt-enforced in v0.1.0 and what a future verification pass could look like.
- How built-in templates are kept as readable Markdown files while still being embedded into the binary.
- Why Markdown cleanup is lightweight instead of a full parser.
- Scope choices that kept the tool a focused CLI instead of a broader document ingestion framework.
