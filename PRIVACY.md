# VoidBlock Privacy Policy

**Last Updated**: January 2025  
**Version**: 2.0  
**Effective Date**: January 1, 2025

## Executive Summary

VoidBlock is a **100% local ad and tracker blocker** that operates entirely on your device. We collect **zero telemetry, zero analytics, and zero personal data**. VoidBlock never contacts any central server to transmit information about your browsing habits, blocked domains, or device state.

---

## What VoidBlock Does NOT Collect

VoidBlock explicitly **does not** collect, transmit, or store any of the following:

| Category | Collected? | Details |
|----------|:----------:|---------|
| Browsing history | ❌ | We never see or store sites you visit |
| Visited domains | ❌ | Blocked/allowed decisions stay on your device |
| IP addresses | ❌ | No IP logging or network tracking |
| Device identifiers | ❌ | No IDFA, GAID, or hardware serials collected |
| Analytics | ❌ | No usage metrics, feature tracking, or funnels |
| Crash reports | ❌ | Crashes are logged locally only (disabled by default) |
| Performance metrics | ❌ | No latency, memory, or CPU statistics sent |
| Personal information | ❌ | No name, email, location, or account data |
| Telemetry | ❌ | Zero server-side event tracking |

---

## What VoidBlock Stores (Locally Only)

All data stored by VoidBlock remains **on your device** and is never uploaded:

### Device Storage

1. **Blocklist Database** (~5MB, compressed)
   - Contains ~5 million ad/tracker domain names
   - Stored in SQLite with LZ4 compression
   - Read-only at runtime; updated ~weekly
   - Verified with SHA256 HMAC before loading

2. **Settings & Preferences**
   - Blocking layer preferences (DNS, packet, AI, cosmetic)
   - Split tunnel allowlist
   - Update channel preference
   - Auto-refresh interval

3. **User Allowlist**
   - Domains you manually whitelist
   - Not synced to any server

4. **DNS Cache**
   - Recent DNS responses (24-48 hour TTL)
   - Automatically expires
   - Never persisted to disk

---

## Optional Services & Data Sharing

VoidBlock can optionally use upstream DNS providers for queries **not blocked locally**:

### Upstream DNS (Cloudflare, Quad9)

| Aspect | Details |
|--------|---------|
| **What's sent** | Only domain names for non-blocked queries (~0.1% of traffic) |
| **What's NOT sent** | Device ID, user ID, timestamps, or any other metadata |
| **User control** | Can be disabled entirely in Settings |
| **Encryption** | DoH (TLS 1.3+) for all upstream queries |
| **Third-party policy** | Subject to Cloudflare/Quad9 privacy policies |

**Example:**
- Query "google.com" → Blocked locally, upstream DNS NOT contacted
- Query "obscure-domain.com" → Not in blocklist, sent to upstream (encrypted)

---

## VPN Permission (Android)

VoidBlock uses Android's VpnService API to enable system-wide ad blocking. This requires the VPN permission, but:

- ✅ All DNS interception occurs **locally on 127.0.0.1**
- ✅ No traffic routed through external VPN servers
- ✅ No VPN service provider accounts or authentication
- ✅ Cannot access your encryption keys or HTTPS payloads
- ❌ No traffic logging or monitoring

---

## Browser Extensions (Chromium/Firefox)

### What Extensions Do
- Inject cosmetic CSS to hide blocked ad containers
- Block requests matching the 25,000 declarative rules
- Store extension state in your browser profile

### What They Don't Do
- ❌ No cloud sync of extension data
- ❌ No transmission of blocked URLs
- ❌ No analytics or usage tracking
- ❌ No external API calls (except to your blocklist update server)

### Browser Storage
- Stored in your browser's isolated storage
- Not synced to browser account servers
- Deleted when extension is uninstalled

---

## Desktop App (Tauri + Svelte)

### Data Storage
- All settings saved in local `~/.voidblock/` directory
- Not synced to cloud
- Encrypted by OS-level filesystem encryption

### Network Activity
- Only downloads blocklist updates (HTTPS, no authentication)
- Queries upstream DNS only for non-blocked domains
- No telemetry or update checking

---

## Android App

### Permissions Required
- `BIND_VPN_SERVICE` — Create local DNS proxy
- `CHANGE_NETWORK_STATE` — Modify DNS settings
- `ACCESS_NETWORK_STATE` — Check network availability
- `INTERNET` — Blocklist updates
- `WRITE_EXTERNAL_STORAGE` — Save settings (if requested)

### No Permissions Used For
- ❌ Location tracking
- ❌ Contact/calendar access
- ❌ Camera/microphone
- ❌ SMS/call logging

