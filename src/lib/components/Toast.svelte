<script lang="ts">
  interface Props {
    message: string;
    visible: boolean;
  }

  let { message, visible }: Props = $props();
</script>

{#if visible}
  <div class="toast" class:show={visible}>
    <span class="toast-icon">\u2713</span>
    <span class="toast-message">{message}</span>
  </div>
{/if}

<style>
  .toast {
    position: fixed;
    bottom: 80px;
    left: 50%;
    transform: translateX(-50%) translateY(20px);
    background: #2d3748;
    color: #e2e8f0;
    padding: 10px 20px;
    border-radius: 8px;
    font-size: 13px;
    box-shadow: 0 4px 20px rgba(0, 0, 0, 0.4);
    opacity: 0;
    transition: opacity 0.3s, transform 0.3s;
    z-index: 100;
    display: flex;
    align-items: center;
    gap: 8px;
    pointer-events: none;
    border: 1px solid #4a5568;
  }

  .toast.show {
    opacity: 1;
    transform: translateX(-50%) translateY(0);
  }

  .toast-icon {
    color: #48bb78;
    font-weight: bold;
  }
</style>
