<template>
  <div class="package-browser">
    <div class="browser-header">
      <h3>📦 Browse Packages</h3>
    </div>

    <!-- Search Bar -->
    <div class="search-section">
      <input
        v-model="searchQuery"
        @input="debouncedSearch"
        type="text"
        placeholder="Search packages..."
        class="search-input"
      />
      <button @click="handleSearch" class="btn-search">
        🔍 Search
      </button>
    </div>

    <!-- Loading State -->
    <div v-if="isLoading" class="loading-state">
      <div class="spinner"></div>
      <p>Loading packages...</p>
    </div>

    <!-- Error State -->
    <div v-else-if="error" class="error-state">
      <p>❌ {{ error }}</p>
      <button @click="handleSearch" class="btn-retry">Retry</button>
    </div>

    <!-- Package List -->
    <div v-else-if="packages.length > 0" class="package-list">
      <div
        v-for="pkg in packages"
        :key="pkg.id"
        class="package-card"
        @click="selectPackage(pkg)"
        :class="{ selected: selectedPackage?.id === pkg.id }"
      >
        <div class="package-header">
          <h4>{{ pkg.namespace }}/{{ pkg.name }}</h4>
          <span v-if="pkg.author" class="package-author">by {{ pkg.author }}</span>
          <span
            v-else-if="pkg.author_persona_id"
            class="package-author"
            :title="pkg.author_persona_id"
          >
            <template v-if="authorNameByPersonaId[pkg.author_persona_id]">
              by {{ authorNameByPersonaId[pkg.author_persona_id] }}
            </template>
          </span>
        </div>
        <p class="package-description">{{ pkg.description }}</p>
        <div class="package-meta">
          <span class="package-version">
            v{{ pkg.versions?.[0]?.version || '1.0.0' }}
          </span>
          <span class="package-date">
            {{ formatDate(pkg.updated_at) }}
          </span>
        </div>
        <div v-if="pkg.tags && pkg.tags.length > 0" class="package-tags">
          <span v-for="tag in pkg.tags" :key="tag" class="tag">{{ tag }}</span>
        </div>
      </div>
    </div>

    <!-- Empty State -->
    <div v-else class="empty-state">
      <p>No packages found. Try a different search query.</p>
    </div>

    <!-- Package Details Panel -->
    <div v-if="selectedPackage" class="package-details">
      <div class="details-header">
        <h3>{{ selectedPackage.namespace }}/{{ selectedPackage.name }}</h3>
        <button @click="selectedPackage = null" class="btn-close-small">×</button>
      </div>

      <p class="details-description">{{ selectedPackage.description }}</p>

      <div class="details-section">
        <h4>Versions</h4>
        <div v-if="(selectedPackage?.versions || []).length > 0">
          <select v-model="selectedVersion" class="version-select">
            <option
              v-for="version in (selectedPackage?.versions || [])"
              :key="version.version"
              :value="version.version"
            >
              {{ version.version }} - {{ formatDate(version.published_at) }}
            </option>
          </select>
        </div>
        <div v-else class="version-empty">No versions listed (yet).</div>
      </div>

      <div class="details-actions">
        <button
          @click="handleInstall"
          :disabled="isInstalling"
          class="btn-install"
        >
          <span v-if="isInstalling">Installing...</span>
          <span v-else>📥 Install Package</span>
        </button>
        <button @click="handlePreview" class="btn-preview">
          👁️ Preview
        </button>
      </div>

      <div v-if="installSuccess" class="success-message">
        ✅ Package installed successfully!
      </div>
      <div v-if="installError" class="error-message">
        ❌ {{ installError }}
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { onMounted, ref } from 'vue';
import { MarketplaceClient, type MarketplacePackage } from '../services/marketplace-client';

const emit = defineEmits<{
  close: [];
  install: [pkg: MarketplacePackage, version: string];
}>();

const marketplaceClient = new MarketplaceClient();

const searchQuery = ref('');
const packages = ref<MarketplacePackage[]>([]);
const selectedPackage = ref<MarketplacePackage | null>(null);
const selectedVersion = ref('');
const isLoading = ref(false);
const isInstalling = ref(false);
const error = ref<string | null>(null);
const installSuccess = ref(false);
const installError = ref<string | null>(null);
const authorNameByPersonaId = ref<Record<string, string>>({});

let searchTimeout: ReturnType<typeof setTimeout> | null = null;

/**
 * Resolve author persona IDs to intuitive display names
 */
async function hydrateAuthorNames(pkgs: MarketplacePackage[]) {
  // Temporarily disabled: personas endpoint is returning 401 in current environments.
  // We keep the UI working without author resolution until marketplace auth/persona access is clarified.
  void pkgs;
}

/**
 * Search for packages
 */
