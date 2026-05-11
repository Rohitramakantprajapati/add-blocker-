import { browser } from "./firefox-api.js";

const blockedHosts = new Set(["doubleclick.net", "googlesyndication.com", "adservice.google.com"]);

browser.webRequest.onBeforeRequest.addListener(
  (details) => {
    try {
      const shouldBlock = [...blockedHosts].some((host) => details.url.includes(host));
      if (shouldBlock) {
        return { cancel: true };
      }
    } catch (error) {
      console.error("VoidBlock Firefox blocking failed", error);
    }
    return undefined;
  },
  { urls: ["<all_urls>"] },
  ["blocking"],
);

browser.runtime.onMessage.addListener((message) => {
  if (message.type === "get-stats") {
    void browser.storage.local.get(["blocked", "latencyMs"]).then((state) => {
      void browser.runtime.sendMessage({
        blocked: state.blocked ?? 0,
        latencyMs: state.latencyMs ?? 0,
      });
    }).catch((error) => {
      console.error("VoidBlock Firefox stats failed", error);
    });
  }
  return undefined;
});
