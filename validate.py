#!/usr/bin/env python3
"""
VoidBlock Blocklist Validator
Checks false positive rate against Tranco top 1000 domains.
Fails with exit code 1 if false positive rate exceeds 0.1%.
"""

import sqlite3
import urllib.request
import sys
import logging

logging.basicConfig(level=logging.INFO, format="%(asctime)s %(levelname)s %(message)s")
log = logging.getLogger(__name__)

TRANCO_URL = "https://tranco-list.eu/top-1m.csv.zip"
FALSE_POSITIVE_THRESHOLD = 0.001  # 0.1%
TOP_N = 1000

DB_PATH = "blocklists/blocklist.db"

# Known legitimate domains that appear in some blocklists due to subdomain overlap.
# These are manually reviewed exceptions. Add only with documented justification.
KNOWN_EXCEPTIONS: set[str] = set()


def fetch_tranco_top_n(n: int) -> list[str]:
    import io
    import zipfile

    log.info(f"Fetching Tranco top {n}")
    with urllib.request.urlopen(TRANCO_URL, timeout=30) as resp:
        data = resp.read()
    with zipfile.ZipFile(io.BytesIO(data)) as z:
        with z.open("top-1m.csv") as f:
            lines = f.read().decode().splitlines()

    domains = []
    for line in lines[:n]:
        parts = line.strip().split(",")
        if len(parts) >= 2:
            domains.append(parts[1].lower())
    return domains


def check(domains: list[str]) -> list[str]:
    conn = sqlite3.connect(DB_PATH)
    false_positives = []
    for domain in domains:
        if domain in KNOWN_EXCEPTIONS:
            continue
        row = conn.execute(
            "SELECT 1 FROM blocked WHERE domain = ?", (domain,)
        ).fetchone()
        if row:
            false_positives.append(domain)
    conn.close()
    return false_positives


def main() -> None:
    top_domains = fetch_tranco_top_n(TOP_N)
    false_positives = check(top_domains)

    rate = len(false_positives) / TOP_N
    log.info(f"False positive rate: {rate:.4%} ({len(false_positives)}/{TOP_N})")

    if false_positives:
        log.warning("False positives detected:")
        for d in false_positives:
            log.warning(f"  {d}")

    if rate > FALSE_POSITIVE_THRESHOLD:
        log.error(
            f"False positive rate {rate:.4%} exceeds threshold {FALSE_POSITIVE_THRESHOLD:.4%}"
        )
        log.error("Blocklist update REJECTED. Review and fix before shipping.")
        sys.exit(1)
    else:
        log.info("Validation PASSED. Blocklist is safe to ship.")


if __name__ == "__main__":
    main()
