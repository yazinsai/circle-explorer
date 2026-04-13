<script lang="ts">
  import type { FileEntry } from '$lib/types';
  import { formatSize, formatDate } from '$lib/utils';
  import { getFileColor } from '$lib/colors';

  interface Props {
    data: FileEntry;
    onNavigate: (path: string) => void;
    onFileOpen: (entry: FileEntry) => void;
    onHover: (entry: FileEntry | null, event?: MouseEvent) => void;
  }

  let { data, onNavigate, onFileOpen, onHover }: Props = $props();

  type SortKey = 'name' | 'size' | 'modified' | 'type';
  type SortDir = 'asc' | 'desc';

  let sortKey = $state<SortKey>('name');
  let sortDir = $state<SortDir>('asc');

  function toggleSort(key: SortKey) {
    if (sortKey === key) {
      sortDir = sortDir === 'asc' ? 'desc' : 'asc';
    } else {
      sortKey = key;
      sortDir = key === 'size' ? 'desc' : 'asc';
    }
  }

  let sortedChildren = $derived.by(() => {
    const items = [...data.children];
    const dir = sortDir === 'asc' ? 1 : -1;

    items.sort((a, b) => {
      // Always keep directories first
      if (a.is_dir !== b.is_dir) return a.is_dir ? -1 : 1;

      switch (sortKey) {
        case 'name':
          return dir * a.name.localeCompare(b.name);
        case 'size':
          return dir * (a.size - b.size);
        case 'modified':
          return dir * ((a.modified || 0) - (b.modified || 0));
        case 'type':
          return dir * (a.extension || '').localeCompare(b.extension || '');
        default:
          return 0;
      }
    });

    return items;
  });

  function getIcon(entry: FileEntry): string {
    if (entry.is_dir) return '\uD83D\uDCC1';
    const ext = entry.extension;
    if (!ext) return '\uD83D\uDCC4';
    if (['png','jpg','jpeg','gif','svg','webp','bmp','ico','tiff'].includes(ext)) return '\uD83D\uDDBC\uFE0F';
    if (['mp3','wav','flac','ogg','aac'].includes(ext)) return '\uD83C\uDFB5';
    if (['mp4','avi','mov','mkv','webm'].includes(ext)) return '\uD83C\uDFAC';
    if (['zip','tar','gz','rar','7z'].includes(ext)) return '\uD83D\uDCE6';
    if (['pdf'].includes(ext)) return '\uD83D\uDCD5';
    if (['md','txt','doc','docx','rtf'].includes(ext)) return '\uD83D\uDCDD';
    if (['js','ts','jsx','tsx','py','rs','go','rb','java','c','cpp','swift','kt','php','html','css','svelte','vue'].includes(ext)) return '\uD83D\uDCBB';
    return '\uD83D\uDCC4';
  }

  function handleClick(entry: FileEntry) {
    if (entry.is_dir) {
      onNavigate(entry.path);
    }
  }

  function handleDblClick(entry: FileEntry) {
    if (!entry.is_dir) {
      onFileOpen(entry);
    }
  }

  function sortArrow(key: SortKey): string {
    if (sortKey !== key) return '';
    return sortDir === 'asc' ? ' \u25B2' : ' \u25BC';
  }
</script>