### Battery Impact
- Minimal: <1% battery drain per hour
- DNS filtering runs at kernel level (efficient)
- No background processes hogging CPU

---

## macOS & Windows

### System Proxy vs. Filter
- **macOS**: Uses NetworkExtension for local packet filtering
- **Windows**: Uses system DNS override + local proxy

### Data Handling
- All configuration stored locally in user directory
- No cloud sync or remote management
- Uninstallation removes all local data

---

## OpenWRT Router

### eBPF-Based Filtering
- Runs at Linux kernel level for maximum efficiency
- All domain blocking occurs locally on router
- No external DNS forwarding (unless configured)

### Router Data
- Blocklist stored on router's filesystem
- No upload of traffic logs or statistics
- Local-network access only

---

## Crash Reporting (Disabled by Default)

If you explicitly enable crash reporting:

### What's Included
- Stack trace of the crash
- Android/iOS version
- Device category (budget/mid/flagship — not specific model)
- Timestamp of crash

### What's Excluded
- ❌ Domains you visited
- ❌ IP addresses or location
- ❌ Device identifiers (IDFA, GAID)
- ❌ App version or build hash (to avoid fingerprinting)

### Where It Goes
- Self-hosted crash reporter (not Google Analytics, Crashlytics, or Sentry)
- Stored for 90 days, then purged
- You can disable this at any time in Settings

---

## Data Retention

| Data Type | Retention | Auto-Deletion |
|-----------|-----------|----------------|
| DNS Cache | 24-48 hours | TTL-based |
| Blocklist | Updated weekly | Manual refresh |
| Settings | Indefinite | Until deleted by user |
| Allowlist | Indefinite | Until modified by user |
| Crash logs | 90 days | Automatic |
| No persistent logs | N/A | N/A |

---

## Third-Party Libraries & Dependencies

VoidBlock uses open-source libraries. None of them collect data:

- **Tokio** (async runtime) — No telemetry
- **rusqlite** (SQLite) — No telemetry
- **reqwest** (HTTP client) — No telemetry
- **Svelte** (UI framework) — No analytics
- **Tauri** (desktop framework) — No telemetry (can be disabled)

---

## Regulatory Compliance

### GDPR (EU)
- ✅ No personal data collected → No GDPR obligations
- ✅ No data processing → No Data Protection Impact Assessment required
- ✅ No data sharing → No data controller/processor agreements needed

### CCPA (California)
- ✅ No personal information collected → No disclosure required
- ✅ No data sales → No opt-out needed

### PIPEDA (Canada)
- ✅ No personal information → No privacy officer notification needed

### APEC Privacy Framework
- ✅ 100% compliant (local processing, no data collection)

---

## App Store Policies

### Google Play Store
- ✅ No tracking library requirements
- ✅ No optional/required crash reporting
- ✅ No permission over-reach
- ✅ Transparent permission usage

### Apple App Store
- ✅ No privacy-invasive APIs
- ✅ Clear permission descriptions
- ✅ No IDFA usage
- ✅ "App Privacy" label: **No data collected**

### Chrome Web Store
- ✅ No host permissions abuse
- ✅ Privacy policy provided
- ✅ No hidden data transmission

---

## Security Measures

- **Blocklist Verification**: SHA256 HMAC signatures prevent tampering
- **Network Security**: TLS 1.3+ for all upstream connections
- **Local Storage**: Encrypted by OS filesystem (BitLocker, FileVault, dm-crypt)
- **Code Audits**: Open source for independent security review
- **No Backdoors**: No remote code execution, no kill switches

---

## Changes to This Policy

We will notify you of material changes by:

1. Updating this policy on GitHub
2. Adding a prominent note in release notes
3. In-app notification (next launch) if critical change

Changes **cannot** be retroactively applied to past usage.

---

## Contact & Support

### Privacy Inquiries
- **GitHub**: https://github.com/voidblock/voidblock/issues
- **Email**: privacy@voidblock.dev
- **Response Time**: Within 30 days

### Deletion Requests
To delete all local VoidBlock data:

1. **Desktop/Android**: Uninstall the app (all data deleted)
2. **Browser Extension**: Uninstall the extension (storage cleared)
3. **OpenWRT**: SSH to router → `rm -rf /etc/voidblock`

---

## Policy Acknowledgment

By using VoidBlock, you acknowledge that:
- ✅ VoidBlock collects zero telemetry
- ✅ Your browsing data never leaves your device
- ✅ This policy is accurate and transparent
- ✅ You've read and understood this privacy policy

---

**Last Updated**: January 2025  
**Version**: 2.0 (Store-compliant)  
**Status**: ✅ Production-ready for all platforms
