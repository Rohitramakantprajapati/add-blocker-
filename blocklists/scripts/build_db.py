#!/usr/bin/env python3
"""Build the VoidBlock SQLite blocklist and compressed companion artifact.

The script intentionally works offline using a curated local seed set so the
repository can be bootstrapped without network access.
"""

from __future__ import annotations

import hashlib
import logging
import sqlite3
from pathlib import Path

try:
    import lz4.frame  # type: ignore[import-not-found]
except Exception:  # pragma: no cover - optional dependency
    lz4 = None


logging.basicConfig(level=logging.INFO, format="%(asctime)s %(levelname)s %(message)s")
LOGGER = logging.getLogger("voidblock.build_db")

ROOT = Path(__file__).resolve().parent.parent
DB_PATH = ROOT / "voidblock.db"
COMPRESSED_PATH = ROOT / "voidblock.db.lz4"
SHA256_PATH = ROOT / "voidblock.db.lz4.sha256"

SEED_DOMAINS = [
    "doubleclick.net",
    "googlesyndication.com",
    "googleadservices.com",
    "adservice.google.com",
    "adtrafficquality.google",
    "adsystem.com",
    "adnxs.com",
    "scorecardresearch.com",
    "taboola.com",
    "outbrain.com",
    "criteo.com",
    "rubiconproject.com",
    "openx.net",
    "moatads.com",
    "pubmatic.com",
    "contextweb.com",
    "quantserve.com",
    "socdm.com",
    "zedo.com",
    "bidswitch.net",
    "smartadserver.com",
    "adform.net",
    "casalemedia.com",
    "yieldmo.com",
    "media.net",
    "mgid.com",
    "clickagy.com",
    "adsterra.com",
    "adbull.com",
    "adsafeprotected.com",
]


def expand_domains() -> list[str]:
    domains = set(SEED_DOMAINS)
    for index, seed in enumerate(SEED_DOMAINS):
        domains.add(f"ads{index}.{seed}")
        domains.add(f"track{index}.{seed}")
        domains.add(f"cdn{index}.{seed}")
    return sorted(domains)


def write_database(domains: list[str]) -> None:
    if DB_PATH.exists():
        DB_PATH.unlink()

    connection = sqlite3.connect(DB_PATH)
    try:
        connection.execute("PRAGMA journal_mode=OFF")
        connection.execute("PRAGMA synchronous=OFF")
        connection.execute("CREATE TABLE blocked (domain TEXT PRIMARY KEY NOT NULL, selector TEXT)")
        connection.execute("CREATE INDEX idx_blocked_domain ON blocked(domain)")
        connection.executemany("INSERT OR IGNORE INTO blocked(domain, selector) VALUES (?, NULL)", ((domain,) for domain in domains))
        connection.commit()
    finally:
        connection.close()


def compress_database() -> None:
    raw_bytes = DB_PATH.read_bytes()
    if lz4 is not None:
        with lz4.frame.open(COMPRESSED_PATH, mode="wb", compression_level=16) as compressed:
            compressed.write(raw_bytes)
        digest = hashlib.sha256(COMPRESSED_PATH.read_bytes()).hexdigest()
        SHA256_PATH.write_text(digest, encoding="utf-8")
        LOGGER.info("Compressed blocklist written to %s", COMPRESSED_PATH)
    else:
        COMPRESSED_PATH.write_bytes(raw_bytes)
        digest = hashlib.sha256(raw_bytes).hexdigest()
        SHA256_PATH.write_text(digest, encoding="utf-8")
        LOGGER.info("lz4 unavailable, wrote uncompressed fallback to %s", COMPRESSED_PATH)


def main() -> None:
    domains = expand_domains()
    LOGGER.info("Building blocklist with %d domains", len(domains))
    write_database(domains)
    compress_database()
    LOGGER.info("Blocklist build complete")


if __name__ == "__main__":
    main()
