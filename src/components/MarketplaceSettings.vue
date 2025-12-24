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

    <!-- Package Browser Section (shown when authenticated) -->
    <div v-if="isAuthenticated" class="packages-section">
      <h3>Browse Marketplace Packages</h3>
      <PackageBrowser
        @install="handlePackageInstall"
      />
    </div>
  </div>
</template>

<script setup lang="ts">
import { useMarketplace } from '../composables/useMarketplace';
import PackageBrowser from './PackageBrowser.vue';
import type { MarketplacePackage } from '../services/marketplace-client';

const emit = defineEmits<{
  install: [pkg: MarketplacePackage, version: string];
}>();

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

function handlePackageInstall(pkg: MarketplacePackage, version: string) {
  emit('install', pkg, version);
}
</script>

<style scoped>
.marketplace-settings {
  padding: 1.5rem;
  border: 1px solid var(--border-color, #e0e0e0);
  border-radius: 8px;
  background-color: var(--bg-secondary, #f9f9f9);
  color: var(--text-primary, #333);
}

h3 {
  margin-top: 0;
  margin-bottom: 1rem;
  color: var(--text-primary, #333);
}

.connection-section,
.connected-section {
  margin-top: 1rem;
}

.info-text {
  margin-bottom: 1rem;
  color: var(--text-secondary, #666);
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
  font-weight: 500;
}

.connect-button {
  background-color: var(--button-primary, #4CAF50);
  color: white;
}

.connect-button:hover:not(:disabled) {
  background-color: var(--button-primary-hover, #45a049);
}

.connect-button:disabled {
  background-color: var(--button-disabled, #cccccc);
  cursor: not-allowed;
  opacity: 0.6;
}

.disconnect-button {
  background-color: var(--button-danger, #f44336);
  color: white;
}

.disconnect-button:hover {
  background-color: var(--button-danger-hover, #da190b);
}

.error-message {
  margin-top: 1rem;
  padding: 1rem;
  background-color: var(--error-bg, #ffebee);
  border-left: 4px solid var(--error-border, #f44336);
  border-radius: 4px;
}

.error-message p {
  margin: 0 0 0.5rem 0;
  color: var(--error-text, #c62828);
}

.clear-error-button {
  padding: 0.5rem 1rem;
  font-size: 0.875rem;
  background-color: transparent;
  border: 1px solid var(--error-border, #c62828);
  color: var(--error-text, #c62828);
  border-radius: 4px;
  cursor: pointer;
  transition: background-color 0.2s;
}

.clear-error-button:hover {
  background-color: var(--error-hover-bg, #ffebee);
}

.success-message {
  color: var(--success-text, #2e7d32);
  font-weight: 500;
  margin-bottom: 1rem;
}

.user-info {
  margin-bottom: 1rem;
}

.user-details {
  padding: 1rem;
  background-color: var(--bg-primary, white);
  border: 1px solid var(--border-color, #e0e0e0);
  border-radius: 6px;
  margin-top: 0.5rem;
}

.user-details p {
  margin: 0.5rem 0;
  color: var(--text-secondary, #666);
}

.user-details strong {
  color: var(--text-primary, #333);
}

.packages-section {
  margin-top: 2rem;
  padding-top: 1.5rem;
  border-top: 1px solid var(--border-color, #e0e0e0);
}

.packages-section h3 {
  margin-bottom: 1rem;
}

/* Dark theme support */
@media (prefers-color-scheme: dark) {
  .marketplace-settings {
    --bg-primary: #1a1a1a;
    --bg-secondary: #2a2a2a;
    --text-primary: #e0e0e0;
    --text-secondary: #a0a0a0;
    --border-color: #404040;
    --button-primary: #4CAF50;
    --button-primary-hover: #45a049;
    --button-danger: #f44336;
    --button-danger-hover: #da190b;
    --button-disabled: #4a4a4a;
    --error-bg: #3a1f1f;
    --error-border: #f44336;
    --error-text: #ff8a80;
    --error-hover-bg: #4a2525;
    --success-text: #81c784;

    background-color: var(--bg-secondary);
    color: var(--text-primary);
    border-color: var(--border-color);
  }
}
</style>