<div class="list-view">
  <div class="list-header">
    <button class="col col-name" onclick={() => toggleSort('name')}>
      Name{sortArrow('name')}
    </button>
    <button class="col col-size" onclick={() => toggleSort('size')}>
      Size{sortArrow('size')}
    </button>
    <button class="col col-type" onclick={() => toggleSort('type')}>
      Type{sortArrow('type')}
    </button>
    <button class="col col-modified" onclick={() => toggleSort('modified')}>
      Modified{sortArrow('modified')}
    </button>
  </div>

  <div class="list-body">
    <!-- Parent directory row -->
    <button
      class="list-row parent-row"
      onclick={() => {
        const parentPath = data.path.split('/').slice(0, -1).join('/') || '/';
        onNavigate(parentPath);
      }}
    >
      <span class="col col-name">
        <span class="icon">\u2B06\uFE0F</span>
        <span class="filename">..</span>
      </span>
      <span class="col col-size"></span>
      <span class="col col-type">Parent</span>
      <span class="col col-modified"></span>
    </button>

    {#each sortedChildren as entry (entry.path)}
      <button
        class="list-row"
        class:is-dir={entry.is_dir}
        onclick={() => handleClick(entry)}
        ondblclick={() => handleDblClick(entry)}
        onmouseenter={(e: MouseEvent) => onHover(entry, e)}
        onmouseleave={() => onHover(null)}
      >
        <span class="col col-name">
          <span class="icon">{getIcon(entry)}</span>
          <span
            class="color-bar"
            style="background: {getFileColor(entry.extension, entry.is_dir)}"
          ></span>
          <span class="filename" title={entry.name}>{entry.name}</span>
        </span>
        <span class="col col-size">{formatSize(entry.size)}</span>
        <span class="col col-type">
          {entry.is_dir ? 'Folder' : entry.extension ? `.${entry.extension}` : 'File'}
        </span>
        <span class="col col-modified">{formatDate(entry.modified)}</span>
      </button>
    {/each}

    {#if sortedChildren.length === 0}
      <div class="empty-state">
        <span class="empty-icon">📂</span>
        <span>This folder is empty</span>
      </div>
    {/if}

    {#if data.truncated && data.child_count > data.children.length}
      <div class="truncated-notice">
        Showing {data.children.length} of {data.child_count.toLocaleString()} items (largest by size)
      </div>
    {/if}
  </div>
</div>

<style>
  .list-view {
    display: flex;
    flex-direction: column;
    height: 100%;
    overflow: hidden;
    font-size: 13px;
  }

  .list-header {
    display: flex;
    border-bottom: 1px solid #2d3748;
    background: #1a202c;
    position: sticky;
    top: 0;
    z-index: 1;
  }

  .list-header button {
    background: none;
    border: none;
    color: #a0aec0;
    font-size: 11px;
    font-weight: 600;
    text-transform: uppercase;
    letter-spacing: 0.5px;
    padding: 8px 12px;
    cursor: pointer;
    text-align: left;
    transition: color 0.15s;
    font-family: inherit;
  }

  .list-header button:hover {
    color: #e2e8f0;
  }

  .list-body {
    flex: 1;
    overflow-y: auto;
    scrollbar-width: thin;
    scrollbar-color: #4a5568 transparent;
  }

  .list-body::-webkit-scrollbar {
    width: 6px;
  }

  .list-body::-webkit-scrollbar-thumb {
    background: #4a5568;
    border-radius: 3px;
  }

  .list-row {
    display: flex;
    width: 100%;
    background: none;
    border: none;
    border-bottom: 1px solid rgba(45, 55, 72, 0.5);
    color: #e2e8f0;
    cursor: pointer;
    padding: 0;
    text-align: left;
    font-size: inherit;
    font-family: inherit;
    transition: background 0.1s;
  }

  .list-row:hover {
    background: rgba(74, 85, 104, 0.3);
  }

  .list-row.is-dir .filename {
    font-weight: 500;
  }

  .parent-row {
    color: #718096;
  }

  .col {
    padding: 8px 12px;
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
    display: flex;
    align-items: center;
  }

  .col-name {
    flex: 1;
    min-width: 200px;
    gap: 6px;
  }

  .col-size {
    width: 90px;
    flex-shrink: 0;
    color: #a0aec0;
  }

  .col-type {
    width: 80px;
    flex-shrink: 0;
    color: #718096;
    font-size: 12px;
  }

  .col-modified {
    width: 170px;
    flex-shrink: 0;
    color: #718096;
    font-size: 12px;
  }

  .icon {
    flex-shrink: 0;
    font-size: 14px;
  }

  .color-bar {
    width: 3px;
    height: 16px;
    border-radius: 2px;
    flex-shrink: 0;
  }

  .filename {
    overflow: hidden;
    text-overflow: ellipsis;
    white-space: nowrap;
  }

  .empty-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    gap: 8px;
    padding: 48px 16px;
    color: #4a5568;
  }

  .empty-icon {
    font-size: 32px;
  }

  .truncated-notice {
    padding: 10px 16px;
    text-align: center;
    color: #a0aec0;
    font-size: 12px;
    background: rgba(45, 55, 72, 0.5);
    border-top: 1px solid #2d3748;
    font-style: italic;
  }
</style>
