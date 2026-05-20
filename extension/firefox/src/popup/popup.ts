import { browser } from "../firefox-api.js";

async function loadStats(): Promise<void> {
  try {
    const response = await browser.runtime.sendMessage({ type: "get-stats" });
    const blocked = Number(response.blocked ?? 0);
    const latencyMs = Number(response.latencyMs ?? 0);
    const stats = document.querySelector<HTMLParagraphElement>("#stats");
    if (stats) {
      stats.textContent = `Blocked: ${blocked} | Latency: ${latencyMs} ms`;
    }
  } catch (error) {
    console.error("VoidBlock Firefox popup failed", error);
  }
}

void loadStats();
