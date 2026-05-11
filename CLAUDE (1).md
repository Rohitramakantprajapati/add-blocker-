# VOIDBLOCK — Master Engineering Blueprint

> **The Serverless, AI-Powered, Omni-Platform Ad & Tracker Eliminator**
> Version 2.0 | 2025 | Confidential

---

## Vision

100% local, zero-server, self-evolving ad and tracker elimination.
Blocks at DNS + Packet + AI + Cosmetic layers simultaneously.
No central server. No cloud dependency. No ongoing infrastructure.

---

## Realistic Scope Statement

**This is a multi-phase, multi-year project.**
A solo developer can execute this — but only by shipping one phase completely before starting the next.
Phase 1 (Android) is the entire business for the first 6 months.
Do not touch Phase 2 until Phase 1 has 1,000+ active users and measurable retention.
Every phase listed here is a real commitment. Scope creep kills privacy tools faster than competitors.

---

## Team Composition (Honest Assessment)

| Phase | Minimum Viable Team | Ideal Team |
|-------|---------------------|------------|
| Phase 1 (Android) | 1 developer (Rust + Kotlin) | 2 developers |
| Phase 2 (Browser + Windows) | 1–2 developers | 3 developers |
| Phase 3+ (macOS, iOS, Linux) | 2–3 developers | 4–5 developers |

If solo: complete Phase 1 and Phase 2 only. Use Phase 3+ revenue to hire.

---

## Architecture Overview

VoidBlock operates across four independent blocking layers.
Each layer catches what the previous one misses.
Together they achieve near-100% ad elimination without any central server.

| Layer | Method | Coverage | Platforms |
|-------|--------|----------|-----------|
| 1 — DNS | Local DoH resolver, blocklist lookup | Domain-level blocking | All |
| 2 — Packet | SNI inspection, URL pattern matching | HTTPS without MITM | All |
| 3 — AI Engine | On-device ONNX model (8MB) | Unknown / new ad patterns | All |
| 4 — Cosmetic | CSS injection, element hiding | First-party & visual ads | Browser only |

### Platform Support Matrix

| Platform | Method | Root Required? | Priority | Risk Level |
|----------|--------|---------------|----------|------------|
| Android | VpnService local loopback | No | Phase 1 | Low |
| Browser Extension | MV3 + MV2 declarativeNetRequest | No | Phase 2 | Medium (MV3 limits) |
| Windows | Userspace proxy → WFP driver (later) | Installer only | Phase 3 | Medium |
| macOS | NetworkExtension (entitlement needed) | First run only | Phase 4 | **High — see §Apple Risk** |
| Linux | eBPF + XDP + nftables | Yes | Phase 5 | Low |
| iOS | NEPacketTunnelProvider | No | Phase 6 | **High — see §Apple Risk** |
| Router | OpenWRT .ipk package | Admin panel | Phase 7 | Low |

### Core Tech Stack

| Component | Language | Reason |
|-----------|----------|--------|
| DNS Engine | Rust (Tokio) | Async, memory-safe, <0.5ms latency |
| Packet Filter | Rust + eBPF (C) | Kernel-speed, zero-copy |
| AI Engine | Rust + ONNX Runtime | 8MB model, runs on any device |
| Android App | Kotlin + Rust (JNI) | Native performance |
| iOS App | Swift + Rust (C FFI) | No jailbreak required |
| Windows Service | C++ + WFP (later) | System-wide filtering |
| macOS Extension | Swift + NetworkExtension | Apple-sanctioned |
| Desktop UI | Tauri (Rust + Svelte) | 5MB binary, cross-platform |
| Browser Extension | TypeScript MV3/MV2 | Standard extension API |
| P2P Sync | Rust (libp2p) | No central server |
| Blocklist Storage | SQLite + LZ4 | 5M domains in 5MB |

---

## Phased Build Roadmap

### Phase 1 — Android MVP (Months 1–3)

**Goal:** Ship one APK. Block 80% of ads on 80% of apps. Nothing else.

**Deliverables:**
- Local DNS resolver on 127.0.0.1 via VpnService loopback
- Bundled blocklist: Steven Black hosts + EasyList domains merged
- 5MB compressed SQLite database — no internet needed ever
- IPv4 + IPv6 dual-stack TUN interface
- Split-tunneling UI: exclude banking, VPN, payment apps by default
- Battery profiling target: <1%/hr on Snapdragon 665+
- RAM target: <15MB working set
- Test matrix: Android 8 through 14, budget to flagship

