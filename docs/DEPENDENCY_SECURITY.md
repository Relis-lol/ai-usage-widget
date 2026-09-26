# Dependency security review

Review date: 2026-09-26. Command: `cargo audit --file src-tauri/Cargo.lock`.

The release-candidate lockfile has **zero known vulnerabilities**. Cargo Audit reports seven warnings. Warnings are not suppressed; each is reviewed below. The project uses the current compatible Tauri 2 dependency set and Dependabot monitors Cargo updates weekly.

| Dependency | Version | RustSec warning | Relationship and reachability | Upgrade assessment | 0.1.0 decision |
|---|---:|---|---|---|---|
| `proc-macro-error` | 1.0.4 | RUSTSEC-2024-0370, unmaintained | Transitive through GTK macros. `cargo tree --target x86_64-pc-windows-msvc` confirms it is not in the Windows graph. It is build-time macro infrastructure on Linux, not application runtime input handling. | No safe direct upgrade is available without an upstream GTK/Tauri dependency change. | **ACCEPTED FOR 0.1.0** (not built for the Windows release). |
| `unic-char-property` | 0.9.0 | RUSTSEC-2025-0081, unmaintained | Transitive through `unic-ucd-ident` → `urlpattern` → `tauri-utils`. Present in the Windows dependency graph, but the warning is maintenance status rather than a vulnerability. | No maintained drop-in release is selected by current Tauri. Replacing it locally would fork Tauri's URL-pattern stack. | **ACCEPTED FOR 0.1.0**; monitor upstream. |
| `unic-char-range` | 0.9.0 | RUSTSEC-2025-0075, unmaintained | Same transitive `urlpattern` chain. No provider data is used to construct URL patterns. | No low-risk direct upgrade in the current graph. | **ACCEPTED FOR 0.1.0**; monitor upstream. |
| `unic-common` | 0.9.0 | RUSTSEC-2025-0080, unmaintained | Same transitive `urlpattern` chain; not directly called by application code. | No low-risk direct upgrade in the current graph. | **ACCEPTED FOR 0.1.0**; monitor upstream. |
| `unic-ucd-ident` | 0.9.0 | RUSTSEC-2025-0100, unmaintained | Direct parent of the other `unic-*` warnings through Tauri's URL-pattern implementation. The widget loads only bundled local assets and defines no remote navigation surface. | No maintained compatible alternative can be selected without upstream changes. | **ACCEPTED FOR 0.1.0**; monitor upstream. |
| `unic-ucd-version` | 0.9.0 | RUSTSEC-2025-0098, unmaintained | Same transitive `urlpattern` chain; metadata helper, not direct application logic. | No low-risk direct upgrade in the current graph. | **ACCEPTED FOR 0.1.0**; monitor upstream. |
| `glib` | 0.18.5 | RUSTSEC-2024-0429, unsound iterator implementation | Transitive Linux GTK dependency. `cargo tree --target x86_64-pc-windows-msvc` confirms it is absent from the Windows graph. AI Usage Widget does not call the affected iterator APIs. | A direct version override would cross the GTK ABI/dependency boundary. Upgrade only through Tauri/GTK upstream. | **ACCEPTED FOR 0.1.0** Windows release; Linux is not a supported 0.1.0 target. |

## Previously fixed findings

- `quick-xml` 0.38.4 (RUSTSEC-2026-0194 and RUSTSEC-2026-0195) was upgraded through `plist` 1.10.1 to `quick-xml` 0.42.0.
- `time` 0.3.45 (RUSTSEC-2026-0009) was upgraded to 0.3.55.

Any future advisory classified as a vulnerability and reachable in the supported Windows build is a release blocker. Maintenance warnings must continue to be reviewed, not globally ignored.
