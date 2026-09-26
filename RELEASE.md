# Release process

1. Update the version in `src-tauri/Cargo.toml`, `src-tauri/tauri.conf.json`, and the changelog.
2. Run formatting, Clippy, tests, build validation, dependency audit, and secret scan.
3. Perform the manual repository and Git-history audit documented in `HANDOVER.md`.
4. Build on Windows with `cargo tauri build`.
5. Smoke-test the NSIS install, Start Menu entry, normal-user launch, startup option, tray behavior, upgrade, and uninstall.
6. Sign the executable and installer using the publisher's protected certificate workflow. Signing credentials must never enter the repository.
7. Generate SHA-256 checksums with `Get-FileHash` and attach installer plus checksum to a draft release.
8. Review the draft manually before publishing. Release publication is intentionally not automated.

Unsigned development installers commonly trigger Windows SmartScreen reputation warnings. Code signing reduces warnings but does not instantly establish reputation.