**Blocklist bootstrap (before launch):**
The "community maintains it" assumption is wrong at Day 0. You have no community.
Initial 5M-domain SQLite is built by merging these maintained upstream sources:
- Steven Black unified hosts (maintained daily by external team)
- EasyList (maintained by external team)
- EasyPrivacy
- AdGuard DNS filter

These are all MIT/CC-licensed. Merge them in `blocklists/scripts/merge.py` on a weekly cron.
You are not maintaining blocklists — you are consuming existing open-source ones.
Community PRs come after 1,000 GitHub stars, not before.

**Privacy/Legal for Play Store submission:**
- Register a legal entity (LLC or equivalent) before submitting
- Write a privacy policy: state you intercept DNS traffic locally, store nothing remotely, collect no data
- Host privacy policy at a stable URL (GitHub Pages is fine)
- Declare VPN permission usage clearly in Play Store listing
- Android's VPN permission triggers enhanced review — have your privacy policy ready before submission

**Do NOT ship YouTube ad blocking in Phase 1.**
Reason: Google Play policy §4.8 prohibits "interfering with ads served by Google's products."
YouTube is a Google product. This is a Play Store removal risk, not a technical problem.
YouTube skip automation ships in the browser extension (Firefox/Chrome) only, never in the Android APK.

---

### Phase 2 — Browser Extension + Harden Android (Months 3–6)

**Deliverables:**
- SNI-based HTTPS blocking — no MITM, connection drop only
- Signed blocklist update: static HTTPS file fetch (no P2P yet)
- Chrome/Edge MV3 extension (see MV3 constraints below)
- Firefox MV2 extension: full webRequest blocking, no rule cap
- Cosmetic filter injection: element hiding, ad placeholder removal
- YouTube ad skip automation — **Firefox and Chrome extensions only, not Android APK**
- Anti-anti-adblock bypass engine

**MV3 Chrome Extension — Real Constraints:**
MV3 is not a drop-in replacement for MV2. Specific blockers:
1. **30K static rule cap** — workaround: curate top 25K rules by traffic volume using Alexa/Tranco data
2. **No persistent background pages** — use Service Workers; they terminate after 30s of inactivity. Any state that needs to persist across page loads goes to `chrome.storage`, not in-memory
3. **Dynamic rule updates are throttled** — max 5K dynamic rules, updated via `updateDynamicRules()`. For cosmetic filters and anti-anti-adblock JS injection, use `scripting.executeScript()` instead of background page tricks — this API is MV3-native
4. **No `webRequest` blocking** — `declarativeNetRequest` only. Cosmetic filters and anti-anti-adblock bypass must be implemented as content scripts injected via `scripting.registerContentScripts()`, not as background webRequest listeners

Firefox MV2 has none of these constraints. Ship the full-featured version there first, then port to MV3 with documented degradations.

---

### Phase 3 — Windows (Months 6–9)

**Deliverables:**
- Userspace local DNS + HTTP proxy — no kernel driver, no signing cert
- Tauri tray app (5MB binary, Svelte UI)
- System-wide DNS override via Windows DNS Client service
- WFP kernel driver: only if userspace proves insufficient (Phase 5+)
- EV code signing cert: $400/yr — acquire at Month 1 of this phase, not later

**Userspace first, kernel driver later:**
The WFP driver path requires an Extended Validation code signing certificate.
Shipping userspace proxy first covers 90% of use cases without the cert.
The EV cert takes 2–4 weeks to issue. Order it at the start of Phase 3.

---

### Phase 4 — AI Engine (Months 9–14)

**Deliverables:**
- Dataset: partner with privacy-respecting analytics firm or university for labeled ad/non-ad URL corpus
- Model: lightweight gradient-boosted classifier (XGBoost, not MobileNet — MobileNet is image classification overkill)
- Features: URL entropy, domain age, CNAME depth, response size, timing patterns, TLD distribution
- Target: >92% precision on held-out test set (false positives destroy user trust irreversibly)
- Ship as optional "Enhanced Blocking" toggle — off by default until precision is validated in production
- ONNX INT8 quantized — <0.1ms inference on x86, <2ms on budget ARM
- Quarterly retrain cycle using community-reported false positives as labeled data

**False positive handling:**
Ship a one-tap "report false block" button from Day 1 of AI toggle.
Every false positive report is a labeled training sample for the next retrain.
Without this feedback loop, the model degrades over time as ad patterns evolve.

---

### Phase 5 — macOS (Months 12–16)

