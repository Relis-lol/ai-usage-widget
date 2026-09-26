# Installer and uninstall behavior

The Windows 0.1.0 build produces one per-user NSIS executable installer. It installs the application as a normal desktop utility, creates a Start Menu entry, and registers it in Windows Installed Apps without requiring administrator privileges.

The MSI target was evaluated and removed from 0.1.0 after its clean-machine test failed with Windows Installer error 1925/1603: the WiX bundle attempted an all-users installation requiring elevated privileges. Publishing a second installer that contradicts the normal-user requirement would be misleading. MSI can be reconsidered only when it supports and passes the same non-elevated QA gate.

The application stores preferences in the standard per-user application configuration directory. Uninstall removes program files, shortcuts, and uninstall registration. Preferences intentionally remain so reinstalling preserves theme, provider toggles, refresh interval, and window position. Users can remove the small settings directory manually if they want a complete reset.

The uninstall process must never remove provider data, Codex/Claude files, projects, or unrelated user data.
