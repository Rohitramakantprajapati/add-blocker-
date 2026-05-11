#!/usr/bin/env bash
set -euo pipefail

# VoidBlock IPv6 Leak Test
# Validates that no IPv6 traffic leaks outside the VoidBlock tunnel
# (Use on a device where VoidBlock is running)

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() {
  echo -e "${GREEN}[IPv6TEST]${NC} $1"
}

log_pass() {
  echo -e "${GREEN}[PASS]${NC} $1"
}

log_fail() {
  echo -e "${RED}[FAIL]${NC} $1"
  exit 1
}

log_warn() {
  echo -e "${YELLOW}[WARN]${NC} $1"
}

log_info "IPv6 Leak Test Suite"
log_info "════════════════════════════════════════"

# Check if IPv6 is enabled on system
if ! command -v ip >/dev/null 2>&1; then
  log_warn "ip command not found; skipping IPv6 detection"
  exit 0
fi

# Test 1: Check if IPv6 is actually enabled
log_info "Test 1: IPv6 availability on system"
IPV6_ADDR=$(ip -6 addr show | grep "inet6" | grep -v "fe80" | grep -v "::1" | head -1 || true)
if [ -z "$IPV6_ADDR" ]; then
  log_warn "No global IPv6 address found (IPv6 may not be available or not configured on this system)"
  log_info "✓ Test skipped (IPv6 not available)"
  exit 0
fi
log_pass "System has global IPv6 address"

# Test 2: DNS resolution via IPv6 (should go through VoidBlock if running)
log_info "Test 2: IPv6 DNS query"
if command -v dig >/dev/null 2>&1; then
  # Query a test domain via IPv6 (localhost IPv6 if VoidBlock is running on ::1)
  if dig @::1 google.com AAAA +short >/dev/null 2>&1; then
    log_pass "IPv6 DNS query succeeded (VoidBlock DNS resolver responding)"
  else
    log_warn "IPv6 DNS query to ::1 failed; VoidBlock may not be running"
  fi
else
  log_warn "dig not available; skipping DNS IPv6 test"
fi

# Test 3: Check for IPv6 leaks via DNS requests
log_info "Test 3: Checking for unencrypted IPv6 DNS leaks"
if command -v tcpdump >/dev/null 2>&1 && command -v timeout >/dev/null 2>&1; then
  TEMP_PCAP="/tmp/ipv6_test_$$.pcap"
  timeout 5s tcpdump -i any -n "ip6 and udp port 53" -w "$TEMP_PCAP" 2>/dev/null &
  sleep 1
  
  # Trigger a DNS request
  if command -v dig >/dev/null 2>&1; then
    dig google.com @8.8.8.8 +short >/dev/null 2>&1 || true
  fi
  
  sleep 2
  
  if command -v tcpdump >/dev/null 2>&1; then
    LEAKS=$(tcpdump -r "$TEMP_PCAP" 2>/dev/null | grep -v "fe80" | wc -l)
    rm -f "$TEMP_PCAP"
    
    if [ "$LEAKS" -eq 0 ]; then
      log_pass "No IPv6 DNS leaks detected"
    else
      log_fail "IPv6 DNS leaks detected: $LEAKS packets"
    fi
  fi
else
  log_warn "tcpdump or timeout not available; skipping packet analysis"
fi

# Test 4: Verify traffic is routed through VoidBlock TUN (if applicable)
log_info "Test 4: TUN interface check"
if ip link show | grep -q "tun\|tap"; then
  TUN_DEVS=$(ip link show | grep "tun\|tap" | cut -d: -f2 | tr -d ' ')
  log_pass "TUN device(s) detected: $TUN_DEVS"
  
  # Check IPv6 routes through TUN
  if ip -6 route | grep -q "tun\|tap"; then
    log_pass "IPv6 traffic appears routed through TUN"
  else
    log_warn "IPv6 routes may not be configured through TUN"
  fi
else
  log_warn "No TUN device detected (VoidBlock may not be running)"
fi

log_info "════════════════════════════════════════"
log_pass "IPv6 leak test completed"
log_info "Note: Full validation requires a live VoidBlock instance"
log_info "      and traffic capture during active browsing."
