import { chrome } from "../chrome-api.js";

type Stats = {
  installedAt: number | null;
  blocked: number;
  latencyMs: number;
};

async function renderStats(): Promise<void> {
  try {
    const response = await chrome.runtime.sendMessage({ type: "get-stats" });
    const stats = response as Stats;
    const container = document.getElementById("stats");
    if (!container) {
      return;
    }
    container.textContent = `Blocked: ${stats.blocked}\nLatency: ${stats.latencyMs} ms`;
  } catch (error) {
    console.error("VoidBlock popup render failed", error);
  }
}

document.getElementById("refresh")?.addEventListener("click", () => {
  void renderStats();
});

void renderStats();
