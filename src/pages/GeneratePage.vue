// Generate Page - Multi-package prompt generation

<template>
  <div class="generate-page">
    <header class="page-header">
      <button @click="$router.push('/')" class="back-button">
        ← Back to Editor
      </button>
      <h1>⚡ Generate Prompts</h1>
    </header>

    <main class="page-content">
      <div class="generate-layout">
        <!-- Left: Package/Rulebook Selection -->
        <aside class="rulebook-selector">
          <h2>Select Rulebook</h2>

          <div v-if="loading" class="loading-state">
            <div class="spinner"></div>
            <p>Loading packages...</p>
          </div>

          <div v-else-if="error" class="error-state">
            <p class="error-message">❌ {{ error }}</p>
            <button @click="loadPackages" class="btn-secondary">Retry</button>
          </div>

          <div v-else-if="packages.length === 0" class="empty-state">
            <p>No packages in library</p>
            <button @click="$router.push('/marketplace')" class="btn-primary">
              Browse Marketplace
            </button>
          </div>

          <div v-else class="packages-list">
            <div
              v-for="pkg in packages"
              :key="pkg.id"
              class="package-item"
              :class="{ expanded: expandedPackages.has(pkg.id) }"
            >
              <div class="package-header" @click="togglePackage(pkg.id)">
                <span class="expand-icon">{{ expandedPackages.has(pkg.id) ? '▼' : '▶' }}</span>
                <span class="package-name">{{ pkg.metadata.name }}</span>
                <span class="package-version">v{{ pkg.version }}</span>
              </div>

              <div v-if="expandedPackages.has(pkg.id)" class="rulebooks-list">
                <div v-if="!pkg.rulebooks || pkg.rulebooks.length === 0" class="no-rulebooks">
                  No rulebooks in this package
                </div>
                <div
                  v-for="rulebook in pkg.rulebooks"
                  :key="`${pkg.id}:${rulebook.namespace}:${rulebook.name}`"
                  class="rulebook-item"
                  :class="{ selected: isRulebookSelected(pkg.id, rulebook.namespace, rulebook.name) }"
                  @click="selectRulebook(pkg, rulebook)"
                >
                  <div class="rulebook-info">
                    <strong>{{ rulebook.name }}</strong>
                    <span class="namespace-badge">{{ rulebook.namespace }}</span>
                  </div>
                  <p v-if="rulebook.description" class="rulebook-description">
                    {{ rulebook.description }}
                  </p>
                </div>
              </div>
            </div>
          </div>
        </aside>

        <!-- Right: Generation Interface -->
        <div class="generation-panel">
          <div v-if="!selectedRulebook" class="placeholder">
            <h2>👈 Select a rulebook to begin</h2>
            <p>Choose a rulebook from your installed packages to generate prompts</p>
          </div>

          <div v-else class="generation-active">
            <div class="selected-rulebook-info">
              <h2>{{ selectedRulebook.name }}</h2>
              <p class="package-info">
                from <strong>{{ selectedPackage.metadata.name }}</strong>
                <span class="namespace-badge">{{ selectedRulebook.namespace }}</span>
              </p>
              <p v-if="selectedRulebook.description" class="description">
                {{ selectedRulebook.description }}
              </p>
            </div>

            <!-- Generation Options -->
            <div class="generation-options">
              <h3>Generation Settings</h3>

              <div class="option-group">
                <label for="seed">Seed (optional)</label>
                <input
                  id="seed"
                  v-model="seed"
                  type="number"
                  placeholder="Random seed for reproducibility"
                  class="input-field"
                />
              </div>

              <div class="option-group">
                <label for="count">Number of Prompts</label>
                <input
                  id="count"
                  v-model.number="count"
                  type="number"
                  min="1"
                  max="100"
                  class="input-field"
                />
              </div>

              <div class="option-group" v-if="selectedRulebook.batch_variety">
                <label>
                  <input type="checkbox" v-model="useBatchVariety" />
                  Use Batch Variety
                </label>
                <p class="help-text">Generate varied prompts using different entry points</p>
              </div>

              <!-- Context Variables -->
              <div v-if="contextDefaults && Object.keys(contextDefaults).length > 0" class="context-section">
                <h3>Context Variables</h3>
                <div
                  v-for="(defaultValue, key) in contextDefaults"
                  :key="key"
                  class="option-group"
                >
                  <label :for="`context-${key}`">{{ key }}</label>
                  <input
                    :id="`context-${key}`"
                    v-model="contextOverrides[key]"
                    type="text"
                    :placeholder="`Default: ${defaultValue}`"
                    class="input-field"
                  />
                </div>
              </div>

              <button
                @click="generate"
                :disabled="generating"
                class="btn-generate"
              >
                {{ generating ? 'Generating...' : '⚡ Generate Prompts' }}
              </button>
            </div>

            <!-- Results -->
            <div v-if="results.length > 0" class="results-section">
              <div class="results-header">
                <h3>Generated Prompts ({{ results.length }})</h3>
                <div class="results-actions">
                  <button @click="copyAll" class="btn-secondary btn-sm">
                    📋 Copy All
                  </button>
                  <button @click="exportResults" class="btn-secondary btn-sm">
                    💾 Export
                  </button>
                  <button @click="clearResults" class="btn-secondary btn-sm">
                    🗑️ Clear
                  </button>
                </div>
              </div>

              <div class="results-list">
                <div
                  v-for="(result, index) in results"
                  :key="index"
                  class="result-item"
                >
                  <div class="result-header">
                    <span class="result-number">#{{ index + 1 }}</span>
                    <button @click="copyResult(result)" class="btn-copy">
                      📋 Copy
                    </button>
                  </div>
                  <pre class="result-text">{{ result }}</pre>
                </div>
              </div>
            </div>

            <div v-if="generationError" class="error-message">
              ❌ {{ generationError }}
            </div>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { useRouter } from 'vue-router';
