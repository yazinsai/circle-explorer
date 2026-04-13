<script lang="ts">
  import { onMount } from 'svelte';
  import { invoke } from '@tauri-apps/api/core';
  import type { FileEntry } from '$lib/types';
  import Sunburst from '$lib/components/Sunburst.svelte';
  import Breadcrumb from '$lib/components/Breadcrumb.svelte';
  import InfoPanel from '$lib/components/InfoPanel.svelte';
  import ListView from '$lib/components/ListView.svelte';
  import Toast from '$lib/components/Toast.svelte';
  import FeedbackButton from '$lib/components/FeedbackButton.svelte';

  // App state
  let currentData = $state<FileEntry | null>(null);
  let currentPath = $state('');
  let homePath = $state('');
  let hoveredEntry = $state<FileEntry | null>(null);
  let loading = $state(false);
  let error = $state('');
  let viewMode = $state<'circle' | 'list'>('circle');

  // Toast state
  let toastMessage = $state('');
  let toastVisible = $state(false);
  let toastTimeout: ReturnType<typeof setTimeout> | null = null;

  function showToast(message: string) {
    toastMessage = message;
    toastVisible = true;
    if (toastTimeout) clearTimeout(toastTimeout);
    toastTimeout = setTimeout(() => {
      toastVisible = false;
    }, 2500);
  }

  // Load view preference from localStorage
  onMount(async () => {
    const savedView = localStorage.getItem('circle-explorer-view');
    if (savedView === 'list' || savedView === 'circle') {
      viewMode = savedView;
    }

    try {
      homePath = await invoke<string>('get_home_dir');
      await navigateTo(homePath);
    } catch (e) {
      error = `Failed to initialize: ${e}`;
      await navigateTo('/');
    }
  });

  async function navigateTo(path: string) {
    if (loading) return;
    if (!path || path === currentPath) return;

    loading = true;
    error = '';

    try {
      const data = await invoke<FileEntry>('scan_directory', { path });
      currentData = data;
      currentPath = data.path;
    } catch (e) {
      error = `Failed to scan directory: ${e}`;
      console.error('Navigation error:', e);
    } finally {
      loading = false;
    }
  }

  function handleHover(entry: FileEntry | null, _event?: MouseEvent) {
    hoveredEntry = entry;
  }

  async function handleFileOpen(entry: FileEntry) {
    if (entry.is_dir) return;

    try {
      await invoke<string>('open_file', { path: entry.path });
      showToast(`Opened: ${entry.name}`);
    } catch (e) {
      showToast(`Failed to open: ${entry.name}`);
      console.error('Failed to open file:', e);
    }
  }

  async function handleRevealInFinder(entry: FileEntry) {
    try {
      await invoke<string>('reveal_in_finder', { path: entry.path });
      showToast(`Revealed: ${entry.name}`);
    } catch (e) {
      showToast(`Failed to reveal: ${entry.name}`);
      console.error('Failed to reveal in Finder:', e);
    }
  }

  function toggleView() {
    viewMode = viewMode === 'circle' ? 'list' : 'circle';
    localStorage.setItem('circle-explorer-view', viewMode);
  }
</script>

