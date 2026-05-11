# Contributing to VoidBlock

## Where Contributions Are Welcome

| Area | Status | Notes |
|------|--------|-------|
| `core/dns_engine` | ✅ Open | Rust. All PRs welcome. |
| `core/packet_filter` | ✅ Open | Rust + C (eBPF). |
| `core/ai_engine` | ✅ Open | Rust + ONNX. Model retraining on roadmap. |
| `blocklists/` | ✅ Open | False positive reports, source suggestions. |
| `platform/android` | ✅ Open | Kotlin + Rust JNI. |
| `extension/firefox` | ✅ Open | TypeScript MV2. |
| `extension/chromium` | ✅ Open | TypeScript MV3. |
| `platform/ios` | 🗓 Not yet | Planned Phase 6. |
| `platform/windows` | 🗓 Not yet | Planned Phase 3. |

## Before You Open a PR

1. Open an Issue first for any non-trivial change. Discuss the approach before writing code.
2. All PRs must pass CI: `./scripts/build.sh && ./scripts/benchmark.sh`
3. Performance regressions (any benchmark target missed) will not be merged.
4. No new runtime dependencies without discussion. Binary size and RAM targets are hard limits.

## False Positive Reports

If VoidBlock is blocking a legitimate site or app:
1. Open a GitHub Issue with label `false-positive`
2. Include: domain or app name, which blocking layer triggered (DNS / Packet / AI), platform and version
3. Do not include personal URLs or sensitive browsing data

False positive reports feed directly into AI model retraining.

## Blocklist Contributions

Blocklist sources are merged automatically from upstream open-source lists.
If you find a domain that should be blocked and is not in any upstream list:
1. Check if it is already in Steven Black, EasyList, EasyPrivacy, or AdGuard DNS filter
2. If not, open a GitHub Issue with label `blocklist-addition` and the domain + evidence it serves ads/trackers

## Code Style

- Rust: `rustfmt` defaults, `clippy` with no warnings allowed
- Kotlin: ktlint defaults
- TypeScript: ESLint + Prettier defaults
- All languages: no unsafe blocks without a documented justification comment

## Security Issues

Do not open public Issues for security vulnerabilities.
Email: security@[domain TBD before launch]
