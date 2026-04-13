<script lang="ts">
  const GITHUB_REPO = 'circle-explorer/circle-explorer';
  const APP_VERSION = '0.1.0';

  function openFeedback() {
    const os = detectOS();
    const url = `https://github.com/${GITHUB_REPO}/issues/new?template=feedback.yml&version=${APP_VERSION}&os=${encodeURIComponent(os)}`;
    // Use window.open as a fallback; in Tauri, this opens the default browser
    window.open(url, '_blank');
  }

  function openGitHub() {
    window.open(`https://github.com/${GITHUB_REPO}`, '_blank');
  }

  function detectOS(): string {
    const ua = navigator.userAgent;
    if (ua.includes('Mac')) return 'macOS';
    if (ua.includes('Linux')) return 'Linux';
    if (ua.includes('Win')) return 'Windows';
    return 'Unknown';
  }

  let expanded = $state(false);
</script>

<div class="feedback-container" class:expanded>
  {#if expanded}
    <div class="feedback-menu">
      <button class="menu-item" onclick={openFeedback}>
        <span class="menu-icon">💬</span>
        <span>Send Feedback</span>
      </button>
      <button class="menu-item" onclick={openGitHub}>
        <span class="menu-icon">⭐</span>
        <span>Star on GitHub</span>
      </button>
    </div>
  {/if}
  <button
    class="feedback-trigger"
    onclick={() => expanded = !expanded}
    title="Feedback & Links"
  >
    {#if expanded}
      <svg width="14" height="14" viewBox="0 0 14 14" fill="currentColor">
        <path d="M1.4 13.3L0.7 12.6L6.3 7L0.7 1.4L1.4 0.7L7 6.3L12.6 0.7L13.3 1.4L7.7 7L13.3 12.6L12.6 13.3L7 7.7L1.4 13.3Z"/>
      </svg>
    {:else}
      <svg width="14" height="14" viewBox="0 0 14 14" fill="currentColor">
        <path d="M7 0C3.1 0 0 2.8 0 6.2c0 1.9 1 3.6 2.5 4.7L2 13.5c-.1.3.1.5.4.4l3.1-1.3c.5.1 1 .1 1.5.1 3.9 0 7-2.8 7-6.2S10.9 0 7 0zm0 11c-.4 0-.9 0-1.3-.1l-.3-.1-2 .8.3-1.6-.3-.2C2.1 8.8 1.3 7.5 1.3 6.2 1.3 3.5 3.8 1.3 7 1.3s5.7 2.2 5.7 4.9S10.2 11 7 11z"/>
      </svg>
    {/if}
  </button>
</div>

<style>
  .feedback-container {
    position: fixed;
    bottom: 76px;
    right: 16px;
    z-index: 50;
    display: flex;
    flex-direction: column;
    align-items: flex-end;
    gap: 8px;
  }

  .feedback-trigger {
    width: 36px;
    height: 36px;
    border-radius: 50%;
    background: #2d3748;
    border: 1px solid #4a5568;
    color: #a0aec0;
    cursor: pointer;
    display: flex;
    align-items: center;
    justify-content: center;
    transition: all 0.2s;
    box-shadow: 0 2px 8px rgba(0, 0, 0, 0.3);
  }

  .feedback-trigger:hover {
    background: #4a5568;
    color: #e2e8f0;
    border-color: #718096;
    transform: scale(1.05);
  }

  .feedback-menu {
    display: flex;
    flex-direction: column;
    gap: 4px;
    background: #2d3748;
    border: 1px solid #4a5568;
    border-radius: 8px;
    padding: 6px;
    box-shadow: 0 4px 16px rgba(0, 0, 0, 0.4);
    animation: slideUp 0.15s ease-out;
  }

  @keyframes slideUp {
    from {
      opacity: 0;
      transform: translateY(8px);
    }
    to {
      opacity: 1;
      transform: translateY(0);
    }
  }

  .menu-item {
    display: flex;
    align-items: center;
    gap: 8px;
    padding: 8px 14px;
    background: none;
    border: none;
    color: #e2e8f0;
    cursor: pointer;
    border-radius: 6px;
    font-size: 13px;
    font-family: inherit;
    white-space: nowrap;
    transition: background 0.1s;
  }

  .menu-item:hover {
    background: rgba(74, 85, 104, 0.4);
  }

  .menu-icon {
    font-size: 14px;
    flex-shrink: 0;
  }
</style>
