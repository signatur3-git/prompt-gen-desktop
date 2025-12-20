<template>
  <div class="metadata-editor">
    <div class="editor-header">
      <h2>
        <span class="component-icon">📦</span>
        Package Metadata
      </h2>
      <button @click="$emit('close')" class="btn-secondary">Close</button>
    </div>

    <div class="editor-content">
      <!-- Package ID (Read-only) -->
      <div class="section">
        <div class="form-group">
          <label for="package-id">Package ID</label>
          <input
            id="package-id"
            v-model="packageData.id"
            type="text"
            class="metadata-input"
            readonly
            disabled
          />
          <small>Package ID cannot be changed after creation</small>
        </div>
      </div>

      <!-- Editable Metadata -->
      <div class="section">
        <h3>Package Information</h3>

        <!-- Version -->
        <div class="form-group">
          <label for="version">Version *</label>
          <input
            id="version"
            v-model="packageData.version"
            @input="emitUpdate"
            type="text"
            placeholder="1.0.0"
            pattern="\d+\.\d+\.\d+"
            class="metadata-input"
          />
          <small>Semantic version (X.Y.Z)</small>
        </div>

        <!-- Name -->
        <div class="form-group">
          <label for="name">Display Name *</label>
          <input
            id="name"
            v-model="packageData.metadata.name"
            @input="emitUpdate"
            type="text"
            placeholder="My Package"
            class="metadata-input"
          />
          <small>Human-readable package name</small>
        </div>

        <!-- Description -->
        <div class="form-group">
          <label for="description">Description</label>
          <textarea
            id="description"
            v-model="packageData.metadata.description"
            @input="emitUpdate"
            rows="4"
            placeholder="Brief description of what this package does..."
            class="metadata-textarea"
          />
          <small>Optional description for users</small>
        </div>

        <!-- Authors -->
        <div class="form-group">
          <label for="authors">Authors</label>
          <input
            id="authors"
            v-model="authorsInput"
            @input="updateAuthors"
            type="text"
            placeholder="Author Name, Another Author"
            class="metadata-input"
          />
          <small>Comma-separated list of author names</small>
        </div>

        <div v-if="packageData.metadata.authors.length > 0" class="authors-list">
          <span
            v-for="(author, index) in packageData.metadata.authors"
            :key="index"
            class="author-badge"
          >
            {{ author }}
          </span>
        </div>

        <!-- Bypass Filters -->
        <div class="form-group">
          <label class="checkbox-label">
            <input
              v-model="packageData.metadata.bypass_filters"
              @input="emitUpdate"
              type="checkbox"
            />
            <span>Bypass tag filters</span>
          </label>
          <small>Allow this package's components to bypass tag filtering (advanced)</small>
        </div>
      </div>

      <!-- Dependencies -->
      <div class="section">
        <DependenciesEditor
          v-model="packageData.dependencies"
          @update:modelValue="emitUpdate"
        />
      </div>

      <!-- Package Stats -->
      <div class="section stats-section">
        <h3>Package Statistics</h3>

        <div class="stats-grid">
          <div class="stat-item">
            <div class="stat-value">{{ namespaceCount }}</div>
            <div class="stat-label">Namespaces</div>
          </div>
          <div class="stat-item">
            <div class="stat-value">{{ datatypeCount }}</div>
            <div class="stat-label">Datatypes</div>
          </div>
          <div class="stat-item">
            <div class="stat-value">{{ promptSectionCount }}</div>
            <div class="stat-label">Prompt Sections</div>
          </div>
          <div class="stat-item">
            <div class="stat-value">{{ separatorCount }}</div>
            <div class="stat-label">Separators</div>
          </div>
          <div class="stat-item">
            <div class="stat-value">{{ ruleCount }}</div>
            <div class="stat-label">Rules</div>
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch } from 'vue'
import DependenciesEditor from './DependenciesEditor.vue'

const props = defineProps({
  data: {
    type: Object,
    required: true
  }
})

const emit = defineEmits(['update', 'close'])

// Local state
const packageData = ref({
  id: props.data.id || '',
  version: props.data.version || '1.0.0',
  metadata: {
    name: props.data.metadata?.name || '',
    description: props.data.metadata?.description || '',
    authors: [...(props.data.metadata?.authors || [])],
    bypass_filters: props.data.metadata?.bypass_filters || false
  },
  namespaces: props.data.namespaces || {},
  dependencies: [...(props.data.dependencies || [])]
})

const authorsInput = ref(packageData.value.metadata.authors.join(', '))

