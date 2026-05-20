# 🚀 VoidBlock Phase 1 Launch Guide

**Current Status**: Repository cleaned, health checked, and pushed to GitHub  
**Target**: Make the repository live and launch Android MVP  
**Timeline**: 6-8 weeks to Phase 1 shipping

---

## 📋 Pre-Launch Checklist (Week 1-2)

### 1. ✅ GitHub Repository Setup
- [x] Repository already public at: `https://github.com/Rohitramakantprajapati/add-blocker-`
- [ ] Add repository description: "100% local, zero-server ad blocker with AI"
- [ ] Add topics/tags: `android`, `privacy`, `ad-blocker`, `dns`, `open-source`
- [ ] Pin key files in repo root:
  - [ ] Add star badge to README.md
  - [ ] Add build status badge (GitHub Actions)
  - [ ] Add license badge (already present)
- [ ] Create GitHub Discussions board for community support
- [ ] Enable GitHub Releases (create first release template)

**Action**: Go to your GitHub repo settings and configure:
```
Settings → General → Repository description
Settings → Topics (add 5-6 relevant tags)
Discussions → Enable
```

---

### 2. 🏢 Legal Entity Registration (CRITICAL)
**Timeline**: Start immediately — takes 2-4 weeks

#### Choose Entity Type:
- **Recommended**: LLC (Limited Liability Company)
- **Why**: Protects personal liability, required for Play Store
- **Cost**: $50-300 depending on state
- **Tax**: Self-employed/business tax (separate from personal)

#### Setup Steps:
```
Step 1: Choose jurisdiction
  → Delaware (popular for tech), California, or your state

Step 2: Register LLC
  → Use service: LegalZoom, Stripe Atlas, or state website
  → Required info:
     • Business name: "VoidBlock" or "VoidBlock Inc."
     • Your name and address
     • Business address (can be home)

Step 3: Get EIN (Employer Identification Number)
  → Free from IRS (even if no employees)
  → Apply: https://www.irs.gov/ein

Step 4: Open business bank account
  → Chase, Bank of America, etc.
  → Separate personal and business finances
```

**Why this is critical**: Google Play Store requires legal entity registration before payment processing.

---

### 3. 🔐 Privacy Policy (READY)

Your Privacy Policy is already written in `PRIVACY.md`. Now publish it:

#### Publish to Static URL:

**Option A: GitHub Pages (Recommended)**
```bash
# 1. Create gh-pages branch
git checkout -b gh-pages
git push origin gh-pages

# 2. Rename PRIVACY.md to index.html via GitHub web UI
# Or keep as privacy/index.md and access at:
# https://rohitramakantprajapati.github.io/add-blocker-/PRIVACY.md

# 3. Test the URL works
```

**Option B: GitBook (Free)**
- Import your GitHub repo to GitBook
- GitBook will host at: `https://yourname.gitbook.io/voidblock`
- Professional looking, great for documentation

**Option C: Netlify (Free)**
- Connect GitHub repo
- Deploy automatically
- Custom domain support

**Action Item**: Pick one. Privacy policy must be live at stable URL before Play Store submission.

---

### 4. 🎨 Branding & Visual Assets

Create minimal branding:

#### App Logo (512×512 PNG)
- Simple geometric design (VB monogram or shield icon)
- Use online tool: Looka.com (free tier), Canva, Figma
- Requirements for Play Store: PNG, RGB, no transparency

#### Screenshots for Play Store (5 required)
- Show the VPN enable/disable toggle
- Show blocklist stats (domains blocked)
- Show split-tunneling app exclusion list
- Show privacy guarantee messaging
- Font: Clear, readable on 5" phone screens

#### Feature Graphics (1024×500)
- For Play Store listing header
- Text: "VoidBlock - 100% Local Ad Blocker"
- Style: Minimalist, professional

**Action**: Create these in Week 1-2. Budget: 2-3 hours using free tools.

---

## 🛠️ Development Phase (Week 2-5)

### 1. Build Android APK

```bash
# Prerequisites
# - Android Studio installed
# - Android SDK (API 28-34)
# - Gradle 8.5+
# - Kotlin 2.0+

# Navigate to Android project
cd platform/android

# Build APK (debug)
./gradlew assembleDebug
# Output: app/build/outputs/apk/debug/app-debug.apk

# Build APK (release)
./gradlew assembleRelease
# Output: app/build/outputs/apk/release/app-release.apk

# Note: Release requires signing certificate (see below)
```

