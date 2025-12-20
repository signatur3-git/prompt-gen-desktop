<script setup lang="ts">
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { open } from '@tauri-apps/plugin-dialog'
import RulebookViewer from './RulebookViewer.vue'

interface Package {
  id: string
  version: string
  metadata: {
    name: string
    description?: string
    authors: string[]
    bypass_filters: boolean
  }
  namespaces: Record<string, any>
  dependencies: any[]
}

interface PackageWithDependencies {
  package: Package
  dependencies: Record<string, Package>
}

interface PackageInfo {
  id: string
  version: string
  name: string
  description?: string
  namespace_count: number
  datatype_count: number
  promptsection_count: number
}

const emit = defineEmits<{
  'package-loaded': [pkg: Package]
}>()

const loadedPackage = ref<Package | null>(null)
const loadedDependencies = ref<Record<string, Package>>({})
const packageInfo = ref<PackageInfo | null>(null)
const loading = ref(false)
const error = ref<string | null>(null)
const selectedRulebook = ref<{ namespace: string, id: string } | null>(null)

// Count total rulebooks across all namespaces
const totalRulebooks = computed(() => {
  if (!loadedPackage.value) return 0
  return Object.values(loadedPackage.value.namespaces).reduce((sum, ns: any) => {
    return sum + Object.keys(ns.rulebooks || {}).length
  }, 0)
})

async function loadPackage() {
  try {
    loading.value = true
    error.value = null

    // Open file dialog
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'Package',
        extensions: ['yaml', 'yml', 'json']
      }]
    })

    if (!selected || Array.isArray(selected)) {
      loading.value = false
      return
    }

    // Load package with dependencies via Tauri command
    const result = await invoke<PackageWithDependencies>('load_package_with_dependencies', {
      path: selected,
      searchPaths: ['./packages', './test-packages']
    })

    loadedPackage.value = result.package
    loadedDependencies.value = result.dependencies

    // Get package info
    const info = await invoke<PackageInfo>('get_package_info', { package: result.package })
    packageInfo.value = info

    // Emit loaded package for LivePreview
    emit('package-loaded', result.package)

  } catch (e) {
    error.value = String(e)
    console.error('Failed to load package:', e)
  } finally {
    loading.value = false
  }
}
</script>

