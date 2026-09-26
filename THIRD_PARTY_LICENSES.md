# Third-party licenses

Runtime dependencies are declared in `src-tauri/Cargo.toml` and locked in `src-tauri/Cargo.lock` after the first build. Tauri is licensed under MIT or Apache-2.0. Serde, serde_json, chrono, and their transitive dependencies use permissive licenses; verify the lockfile with an automated license tool before a stable release.

No third-party executable is downloaded by the application at runtime.