### 2. Create Signing Certificate (Required for Release APK)

```bash
# Generate keystore
keytool -genkey -v -keystore voidblock.keystore \
  -keyalg RSA -keysize 2048 -validity 10000 \
  -alias voidblock

# You'll be prompted for:
# - Keystore password
# - First name: VoidBlock
# - Last name: Project
# - Company: VoidBlock Inc.
# - City: [Your City]
# - State/Province: [Your State]
# - Country: [Your Country Code]

# This generates voidblock.keystore file (~2.5 KB)
# KEEP THIS SAFE — you need it for every future APK update
```

### 3. Sign Release APK

```bash
# Locate build tools
# macOS/Linux: ~/Library/Android/sdk/build-tools/34.0.0/apksigner
# Windows: %ANDROID_HOME%\build-tools\34.0.0\apksigner.bat

apksigner sign --ks voidblock.keystore \
  --ks-pass pass:YourKeystorePassword \
  app/build/outputs/apk/release/app-release.apk
```

### 4. Performance Validation (CRITICAL)

**Before Play Store submission**, validate these on REAL hardware:

```bash
# Battery Drain Test (8 hour profile)
# - Device: Snapdragon 665 budget phone (or equivalent)
# - Run: Enable VoidBlock blocking
# - Measure: Battery % drop per hour
# - Target: <1%/hr
# - Tool: Android Battery Historian

# Memory Profiling
# - Device: Same hardware
# - Tool: Android Profiler (Android Studio)
# - Check: Peak memory <15MB, stable <10MB
# - Method: Open Settings, enable all domains, navigate apps

# False Positive Rate Test
# - Visit: Top 1000 sites from Alexa/Tranco
# - Check: Any sites incorrectly blocked?
# - Target: <0.1% false positive rate

# IPv6 Leak Test
# - Visit: https://ipleak.net from VoidBlock-enabled device
# - Verify: No IPv6 addresses leak
# - Verify: DNS queries route through VoidBlock

# Connection Speed Test
# - Download speedtest app
# - Run with VoidBlock enabled
# - Verify: No degradation in speeds
```

### 5. Test on Multiple Android Versions

**Minimum test matrix** (use emulator or device):
```
✅ Android 8.0  (API 26) - Budget phones
✅ Android 10.0 (API 29) - Mid-range
✅ Android 12.0 (API 31) - Common
✅ Android 14.0 (API 34) - Latest

For each:
- Enable VPN permission dialog appears
- DNS blocking starts within 200ms
- No crashes after 30 min of use
- Settings screen responsive
```

---

## 📤 Play Store Submission (Week 5-6)

### 1. Prepare Play Store Account

```
Step 1: Create Google Play Developer Account
  → https://play.google.com/apps/publish
  → Cost: $25 one-time registration fee
  → Requires: Google account, valid payment method

Step 2: Complete Developer Profile
  → Name: VoidBlock or your company name
  → Email: Your business email
  → Phone: Your business phone (required)
  → Address: Your business address
  → Privacy policy URL: Your published GitHub Pages URL
```

### 2. Create App Listing

**Play Store App Details Form**:

```
App Name:
  → VoidBlock

Short Description (50 chars):
  → 100% local ad blocker with zero tracking

Full Description:
  → VoidBlock is a privacy-first ad blocker that works entirely 
    on your device. No servers. No tracking. Blocks ads, trackers, 
    and malware at DNS, packet, and AI layers simultaneously.
    
    Features:
    • 100% local processing (zero cloud dependency)
    • AI-powered pattern detection
    • Splits-tunneling (exclude banking apps)
    • <1%/hour battery drain
    • Open source (MIT license)
    
    Your privacy is our only business model.

Category:
  → Tools (or Productivity)

Content Rating:
  → Select questionnaire → complete form

Screenshots:
  → Upload 5 screenshots (1440×2560 pixels)
  
Privacy Policy Link:
  → Your GitHub Pages URL

```

### 3. Configure Technical Settings