<template>
  <div class="package-viewer">
    <div class="controls">
      <button
        @click="loadPackage"
        :disabled="loading"
        class="btn-primary"
      >
        {{ loading ? 'Loading...' : 'Load Package' }}
      </button>
    </div>

    <div v-if="error" class="error-message">
      <h3>Error Loading Package</h3>
      <pre>{{ error }}</pre>
    </div>

    <div v-if="packageInfo" class="package-info">
      <h2>{{ packageInfo.name }}</h2>
      <div class="info-grid">
        <div class="info-item">
          <label>Package ID:</label>
          <span>{{ packageInfo.id }}</span>
        </div>
        <div class="info-item">
          <label>Version:</label>
          <span>{{ packageInfo.version }}</span>
        </div>
        <div class="info-item">
          <label>Description:</label>
          <span>{{ packageInfo.description || 'None' }}</span>
        </div>
        <div class="info-item">
          <label>Namespaces:</label>
          <span>{{ packageInfo.namespace_count }}</span>
        </div>
        <div class="info-item">
          <label>Datatypes:</label>
          <span>{{ packageInfo.datatype_count }}</span>
        </div>
        <div class="info-item">
          <label>Prompt Sections:</label>
          <span>{{ packageInfo.promptsection_count }}</span>
        </div>
        <div class="info-item">
          <label>Rulebooks:</label>
          <span>{{ totalRulebooks }}</span>
        </div>
      </div>
    </div>

    <!-- Dependencies Section -->
    <div v-if="loadedPackage && loadedPackage.dependencies && loadedPackage.dependencies.length > 0" class="dependencies-section">
      <h3>Dependencies ({{ loadedPackage.dependencies.length }})</h3>
      <div class="dependencies-list">
        <div
          v-for="(dep, index) in loadedPackage.dependencies"
          :key="index"
          class="dependency-item"
        >
          <div class="dependency-header">
            <strong>{{ dep.package }}</strong>
            <code class="version-badge">{{ dep.version }}</code>
          </div>
          <div v-if="dep.path" class="dependency-path">
            📁 {{ dep.path }}
          </div>
        </div>
      </div>
    </div>

    <div v-if="loadedPackage" class="package-details">
      <h3>Package Contents</h3>
      <div class="namespaces">
        <div
          v-for="(namespace, nsId) in loadedPackage.namespaces"
          :key="nsId"
          class="namespace-card"
        >
          <h4>{{ nsId }}</h4>
          <div class="namespace-stats">
            <span>{{ Object.keys(namespace.datatypes || {}).length }} datatypes</span>
            <span>{{ Object.keys(namespace.prompt_sections || {}).length }} prompt sections</span>
            <span>{{ Object.keys(namespace.rulebooks || {}).length }} rulebooks</span>
            <span>{{ (namespace.rules || []).length }} rules</span>
            <span>{{ (namespace.decisions || []).length }} decisions</span>
          </div>
        </div>
      </div>
    </div>

    <!-- Rulebooks Section -->
    <div v-if="loadedPackage && totalRulebooks > 0" class="rulebooks-section">
      <h3>Rulebooks ({{ totalRulebooks }})</h3>

      <!-- Rulebook List -->
      <div class="rulebook-list">
        <div
          v-for="(namespace, nsId) in loadedPackage.namespaces"
          :key="nsId"
        >
          <div v-if="Object.keys(namespace.rulebooks || {}).length > 0" class="namespace-group">
            <h4 class="namespace-title">{{ nsId }}</h4>
            <div class="rulebook-items">
              <div
                v-for="(rulebook, rbId) in namespace.rulebooks"
                :key="rbId"
                :class="['rulebook-item', { 'selected': selectedRulebook?.namespace === String(nsId) && selectedRulebook?.id === String(rbId) }]"
                @click="selectedRulebook = { namespace: String(nsId), id: String(rbId) }"
              >
                <div class="rulebook-name">{{ rulebook.name }}</div>
                <div class="rulebook-meta">
                  {{ rulebook.entry_points.length }} entry points
                  <span v-if="rulebook.batch_variety" class="variety-badge">📊 Variety</span>
                </div>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Selected Rulebook Viewer -->
      <div v-if="selectedRulebook" class="selected-rulebook">
        <RulebookViewer
          :rulebook="loadedPackage.namespaces[selectedRulebook.namespace].rulebooks[selectedRulebook.id]"
          :rulebook-id="`${selectedRulebook.namespace}:${selectedRulebook.id}`"
          :package-data="loadedPackage"
        />
      </div>
    </div>

    <div v-if="!loadedPackage && !loading && !error" class="empty-state">
      <p>Click "Load Package" to open a YAML or JSON package file</p>
      <p class="hint">M2 Foundation: Package loading and basic display</p>
    </div>
  </div>
</template>

<style scoped>
.package-viewer {
  max-width: 1200px;
  margin: 0 auto;
}

.controls {
  margin-bottom: 2rem;
}

.btn-primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  padding: 0.75rem 1.5rem;
  font-size: 1rem;
  border-radius: 0.5rem;
  cursor: pointer;
  transition: transform 0.2s, box-shadow 0.2s;
}

.btn-primary:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(102, 126, 234, 0.3);
}

.btn-primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.error-message {
  background: #fed7d7;
  border: 1px solid #fc8181;
  border-radius: 0.5rem;
  padding: 1.5rem;
  margin-bottom: 2rem;
}

.error-message h3 {
  color: #c53030;
  margin-top: 0;
}

.error-message pre {
  color: #742a2a;
  white-space: pre-wrap;
  word-wrap: break-word;
}

.package-info {
  background: white;
  border-radius: 0.5rem;
  padding: 2rem;
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
  margin-bottom: 2rem;
}

.package-info h2 {
  margin-top: 0;
  color: #2d3748;
}

.info-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 1rem;
  margin-top: 1.5rem;
}

.info-item {
  display: flex;
  flex-direction: column;
  gap: 0.25rem;
}

