---
"tool": patch
---

**fix**: `wifi-qr` now escapes `"` in the SSID and password, as the WiFi QR format requires, so scanners read credentials containing quotes exactly as typed.
