# UI quality assurance

## Automated release-UI render matrix

The release frontend was rendered with the same HTML, CSS, and JavaScript used by the application. The screenshot harness supplies clearly labelled synthetic provider values and does not exist in normal runtime behavior.

| Scale | Dark | Light | Result |
|---:|---|---|---|
| 100% | Pass | Pass | No clipping, overlap, or unreadable values. |
| 125% | Pass | Pass | No clipping, overlap, or unreadable values. |
| 150% | Pass | Pass | No clipping, overlap, or unreadable values. |
| 200% | Pass | Pass | No clipping, overlap, or unreadable values. |

System theme rendered as Light on the QA machine and matched the Light result. Used/remaining percentages, both reset representations, provider state, window freshness, buttons, Claude unavailable state, and footer remained readable. The README screenshot is the 200% dark-theme render and is visibly marked `DEMO DATA`.

## Native Windows QA status: PASS

Manual Windows QA was completed successfully for:

- Tray menu layout and tooltip
- Header drag behavior and restored window position
- Always-on-top toggle
- Start-with-Windows toggle
- Close-to-tray and left-click restore
- Manual refresh through both window and tray
- Dark, Light, and System themes
- Windows DPI scaling

The earlier automation limitation did not replace manual acceptance testing. Combined with the automated render matrix and installer smoke test, the native UI gate is complete.
