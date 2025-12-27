<template>
  <div class="package-library-browser">
    <div v-if="loading" class="loading-state">
      <div class="spinner"></div>
      <p>Loading library...</p>
    </div>

    <div v-else-if="error" class="error-state">
      <p class="error-message">❌ {{ error }}</p>
    </div>

    <div v-else-if="packages.length === 0" class="empty-state">
      <h2>No Packages in Library</h2>
      <p>Your package library is empty. Install packages from the Marketplace or import your own.</p>
      <div class="empty-actions">
        <button @click="$router.push('/marketplace')" class="btn-primary">
          Browse Marketplace
        </button>
        <button @click="$emit('import')" class="btn-secondary">
          Import Package
        </button>
      </div>
    </div>

    <div v-else class="packages-grid">
      <div
        v-for="pkg in sortedPackages"
        :key="`${pkg.id}@${pkg.version}`"
        class="package-card"
        :class="{ 'marketplace': pkg.source === 'marketplace', 'local': pkg.source === 'local' }"
      >
        <div class="package-header">
          <div class="package-title">
            <h3>{{ pkg.name }}</h3>
            <span class="package-version">v{{ pkg.version }}</span>
          </div>
          <span class="source-badge" :class="pkg.source">
            {{ pkg.source }}
          </span>
        </div>

        <p v-if="pkg.metadata.description" class="package-description">
          {{ pkg.metadata.description }}
        </p>

        <div class="package-meta">
          <div class="meta-item">
            <span class="meta-label">Authors:</span>
            <span class="meta-value">{{ pkg.metadata.authors.join(', ') || 'Unknown' }}</span>
          </div>
          <div class="meta-item">
            <span class="meta-label">Installed:</span>
            <span class="meta-value">{{ formatDate(pkg.installedAt) }}</span>
          </div>
          <div class="meta-item">
            <span class="meta-label">Last Used:</span>
            <span class="meta-value">{{ formatDate(pkg.lastUsed) }}</span>
          </div>
        </div>

        <div v-if="pkg.metadata.tags.length > 0" class="package-tags">
          <span v-for="tag in pkg.metadata.tags" :key="tag" class="tag">
            {{ tag }}
          </span>
        </div>

        <div class="package-actions">
          <button @click="$emit('load', pkg)" class="btn-load">
            📂 Load in Editor
          </button>
          <button @click="$emit('export', pkg)" class="btn-export">
            📤 Export
          </button>
          <button @click="$emit('delete', pkg)" class="btn-delete">
            🗑️ Delete
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed } from 'vue';
import type { LibraryEntry } from '../services/package-library.service';

const props = defineProps<{
  packages: LibraryEntry[];
  loading: boolean;
  error: string | null;
}>();

defineEmits<{
  load: [entry: LibraryEntry];
  delete: [entry: LibraryEntry];
  export: [entry: LibraryEntry];
  import: [];
}>();


const sortedPackages = computed(() => {
  return [...props.packages].sort((a, b) => {
    // Sort by last used (most recent first)
    return b.lastUsed - a.lastUsed;
  });
});

function formatDate(timestamp: number): string {
  const date = new Date(timestamp * 1000);
  const now = new Date();
  const diffMs = now.getTime() - date.getTime();
  const diffDays = Math.floor(diffMs / (1000 * 60 * 60 * 24));

  if (diffDays === 0) return 'Today';
  if (diffDays === 1) return 'Yesterday';
  if (diffDays < 7) return `${diffDays} days ago`;
  if (diffDays < 30) return `${Math.floor(diffDays / 7)} weeks ago`;
  if (diffDays < 365) return `${Math.floor(diffDays / 30)} months ago`;

  return date.toLocaleDateString();
}
</script>

<style scoped>
.package-library-browser {
  min-height: 400px;
}

.loading-state,
.error-state,
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  justify-content: center;
  padding: 4rem 2rem;
  text-align: center;
}

