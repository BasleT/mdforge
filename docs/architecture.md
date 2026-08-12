# Architecture

mdforge is built around a small pipeline:

```text
CLI command
   |
   v
Input reader
   |
   v
Template lookup
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

## Modules

- `cli`: clap definitions for commands and global options.
- `input`: UTF-8 text file loading and input validation.
- `templates`: built-in template discovery and loading.
- `prompt`: layered prompt composition.
- `providers`: provider trait and provider request boundary.
- `providers::ollama`: Ollama `/api/generate` implementation.
- `generator`: connects templates, prompt composition, provider calls, and cleanup.
- `validation`: lightweight Markdown response cleanup.
- `output`: default output paths and file writing.

## Prompt Layers

Prompts are composed in this order:

1. Global mdforge policy
2. Optional strict source-grounding policy
3. Selected template
4. User instruction
5. Source input

Each layer is clearly labeled in the flattened model prompt so policy, template rules, user intent, and source data stay distinct.

## Provider Boundary

v0.1.0 supports Ollama only. The `LlmProvider` trait keeps HTTP details out of the rest of the application so future providers can be explored without rewriting CLI, template, prompt, validation, or output code.

## Validation

The cleanup pass intentionally stays lightweight:

- reject empty responses
- trim leading and trailing whitespace
- remove one accidental outer Markdown fence
- preserve internal fenced code blocks

Strict source-grounding verification is a future extension point, not a v0.1.0 feature.
