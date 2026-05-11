#!/usr/bin/env bash
set -euo pipefail

echo "==> VoidBlock benchmarks"
cargo bench --workspace 2>&1 | tee /tmp/bench_output.txt

echo ""
echo "==> Checking latency targets..."
python3 scripts/ci/check_bench.py /tmp/bench_output.txt dns_block_decision_allowed 500
python3 scripts/ci/check_bench.py /tmp/bench_output.txt dns_block_decision_blocked 500

echo ""
echo "==> All benchmarks passed."
