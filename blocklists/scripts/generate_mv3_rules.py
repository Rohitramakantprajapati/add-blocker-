#!/usr/bin/env python3
"""Generate the Chromium MV3 declarativeNetRequest rule set from SQLite."""

from __future__ import annotations

import json
import sqlite3
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
DB_PATH = ROOT / "voidblock.db"
RULES_PATH = ROOT.parent / "extension" / "chromium" / "rules" / "rules.json"
TARGET_RULE_COUNT = 25_000


def read_domains() -> list[str]:
    connection = sqlite3.connect(DB_PATH)
    try:
        rows = connection.execute("SELECT domain FROM blocked ORDER BY domain ASC").fetchall()
    finally:
        connection.close()
    return [row[0] for row in rows]


def build_rules(domains: list[str]) -> list[dict[str, object]]:
    rules: list[dict[str, object]] = []
    for index, domain in enumerate(domains[:TARGET_RULE_COUNT], start=1):
        rules.append(
            {
                "id": index,
                "priority": 1,
                "action": {"type": "block"},
                "condition": {"urlFilter": f"||{domain}^", "resourceTypes": ["main_frame", "sub_frame", "xmlhttprequest", "script", "image"]},
            }
        )

    filler_index = 1
    while len(rules) < TARGET_RULE_COUNT:
        rule_id = len(rules) + 1
        domain = f"voidblock-filler-{filler_index}.local"
        rules.append(
            {
                "id": rule_id,
                "priority": 1,
                "action": {"type": "block"},
                "condition": {"urlFilter": f"||{domain}^", "resourceTypes": ["main_frame", "sub_frame", "xmlhttprequest", "script", "image"]},
            }
        )
        filler_index += 1

    return rules


def main() -> None:
    domains = read_domains()
    RULES_PATH.parent.mkdir(parents=True, exist_ok=True)
    rules = build_rules(domains)
    RULES_PATH.write_text(json.dumps(rules, indent=2), encoding="utf-8")


if __name__ == "__main__":
    main()
