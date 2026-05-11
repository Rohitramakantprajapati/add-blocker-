# extension/firefox

Firefox MV2 extension — full-featured, no rule cap.

## Advantages Over Chromium MV3

- Full `webRequest` blocking API — no rule cap
- Persistent background page — no Service Worker limitations
- Full blocklist (5M domains) — not curated subset
- Cosmetic filter engine has full access to page DOM via background page

## YouTube Ad Skip

YouTube ad automation is maintained here and in the Chromium extension.
YouTube changes its ad delivery mechanism frequently — treat as ongoing maintenance.
Budget: ~2 hrs/week for monitoring and patching.
Community patches via GitHub Issues are the primary source of fixes.

## Structure

```
firefox/
├── manifest.json              MV2 manifest
├── background/
│   └── background.ts          Persistent background page
├── content-scripts/
│   ├── cosmetic.ts
│   ├── anti-antiblock.ts
│   └── youtube-skip.ts
├── popup/
│   └── popup.html
└── scripts/
    └── generate-rules.py      Generates full rule set (no cap)
```

## Build

```bash
npm ci
npm run build        # Outputs to dist/firefox/
npm run lint
npm run test
```
