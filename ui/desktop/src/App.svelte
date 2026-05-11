<script lang="ts">
  import { onMount } from "svelte";
  import Stats from "./Stats.svelte";
  import Settings from "./Settings.svelte";
  import Allowlist from "./Allowlist.svelte";

  type StatsSnapshot = {
    blocked: number;
    latencyMs: number;
    memoryMb: number;
  };

  let blocked = 0;
  let latencyMs = 0.23;
  let memoryMb = 12.4;
  let enabled = true;
  let splitTunnel = true;
  let aiEnabled = false;
  let updateChannel = "stable";
  let allowlist: string[] = ["bank.example.com", "payments.example.com"];
  let blockCounter = 0;
  let autoRefreshInterval: number | null = null;
  let isInTauriContext = false;

  // Check if Tauri API is available
  async function checkTauriAvailable(): Promise<boolean> {
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const result = await invoke("ping");
      return result !== undefined;
    } catch {
      return false;
    }
  }

  async function refreshStats(): Promise<void> {
    // In demo mode, always simulate blocking
    if (!isInTauriContext) {
      // Increment blocked count (simulating active blocking)
      if (Math.random() > 0.3) {
        blockCounter += Math.floor(Math.random() * 5) + 1;
        blocked = blockCounter;
      }
      
      // Vary latency slightly (0.15 - 0.45ms)
      latencyMs = parseFloat((0.15 + Math.random() * 0.3).toFixed(2));
      
      // Vary memory slightly
      memoryMb = parseFloat((12 + Math.random() * 2 - 1).toFixed(1));
      
      return;
    }

    // Use Tauri API if available
    try {
      const { invoke } = await import("@tauri-apps/api/core");
      const snapshot = await invoke<StatsSnapshot>("get_stats");
      blocked = snapshot.blocked;
      latencyMs = snapshot.latencyMs;
      memoryMb = snapshot.memoryMb;
    } catch (error) {
      console.warn("Stats refresh failed (demo mode active)", error);
    }
  }

  async function toggleBlocking(): Promise<void> {
    enabled = !enabled;
    
    if (!isInTauriContext) {
      // In demo mode, just toggle the state
      // Stats continue updating in background regardless
      return;
    }

    try {
      const { invoke } = await import("@tauri-apps/api/core");
      await invoke("toggle_blocking", { enabled });
    } catch (error) {
      console.error("VoidBlock toggle failed", error);
      enabled = !enabled;
    }
  }

  onMount(async () => {
    // Check if running in Tauri context
    isInTauriContext = await checkTauriAvailable();
    
    // Initial refresh
    await refreshStats();
    
    // Set up auto-refresh interval
    if (enabled && !autoRefreshInterval) {
      autoRefreshInterval = window.setInterval(() => {
        void refreshStats();
      }, 1500);
    }

    // Cleanup on unmount
    return () => {
      if (autoRefreshInterval) {
        clearInterval(autoRefreshInterval);
      }
    };
  });
</script>

<main class="shell">
  <section class="hero">
    <div>
      <p class="eyebrow">VoidBlock</p>
      <h1>Local blocking with four layers and no server.</h1>
      <p class="lede">DNS, packet, AI, and cosmetic filtering from a single desktop control panel.</p>
    </div>
    <button class:active={enabled} type="button" on:click={toggleBlocking}>
      {enabled ? "Blocking On" : "Blocking Off"}
    </button>
  </section>

  <Stats {blocked} {latencyMs} {memoryMb} />
  <Settings bind:splitTunnel bind:aiEnabled bind:updateChannel />
  <Allowlist bind:domains={allowlist} />

  <footer class="footer">
    <button type="button" on:click={refreshStats}>Refresh stats</button>
    <span>Profiles stay on device.</span>
  </footer>
</main>
