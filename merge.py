#!/usr/bin/env python3
"""
VoidBlock Blocklist Merger
Downloads, merges, deduplicates, and compresses upstream blocklist sources.
Output: blocklists/blocklist.db (SQLite + LZ4)
"""

import sqlite3
import lz4.frame
import urllib.request
import hashlib
import os
import sys
import logging
from pathlib import Path

logging.basicConfig(level=logging.INFO, format="%(asctime)s %(levelname)s %(message)s")
log = logging.getLogger(__name__)

SOURCES = [
    {
        "name": "Steven Black unified hosts",
        "url": "https://raw.githubusercontent.com/StevenBlack/hosts/master/hosts",
        "format": "hosts",
        "license": "MIT",
    },
    {
        "name": "EasyList domains",
        "url": "https://easylist.to/easylist/easylist.txt",
        "format": "adblock",
        "license": "CC-BY-SA-3.0",
    },
    {
        "name": "EasyPrivacy",
        "url": "https://easylist.to/easylist/easyprivacy.txt",
        "format": "adblock",
        "license": "CC-BY-SA-3.0",
    },
    {
        "name": "AdGuard DNS filter",
        "url": "https://adguardteam.github.io/AdGuardSDNSFilter/Filters/filter.txt",
        "format": "adblock",
        "license": "GPL-3.0",
    },
]

OUTPUT_DIR = Path(__file__).parent.parent
DB_PATH = OUTPUT_DIR / "blocklist.db"
COMPRESSED_PATH = OUTPUT_DIR / "blocklist.db.lz4"


def download(url: str) -> list[str]:
    log.info(f"Downloading {url}")
    with urllib.request.urlopen(url, timeout=30) as resp:
        return resp.read().decode("utf-8", errors="ignore").splitlines()


def parse_hosts(lines: list[str]) -> set[str]:
    domains = set()
    for line in lines:
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        parts = line.split()
        # hosts format: 0.0.0.0 example.com or 127.0.0.1 example.com
        if len(parts) >= 2 and parts[0] in ("0.0.0.0", "127.0.0.1"):
            domain = parts[1].lower()
            if domain not in ("localhost", "broadcasthost", "local"):
                domains.add(domain)
    return domains


def parse_adblock(lines: list[str]) -> set[str]:
    domains = set()
    for line in lines:
        line = line.strip()
        if not line or line.startswith("!") or line.startswith("["):
            continue
        # Domain-only rules: ||example.com^
        if line.startswith("||") and line.endswith("^"):
            domain = line[2:-1].lower()
            # Filter out rules with path components or wildcards
            if "/" not in domain and "*" not in domain and "." in domain:
                domains.add(domain)
    return domains


def build_db(all_domains: set[str]) -> None:
    log.info(f"Building SQLite database with {len(all_domains):,} domains")
    if DB_PATH.exists():
        DB_PATH.unlink()

    conn = sqlite3.connect(DB_PATH)
    conn.execute("PRAGMA journal_mode=WAL")
    conn.execute("PRAGMA synchronous=NORMAL")
    conn.execute("CREATE TABLE blocked (domain TEXT PRIMARY KEY NOT NULL)")
    conn.execute("CREATE INDEX idx_domain ON blocked(domain)")

    # Batch insert for performance
    batch = []
    for domain in all_domains:
        batch.append((domain,))
        if len(batch) >= 10_000:
            conn.executemany("INSERT OR IGNORE INTO blocked VALUES (?)", batch)
            batch.clear()
    if batch:
        conn.executemany("INSERT OR IGNORE INTO blocked VALUES (?)", batch)

    conn.commit()
    conn.close()
    log.info(f"Database written to {DB_PATH}")


def compress() -> None:
    log.info("Compressing with LZ4")
    with open(DB_PATH, "rb") as f_in:
        data = f_in.read()
    with lz4.frame.open(COMPRESSED_PATH, "wb", compression_level=9) as f_out:
        f_out.write(data)
    raw_mb = len(data) / 1_048_576
    compressed_mb = COMPRESSED_PATH.stat().st_size / 1_048_576
    log.info(f"Compressed: {raw_mb:.1f}MB → {compressed_mb:.1f}MB")


def main() -> None:
    all_domains: set[str] = set()

    for source in SOURCES:
        try:
            lines = download(source["url"])
            if source["format"] == "hosts":
                domains = parse_hosts(lines)
            elif source["format"] == "adblock":
                domains = parse_adblock(lines)
            else:
                log.warning(f"Unknown format {source['format']} for {source['name']}")
                continue
            log.info(f"{source['name']}: {len(domains):,} domains")
            all_domains |= domains
        except Exception as e:
            log.error(f"Failed to process {source['name']}: {e}")
            sys.exit(1)

    log.info(f"Total after deduplication: {len(all_domains):,} domains")
    build_db(all_domains)
    compress()

    # Checksum for update verification
    sha256 = hashlib.sha256(COMPRESSED_PATH.read_bytes()).hexdigest()
    (OUTPUT_DIR / "blocklist.db.lz4.sha256").write_text(sha256)
    log.info(f"SHA256: {sha256}")


if __name__ == "__main__":
    main()