async function handleSearch() {
  console.log('[marketplace] PackageBrowser.handleSearch start', {
    query: searchQuery.value,
  });

  isLoading.value = true;
  error.value = null;

  try {
    const result = await marketplaceClient.searchPackages(searchQuery.value);
    console.log('[marketplace] PackageBrowser.handleSearch result', {
      count: result.length,
      first: result[0]?.namespace && result[0]?.name ? `${result[0]?.namespace}/${result[0]?.name}` : null,
    });
    packages.value = result;
    // Best-effort: resolve author persona ids to display names.
    hydrateAuthorNames(result).catch(() => {});
  } catch (err) {
    error.value = (err as Error).message;
    console.error('[marketplace] PackageBrowser.handleSearch failed:', err);
  } finally {
    isLoading.value = false;
    console.log('[marketplace] PackageBrowser.handleSearch done', { isLoading: isLoading.value, error: error.value });
  }
}

/**
 * Debounced search
 */
function debouncedSearch() {
  if (searchTimeout) {
    clearTimeout(searchTimeout);
  }
  searchTimeout = setTimeout(() => {
    handleSearch();
  }, 500);
}

/**
 * Select a package
 */
function selectPackage(pkg: MarketplacePackage) {
  selectedPackage.value = pkg;
  selectedVersion.value = pkg.versions?.[0]?.version || '1.0.0';
  installSuccess.value = false;
  installError.value = null;
}

/**
 * Install selected package
 */
async function handleInstall() {
  if (!selectedPackage.value || !selectedVersion.value) {
    return;
  }

  isInstalling.value = true;
  installError.value = null;
  installSuccess.value = false;

  try {
    // Emit install event to parent
    emit('install', selectedPackage.value, selectedVersion.value);
    installSuccess.value = true;

    // Show success for 2 seconds, then clear (parent handles the actual install)
    setTimeout(() => {
      installSuccess.value = false;
      selectedPackage.value = null;
    }, 2000);
  } catch (err) {
    installError.value = (err as Error).message;
    console.error('Install failed:', err);
  } finally {
    isInstalling.value = false;
  }
}

/**
 * Preview package
 */
function handlePreview() {
  // TODO: Implement preview functionality
  alert('Preview feature coming soon!');
}

/**
 * Format date for display
 */
function formatDate(dateString: string): string {
  const date = new Date(dateString);
  return date.toLocaleDateString('en-US', {
    year: 'numeric',
    month: 'short',
    day: 'numeric'
  });
}

// Load initial packages on mount
onMounted(() => {
  handleSearch();
});
</script>