import { invoke } from '@tauri-apps/api/core';
import { loadAllLibraryPackages, type Package } from '../services/package-library.service';

const router = useRouter();

// State
const packages = ref<PackageWithRulebooks[]>([]);
const loading = ref(true);
const error = ref<string | null>(null);
const expandedPackages = ref<Set<string>>(new Set());

const selectedPackage = ref<PackageWithRulebooks | null>(null);
const selectedRulebook = ref<any | null>(null);

const seed = ref<number | null>(null);
const count = ref(5);
const useBatchVariety = ref(false);
const contextOverrides = ref<Record<string, string>>({});

const generating = ref(false);
const results = ref<string[]>([]);
const generationError = ref<string | null>(null);

interface PackageWithRulebooks extends Package {
  rulebooks: Array<{
    namespace: string;
    name: string;
    description?: string;
    batch_variety: boolean;
    context_defaults?: Record<string, any>;
  }>;
}

// Computed
const contextDefaults = computed(() => {
  return selectedRulebook.value?.context_defaults || {};
});

// Methods
onMounted(async () => {
  await loadPackages();
});

async function loadPackages() {
  try {
    loading.value = true;
    error.value = null;

    // Load all packages from library
    const packagesMap = await loadAllLibraryPackages();

    // Extract rulebooks from each package
    packages.value = Object.values(packagesMap).map((pkg) => {
      const rulebooks: any[] = [];

      // Iterate through namespaces to find rulebooks
      for (const [nsId, namespace] of Object.entries(pkg.namespaces)) {
        if (namespace.rulebooks) {
          for (const [rbName, rulebook] of Object.entries(namespace.rulebooks)) {
            rulebooks.push({
              namespace: nsId,
              name: rbName,
              description: rulebook.description,
              batch_variety: rulebook.batch_variety || false,
              context_defaults: rulebook.context_defaults || {},
            });
          }
        }
      }

      return {
        ...pkg,
        rulebooks,
      };
    });

    // Auto-expand first package if there are packages
    if (packages.value.length > 0) {
      expandedPackages.value.add(packages.value[0].id);
    }
  } catch (e) {
    error.value = (e as Error).message;
    console.error('Failed to load packages:', e);
  } finally {
    loading.value = false;
  }
}

function togglePackage(packageId: string) {
  if (expandedPackages.value.has(packageId)) {
    expandedPackages.value.delete(packageId);
  } else {
    expandedPackages.value.add(packageId);
  }
}

function isRulebookSelected(packageId: string, namespace: string, name: string): boolean {
  return (
    selectedPackage.value?.id === packageId &&
    selectedRulebook.value?.namespace === namespace &&
    selectedRulebook.value?.name === name
  );
}

function selectRulebook(pkg: PackageWithRulebooks, rulebook: any) {
  selectedPackage.value = pkg;
  selectedRulebook.value = rulebook;
  contextOverrides.value = {};
  results.value = [];
  generationError.value = null;
}

