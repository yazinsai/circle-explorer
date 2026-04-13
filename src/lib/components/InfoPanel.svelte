<script lang="ts">
  import type { FileEntry } from '$lib/types';
  import { formatSize, formatDate } from '$lib/utils';
  import { getFileColor } from '$lib/colors';

  interface Props {
    entry: FileEntry | null;
    currentDir: FileEntry | null;
  }

  let { entry, currentDir }: Props = $props();

  let displayEntry = $derived(entry || currentDir);

  let fileTypeLabel = $derived.by(() => {
    if (!displayEntry) return '';
    if (displayEntry.is_dir) {
      const total = displayEntry.child_count ?? displayEntry.children?.length ?? 0;
      const shown = displayEntry.children?.length ?? 0;
      const truncated = displayEntry.truncated;
      if (truncated && total > shown) {
        return `Folder \u00B7 ${total.toLocaleString()} items (showing ${shown})`;
      }
      return `Folder \u00B7 ${total} item${total !== 1 ? 's' : ''}`;
    }
    if (displayEntry.extension) {
      return `.${displayEntry.extension.toUpperCase()} file`;
    }
    return 'File';
  });

  let colorDot = $derived(
    displayEntry ? getFileColor(displayEntry.extension, displayEntry.is_dir) : '#4a5568'
  );
</script>

<div class="info-panel">
  {#if displayEntry}
    <div class="info-row">
      <span class="color-dot" style="background: {colorDot}"></span>
      <span class="name" title={displayEntry.name}>{displayEntry.name}</span>
      <span class="type-label">{fileTypeLabel}</span>
    </div>
    <div class="info-row details">
      <span class="detail">
        <span class="detail-label">Size:</span>
        <span class="detail-value">{formatSize(displayEntry.size)}</span>
      </span>
      <span class="detail">
        <span class="detail-label">Modified:</span>
        <span class="detail-value">{formatDate(displayEntry.modified)}</span>
      </span>
      <span class="detail path" title={displayEntry.path}>
        <span class="detail-label">Path:</span>
        <span class="detail-value">{displayEntry.path}</span>
      </span>
    </div>
    {#if !entry}
      <div class="hint">Hover over a segment to see details. Double-click files to open.</div>
    {/if}
  {:else}
    <div class="hint">Loading...</div>
  {/if}
</div>

<style>
  .info-panel {
    padding: 10px 16px;
    border-top: 1px solid #2d3748;
    background: #1a202c;
    font-size: 12px;
    min-height: 60px;
    display: flex;
    flex-direction: column;
    gap: 4px;
  }

  .info-row {
    display: flex;
    align-items: center;
    gap: 8px;
    overflow: hidden;
  }

  .color-dot {
    width: 10px;
    height: 10px;
    border-radius: 50%;
    flex-shrink: 0;
  }

  .name {
    font-weight: 600;
    color: #e2e8f0;
    font-size: 13px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .type-label {
    color: #718096;
    font-size: 11px;
    flex-shrink: 0;
    margin-left: auto;
  }

  .details {
    gap: 16px;
    flex-wrap: wrap;
  }

  .detail {
    display: flex;
    gap: 4px;
    align-items: center;
  }

  .detail-label {
    color: #718096;
  }

  .detail-value {
    color: #a0aec0;
  }

  .detail.path {
    overflow: hidden;
  }

  .detail.path .detail-value {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    font-family: 'SF Mono', 'Fira Code', monospace;
    font-size: 11px;
  }

  .hint {
    color: #4a5568;
    font-style: italic;
    font-size: 11px;
  }
</style>
