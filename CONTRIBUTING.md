# Contributing

1. Keep provider code behind `UsageProvider` and return `UNAVAILABLE` when a value cannot be supported.
2. Never scrape provider websites, browser cookies, authentication files, or arbitrary user directories.
3. Never add a real provider session, rollout, diagnostic log, credential, email address, account identifier, machine name, or personal path.
4. Build synthetic fixtures from scratch.
5. Run formatting, Clippy, tests, a dependency audit, and secret scanning before submitting changes.

```powershell
cargo fmt --manifest-path src-tauri/Cargo.toml -- --check
cargo clippy --manifest-path src-tauri/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path src-tauri/Cargo.toml
cargo audit
powershell -File scripts/secret-scan.ps1
```

Dependencies should be minimal, maintained, and justified. GitHub Actions should be pinned whenever practical. Do not add telemetry.