async function generate() {
  if (!selectedPackage.value || !selectedRulebook.value) return;

  try {
    generating.value = true;
    generationError.value = null;

    // Build context
    const context = { ...contextOverrides.value };

    // Generate prompts
    const generated: string[] = [];
    for (let i = 0; i < count.value; i++) {
      const currentSeed = seed.value !== null ? seed.value + i : Date.now() + i;

      const result = await invoke('render_from_rulebook_with_dependencies', {
        package: selectedPackage.value,
        dependencies: {}, // TODO: Load dependencies if needed
        rulebookRef: `${selectedRulebook.value.namespace}:${selectedRulebook.value.name}`,
        context,
        seed: currentSeed,
      });

      // Parse the result and extract just the output field
      let outputText: string;
      if (typeof result === 'string') {
        try {
          const parsed = JSON.parse(result);
          outputText = parsed.output || result;
        } catch {
          outputText = result;
        }
      } else if (typeof result === 'object' && result !== null && 'output' in result) {
        outputText = (result as any).output;
      } else {
        outputText = String(result);
      }

      generated.push(outputText);
    }

    results.value = generated;
  } catch (e) {
    generationError.value = (e as Error).message;
    console.error('Generation failed:', e);
  } finally {
    generating.value = false;
  }
}

function copyResult(text: string) {
  navigator.clipboard.writeText(text);
  alert('Copied to clipboard!');
}

function copyAll() {
  const allText = results.value.join('\n\n---\n\n');
  navigator.clipboard.writeText(allText);
  alert(`Copied ${results.value.length} prompts to clipboard!`);
}

function exportResults() {
  const allText = results.value.join('\n\n---\n\n');
  const blob = new Blob([allText], { type: 'text/plain' });
  const url = URL.createObjectURL(blob);
  const a = document.createElement('a');
  a.href = url;
  a.download = `generated-prompts-${Date.now()}.txt`;
  a.click();
  URL.revokeObjectURL(url);
}

function clearResults() {
  if (confirm('Clear all generated prompts?')) {
    results.value = [];
    generationError.value = null;
  }
}
</script>

<style scoped>
.generate-page {
  display: flex;
  flex-direction: column;
  height: 100vh;
  background-color: var(--bg-primary);
  color: var(--text-primary);
}

.page-header {
  display: flex;
  align-items: center;
  gap: 1rem;
  padding: 1rem 1.5rem;
  background-color: var(--bg-secondary);
  border-bottom: 1px solid var(--border-color);
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.05);
}

.back-button {
  padding: 0.5rem 1rem;
  background-color: var(--button-bg);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  font-size: 0.95rem;
  cursor: pointer;
  transition: all 0.2s;
}

.back-button:hover {
  background-color: var(--button-hover-bg);
  border-color: var(--border-hover);
}

.page-header h1 {
  margin: 0;
  font-size: 1.5rem;
  font-weight: 600;
}

.page-content {
  flex: 1;
  overflow: hidden;
}

.generate-layout {
  display: flex;
  height: 100%;
}

/* Left Panel - Rulebook Selector */
.rulebook-selector {
  width: 350px;
  background-color: var(--bg-secondary);
  border-right: 1px solid var(--border-color);
  overflow-y: auto;
  padding: 1.5rem;
}

.rulebook-selector h2 {
  margin: 0 0 1rem 0;
  font-size: 1.2rem;
}

.loading-state,
.error-state,
.empty-state {
  display: flex;
  flex-direction: column;
  align-items: center;
  padding: 2rem 1rem;
  text-align: center;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 3px solid var(--border-color);
  border-top-color: var(--accent-color);
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin-bottom: 1rem;
}

@keyframes spin {
  to { transform: rotate(360deg); }
}

.packages-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.package-item {
  background-color: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  overflow: hidden;
}

.package-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.75rem;
  cursor: pointer;
  transition: background-color 0.2s;
}

.package-header:hover {
  background-color: var(--bg-tertiary);
}

.expand-icon {
  font-size: 0.75rem;
  color: var(--text-muted);
}

.package-name {
  flex: 1;
  font-weight: 500;
}

.package-version {
  font-size: 0.85rem;
  color: var(--text-muted);
}

.rulebooks-list {
  padding: 0.5rem;
  background-color: var(--bg-tertiary);
}

.no-rulebooks {
  padding: 1rem;
  text-align: center;
  color: var(--text-muted);
  font-size: 0.9rem;
}

