# Handover

## Current state

Version 0.1.0 has passed its automated, installer, security, privacy, and manual Windows acceptance gates. **SAFE TO MAKE PUBLIC: YES.** The initial local release commit is created only after the final checks pass. Publishing remains an explicit owner action.

The release frontend passed synthetic-data render checks at 100%, 125%, 150%, and 200% in Dark and Light themes. The System theme followed the QA machine's Light theme. The README screenshot is generated from the real release UI code with an explicit `DEMO DATA` label; normal runtime contains no demo provider.

## Architecture and providers

Tauri 2 hosts a static HTML/CSS/JavaScript interface. Rust owns settings, tray/startup behavior, and provider acquisition. `UsageProvider`, `ProviderSnapshot`, and `UsageWindow` keep provider parsing outside the UI.

- Codex: short-lived local `codex app-server --stdio`, narrow typed parsing of `account/rateLimits/read` only. No raw responses are logged or persisted.
- Claude: adapter retained; returns `UNAVAILABLE` with “No supported local usage source detected.” Private sessions and credentials are never parsed.

## Verification state

- Rustfmt, Clippy, JavaScript syntax, and 8 automated tests pass.
- Opt-in Codex live probe passes and agrees with the app-server response.
- RustSec: zero vulnerabilities; seven reviewed warnings are documented in `docs/DEPENDENCY_SECURITY.md`.
- Gitleaks: clean across the exact Git file set.
- NSIS: per-user silent install, Start Menu entry, Installed Apps entry, launch, and silent uninstall pass.
- MSI: intentionally removed from 0.1.0 after non-elevated installation failed with Windows error 1925/1603.
- Manual Windows QA: tray menu and tooltip, dragging, position persistence, always-on-top, startup integration, close-to-tray, tray restore, manual refresh, Dark/Light/System themes, and Windows DPI scaling all pass.
- Latest measured working set: approximately 25.9 MiB.

## Commands

```powershell
# Development
cargo tauri dev

# Tests
cargo test --manifest-path src-tauri/Cargo.toml

# Lint
cargo fmt --manifest-path src-tauri/Cargo.toml --all -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings

# Production executable and NSIS installer
cargo tauri build

# Secret scan (gitleaks must be on PATH)
powershell -File scripts/secret-scan.ps1
```

## Next action

The source is ready for owner review and optional publication. Do not push, publish a release, or change repository visibility without an explicit owner action.
