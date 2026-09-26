# Privacy

AI Usage Widget is local-first and contains no analytics, telemetry, advertising, cloud backend, or account system.

The app asks supported provider tooling for a minimal usage snapshot. A narrow typed parser discards unrelated app-server fields, and raw responses are never logged or persisted. It retains only local settings such as theme, provider toggles, refresh interval, and window position. Normal operation does not log prompts, conversations, response bodies, authentication headers, cookies, tokens, or personal paths.

Codex may use its own existing authenticated connection to retrieve current limits. That behavior is governed by Codex and its provider terms. AI Usage Widget does not receive or store Codex credentials.

Uninstalling removes the executable and installer records. The per-user settings file may remain in the standard application config directory and can be deleted manually; a dedicated clear-data control is planned before a stable release.
