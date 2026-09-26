# AI Usage Widget 0.1.0 release-candidate report

## Acceptance summary

1. **Initial local commit:** Authorized after this report update and the final gates pass. The commit hash is reported in the final handoff response because a commit cannot contain its own hash.
2. **Files changed:** Application UI, provider parser, original icon source/generated icons, README, CI, security/privacy documentation, dependency review, installer/signing/UI-QA documentation, release procedure, and screenshot harness/output.
3. **UI QA by DPI/theme:** **PASS.** The release frontend passed render checks at 100%, 125%, 150%, and 200% in Dark and Light with no observed clipping or overlap. System followed Light. Manual Windows QA also passed for Dark, Light, and System themes and Windows DPI scaling; see `docs/UI_QA.md`.
4. **Codex live verification:** Passed through local `codex app-server --stdio`. The latest verification returned the available weekly window with internally consistent used/remaining/reset data.
5. **Claude status:** Fail-safe `UNAVAILABLE`; clean secondary text states no supported local source was detected. No unsafe workaround was added.
6. **RustSec:** Zero vulnerabilities. `quick-xml` and `time` vulnerabilities were fixed. Seven transitive maintenance/unsoundness warnings are individually reviewed and accepted for the Windows 0.1.0 scope in `docs/DEPENDENCY_SECURITY.md`; none is classified as an exploitable Windows finding for this app.
7. **Gitleaks:** Passed across the exact Git file set. No real provider file, credential, personal path, or diagnostic log is tracked.
8. **CI review:** Read-only contents permission; pinned checkout/toolchain SHAs; pinned and checksum-verified Gitleaks binary; pinned cargo-audit version; format, Clippy, tests, release build, secret scan, and dependency audit; no repository secret required.
9. **Installer:** NSIS only, per-user, no administrator requirement. MSI intentionally removed after failing the normal-user acceptance gate.
10. **Installer/uninstall and native QA:** **PASS.** Silent NSIS install, Start Menu entry, Installed Apps entry, installed launch, silent uninstall, and uninstall-entry cleanup passed. Manual Windows QA passed for tray menu and tooltip, dragging, position persistence, always-on-top, startup integration, close-to-tray, tray restore, and manual refresh.
11. **Memory:** Approximately 25.9 MiB working set in the final installed-app smoke test.
12. **Screenshot:** `docs/screenshot-rc.png`, tightly cropped, no personal information, visibly labelled `DEMO DATA`.
13. **Known limitations:** Windows-only 0.1.0; local Codex CLI required; Claude usage unavailable; unsigned artifacts may trigger SmartScreen; reviewed transitive warnings.
14. **Git status:** Local repository only. All intended source changes are included in the initial local release commit after the final audit. No push, GitHub repository modification, or release publication was performed.
15. **SAFE TO MAKE PUBLIC:** **YES**.

## Release decision

All required automated, installer, security, privacy, and manual Windows QA gates for 0.1.0 have passed. The source is safe to make public. Publishing remains an explicit owner action.
