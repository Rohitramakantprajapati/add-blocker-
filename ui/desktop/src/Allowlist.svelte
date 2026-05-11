<script lang="ts">
  export let domains: string[] = [];
  let input = "";

  function addDomain(): void {
    const trimmed = input.trim().toLowerCase();
    if (trimmed.length === 0 || domains.includes(trimmed)) {
      input = "";
      return;
    }
    domains = [...domains, trimmed];
    input = "";
  }

  function removeDomain(index: number): void {
    domains = domains.filter((_, i) => i !== index);
  }
</script>

<section class="panel allowlist">
  <h2>Allowlist</h2>
  <div class="row">
    <input bind:value={input} placeholder="add domain" />
    <button type="button" on:click={addDomain}>Add</button>
  </div>
  <ul>
    {#each domains as domain, index}
      <li>
        {domain}
        <button type="button" class="remove" on:click={() => removeDomain(index)} title="Remove domain">×</button>
      </li>
    {/each}
  </ul>
</section>

<style>
  li {
    display: flex;
    justify-content: space-between;
    align-items: center;
    padding: 0.5rem 0;
  }

  .remove {
    background: none;
    border: none;
    color: #ff4444;
    cursor: pointer;
    font-size: 1.5rem;
    padding: 0;
    margin-left: 1rem;
    line-height: 1;
  }

  .remove:hover {
    color: #ff6666;
  }
</style>
