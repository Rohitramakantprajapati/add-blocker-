# platform/android

Kotlin + Rust (JNI) Android application.

## Architecture

- `VpnService` subclass creates a local TUN interface on 127.0.0.1
- All DNS traffic from all apps is routed through the TUN interface
- DNS queries are resolved against `core/dns_engine` via JNI
- HTTPS SNI inspection handled by `core/packet_filter` via JNI
- Split-tunneling: banking apps, VPN apps, and payment apps excluded by default

## Requirements

- Android 8.0 (API 26) minimum
- No root required
- Rust cross-compilation targets: `aarch64-linux-android`, `armv7-linux-androideabi`, `x86_64-linux-android`
- Android NDK r25+

## Build

```bash
# Install Rust Android targets
rustup target add aarch64-linux-android armv7-linux-androideabi x86_64-linux-android

# Build core libraries for Android
cd ../../
cargo build --release --target aarch64-linux-android -p dns-engine -p packet-filter -p ai-engine

# Build APK
cd platform/android
./gradlew assembleRelease
```

## Performance Targets

| Metric | Target | Test |
|--------|--------|------|
| Battery drain | <1%/hr | Physical Snapdragon 665, 8hr |
| RAM | <15MB | Memory profiler, steady state |
| Startup to first block | <200ms | Instrumented launch |
| DNS block decision | <0.5ms | 10K query benchmark |

## Key Classes

- `VoidBlockVpnService.kt` — VpnService implementation, TUN interface management
- `DnsProxy.kt` — DNS query interception and routing to core engine
- `SplitTunnel.kt` — Per-app exclusion management
- `BlocklistManager.kt` — SQLite database lifecycle, update polling
- `NativeBridge.kt` — JNI bridge to Rust core

## Notes

- YouTube ad skip is **not** implemented in this app (Play Store policy §4.8 risk)
- Battery profiling must be run on physical hardware — emulator results are not valid
- IPv6 leak prevention: TUN interface captures both IPv4 and IPv6 DNS traffic