.rulebook-item {
  padding: 0.75rem;
  margin-bottom: 0.5rem;
  background-color: var(--bg-primary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
}

.rulebook-item:hover {
  border-color: var(--accent-color);
}

.rulebook-item.selected {
  border-color: var(--accent-color);
  background-color: var(--bg-secondary);
  box-shadow: 0 0 0 2px var(--accent-color) inset;
}

.rulebook-info {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  margin-bottom: 0.25rem;
}

.namespace-badge {
  padding: 0.125rem 0.5rem;
  background-color: var(--bg-tertiary);
  border-radius: 12px;
  font-size: 0.75rem;
  color: var(--text-muted);
}

.rulebook-description {
  margin: 0.5rem 0 0 0;
  font-size: 0.85rem;
  color: var(--text-secondary);
  line-height: 1.4;
}

/* Right Panel - Generation */
.generation-panel {
  flex: 1;
  overflow-y: auto;
  padding: 2rem;
}

.placeholder {
  text-align: center;
  padding: 4rem 2rem;
}

.placeholder h2 {
  margin-bottom: 1rem;
  color: var(--text-primary);
}

.placeholder p {
  color: var(--text-muted);
}

.selected-rulebook-info {
  margin-bottom: 2rem;
  padding-bottom: 1.5rem;
  border-bottom: 1px solid var(--border-color);
}

.selected-rulebook-info h2 {
  margin: 0 0 0.5rem 0;
}

.package-info {
  margin: 0.5rem 0;
  color: var(--text-secondary);
}

.description {
  margin-top: 0.75rem;
  color: var(--text-secondary);
  line-height: 1.6;
}

.generation-options {
  margin-bottom: 2rem;
}

.generation-options h3 {
  margin: 0 0 1rem 0;
  font-size: 1.1rem;
}

.option-group {
  margin-bottom: 1.5rem;
}

.option-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: 500;
  color: var(--text-primary);
}

.input-field {
  width: 100%;
  max-width: 400px;
  padding: 0.5rem 0.75rem;
  background-color: var(--bg-primary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 0.95rem;
}

.input-field:focus {
  outline: none;
  border-color: var(--accent-color);
}

.help-text {
  margin: 0.5rem 0 0 0;
  font-size: 0.85rem;
  color: var(--text-muted);
}

.context-section {
  margin-top: 2rem;
  padding-top: 1.5rem;
  border-top: 1px solid var(--border-color);
}

.context-section h3 {
  margin: 0 0 1rem 0;
  font-size: 1.1rem;
}

.btn-generate {
  padding: 0.75rem 2rem;
  background-color: var(--accent-color);
  color: white;
  border: none;
  border-radius: 6px;
  font-size: 1rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-generate:hover:not(:disabled) {
  background-color: var(--accent-hover);
}

.btn-generate:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.results-section {
  margin-top: 2rem;
  padding-top: 2rem;
  border-top: 2px solid var(--border-color);
}

.results-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1.5rem;
}

.results-header h3 {
  margin: 0;
}

.results-actions {
  display: flex;
  gap: 0.5rem;
}

.btn-secondary {
  padding: 0.5rem 1rem;
  background-color: var(--button-secondary-bg);
  color: var(--text-primary);
  border: none;
  border-radius: 4px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-secondary:hover {
  background-color: var(--button-secondary-hover);
}

.btn-sm {
  padding: 0.375rem 0.75rem;
  font-size: 0.875rem;
}

.results-list {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.result-item {
  background-color: var(--bg-secondary);
  border: 1px solid var(--border-color);
  border-radius: 6px;
  padding: 1rem;
}

.result-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.75rem;
}

.result-number {
  font-weight: 600;
  color: var(--accent-color);
}

.btn-copy {
  padding: 0.25rem 0.75rem;
  background-color: var(--bg-primary);
  color: var(--text-primary);
  border: 1px solid var(--border-color);
  border-radius: 4px;
  font-size: 0.85rem;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-copy:hover {
  background-color: var(--button-hover-bg);
}

.result-text {
  margin: 0;
  padding: 1rem;
  background-color: var(--bg-primary);
  border-radius: 4px;
  font-family: monospace;
  font-size: 0.9rem;
  line-height: 1.6;
  white-space: pre-wrap;
  word-wrap: break-word;
}

.error-message {
  margin-top: 1rem;
  padding: 1rem;
  background-color: #fee;
  border: 1px solid #fcc;
  border-radius: 4px;
  color: #c33;
}

.btn-primary {
  padding: 0.75rem 1.5rem;
  background-color: var(--accent-color);
  color: white;
  border: none;
  border-radius: 6px;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-primary:hover {
  background-color: var(--accent-hover);
}
</style>

