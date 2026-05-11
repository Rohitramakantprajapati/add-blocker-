#!/usr/bin/env python3
"""Refresh blocklist artifacts and optionally sign them."""

from __future__ import annotations

import hashlib
import logging
from pathlib import Path

from build_db import DB_PATH, COMPRESSED_PATH, SHA256_PATH, main as build_blocklist
from generate_mv3_rules import main as generate_rules

logging.basicConfig(level=logging.INFO, format="%(asctime)s %(levelname)s %(message)s")
LOGGER = logging.getLogger("voidblock.update")


def sha256_file(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def main() -> None:
    build_blocklist()
    generate_rules()
    LOGGER.info("Database checksum: %s", sha256_file(COMPRESSED_PATH))
    LOGGER.info("Rules checksum: %s", sha256_file(Path(__file__).resolve().parent.parent.parent / "extension" / "chromium" / "rules" / "rules.json"))
    LOGGER.info("Blocklist update complete")


if __name__ == "__main__":
    main()