<style scoped>
.package-browser {
  display: flex;
  flex-direction: column;
  height: 600px;
  background: var(--browser-bg, #ffffff);
  color: var(--browser-text, #1a1a1a);
  border-radius: 8px;
  overflow: hidden;
  border: 1px solid var(--browser-border, #e0e0e0);
}

.browser-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  background: var(--browser-header-bg, #f5f5f5);
  border-bottom: 1px solid var(--browser-border, #e0e0e0);
}

.browser-header h3 {
  margin: 0;
  font-size: 18px;
  color: var(--browser-text, #1a1a1a);
}

.btn-close {
  background: none;
  border: none;
  color: var(--browser-text, #1a1a1a);
  font-size: 24px;
  cursor: pointer;
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
}

.btn-close:hover {
  background: var(--browser-hover, #e0e0e0);
}

.search-section {
  display: flex;
  gap: 8px;
  padding: 16px 20px;
  background: var(--browser-header-bg, #f5f5f5);
  border-bottom: 1px solid var(--browser-border, #e0e0e0);
}

.search-input {
  flex: 1;
  padding: 8px 12px;
  background: var(--input-bg, #ffffff);
  border: 1px solid var(--input-border, #c0c0c0);
  border-radius: 4px;
  color: var(--browser-text, #1a1a1a);
  font-size: 14px;
}

.search-input:focus {
  outline: none;
  border-color: var(--accent-color, #007acc);
}

.btn-search {
  padding: 8px 16px;
  background: var(--accent-color, #0e639c);
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
}

.btn-search:hover {
  background: var(--accent-hover, #1177bb);
}

.loading-state,
.error-state,
.empty-state {
  flex: 1;
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  gap: 16px;
  padding: 40px;
  color: var(--browser-text-muted, #666);
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid var(--spinner-bg, #e0e0e0);
  border-top-color: var(--accent-color, #007acc);
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.btn-retry {
  padding: 8px 16px;
  background: var(--accent-color, #0e639c);
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.package-list {
  flex: 1;
  overflow-y: auto;
  padding: 16px 20px;
  background: var(--browser-bg, #ffffff);
}

.package-card {
  padding: 16px;
  margin-bottom: 12px;
  background: var(--card-bg, #f9f9f9);
  border: 1px solid var(--browser-border, #e0e0e0);
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
}

.package-card:hover {
  background: var(--card-hover, #f0f0f0);
  border-color: var(--accent-color, #007acc);
}

.package-card.selected {
  background: var(--card-selected, #e8f4fd);
  border-color: var(--accent-color, #007acc);
  box-shadow: 0 0 0 1px var(--accent-color, #007acc);
}

.package-header {
  display: flex;
  justify-content: space-between;
  align-items: baseline;
  margin-bottom: 8px;
}

.package-header h4 {
  margin: 0;
  font-size: 16px;
  color: var(--package-name-color, #0066cc);
}

.package-author {
  font-size: 12px;
  color: var(--browser-text-muted, #666);
}

.package-description {
  margin: 8px 0;
  font-size: 14px;
  color: var(--browser-text, #1a1a1a);
}

.package-meta {
  display: flex;
  gap: 16px;
  margin-top: 8px;
  font-size: 12px;
  color: var(--browser-text-muted, #666);
}

.package-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-top: 8px;
}

.tag {
  padding: 2px 8px;
  background: var(--tag-bg, #e0e0e0);
  border-radius: 12px;
  font-size: 11px;
  color: var(--browser-text, #1a1a1a);
}

.package-details {
  position: fixed;
  right: 0;
  top: 0;
  bottom: 0;
  width: 400px;
  background: var(--details-bg, #ffffff);
  border-left: 1px solid var(--browser-border, #e0e0e0);
  padding: 20px;
  overflow-y: auto;
  box-shadow: -2px 0 8px rgba(0, 0, 0, 0.1);
}

.details-header {
  display: flex;
  justify-content: space-between;
  align-items: start;
  margin-bottom: 16px;
}

.details-header h3 {
  margin: 0;
  font-size: 18px;
  color: var(--package-name-color, #0066cc);
}

.btn-close-small {
  background: none;
  border: none;
  color: var(--browser-text, #1a1a1a);
  font-size: 20px;
  cursor: pointer;
  padding: 0;
}

.details-description {
  margin-bottom: 24px;
  color: var(--browser-text, #1a1a1a);
  line-height: 1.5;
}

.details-section {
  margin-bottom: 24px;
}

.details-section h4 {
  margin: 0 0 8px 0;
  font-size: 14px;
  color: var(--browser-text-muted, #666);
}

.version-select {
  width: 100%;
  padding: 8px;
  background: var(--input-bg, #ffffff);
  border: 1px solid var(--input-border, #c0c0c0);
  border-radius: 4px;
  color: var(--browser-text, #1a1a1a);
  font-size: 14px;
}

.version-empty {
  color: var(--browser-text-muted, #666);
  font-size: 13px;
}

.details-actions {
  display: flex;
  flex-direction: column;
  gap: 12px;
  margin-bottom: 16px;
}

.btn-install,
.btn-preview {
  padding: 12px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
}

.btn-install {
  background: var(--accent-color, #0e639c);
  color: white;
}

.btn-install:hover:not(:disabled) {
  background: var(--accent-hover, #1177bb);
}

.btn-install:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-preview {
  background: var(--button-secondary, #e0e0e0);
  color: var(--browser-text, #1a1a1a);
}

.btn-preview:hover {
  background: var(--button-secondary-hover, #d0d0d0);
}

.success-message,
.error-message {
  padding: 12px;
  border-radius: 4px;
  font-size: 14px;
}

.success-message {
  background: var(--success-bg, #e8f5e9);
  color: var(--success-text, #2e7d32);
  border: 1px solid var(--success-border, #4caf50);
}

.error-message {
  background: var(--error-bg, #ffebee);
  color: var(--error-text, #c62828);
  border: 1px solid var(--error-border, #f44336);
}

/* Dark theme */
@media (prefers-color-scheme: dark) {
  .package-browser {
    --browser-bg: #1e1e1e;
    --browser-text: #d4d4d4;
    --browser-text-muted: #858585;
    --browser-border: #3e3e42;
    --browser-header-bg: #252526;
    --browser-hover: #3e3e42;
    --input-bg: #3c3c3c;
    --input-border: #555;
    --accent-color: #0e639c;
    --accent-hover: #1177bb;
    --spinner-bg: #3e3e42;
    --card-bg: #252526;
    --card-hover: #2d2d30;
    --card-selected: #2d2d30;
    --package-name-color: #4ec9b0;
    --tag-bg: #3e3e42;
    --details-bg: #1e1e1e;
    --button-secondary: #3e3e42;
    --button-secondary-hover: #505050;
    --success-bg: #1e3a1e;
    --success-text: #4ec9b0;
    --success-border: #4ec9b0;
    --error-bg: #3a1e1e;
    --error-text: #f48771;
    --error-border: #f48771;
  }

  .package-details {
    box-shadow: -2px 0 8px rgba(0, 0, 0, 0.3);
  }
}
</style>

