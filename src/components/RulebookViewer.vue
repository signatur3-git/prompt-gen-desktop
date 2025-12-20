<script setup lang="ts">
import { ref, computed } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface Rulebook {
  name: string
  description: string
  entry_points: EntryPoint[]
  batch_variety: boolean
  context_defaults: Record<string, string>
}

interface EntryPoint {
  prompt_section: string
  weight: number
}

interface RenderResult {
  prompt: string
  seed: number
  context: Record<string, string>
}

const props = defineProps<{
  rulebook: Rulebook
  rulebookId: string
  packageData: any
}>()

const rendering = ref(false)
const renderError = ref<string | null>(null)
const singleResult = ref<RenderResult | null>(null)
const batchResults = ref<RenderResult[]>([])
const seed = ref(42)
const batchCount = ref(5)
const showBatch = ref(false)

// Calculate total weight
const totalWeight = computed(() => {
  return props.rulebook.entry_points.reduce((sum, ep) => sum + ep.weight, 0)
})

// Calculate percentages for each entry point
const entryPointPercentages = computed(() => {
  const total = totalWeight.value
  return props.rulebook.entry_points.map(ep => ({
    ...ep,
    percentage: total > 0 ? (ep.weight / total * 100).toFixed(1) : '0.0'
  }))
})

// Check if context defaults exist
const hasContextDefaults = computed(() => {
  return Object.keys(props.rulebook.context_defaults).length > 0
})

async function renderSingle() {
  try {
    rendering.value = true
    renderError.value = null

    const result = await invoke<RenderResult>('render_from_rulebook', {
      package: props.packageData,
      rulebookRef: props.rulebookId,
      seed: seed.value
    })

    singleResult.value = result
    showBatch.value = false
  } catch (e) {
    renderError.value = `Render error: ${e}`
    console.error('Render error:', e)
  } finally {
    rendering.value = false
  }
}

async function renderBatch() {
  try {
    rendering.value = true
    renderError.value = null

    const result = await invoke<{ results: RenderResult[] }>('render_from_rulebook_batch', {
      package: props.packageData,
      rulebookRef: props.rulebookId,
      count: batchCount.value,
      startSeed: seed.value
    })

    batchResults.value = result.results
    showBatch.value = true
  } catch (e) {
    renderError.value = `Batch render error: ${e}`
    console.error('Batch render error:', e)
  } finally {
    rendering.value = false
  }
}

function copyToClipboard(text: string) {
  navigator.clipboard.writeText(text)
}
</script>

<template>
  <div class="rulebook-viewer">
    <div class="rulebook-header">
      <h3>{{ rulebook.name }}</h3>
      <p class="description">{{ rulebook.description }}</p>
    </div>

    <!-- Entry Points -->
    <div class="section">
      <h4>Entry Points ({{ rulebook.entry_points.length }})</h4>
      <div class="entry-points">
        <div
          v-for="(ep, idx) in entryPointPercentages"
          :key="idx"
          class="entry-point"
        >
          <div class="ep-ref">{{ ep.prompt_section }}</div>
          <div class="ep-weight">
            <span class="weight-value">{{ ep.weight }}</span>
            <span class="weight-percent">({{ ep.percentage }}%)</span>
          </div>
        </div>
      </div>
      <div class="total-weight">Total Weight: {{ totalWeight }}</div>
    </div>

    <!-- Batch Variety -->
    <div class="section">
      <div class="info-row">
        <span class="label">Batch Variety:</span>
        <span :class="['badge', rulebook.batch_variety ? 'badge-enabled' : 'badge-disabled']">
          {{ rulebook.batch_variety ? 'Enabled' : 'Disabled' }}
        </span>
      </div>
      <p class="info-text" v-if="rulebook.batch_variety">
        Each prompt in a batch will use a different entry point when possible.
      </p>
    </div>

    <!-- Context Defaults -->
    <div class="section" v-if="hasContextDefaults">
      <h4>Context Defaults ({{ Object.keys(rulebook.context_defaults).length }})</h4>
      <div class="context-defaults">
        <div
          v-for="(value, key) in rulebook.context_defaults"
          :key="key"
          class="context-entry"
        >
          <span class="context-key">{{ key }}:</span>
          <span class="context-value">{{ value }}</span>
        </div>
      </div>
    </div>

    <!-- Live Preview -->
    <div class="section live-preview">
      <h4>Live Preview</h4>

      <div class="controls">
        <div class="control-group">
          <label>Seed:</label>
          <input v-model.number="seed" type="number" min="0" />
        </div>

        <div class="control-group" v-if="showBatch || !singleResult">
          <label>Batch Size:</label>
          <input v-model.number="batchCount" type="number" min="1" max="50" />
        </div>

        <div class="button-group">
          <button @click="renderSingle" :disabled="rendering">
            {{ rendering ? 'Rendering...' : 'Render Single' }}
          </button>
          <button @click="renderBatch" :disabled="rendering">
            {{ rendering ? 'Rendering...' : `Render Batch (${batchCount})` }}
          </button>
        </div>
      </div>

      <div v-if="renderError" class="error">{{ renderError }}</div>

      <!-- Single Result -->
      <div v-if="singleResult && !showBatch" class="result-box">
        <div class="result-header">
          <span class="result-seed">Seed: {{ singleResult.seed }}</span>
          <button @click="copyToClipboard(singleResult.prompt)" class="copy-btn">📋 Copy</button>
        </div>
        <div class="result-prompt">{{ singleResult.prompt }}</div>
        <div v-if="Object.keys(singleResult.context).length > 0" class="result-context">
          <strong>Context:</strong>
          <div class="context-values">
            <span v-for="(value, key) in singleResult.context" :key="key" class="context-item">
              {{ key }}={{ value }}
            </span>
          </div>
        </div>
      </div>

      <!-- Batch Results -->
      <div v-if="showBatch && batchResults.length > 0" class="batch-results">
        <div v-for="(result, idx) in batchResults" :key="idx" class="result-box">
          <div class="result-header">
            <span class="result-number">#{{ idx + 1 }}</span>
            <span class="result-seed">Seed: {{ result.seed }}</span>
            <button @click="copyToClipboard(result.prompt)" class="copy-btn">📋</button>
          </div>
          <div class="result-prompt">{{ result.prompt }}</div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.rulebook-viewer {
  padding: 1rem;
}

