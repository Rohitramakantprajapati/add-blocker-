import { chrome } from "./chrome-api.js";

type StatsPayload = Record<string, unknown>;

async function syncRulesOnInstall(): Promise<void> {
  try {
    const existingRules = await chrome.declarativeNetRequest.getDynamicRules();
    if (existingRules.length === 0) {
      await chrome.declarativeNetRequest.updateDynamicRules({
        addRules: [
          {
            id: 900001,
            priority: 1,
            action: { type: "block" },
            condition: { urlFilter: "||voidblock.invalid^", resourceTypes: ["main_frame"] },
          },
        ],
      });
    }
    await chrome.storage.local.set({ installedAt: Date.now() });
  } catch (error) {
    console.error("VoidBlock install rule sync failed", error);
  }
}

chrome.runtime.onInstalled.addListener((details) => {
  void details;
  void syncRulesOnInstall();
});

chrome.runtime.onMessage.addListener((message, sender, sendResponse) => {
  // Validate sender origin (prevent injection attacks from malicious pages)
  const allowedOrigins = [
    chrome.runtime.getURL("").slice(0, -1), // extension origin
  ];
  
  if (sender.url && !allowedOrigins.some((origin) => sender.url!.startsWith(origin))) {
    console.warn("VoidBlock: rejected message from untrusted origin", sender.url);
    sendResponse({ error: "unauthorized origin" });
    return false;
  }

  if (message.type === "get-stats") {
    void chrome.storage.local.get(["installedAt", "blocked", "latencyMs"]).then((state) => {
      const payload: StatsPayload = {
        installedAt: state.installedAt ?? null,
        blocked: state.blocked ?? 0,
        latencyMs: state.latencyMs ?? 0,
      };
      sendResponse(payload);
    }).catch((error) => {
      console.error("VoidBlock stats lookup failed", error);
      sendResponse({ installedAt: null, blocked: 0, latencyMs: 0 });
    });
    return true; // Keep channel open for async sendResponse
  }
  return false;
});
