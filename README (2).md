# extension/chromium

Chrome / Edge MV3 extension.

## MV3 Constraints — Read Before Coding

MV3 is not feature-equivalent to MV2. The following constraints are hard platform limits:

### 1. Static rule cap: 30,000 rules
- We ship 25,000 rules curated by traffic volume (Tranco/Alexa weighting)
- Rule selection script: `../../blocklists/scripts/curate_mv3_rules.py`
- Full blocklist (5M domains) ships in Firefox MV2 — document this gap in the extension description

### 2. No persistent background pages
- Use Service Workers only
- Service Workers terminate after ~30 seconds of inactivity
- **Never store state in memory that must survive across page loads**
- All persistent state goes to `chrome.storage.local` or `chrome.storage.session`

### 3. Dynamic rules throttled
- Max 5,000 dynamic rules via `updateDynamicRules()`
- Use these for user-added custom rules only
- Do not attempt to load the full blocklist as dynamic rules

### 4. No `webRequest` blocking
- Use `declarativeNetRequest` for URL blocking
- Cosmetic filters and anti-anti-adblock JS: use `scripting.registerContentScripts()`
- Anti-anti-adblock cannot use background page tricks — implement entirely as content scripts

### 5. YouTube ad skip
- YouTube skip automation is implemented here (not in Android APK)
- Uses `scripting.executeScript()` to inject skip logic as content script
- This is a maintenance item: YouTube changes ad patterns weekly

## Structure

```
chromium/
├── manifest.json              MV3 manifest
├── background/
│   └── service-worker.ts      Service Worker (no persistent state in memory)
├── content-scripts/
│   ├── cosmetic.ts            CSS injection, element hiding
│   ├── anti-antiblock.ts      Anti-anti-adblock bypass
│   └── youtube-skip.ts        YouTube ad automation
├── rules/
│   └── rules.json             25K declarativeNetRequest rules (generated)
├── popup/
│   └── popup.html             Extension popup UI
└── scripts/
    └── generate-rules.py      Generates rules.json from blocklist
```

## Build

```bash
npm ci
npm run build        # Outputs to dist/chromium/
npm run lint
npm run test
```

## Generating Rules

```bash
python scripts/generate-rules.py \
  --input ../../blocklists/blocklist.db \
  --output rules/rules.json \
  --max-rules 25000 \
  --strategy traffic-volume
```
