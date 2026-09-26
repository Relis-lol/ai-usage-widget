# Windows code signing

AI Usage Widget 0.1.0 development and release-candidate artifacts are unsigned. Windows SmartScreen can warn when an unsigned executable or MSI has no established publisher reputation. This warning does not by itself indicate that an artifact is malicious.

Every GitHub release should publish `SHA256SUMS.txt`. Users can verify a download with:

```powershell
Get-FileHash -Algorithm SHA256 -LiteralPath '.\AI Usage Widget_0.1.0_x64-setup.exe'
```

Compare the full hash with the release's checksum file obtained from the same GitHub release page.

## Future signing path

Tauri supports Windows signing without changing the application architecture. A future maintainer can configure a protected code-signing certificate in the private release environment and sign the executable plus NSIS/MSI bundles during release packaging.

- Never commit a certificate, private key, password, hardware-token export, or signing-service credential.
- Keep signing credentials out of pull-request workflows.
- Prefer a protected manual release job or trusted signing service with narrowly scoped access.
- Verify signatures and hashes before attaching artifacts to a GitHub draft release.

No certificate is purchased or configured for 0.1.0. Unsigned status is documented and is not a release blocker.
