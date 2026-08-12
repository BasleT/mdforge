# mdforge v0.1.0 Release Notes

mdforge v0.1.0 is the first public release of a local-first Rust CLI for turning prompts and individual text files into structured Markdown with Ollama.

## Highlights

- Prompt-to-Markdown and file-to-Markdown workflows.
- Built-in templates for general cleanup, summarization, documentation, code documentation, and incident reports.
- Strict source-grounding mode for more conservative transformations.
- Layered prompt composition with global policy, strictness policy, selected template, user instruction, and source input.
- Ollama-only provider implementation behind a provider boundary.
- Markdown cleanup for empty responses and accidental outer code fences.
- Public release docs, examples, CI, security notes, contribution guide, and portfolio material.

## Example

```text
mdforge file examples/incident-notes.txt --template incident-report --strict
```

## Quality Gate

The release is validated with:

```text
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test
cargo package --allow-dirty
```

## Known Limitations

- Ollama is the only provider in v0.1.0.
- Inputs are individual UTF-8 text files or prompt text.
- Strict mode is prompt-enforced and should not be treated as formal factual verification.
- Generated Markdown should be reviewed before use in high-stakes documentation.
