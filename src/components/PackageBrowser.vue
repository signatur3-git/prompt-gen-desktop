<template>
  <div class="package-browser">
    <div class="browser-header">
      <h3>📦 Marketplace Package Browser</h3>
      <button @click="$emit('close')" class="btn-close">×</button>
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

    // Auto-close after success
    setTimeout(() => {
      emit('close');
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
  background: #1e1e1e;
  color: #d4d4d4;
  border-radius: 8px;
  overflow: hidden;
}

.browser-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  background: #252526;
  border-bottom: 1px solid #3e3e42;
}

.browser-header h3 {
  margin: 0;
  font-size: 18px;
}

.btn-close {
  background: none;
  border: none;
  color: #d4d4d4;
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
  background: #3e3e42;
}

.search-section {
  display: flex;
  gap: 8px;
  padding: 16px 20px;
  background: #252526;
  border-bottom: 1px solid #3e3e42;
}

.search-input {
  flex: 1;
  padding: 8px 12px;
  background: #3c3c3c;
  border: 1px solid #555;
  border-radius: 4px;
  color: #d4d4d4;
  font-size: 14px;
}

.search-input:focus {
  outline: none;
  border-color: #007acc;
}

.btn-search {
  padding: 8px 16px;
  background: #0e639c;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
}

.btn-search:hover {
  background: #1177bb;
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
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid #3e3e42;
  border-top-color: #007acc;
  border-radius: 50%;
  animation: spin 1s linear infinite;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.btn-retry {
  padding: 8px 16px;
  background: #0e639c;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
}

.package-list {
  flex: 1;
  overflow-y: auto;
  padding: 16px 20px;
}

.package-card {
  padding: 16px;
  margin-bottom: 12px;
  background: #252526;
  border: 1px solid #3e3e42;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
}

.package-card:hover {
  background: #2d2d30;
  border-color: #007acc;
}

.package-card.selected {
  background: #2d2d30;
  border-color: #007acc;
  box-shadow: 0 0 0 1px #007acc;
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
  color: #4ec9b0;
}

.package-author {
  font-size: 12px;
  color: #858585;
}

.package-description {
  margin: 8px 0;
  font-size: 14px;
  color: #cccccc;
}

.package-meta {
  display: flex;
  gap: 16px;
  margin-top: 8px;
  font-size: 12px;
  color: #858585;
}

.package-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 6px;
  margin-top: 8px;
}

.tag {
  padding: 2px 8px;
  background: #3e3e42;
  border-radius: 12px;
  font-size: 11px;
  color: #d4d4d4;
}

.package-details {
  position: fixed;
  right: 0;
  top: 0;
  bottom: 0;
  width: 400px;
  background: #1e1e1e;
  border-left: 1px solid #3e3e42;
  padding: 20px;
  overflow-y: auto;
  box-shadow: -2px 0 8px rgba(0, 0, 0, 0.3);
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
  color: #4ec9b0;
}

.btn-close-small {
  background: none;
  border: none;
  color: #d4d4d4;
  font-size: 20px;
  cursor: pointer;
  padding: 0;
}

.details-description {
  margin-bottom: 24px;
  color: #cccccc;
  line-height: 1.5;
}

.details-section {
  margin-bottom: 24px;
}

.details-section h4 {
  margin: 0 0 8px 0;
  font-size: 14px;
  color: #858585;
}

.version-select {
  width: 100%;
  padding: 8px;
  background: #3c3c3c;
  border: 1px solid #555;
  border-radius: 4px;
  color: #d4d4d4;
  font-size: 14px;
}

.version-empty {
  color: #858585;
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
  background: #0e639c;
  color: white;
}

.btn-install:hover:not(:disabled) {
  background: #1177bb;
}

.btn-install:disabled {
  opacity: 0.5;
  cursor: not-allowed;
}

.btn-preview {
  background: #3e3e42;
  color: #d4d4d4;
}

.btn-preview:hover {
  background: #505050;
}

.success-message,
.error-message {
  padding: 12px;
  border-radius: 4px;
  font-size: 14px;
}

.success-message {
  background: #1e3a1e;
  color: #4ec9b0;
  border: 1px solid #4ec9b0;
}

.error-message {
  background: #3a1e1e;
  color: #f48771;
  border: 1px solid #f48771;
}
</style>

