#!/usr/bin/env bash
set -euo pipefail

OBJECT_PATH="/etc/voidblock/xdp_filter.o"
BINARY_PATH="/usr/local/bin/voidblock"
SERVICE_PATH="/etc/systemd/system/voidblock.service"

if [[ $EUID -ne 0 ]]; then
  echo "This install script requires root. Re-run with sudo."
  exit 1
fi

install -d /etc/voidblock

# Build eBPF object if sources exist
if [[ -f ./ebpf/Makefile ]]; then
  (cd ebpf && make install)
fi

# Build Rust controller if cargo is available
if command -v cargo >/dev/null 2>&1; then
  cargo build --release --manifest-path platform/linux/Cargo.toml
  install -m 0755 target/release/voidblock-linux-controller "$BINARY_PATH" || true
fi

# Copy object if present
if [[ -f ./ebpf/xdp_filter.o ]]; then
  cp -f ./ebpf/xdp_filter.o "$OBJECT_PATH"
fi

cat > "$SERVICE_PATH" <<'EOF'
[Unit]
Description=VoidBlock Linux controller
After=network-online.target

[Service]
Type=simple
ExecStart=/usr/local/bin/voidblock
Restart=on-failure

[Install]
WantedBy=multi-user.target
EOF

systemctl daemon-reload || true
systemctl enable voidblock.service || true
echo "Install complete. Ensure /etc/voidblock/xdp_filter.o exists and run 'systemctl start voidblock'"
