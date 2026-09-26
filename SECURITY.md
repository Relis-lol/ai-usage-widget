# Security Policy

## Data access and boundaries

The Codex adapter launches the locally installed `codex app-server --stdio` and requests only the account rate-limit snapshot. A narrow typed schema ignores every response field except the request id, named Codex limit bucket, percentages, window durations, and reset times. Raw responses are never logged or persisted. It does not read session files, prompts, auth files, browser profiles, cookies, or tokens.

The Claude adapter does not parse local sessions or credentials. Without a documented read-only local usage interface, it returns `UNAVAILABLE`.

The app sends no telemetry and has no application-owned network client. It never transmits prompts, conversations, file contents, project names, credentials, cookies, or authentication material.

## Reporting a vulnerability

Do not include credentials or private provider files in a report. Until a private GitHub security-reporting channel is configured, contact the repository owner privately before opening a public issue. Include reproduction steps made from synthetic data only.

## Contributor fixture rules

Never copy real Codex or Claude files into the repository. Create fixtures from scratch with identities such as `TestUser`, `fake-session-id`, and `example@example.invalid`. Run secret scanning before every commit. If a real credential reaches Git history, stop: rotate it and clean history before publication.