// Watch for prop changes
watch(() => props.data, (newData) => {
  packageData.value = {
    id: newData.id || '',
    version: newData.version || '1.0.0',
    metadata: {
      name: newData.metadata?.name || '',
      description: newData.metadata?.description || '',
      authors: [...(newData.metadata?.authors || [])],
      bypass_filters: newData.metadata?.bypass_filters || false
    },
    namespaces: newData.namespaces || {},
    dependencies: [...(newData.dependencies || [])]
  }
  authorsInput.value = packageData.value.metadata.authors.join(', ')
}, { deep: true })

// Computed stats
const namespaceCount = computed(() => Object.keys(packageData.value.namespaces).length)
const datatypeCount = computed(() => {
  return Object.values(packageData.value.namespaces)
    .reduce((sum, ns) => sum + Object.keys(ns.datatypes || {}).length, 0)
})
const promptSectionCount = computed(() => {
  return Object.values(packageData.value.namespaces)
    .reduce((sum, ns) => sum + Object.keys(ns.prompt_sections || {}).length, 0)
})
const separatorCount = computed(() => {
  return Object.values(packageData.value.namespaces)
    .reduce((sum, ns) => sum + Object.keys(ns.separator_sets || {}).length, 0)
})
const ruleCount = computed(() => {
  return Object.values(packageData.value.namespaces)
    .reduce((sum, ns) => sum + (ns.rules || []).length, 0)
})

// Methods
function updateAuthors() {
  const authors = authorsInput.value
    .split(',')
    .map(a => a.trim())
    .filter(a => a.length > 0)

  packageData.value.metadata.authors = authors.length > 0 ? authors : ['Unknown Author']
  emitUpdate()
}

function emitUpdate() {
  emit('update', {
    id: packageData.value.id,
    version: packageData.value.version,
    metadata: {
      name: packageData.value.metadata.name,
      description: packageData.value.metadata.description || undefined,
      authors: packageData.value.metadata.authors,
      bypass_filters: packageData.value.metadata.bypass_filters
    },
    namespaces: packageData.value.namespaces,
    dependencies: packageData.value.dependencies
  })
}
</script>

<style scoped>
.metadata-editor {
  display: flex;
  flex-direction: column;
  height: 100%;
  background: #1e1e1e;
}

.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid #3e3e42;
  background: #252526;
}

.editor-header h2 {
  margin: 0;
  font-size: 18px;
  font-weight: 600;
  display: flex;
  align-items: center;
  gap: 8px;
}

.component-icon {
  font-size: 20px;
}

.editor-content {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.section {
  margin-bottom: 24px;
  background: #252526;
  border-radius: 6px;
  padding: 20px;
}


.section h3 {
  margin: 0 0 16px 0;
  font-size: 16px;
  font-weight: 600;
}

.form-group {
  margin-bottom: 20px;
}

.form-group:last-child {
  margin-bottom: 0;
}

.form-group label {
  display: block;
  margin-bottom: 8px;
  font-weight: 500;
  color: #d4d4d4;
  font-size: 13px;
}

.metadata-input,
.metadata-textarea {
  width: 100%;
  background: #3c3c3c;
  border: 1px solid #3e3e42;
  border-radius: 4px;
  color: #d4d4d4;
  padding: 8px 12px;
  font-size: 14px;
  font-family: inherit;
}

.metadata-input:disabled {
  opacity: 0.6;
  cursor: not-allowed;
  background: #2a2d2e;
}

.metadata-input:focus,
.metadata-textarea:focus {
  outline: none;
  border-color: #0e639c;
}

.metadata-textarea {
  resize: vertical;
  min-height: 80px;
}

.form-group small {
  display: block;
  margin-top: 4px;
  color: #858585;
  font-size: 12px;
}

.checkbox-label {
  display: flex;
  align-items: center;
  gap: 8px;
  cursor: pointer;
}

.checkbox-label input[type="checkbox"] {
  cursor: pointer;
}

.authors-list {
  display: flex;
  flex-wrap: wrap;
  gap: 8px;
  margin-top: 8px;
}

.author-badge {
  display: inline-block;
  padding: 4px 12px;
  background: #0e639c;
  color: white;
  border-radius: 12px;
  font-size: 12px;
  font-weight: 500;
}


.stats-section {
  background: #1e1e1e;
  border: 1px solid #3e3e42;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(120px, 1fr));
  gap: 16px;
}

.stat-item {
  text-align: center;
  padding: 16px;
  background: #252526;
  border-radius: 4px;
}

.stat-value {
  font-size: 32px;
  font-weight: 700;
  color: #4fc1ff;
  margin-bottom: 4px;
}

.stat-label {
  font-size: 12px;
  color: #858585;
  text-transform: uppercase;
  font-weight: 600;
}


.btn-secondary {
  background: #3e3e42;
  color: #d4d4d4;
  border: none;
  border-radius: 4px;
  padding: 8px 16px;
  cursor: pointer;
  font-size: 14px;
}

.btn-secondary:hover {
  background: #505050;
}
</style>

