<script lang="ts">
  interface Props {
    path: string;
    homePath: string;
    onNavigate: (path: string) => void;
  }

  let { path, homePath, onNavigate }: Props = $props();

  interface Crumb {
    label: string;
    path: string;
  }

  let crumbs = $derived.by(() => {
    const segments = path.split('/').filter(Boolean);
    const result: Crumb[] = [];

    // Root
    result.push({ label: '/', path: '/' });

    let accumulated = '';
    for (const seg of segments) {
      accumulated += '/' + seg;
      let label = seg;
      // Replace home path with ~
      if (accumulated === homePath) {
        label = '~';
      }
      result.push({ label, path: accumulated });
    }

    return result;
  });
</script>

<nav class="breadcrumb">
  {#each crumbs as crumb, i}
    {#if i > 0}
      <span class="separator">/</span>
    {/if}
    {#if i === crumbs.length - 1}
      <span class="current">{crumb.label}</span>
    {:else}
      <button
        class="crumb"
        onclick={() => onNavigate(crumb.path)}
      >
        {crumb.label}
      </button>
    {/if}
  {/each}
</nav>

<style>
  .breadcrumb {
    display: flex;
    align-items: center;
    gap: 2px;
    padding: 8px 16px;
    font-size: 13px;
    font-family: 'SF Mono', 'Fira Code', 'Cascadia Code', monospace;
    overflow-x: auto;
    white-space: nowrap;
    scrollbar-width: thin;
    min-height: 36px;
  }

  .breadcrumb::-webkit-scrollbar {
    height: 4px;
  }

  .breadcrumb::-webkit-scrollbar-thumb {
    background: #4a5568;
    border-radius: 2px;
  }

  .separator {
    color: #4a5568;
    margin: 0 1px;
    user-select: none;
  }

  .crumb {
    background: none;
    border: none;
    color: #63b3ed;
    cursor: pointer;
    padding: 2px 4px;
    border-radius: 4px;
    font-size: inherit;
    font-family: inherit;
    transition: background 0.15s, color 0.15s;
  }

  .crumb:hover {
    background: rgba(99, 179, 237, 0.1);
    color: #90cdf4;
  }

  .current {
    color: #e2e8f0;
    font-weight: 600;
    padding: 2px 4px;
  }
</style>
