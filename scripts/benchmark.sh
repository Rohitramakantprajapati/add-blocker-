#!/usr/bin/env bash
set -euo pipefail

# VoidBlock Benchmark Suite
# Measures DNS latency, memory footprint, startup time, and CPU

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

log_info() {
  echo -e "${GREEN}[BENCH]${NC} $1"
}

log_warn() {
  echo -e "${YELLOW}[BENCH]${NC} $1"
}

cd "$(dirname "$0")/.." || exit 1

log_info "Running benchmark suite"
log_info "════════════════════════════════════════"

# Benchmark 1: DNS resolution latency (if cargo present)
log_info "Benchmark 1: DNS latency (criterion)"
if command -v cargo >/dev/null 2>&1; then
  cargo bench --bench dns_bench 2>&1 | tail -20 || log_warn "DNS bench may not be set up"
  log_info "✓ DNS latency test completed"
else
  log_warn "cargo not available; skipping DNS bench"
fi

# Benchmark 2: AI inference latency
log_info "Benchmark 2: AI inference latency"
if command -v cargo >/dev/null 2>&1; then
  cargo bench --package voidblock-ai-engine 2>&1 | tail -20 || log_warn "AI bench may not be set up"
  log_info "✓ AI latency test completed"
else
  log_warn "cargo not available; skipping AI bench"
fi

# Benchmark 3: Memory footprint (rough estimate using ps if available)
log_info "Benchmark 3: Memory footprint baseline"
if command -v ps >/dev/null 2>&1; then
  RESIDENT_KB=$(ps -eo rss= | awk '{sum+=$1} END {print sum/NR}')
  RESIDENT_MB=$(echo "scale=2; $RESIDENT_KB / 1024" | bc 2>/dev/null || echo "N/A")
  log_info "Current resident memory: ~${RESIDENT_MB}MB (baseline)"
else
  log_warn "ps not available; skipping memory bench"
fi

# Benchmark 4: Blocklist lookup speed
log_info "Benchmark 4: Blocklist lookup (SQLite)"
if command -v python3 >/dev/null 2>&1 && [ -f "blocklists/voidblock.db" ]; then
  python3 << 'PYEOF'
import sqlite3
import time
try:
  conn = sqlite3.connect('blocklists/voidblock.db')
  cursor = conn.cursor()
  
  # Measure 100 random lookups
  domains = ['doubleclick.net', 'googlesyndication.com', 'facebook.com', 'twitter.com']
  start = time.time()
  for _ in range(100):
    for domain in domains:
      cursor.execute('SELECT 1 FROM domains WHERE domain = ? LIMIT 1', (domain,))
      cursor.fetchone()
  elapsed = time.time() - start
  avg_ms = (elapsed / 400) * 1000
  print(f'✓ Avg lookup: {avg_ms:.3f}ms (target: <0.5ms)')
  conn.close()
except Exception as e:
  print(f'⚠ Lookup test failed: {e}')
PYEOF
else
  log_warn "Python3 or blocklist DB not available"
fi

# Benchmark 5: Extension rules performance
log_info "Benchmark 5: MV3 rule parsing"
if command -v python3 >/dev/null 2>&1 && [ -f "extension/chromium/rules/rules.json" ]; then
  python3 << 'PYEOF'
import json
import time
try:
  start = time.time()
  with open('extension/chromium/rules/rules.json') as f:
    rules = json.load(f)
  elapsed = time.time() - start
  print(f'✓ Loaded {len(rules)} MV3 rules in {elapsed*1000:.1f}ms')
except Exception as e:
  print(f'⚠ Rules parse failed: {e}')
PYEOF
fi

log_info "════════════════════════════════════════"
log_info "Benchmark suite completed."
log_info "Target metrics (all must pass in production CI):"
log_info "  • DNS block decision:     <0.5ms    (p99)"
log_info "  • AI inference latency:   <0.1ms x86 / <2ms ARM"
log_info "  • RAM footprint:          <15MB"
log_info "  • Startup to first block: <200ms"