.rulebook-header h3 {
  margin: 0 0 0.5rem 0;
  color: #2c3e50;
}

.description {
  color: #666;
  margin: 0 0 1rem 0;
}

.section {
  margin-bottom: 1.5rem;
  padding: 1rem;
  background: #f8f9fa;
  border-radius: 4px;
}

.section h4 {
  margin: 0 0 0.75rem 0;
  color: #2c3e50;
  font-size: 0.9rem;
  text-transform: uppercase;
  letter-spacing: 0.5px;
}

/* Entry Points */
.entry-points {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.entry-point {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.5rem;
  background: white;
  border-radius: 4px;
  border: 1px solid #dee2e6;
}

.ep-ref {
  font-family: 'Monaco', 'Courier New', monospace;
  font-size: 0.9rem;
  color: #0066cc;
}

.ep-weight {
  display: flex;
  gap: 0.5rem;
  align-items: center;
}

.weight-value {
  font-weight: 600;
  color: #2c3e50;
}

.weight-percent {
  color: #666;
  font-size: 0.85rem;
}

.total-weight {
  margin-top: 0.5rem;
  padding-top: 0.5rem;
  border-top: 1px solid #dee2e6;
  font-weight: 600;
  color: #2c3e50;
}

/* Info Row */
.info-row {
  display: flex;
  align-items: center;
  gap: 0.75rem;
  margin-bottom: 0.5rem;
}

.label {
  font-weight: 600;
  color: #2c3e50;
}

.badge {
  padding: 0.25rem 0.75rem;
  border-radius: 12px;
  font-size: 0.85rem;
  font-weight: 600;
}

.badge-enabled {
  background: #d4edda;
  color: #155724;
}

.badge-disabled {
  background: #f8d7da;
  color: #721c24;
}

.info-text {
  margin: 0;
  font-size: 0.85rem;
  color: #666;
  font-style: italic;
}

/* Context Defaults */
.context-defaults {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.context-entry {
  padding: 0.5rem;
  background: white;
  border-radius: 4px;
  border: 1px solid #dee2e6;
  font-family: 'Monaco', 'Courier New', monospace;
  font-size: 0.9rem;
}

.context-key {
  color: #6f42c1;
  font-weight: 600;
  margin-right: 0.5rem;
}

.context-value {
  color: #2c3e50;
}

/* Live Preview */
.live-preview {
  background: white;
  border: 2px solid #007bff;
}

.controls {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  margin-bottom: 1rem;
}

.control-group {
  display: flex;
  align-items: center;
  gap: 0.5rem;
}

.control-group label {
  min-width: 80px;
  font-weight: 600;
  color: #2c3e50;
}

.control-group input {
  padding: 0.5rem;
  border: 1px solid #ced4da;
  border-radius: 4px;
  font-size: 1rem;
  width: 120px;
}

.button-group {
  display: flex;
  gap: 0.5rem;
}

button {
  padding: 0.5rem 1rem;
  background: #007bff;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-weight: 600;
}

button:hover:not(:disabled) {
  background: #0056b3;
}

button:disabled {
  background: #6c757d;
  cursor: not-allowed;
}

.copy-btn {
  padding: 0.25rem 0.5rem;
  font-size: 0.85rem;
  background: #28a745;
}

.copy-btn:hover {
  background: #218838;
}

.error {
  padding: 0.75rem;
  background: #f8d7da;
  color: #721c24;
  border-radius: 4px;
  margin-bottom: 1rem;
  border: 1px solid #f5c6cb;
}

/* Results */
.result-box {
  margin-bottom: 1rem;
  padding: 1rem;
  background: #f8f9fa;
  border-radius: 4px;
  border: 1px solid #dee2e6;
}

.result-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.5rem;
  padding-bottom: 0.5rem;
  border-bottom: 1px solid #dee2e6;
}

.result-number {
  font-weight: 600;
  color: #007bff;
}

.result-seed {
  font-family: 'Monaco', 'Courier New', monospace;
  font-size: 0.85rem;
  color: #6c757d;
}

.result-prompt {
  padding: 0.75rem;
  background: white;
  border-radius: 4px;
  font-size: 1rem;
  line-height: 1.5;
  color: #2c3e50;
  margin-bottom: 0.5rem;
}

.result-context {
  padding: 0.5rem;
  background: #e7f3ff;
  border-radius: 4px;
  font-size: 0.85rem;
}

.context-values {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  margin-top: 0.25rem;
}

.context-item {
  padding: 0.25rem 0.5rem;
  background: white;
  border-radius: 3px;
  font-family: 'Monaco', 'Courier New', monospace;
  font-size: 0.8rem;
  color: #6f42c1;
}

.batch-results {
  max-height: 600px;
  overflow-y: auto;
}
</style>

