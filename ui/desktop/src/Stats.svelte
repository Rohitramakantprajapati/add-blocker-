<script lang="ts">
  export let blocked: number;
  export let latencyMs: number;
  export let memoryMb: number;
</script>

<section class="panel stats">
  <div><strong>{blocked}</strong><span>blocked</span></div>
  <div><strong>{latencyMs} ms</strong><span>latency</span></div>
  <div><strong>{memoryMb} MB</strong><span>memory</span></div>
</section>
