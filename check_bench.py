#!/usr/bin/env python3
"""
Parse Criterion benchmark output and fail if a target benchmark
exceeds the specified latency threshold (in microseconds).

Usage: check_bench.py <bench_output.txt> <benchmark_name> <threshold_us>
"""

import sys
import re

def main():
    if len(sys.argv) != 4:
        print("Usage: check_bench.py <file> <benchmark_name> <threshold_us>")
        sys.exit(2)

    output_file = sys.argv[1]
    bench_name = sys.argv[2]
    threshold_us = float(sys.argv[3])

    try:
        with open(output_file) as f:
            content = f.read()
    except FileNotFoundError:
        print(f"ERROR: Benchmark output file not found: {output_file}")
        sys.exit(1)

    # Criterion outputs lines like:
    # bench_name              time:   [123.45 ns 124.56 ns 125.67 ns]
    pattern = rf"{re.escape(bench_name)}\s+time:\s+\[[\d.]+ \w+ ([\d.]+) (\w+) [\d.]+ \w+\]"
    match = re.search(pattern, content)

    if not match:
        print(f"ERROR: Benchmark '{bench_name}' not found in output.")
        print("Available benchmarks:")
        for line in content.splitlines():
            if "time:" in line:
                print(f"  {line.strip()}")
        sys.exit(1)

    value = float(match.group(1))
    unit = match.group(2)

    # Normalize to microseconds
    if unit == "ns":
        value_us = value / 1000
    elif unit == "µs" or unit == "us":
        value_us = value
    elif unit == "ms":
        value_us = value * 1000
    else:
        print(f"ERROR: Unknown time unit '{unit}'")
        sys.exit(1)

    print(f"Benchmark '{bench_name}': {value_us:.3f}µs (threshold: {threshold_us}µs)")

    if value_us > threshold_us:
        print(f"FAIL: {value_us:.3f}µs exceeds threshold {threshold_us}µs")
        sys.exit(1)
    else:
        print("PASS")


if __name__ == "__main__":
    main()