```
Target API Level:
  → 34 (Android 14) or higher required by Google Play

Minimum API Level:
  → 26 (Android 8.0) — your target

Permissions (Explicitly declare):
  ✅ android.permission.BIND_VPN_SERVICE (VPN)
  ✅ android.permission.INTERNET (DNS queries)
  ✅ android.permission.CHANGE_NETWORK_STATE
  ✅ android.permission.NETWORK_STATS (optional stats)
  
  ❌ DO NOT declare: Camera, Microphone, Location, Contacts, SMS
     (These would trigger enhanced review or rejection)

Signing Certificate:
  → Upload your release APK signed with voidblock.keystore
```

### 4. Declare VPN Usage Clearly

**This is CRITICAL** — Google Play has specific VPN policy:

```
In App Store listing, under "About this app" → Data Safety:

Data & Privacy:
  [x] This app does NOT collect personal data
  
Security Practices:
  [x] Data is encrypted in transit
  [x] Data is encrypted at rest
  [x] Deletion of data not available (DNS data is session-only)
  [x] App does not share data with third parties
  
Sensitive Permissions:
  [x] VPN Service — Used to: "Filter network traffic for ads and trackers"
  [x] State of explanation: "VPN is used locally only. No data is transmitted externally."
```

### 5. Submit for Review

```
Step 1: Upload APK
  → Your signed release APK (app-release.apk)

Step 2: Choose Rollout Strategy
  → Option A: Staged rollout (1% → 10% → 50% → 100%)
     Recommended — catch issues before full release
  → Option B: Immediate full release
     Risky — if bugs exist, affects all users immediately

Step 3: Save & Submit for Review
  → Review time: 1-3 hours to a few days
  → May be flagged for "Enhanced Review" (VPN + Permission combo)
  → Enhance review takes 7-14 days

Step 4: Wait for Approval Email
  → If rejected: Read feedback carefully, fix, resubmit
  → Common rejection reasons:
     • YouTube ad blocking in APK (you don't have this ✓)
     • Unclear privacy policy (yours is excellent ✓)
     • Crashes on certain devices (test thoroughly ✓)
```

---

## 🌐 Day 1 Launch Marketing (Week 6-7)

After Play Store approval, execute launch sequence:

### Week 1: Technical Communities

**Monday** (Pick one of these):
- [ ] **Product Hunt**: Post at 12:01 AM Pacific
  - Prepare 1-2 minute demo GIF
  - Write launch story (500 words max)
  - Respond to every comment
  - Target: Top 10 in "Tools" category = 500-2000 installs

- [ ] **Hacker News (Show HN)**: Post Tuesday 10 AM
  - Title: "Show HN: VoidBlock – 100% local ad blocker with on-device AI"
  - Include: GitHub link, demo GIF, benchmarks
  - Target: 100-1000 upvotes on front page = 1000-5000 installs

**Wednesday** (subreddit pushes):
- [ ] r/privacy: Post launch announcement
  - Title: "Launching VoidBlock: fully local ad blocker with no server dependency"
  - Include: Key differentiators vs AdGuard/NextDNS
  - Respond to all technical questions

- [ ] r/androidapps: Post listing
  - Format: "[RELEASE] VoidBlock - Ad blocker with 0% server dependency"

- [ ] r/degoogle: Post
  - Title: "Privacy-first alternative to Google Ads tracking"

- [ ] r/uBlockOrigin: Post (for awareness)
  - Title: "Introducing VoidBlock - system-wide blocking like uBO but for entire phone"

### Week 2: Influencer Outreach

Email privacy YouTubers (they get 100s of pitches, make yours stand out):

```
Subject: Launching VoidBlock - truly serverless ad blocker for Android

Hi [Creator Name],

I've been following your work on [topic you saw in their videos].

I'm launching VoidBlock, an open-source ad blocker that works entirely 
on-device with zero central server — unlike AdGuard or NextDNS.

Key differentiator: If the company disappears, the blocker still works.
No servers. No cloud dependency. Open source.

I'd love for you to review it if you're interested in privacy tools.

GitHub: [your URL]
Demo: [GitHub Pages demo GIF]
Benchmarks: [link to README]

No pressure — I know you get many requests.

Best,
[Your Name]
```

**Target creators** (send emails to 5-10):
- Techlore
- The Hated One
- Rob Braxman
- Restored Empire (privacy focus)
- Side of Burritos (Android focus)

### Week 3: Ongoing Community