.spinner {
  width: 48px;
  height: 48px;
  border: 4px solid var(--border-color);
  border-top-color: var(--accent-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 1rem;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.error-message {
  color: #f44336;
  font-size: 1.1rem;
}

.empty-state h2 {
  margin-bottom: 0.5rem;
  color: var(--text-primary);
}

.empty-state p {
  color: var(--text-secondary);
  margin-bottom: 2rem;
}

.empty-actions {
  display: flex;
  gap: 1rem;
}

.btn-primary,
.btn-secondary {
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 6px;
  font-size: 1rem;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-primary {
  background-color: var(--accent-color);
  color: white;
}

.btn-primary:hover {
  background-color: var(--accent-hover);
}

.btn-secondary {
  background-color: var(--button-secondary-bg);
  color: var(--text-primary);
}

.btn-secondary:hover {
  background-color: var(--button-secondary-hover);
}

.packages-grid {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(350px, 1fr));
  gap: 1.5rem;
}

.package-card {
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 8px;
  padding: 1.5rem;
  transition: all 0.2s;
}

.package-card:hover {
  border-color: var(--accent-color);
  box-shadow: 0 4px 12px rgba(0, 0, 0, 0.1);
}

.package-header {
  display: flex;
  justify-content: space-between;
  align-items: start;
  margin-bottom: 1rem;
}

.package-title {
  display: flex;
  align-items: baseline;
  gap: 0.5rem;
}

.package-title h3 {
  margin: 0;
  font-size: 1.25rem;
  color: var(--text-primary);
}

.package-version {
  font-size: 0.875rem;
  color: var(--text-muted);
}

.source-badge {
  padding: 0.25rem 0.75rem;
  border-radius: 12px;
  font-size: 0.75rem;
  font-weight: 500;
  text-transform: uppercase;
}

.source-badge.marketplace {
  background-color: #e3f2fd;
  color: #1976d2;
}

.source-badge.local {
  background-color: #f3e5f5;
  color: #7b1fa2;
}

.source-badge.imported {
  background-color: #e8f5e9;
  color: #388e3c;
}

.package-description {
  color: var(--text-secondary);
  margin-bottom: 1rem;
  line-height: 1.5;
}

.package-meta {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  margin-bottom: 1rem;
  padding: 0.75rem;
  background-color: var(--bg-tertiary);
  border-radius: 6px;
}

.meta-item {
  display: flex;
  gap: 0.5rem;
  font-size: 0.875rem;
}

.meta-label {
  color: var(--text-muted);
  font-weight: 500;
  min-width: 80px;
}

.meta-value {
  color: var(--text-secondary);
}

.package-tags {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  margin-bottom: 1rem;
}

.tag {
  padding: 0.25rem 0.75rem;
  background-color: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 12px;
  font-size: 0.75rem;
  color: var(--text-secondary);
}

.package-actions {
  display: flex;
  gap: 0.5rem;
  padding-top: 1rem;
  border-top: 1px solid var(--border-color);
}

.btn-load,
.btn-export,
.btn-delete {
  flex: 1;
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 0.875rem;
  cursor: pointer;
  transition: all 0.2s;
  background-color: var(--bg-primary);
  color: var(--text-primary);
}

.btn-load:hover {
  background-color: var(--accent-color);
  color: white;
  border-color: var(--accent-color);
}

.btn-export:hover {
  background-color: var(--button-secondary-hover);
  border-color: var(--border-hover);
}

.btn-delete:hover {
  background-color: #f44336;
  color: white;
  border-color: #f44336;
}

/* Dark theme adjustments */
@media (prefers-color-scheme: dark) {
  .source-badge.marketplace {
    background-color: #1e3a5f;
    color: #90caf9;
  }

  .source-badge.local {
    background-color: #3a1e4f;
    color: #ce93d8;
  }

  .source-badge.imported {
    background-color: #1e4f2f;
    color: #81c784;
  }
}
</style>

