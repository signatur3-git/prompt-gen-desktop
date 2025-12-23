<template>
  <div class="marketplace-settings">
    <h3>Marketplace Connection</h3>

    <div v-if="!isAuthenticated" class="connection-section">
      <p class="info-text">
        Connect to the Prompt Gen Marketplace to browse, install, and publish packages.
      </p>

      <button
        @click="handleConnect"
        :disabled="isConnecting"
        class="connect-button"
      >
        <span v-if="isConnecting">Connecting...</span>
        <span v-else>Connect to Marketplace</span>
      </button>

      <div v-if="authError" class="error-message">
        <p>❌ {{ authError }}</p>
        <button @click="clearError" class="clear-error-button">Dismiss</button>
      </div>
    </div>

    <div v-else class="connected-section">
      <div class="user-info">
        <p class="success-message">✅ Connected to Marketplace</p>
        <div v-if="userInfo" class="user-details">
          <p><strong>Username:</strong> {{ userInfo.username }}</p>
          <p><strong>Email:</strong> {{ userInfo.email }}</p>
        </div>
      </div>

      <button @click="handleDisconnect" class="disconnect-button">
        Disconnect
      </button>
    </div>
  </div>
</template>

<script setup lang="ts">
import { useMarketplace } from '../composables/useMarketplace';

const {
  isAuthenticated,
  isConnecting,
  authError,
  userInfo,
  connect,
  disconnect,
  clearError
} = useMarketplace();

async function handleConnect() {
  const success = await connect();
  if (success) {
    // Could emit event or show notification
    console.log('Successfully connected to marketplace');
  }
}

async function handleDisconnect() {
  const confirmed = confirm('Are you sure you want to disconnect from the marketplace?');
  if (confirmed) {
    await disconnect();
  }
}
</script>

<style scoped>
.marketplace-settings {
  padding: 1.5rem;
  border: 1px solid var(--border-color, #e0e0e0);
  border-radius: 8px;
  background-color: var(--background-color, #f9f9f9);
}

h3 {
  margin-top: 0;
  margin-bottom: 1rem;
  color: var(--heading-color, #333);
}

.connection-section,
.connected-section {
  margin-top: 1rem;
}

.info-text {
  margin-bottom: 1rem;
  color: var(--text-color, #666);
  line-height: 1.5;
}

.connect-button,
.disconnect-button {
  padding: 0.75rem 1.5rem;
  font-size: 1rem;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.connect-button {
  background-color: var(--primary-color, #4CAF50);
  color: white;
}

.connect-button:hover:not(:disabled) {
  background-color: var(--primary-hover, #45a049);
}

.connect-button:disabled {
  background-color: var(--disabled-color, #cccccc);
  cursor: not-allowed;
}

.disconnect-button {
  background-color: var(--danger-color, #f44336);
  color: white;
}

.disconnect-button:hover {
  background-color: var(--danger-hover, #da190b);
}

.error-message {
  margin-top: 1rem;
  padding: 1rem;
  background-color: #ffebee;
  border-left: 4px solid #f44336;
  border-radius: 4px;
}

.error-message p {
  margin: 0 0 0.5rem 0;
  color: #c62828;
}

.clear-error-button {
  padding: 0.5rem 1rem;
  font-size: 0.875rem;
  background-color: transparent;
  border: 1px solid #c62828;
  color: #c62828;
  border-radius: 4px;
  cursor: pointer;
}

.clear-error-button:hover {
  background-color: #ffebee;
}

.success-message {
  color: #2e7d32;
  font-weight: 500;
  margin-bottom: 1rem;
}

.user-info {
  margin-bottom: 1rem;
}

.user-details {
  padding: 1rem;
  background-color: white;
  border-radius: 6px;
  margin-top: 0.5rem;
}

.user-details p {
  margin: 0.5rem 0;
  color: var(--text-color, #666);
}

.user-details strong {
  color: var(--heading-color, #333);
}
</style>