.info-item label {
  font-weight: 600;
  color: #4a5568;
  font-size: 0.875rem;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.info-item span {
  color: #2d3748;
  font-size: 1rem;
}

.package-details {
  background: white;
  border-radius: 0.5rem;
  padding: 2rem;
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
}

.package-details h3 {
  margin-top: 0;
  color: #2d3748;
}

.namespaces {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(300px, 1fr));
  gap: 1rem;
  margin-top: 1.5rem;
}

.namespace-card {
  background: #f7fafc;
  border: 1px solid #e2e8f0;
  border-radius: 0.5rem;
  padding: 1.5rem;
}

.namespace-card h4 {
  margin: 0 0 1rem 0;
  color: #2d3748;
  font-family: 'Courier New', monospace;
}

.namespace-stats {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
  font-size: 0.875rem;
  color: #4a5568;
}

/* Dependencies Section */
.dependencies-section {
  background: white;
  border-radius: 0.5rem;
  padding: 2rem;
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
  margin-top: 2rem;
}

.dependencies-section h3 {
  margin-top: 0;
  color: #2d3748;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.dependencies-list {
  display: grid;
  gap: 0.75rem;
}

.dependency-item {
  background: #f7fafc;
  border: 2px solid #e2e8f0;
  border-radius: 0.5rem;
  padding: 1rem;
  transition: all 0.2s;
}

.dependency-item:hover {
  border-color: #4299e1;
  box-shadow: 0 2px 4px rgba(66, 153, 225, 0.1);
}

.dependency-header {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-bottom: 0.5rem;
}

.dependency-header strong {
  color: #2d3748;
  font-size: 1rem;
}

.version-badge {
  background: #edf2f7;
  color: #2d3748;
  padding: 0.25rem 0.5rem;
  border-radius: 0.25rem;
  font-size: 0.875rem;
  font-family: 'Courier New', monospace;
  border: 1px solid #cbd5e0;
}

.dependency-path {
  font-size: 0.875rem;
  color: #718096;
  font-family: 'Courier New', monospace;
  padding-left: 0.25rem;
}

/* Rulebooks Section */
.rulebooks-section {
  background: white;
  border-radius: 0.5rem;
  padding: 2rem;
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
  margin-top: 2rem;
}

.rulebooks-section h3 {
  margin-top: 0;
  color: #2d3748;
}

.rulebook-list {
  margin-bottom: 2rem;
}

.namespace-group {
  margin-bottom: 1.5rem;
}

.namespace-title {
  color: #4a5568;
  font-family: 'Courier New', monospace;
  font-size: 0.875rem;
  text-transform: uppercase;
  letter-spacing: 0.05em;
  margin: 0 0 0.75rem 0;
  padding-bottom: 0.5rem;
  border-bottom: 2px solid #e2e8f0;
}

.rulebook-items {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(280px, 1fr));
  gap: 0.75rem;
}

.rulebook-item {
  background: #f7fafc;
  border: 2px solid #e2e8f0;
  border-radius: 0.5rem;
  padding: 1rem;
  cursor: pointer;
  transition: all 0.2s;
}

.rulebook-item:hover {
  border-color: #667eea;
  background: #edf2f7;
  transform: translateY(-2px);
  box-shadow: 0 4px 6px rgba(102, 126, 234, 0.1);
}

.rulebook-item.selected {
  border-color: #667eea;
  background: linear-gradient(135deg, #eef2ff 0%, #e0e7ff 100%);
  box-shadow: 0 4px 6px rgba(102, 126, 234, 0.2);
}

.rulebook-name {
  font-weight: 600;
  color: #2d3748;
  margin-bottom: 0.5rem;
  font-size: 1rem;
}

.rulebook-meta {
  font-size: 0.875rem;
  color: #718096;
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.variety-badge {
  font-size: 0.75rem;
  padding: 0.125rem 0.375rem;
  background: #d4edda;
  color: #155724;
  border-radius: 3px;
  font-weight: 600;
}

.selected-rulebook {
  background: #f7fafc;
  border-radius: 0.5rem;
  padding: 1.5rem;
  border: 2px solid #667eea;
}

.empty-state {
  text-align: center;
  padding: 4rem 2rem;
  color: #718096;
}

.empty-state p {
  margin: 0.5rem 0;
}

.hint {
  font-size: 0.875rem;
  font-style: italic;
  color: #a0aec0;
}
</style>

