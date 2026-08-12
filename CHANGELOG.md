# Changelog

All notable changes to mdforge will be documented in this file.

## 0.1.0 - 2026-08-11

Initial public release.

### Added

- `mdforge prompt <TEXT>` for prompt-to-Markdown generation.
- `mdforge file <PATH>` for UTF-8 text file-to-Markdown generation.
- `mdforge templates` for listing built-in templates.
- Built-in templates: `default`, `summarize`, `documentation`, `code-docs`, and `incident-report`.
- Global `--model`, `--template`, `--output`, `--instruction`, and `--strict` options.
- Layered prompt composition with global policy, optional strict policy, template, instruction, and source content.
- Ollama provider boundary with local generation through `/api/generate`.
- Lightweight Markdown cleanup that rejects empty responses and removes accidental outer Markdown fences.
- Focused tests for CLI parsing, prompt composition, templates, input/output behavior, and Markdown cleanup.
- Executable smoke tests for help, template listing, and missing-file behavior.
- GitHub Actions CI for formatting, clippy, tests, and package verification.
- Contribution guide, security policy, issue templates, PR template, architecture notes, and release notes.

### Notes

- v0.1.0 supports Ollama only. Hosted cloud LLM providers are intentionally out of scope.
- v0.1.0 supports individual UTF-8 text files only. Directory processing and binary document parsing are future work.