<div class="app">
  <!-- Top bar: breadcrumb + controls -->
  <header class="top-bar">
    <Breadcrumb path={currentPath} {homePath} onNavigate={navigateTo} />
    <div class="controls">
      <button
        class="view-toggle"
        onclick={toggleView}
        title={viewMode === 'circle' ? 'Switch to list view' : 'Switch to circle view'}
      >
        {#if viewMode === 'circle'}
          <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
            <rect x="1" y="1" width="14" height="3" rx="0.5"/>
            <rect x="1" y="6" width="14" height="3" rx="0.5"/>
            <rect x="1" y="11" width="14" height="3" rx="0.5"/>
          </svg>
          <span>List</span>
        {:else}
          <svg width="16" height="16" viewBox="0 0 16 16" fill="currentColor">
            <circle cx="8" cy="8" r="3"/>
            <circle cx="8" cy="8" r="6" fill="none" stroke="currentColor" stroke-width="1.5"/>
          </svg>
          <span>Circles</span>
        {/if}
      </button>
    </div>
  </header>

  <!-- Main content area -->
  <main class="content">
    {#if loading && !currentData}
      <div class="loading">
        <div class="spinner"></div>
        <span>Scanning directory...</span>
      </div>
    {:else if error && !currentData}
      <div class="error-state">
        <span class="error-icon">!</span>
        <span>{error}</span>
        <button onclick={() => navigateTo(homePath || '/')}>Go Home</button>
      </div>
    {:else if currentData}
      {#if viewMode === 'circle'}
        <Sunburst
          data={currentData}
          onNavigate={navigateTo}
          onHover={handleHover}
          onFileOpen={handleFileOpen}
          onRevealInFinder={handleRevealInFinder}
        />
      {:else}
        <ListView
          data={currentData}
          onNavigate={navigateTo}
          onFileOpen={handleFileOpen}
          onHover={handleHover}
        />
      {/if}
    {/if}

    {#if loading && currentData}
      <div class="loading-overlay">
        <div class="spinner small"></div>
      </div>
    {/if}
  </main>

  <!-- Bottom info panel -->
  <InfoPanel entry={hoveredEntry} currentDir={currentData} />

  <!-- Feedback button -->
  <FeedbackButton />

  <!-- Toast notifications -->
  <Toast message={toastMessage} visible={toastVisible} />
</div>

<style>
  :global(*) {
    margin: 0;
    padding: 0;
    box-sizing: border-box;
  }

  :global(html, body) {
    height: 100%;
    overflow: hidden;
    font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, 'Helvetica Neue', Arial, sans-serif;
    background: #171923;
    color: #e2e8f0;
  }

  .app {
    display: flex;
    flex-direction: column;
    height: 100vh;
    background: #171923;
  }

  .top-bar {
    display: flex;
    align-items: center;
    justify-content: space-between;
    border-bottom: 1px solid #2d3748;
    background: #1a202c;
    padding-right: 12px;
    min-height: 44px;
    -webkit-app-region: drag;
  }

  .top-bar :global(nav) {
    -webkit-app-region: no-drag;
  }

  .controls {
    display: flex;
    align-items: center;
    gap: 8px;
    -webkit-app-region: no-drag;
  }

  .view-toggle {
    display: flex;
    align-items: center;
    gap: 6px;
    background: #2d3748;
    border: 1px solid #4a5568;
    color: #a0aec0;
    padding: 5px 12px;
    border-radius: 6px;
    cursor: pointer;
    font-size: 12px;
    font-family: inherit;
    transition: all 0.15s;
  }

  .view-toggle:hover {
    background: #4a5568;
    color: #e2e8f0;
    border-color: #718096;
  }

  .content {
    flex: 1;
    overflow: hidden;
    position: relative;
    background: #171923;
  }

  .loading {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    gap: 16px;
    color: #718096;
  }

  .loading-overlay {
    position: absolute;
    top: 12px;
    right: 12px;
    z-index: 10;
  }

  .spinner {
    width: 32px;
    height: 32px;
    border: 3px solid #2d3748;
    border-top-color: #63b3ed;
    border-radius: 50%;
    animation: spin 0.8s linear infinite;
  }

  .spinner.small {
    width: 20px;
    height: 20px;
    border-width: 2px;
  }

  @keyframes spin {
    to { transform: rotate(360deg); }
  }

  .error-state {
    display: flex;
    flex-direction: column;
    align-items: center;
    justify-content: center;
    height: 100%;
    gap: 12px;
    color: #fc8181;
  }

  .error-icon {
    width: 40px;
    height: 40px;
    border-radius: 50%;
    background: rgba(252, 129, 129, 0.1);
    border: 2px solid #fc8181;
    display: flex;
    align-items: center;
    justify-content: center;
    font-weight: bold;
    font-size: 20px;
  }

  .error-state button {
    margin-top: 8px;
    background: #2d3748;
    border: 1px solid #4a5568;
    color: #e2e8f0;
    padding: 8px 16px;
    border-radius: 6px;
    cursor: pointer;
    font-family: inherit;
    transition: background 0.15s;
  }

  .error-state button:hover {
    background: #4a5568;
  }
</style>
