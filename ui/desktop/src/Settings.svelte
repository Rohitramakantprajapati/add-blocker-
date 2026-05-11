<script lang="ts">
  export let splitTunnel = true;
  export let aiEnabled = false;
  export let updateChannel = "stable";
</script>

<section class="panel settings">
  <label><input type="checkbox" bind:checked={splitTunnel} /> Split tunnel</label>
  <label><input type="checkbox" bind:checked={aiEnabled} /> AI blocking</label>
  <label>
    Update channel
    <select bind:value={updateChannel}>
      <option value="stable">Stable</option>
      <option value="beta">Beta</option>
      <option value="nightly">Nightly</option>
    </select>
  </label>
</section>
