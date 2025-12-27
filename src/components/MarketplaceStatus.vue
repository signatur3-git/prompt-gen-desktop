<template>
  <div v-if="isAuthenticated" class="marketplace-status">
    <span class="status-led"></span>
    <span class="status-text">Connected</span>
    <button @click="handleDisconnect" class="disconnect-btn" title="Disconnect from marketplace">
      ✕
    </button>
  </div>
</template>

<script setup lang="ts">
import { useMarketplace } from '../composables/useMarketplace';

const { isAuthenticated, disconnect } = useMarketplace();


async function handleDisconnect() {
  const confirmed = confirm('Are you sure you want to disconnect from the marketplace?');
  if (confirmed) {
    await disconnect();
  }
}
</script>

<style scoped>
.marketplace-status {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 6px 12px;
  background: #d4edda; /* Bright green background */
  border: 1px solid #c3e6cb;
  border-radius: 6px;
  font-size: 13px;
}

.status-led {
  display: inline-block;
  width: 8px;
  height: 8px;
  border-radius: 50%;
  background: #28a745; /* Dark green */
  animation: pulse 2s ease-in-out infinite;
}

@keyframes pulse {
  0%, 100% {
    opacity: 1;
    transform: scale(1);
  }
  50% {
    opacity: 0.6;
    transform: scale(0.95);
  }
}

.status-text {
  color: #155724; /* Darker green font */
  font-weight: 500;
  white-space: nowrap;
}

.disconnect-btn {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 20px;
  height: 20px;
  padding: 0;
  background: none;
  border: none;
  border-radius: 4px;
  color: #155724;
  cursor: pointer;
  transition: all 0.2s;
  font-size: 14px;
  line-height: 1;
}

.disconnect-btn:hover {
  background: #c3e6cb;
  color: #0c3d1a;
}
</style>

