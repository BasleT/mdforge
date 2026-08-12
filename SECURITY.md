# Security Policy

mdforge is a local-first CLI that sends source material to the Ollama service configured on your machine. It does not include hosted LLM providers in v0.1.0.

## Supported Versions

Security fixes are currently targeted at the latest released version.

| Version | Supported |
| ------- | --------- |
| 0.1.x   | Yes       |

## Reporting Issues

Please report security issues through GitHub's private vulnerability reporting feature if it is enabled for the repository. If that is unavailable, open a minimal public issue that asks for a private contact path without including exploit details or sensitive data.

## Data Handling Notes

- mdforge reads individual UTF-8 text files and prompt text supplied by the user.
- With the Ollama provider, prompt content is sent to the locally configured Ollama HTTP endpoint.
- Generated Markdown is written to the requested output path, or to `output/` by default.
- `output/`, `.env`, logs, and build artifacts are ignored by default.

Review your local Ollama configuration and model storage behavior before processing sensitive material.