> ⚠️ **CRITICAL:** Submit Apple entitlement application (NEFilterDataProvider) at **Month 1 of the entire project**, not Month 12. Apple review takes 2–6 months. There is documented history of Apple rejecting ad-blocker entitlement requests. Plan for rejection.

**Apple Entitlement Risk — Mitigation Plan:**

| Scenario | Probability | Response |
|----------|-------------|----------|
| Entitlement approved | ~60% | Ship full NEFilterDataProvider implementation |
| Entitlement rejected | ~40% | Ship DNS-over-HTTPS system preference changer as primary product; this is a real product, not a fallback |

The DNS-over-HTTPS fallback is not a degraded product — it is a different product that still solves the core problem for most users. Position it as "VoidBlock DNS" for macOS and launch it regardless of entitlement outcome.

**Deliverables (entitlement approved path):**
- NEFilterDataProvider + App Proxy Provider
- Notarized .app via direct download (App Store optional)
- Menu bar app with Tauri

**Deliverables (DNS fallback path):**
- System DoH configuration utility
- Per-network profile management
- Blocklist sync to upstream DoH resolver (self-hosted or via Cloudflare's DNS firewall)

---

### Phase 6 — Linux + Router (Month 18)

**Deliverables:**
- eBPF XDP program attached to all network interfaces
- nftables secondary IP/domain blocking
- Rust userspace controller for eBPF map management
- OpenWRT .ipk package built from stripped Linux core
- Web UI for router management dashboard

---

### Phase 7 — P2P Blocklist Sync (Month 18+)

**Deliverables:**
- libp2p gossip protocol for blocklist delta propagation
- Trust model: rotating group of Ed25519 key holders (open-source maintainers)
- Bootstrap peers: hardcoded small set of well-known nodes (BitTorrent/Signal model)
- Delta sync only: ~10KB/week per device
- Cryptographic signature verification before any update applied

**Security model:**
Any node can propagate deltas. Only Ed25519-signed deltas from trusted key holders are applied.
Bootstrap peer list is hardcoded in binary and updated via app updates — same model as Signal.
No single point of failure. No central server to take down.

---

### Phase 8 — iOS (Month 24)

> ⚠️ Apply for iOS NEPacketTunnelProvider entitlement at Month 1 alongside macOS.

**Battery drain reality:**
NEPacketTunnelProvider is known to cause significant battery drain on iOS.
This will be the #1 user complaint. Mitigations:
- Implement aggressive connection coalescing
- Use `NEPacketTunnelNetworkSettings.tunnelRemoteAddress` pointing to loopback
- Profile with Xcode Instruments Energy Log before shipping
- Set explicit battery drain target: <2%/hr (iOS hardware is less efficient for this than Android)
- Publish battery benchmark results transparently in App Store listing

**Deliverables:**
- NEPacketTunnelProvider + NetworkExtension framework
- Swift + Rust (C FFI) for core engine
- No jailbreak required (iOS 15+)
- Battery target: <2%/hr with active blocking

---

## Competitive Differentiation

**Why would someone switch from AdGuard?**

AdGuard is 13 years old, has a central update server, requires a subscription for multi-platform, and its "local" mode still phones home for blocklist updates. VoidBlock's answer is:

1. **Truly serverless** — blocklist updates via P2P (Phase 7+). No server to shut down, no company to go bankrupt.
2. **On-device AI** — adapts to new ad patterns without waiting for a blocklist update.
3. **One purchase, all platforms** — no per-platform subscription.
4. **Open-source core** — auditable. AdGuard's core is partially open; their apps are not.

**Positioning statement (one sentence):**
VoidBlock is the only ad blocker that works even if the company disappears — because there's no server to shut down.

**Competitors to reference in README:**
- AdGuard: server-dependent blocklist updates, closed-source apps, subscription model
- NextDNS: cloud DNS, privacy trade-off, requires their servers
- Pi-hole: home network only, requires dedicated hardware, no mobile
- uBlock Origin: browser only, no system-wide blocking

VoidBlock is the only option that covers all four platforms (Android, iOS, desktop, browser) with no server dependency.

---

## Privacy, Legal & Compliance

### Required Before Phase 1 Launch

- [ ] Register legal entity (LLC recommended — limits personal liability)
- [ ] Write and publish Privacy Policy covering:
  - DNS traffic is processed locally only
  - No data is transmitted to VoidBlock servers (none exist)
  - No logs, no analytics, no telemetry by default
  - Optional opt-in crash reporting (if implemented) is clearly described
- [ ] GDPR compliance: no personal data collected = minimal compliance burden, but document this explicitly
- [ ] CCPA compliance: same as above
- [ ] Play Store listing: declare VPN permission, link privacy policy
- [ ] App Store listing: declare network filtering, link privacy policy

### Ongoing Legal Watchlist

- **YouTube / Google ToS:** Browser extension YouTube ad-skip is legally grey. It has not resulted in successful legal action against open-source tools historically (uBlock Origin, SponsorBlock). Monitor. Do not put it in the Android APK.
- **Apple entitlement:** Track Apple's developer forums for any policy changes to content-filtering entitlements.
- **GDPR/CCPA scope creep:** If you ever add any analytics, even opt-in, get legal review first.

---

## Telemetry & Crash Reporting Strategy

A fully local product has zero visibility into production failures without opt-in telemetry.
Without it, the feedback loop is: user emails support → you guess what happened.

**Ship opt-in anonymous diagnostics from Day 1:**
- Opt-in prompt shown once on first launch, default OFF
- If opted in: collect crash stack traces (no URLs, no domains, no personal data)
- Use Sentry self-hosted or a privacy-respecting crash reporter (not Firebase Crashlytics)
- Aggregate: Android version, device class (budget/mid/flagship), blocking layer that triggered, crash module
- Never collect: domains blocked, URLs visited, user IP, or any browsable data

This is the minimum viable feedback loop for a local-only product.

---

## Critical Risks & Mitigations

| Risk | Severity | Mitigation |
|------|----------|------------|
| iOS entitlement rejection | **High** | Apply Month 1. Build macOS/iOS DNS fallback as primary contingency. |
| Play Store removal for YouTube blocking | **High** | YouTube blocking is browser-extension only. Never ships in Android APK. |
| Windows WFP driver EV cert delay | Medium | Ship userspace proxy first — covers 90% without cert. |
| MV3 rule cap on Chrome | Medium | Curate top 25K rules by traffic. Full list in Firefox MV2. Document the gap publicly. |
| P2P network poisoning | Medium | Ed25519 signing with trusted key holders. Bootstrap from hardcoded nodes. |
| AI model false positives | **High** | Off by default. >92% precision threshold enforced by automated test before any model ships. |
| Blocklist stale at launch | Medium | Merge from 4 actively maintained upstream sources. Update weekly via CI. |
| Battery drain complaints on iOS | **High** | Profile before launch. Publish benchmarks. Set expectations in App Store listing. |
| No support infrastructure | Medium | Set up a public GitHub Discussions board before launch. This is your support channel. |
| Competitor copies architecture | Low | First-mover + open-source community + trust = durable moat. Open source is the defense. |
| App store removal for ad blocking | Low | Ad blocking is explicitly permitted on Google Play and App Store. Has precedent. |

---

## Distribution Strategy

**Phase 1 Launch (Android MVP):**

No marketing budget. Privacy tools spread via trust networks.

Week 1 checklist:
- [ ] GitHub repo public with full README, benchmarks vs AdGuard/NextDNS/Pi-hole
- [ ] Product Hunt launch (Tuesday — highest traffic day)
- [ ] r/privacy, r/androidapps, r/degoogle, r/uBlockOrigin posts
- [ ] Hacker News Show HN
- [ ] AlternativeTo.net listing (AdGuard, NextDNS, Pi-hole alternatives)
- [ ] Email 5–10 privacy YouTubers: Techlore, The Hated One, Rob Braxman, Side of Burritos — send free review copy
- [ ] F-Droid submission (open-source Android app store — massive privacy community reach)

**F-Droid is non-negotiable for a privacy tool.**
The privacy community heavily uses F-Droid. Not being there is a credibility gap.
F-Droid requires reproducible builds and no proprietary dependencies — design for this from Day 1.

**Realistic first-month acquisition estimate:**
- Product Hunt: 200–800 installs (not paid users — installs)
- Reddit: 100–500 installs per relevant post
- HN Show HN: 100–1,000 installs depending on traction
- F-Droid: slow start, compounds over 6–12 months
- YouTuber reviews: 500–5,000 installs per video if they cover it

First 500 paid users realistically takes 2–4 months post-launch, not Day 1.
Build this expectation into your runway calculation.

---

## Success Metrics (Non-Negotiable)

These pass as automated tests before any version ships:

| Metric | Target | Test Method |
|--------|--------|-------------|
| DNS block decision latency | <0.5ms | Automated benchmark, 10K queries |
| Battery drain (Android) | <1%/hr | Snapdragon 665 device, 8hr test |
| Battery drain (iOS) | <2%/hr | iPhone 12 or equivalent, 8hr test |
| RAM footprint | <15MB | Memory profiler, steady state |
| App install size | <20MB | CI/CD build output check |
| AI inference latency | <0.1ms x86 / <2ms ARM | ONNX benchmark suite |
| False positive rate | <0.1% on Tranco top 1000 | Automated regression test |
| IPv6 leak test | Zero leaks | ipleak.net equivalent in CI |
| Startup time | <200ms to first block | Instrumented launch benchmark |
| Crash-free rate | >99.5% | Opt-in crash reporter aggregate |

---

## Repository Structure

```
voidblock/
├── CLAUDE.md                    ← This document — master blueprint
├── README.md                    ← Public-facing project overview
├── LICENSE                      ← MIT (core engine) — chosen deliberately
├── .github/
│   └── workflows/
│       ├── ci.yml               ← Build + test on every PR
│       ├── blocklist-update.yml ← Weekly blocklist merge cron
│       └── benchmark.yml        ← Performance regression tests
├── core/
│   ├── dns_engine/              ← Rust Tokio DNS resolver, SQLite blocklist, DoH upstream
│   ├── packet_filter/           ← SNI inspector, URL matcher, cosmetic filter rules
│   └── ai_engine/               ← ONNX runner, feature extractor, model.onnx (8MB)
├── platform/
│   ├── android/                 ← Kotlin VpnService, TUN interface, DNS proxy bridge
│   ├── ios/                     ← Swift NEPacketTunnelProvider, NetworkExtension
│   ├── windows/                 ← C++ userspace proxy, WFP driver (Phase 3), installer
│   ├── macos/                   ← Swift NEFilterDataProvider, App Proxy, menu bar app
│   └── linux/                   ← eBPF XDP/TC filters, Rust controller, install.sh
├── extension/
│   ├── chromium/                ← MV3 manifest, declarativeNetRequest rules, content scripts
│   └── firefox/                 ← MV2 manifest, webRequest blocking, full rule set
├── sync/                        ← libp2p gossip, blocklist delta, Ed25519 verification
├── router/                      ← OpenWRT .ipk, Makefile, LuCI web UI
├── ui/
│   └── desktop/                 ← Tauri + Svelte dashboard
├── blocklists/
│   ├── scripts/
│   │   ├── merge.py             ← Merges upstream sources into SQLite
│   │   └── validate.py          ← False positive check against Tranco top 1000
│   └── README.md                ← Source attribution for all upstream lists
└── scripts/
    ├── build.sh                 ← Unified build entry point
    ├── benchmark.sh             ← Performance benchmarks
    └── ci/                      ← CI/CD helpers
```

---

## License Strategy

- **Core engine** (`core/`): MIT License
  - Reason: Maximizes adoption, community contributions, and trust. The moat is not the code — it is the product quality, platform coverage, and user trust.
- **Platform apps** (`platform/`, `extension/`): Consider GPL-2.0 or a custom non-commercial license
  - Reason: Prevents commercial forks from stripping the open-source label and reselling
- **AI model** (`core/ai_engine/model.onnx`): Apache 2.0 or proprietary depending on training data licensing

Decide on license before the first public commit. Changing license after community adoption is painful.

---

## Phase 1 Definition of Done

Before calling Phase 1 complete, all of the following must be true:

- [ ] APK installable on Android 8–14 without root
- [ ] DNS blocking active within 200ms of VPN enable
- [ ] Battery drain <1%/hr validated on physical hardware (not emulator)
- [ ] RAM <15MB validated on physical hardware
- [ ] Split-tunneling excludes banking apps by default
- [ ] Privacy policy live at stable URL
- [ ] Legal entity registered
- [ ] Play Store listing submitted and approved
- [ ] F-Droid submission in review
- [ ] GitHub repo public with README, benchmarks, and contribution guide
- [ ] Opt-in crash reporting implemented and documented
- [ ] All CI benchmarks passing
- [ ] YouTube blocking absent from APK (confirmed by code review)

---

*VoidBlock — Built lean. Launched correctly. Phase 1 ships in 90 days.*
*Revenue starts on day 91. Every subsequent phase adds coverage, not complexity.*
*The blueprint is the destination. Android is the road. Start. Make it perfect. Ship it.*