- [ ] **F-Droid Submission**: Push your APK
  - F-Droid requires: Reproducible builds, no proprietary dependencies
  - Requires: Full setup (2-3 weeks), but massive privacy community reach

- [ ] **AlternativeTo.net**: Add listing
  - Positioned as: Alternative to AdGuard, NextDNS, Pi-hole
  - Link to GitHub and Play Store

- [ ] **GitHub Trending**: You'll naturally appear if you get enough stars
  - Aim: 500+ stars in Week 1 from organic reach

---

## 📊 Phase 1 Success Metrics

**Track these numbers**:

| Metric | Week 1 Target | Week 2 Target | Week 4 Target |
|--------|---------------|---------------|---------------|
| GitHub Stars | 100 | 300 | 1000 |
| Play Store Installs | 500 | 2000 | 5000 |
| Active Users (DAU) | 100 | 500 | 1500 |
| Reddit Upvotes | 500+ | – | – |
| HN Upvotes | 200+ | – | – |
| Product Hunt | Top 20 | – | – |

**Success Threshold**: 1000+ active users by Week 4 → Proceed to Phase 2

---

## ⚠️ Critical Don'ts

- ❌ **Do NOT ship YouTube ad blocking in Android APK**
  - Google Play Policy §4.8 prohibits interfering with Google's ads
  - Browser extensions are exempt (YouTube skip in Firefox/Chrome only)

- ❌ **Do NOT collect analytics or telemetry by default**
  - Your privacy policy says you don't collect data
  - Breaking this promise = reputation death for a privacy tool

- ❌ **Do NOT use Firebase Crashlytics** (Google telemetry)
  - Use Sentry self-hosted or privacy-respecting alternative
  - Or disable crash reporting by default

- ❌ **Do NOT submit to Play Store without testing on real hardware**
  - False positive rate validation is non-negotiable
  - Battery drain testing is non-negotiable

- ❌ **Do NOT change your privacy policy without legal review**
  - You're a privacy tool — trust is everything

---

## 🎯 Phase 1 Launch Checklist (Revisited)

Before Day 1:
- [ ] Android APK builds without errors
- [ ] Tested on Android 8-14 (at least 2 devices)
- [ ] Battery drain validated <1%/hr
- [ ] Memory footprint validated <15MB
- [ ] False positive rate <0.1% on Tranco 1000
- [ ] IPv6 leak test passes (zero leaks)
- [ ] Legal entity registered
- [ ] Privacy policy live at stable URL
- [ ] Play Store app listing complete
- [ ] Play Store approval received
- [ ] GitHub repo public with full documentation
- [ ] All security concerns audited
- [ ] YouTube blocking explicitly removed from APK
- [ ] Crash reporting optional and documented

---

## 📅 Timeline Summary

```
Week 1-2:    Legal setup, privacy policy publication, branding
Week 2-5:    Build APK, validate performance, create assets
Week 5-6:    Play Store submission and review
Week 6-7:    Marketing blitz (Product Hunt, Reddit, HN)
Week 7-8:    Monitor metrics, fix bugs, prepare Phase 2 planning

Month 3:     Phase 1 complete: 1000+ active users, measurable retention
```

---

## 💰 Revenue Model (Phase 1)

**VoidBlock is FREE on Day 1** (build community trust)

### Monetization Options (Future):
1. **Freemium Desktop** (Phase 3+): Windows/Mac pro version ($2-5)
2. **Optional Donations**: GitHub Sponsors, Patreon
3. **B2B Licensing**: Router/ISP partnerships (Phase 5+)
4. **No subscription**: All pricing is one-time

**Philosophy**: Privacy tool should not require recurring payments to work.

---

## 🚀 Next Steps (Do This Today)

1. [ ] Run `cargo build --release` on Linux machine to verify compilation
2. [ ] Set up GitHub Discussions for community support
3. [ ] Register LLC (start paperwork today)
4. [ ] Publish privacy policy to GitHub Pages
5. [ ] Create app icon + 5 screenshots
6. [ ] Set up Google Play Developer Account
7. [ ] Build and test Android APK
8. [ ] Prepare Product Hunt launch post

---

**You're 90% ready. The last 10% is execution. Ship it.** 🚀

*Phase 1 ships in 90 days. Revenue starts on day 91.*

