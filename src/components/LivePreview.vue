<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import { invoke } from '@tauri-apps/api/core'

interface Package {
  id: string
  version: string
  namespaces: Record<string, any>
}

interface RenderResult {
  output: string
  seed: number
  selected_values?: Record<string, string>
}

interface BatchRenderResult {
  output: string
  seed: number
  index: number
}

interface Reference {
  target: string
  filter?: string
  min?: number
  max?: number
  separator?: string
  unique?: boolean
}

const props = defineProps<{
  package: Package | null
  dependencies?: Record<string, Package> // M9 Phase 3: Dependencies for cross-package rendering
}>()

const seed = ref<number>(42)
const renderMode = ref<'promptsection' | 'rulebook'>('promptsection')
const promptSection = ref<string>('')
const rulebook = ref<string>('')
const rendering = ref(false)
const renderResult = ref<RenderResult | null>(null)
const error = ref<string | null>(null)

// Batch generation
const batchMode = ref(false)
const batchCount = ref<number>(5)
const batchSeedMode = ref<'random' | 'increment'>('increment')
const batchResults = ref<Array<RenderResult | BatchRenderResult>>([])
const batchRendering = ref(false)

// Get list of available prompt sections
const promptSections = computed(() => {
  if (!props.package) return []

  const sections: string[] = []
  for (const [nsId, namespace] of Object.entries(props.package.namespaces)) {
    for (const sectionName of Object.keys(namespace.prompt_sections || {})) {
      sections.push(`${nsId}:${sectionName}`)
    }
  }
  return sections
})

// Get list of available rulebooks
const rulebooks = computed(() => {
  if (!props.package) return []

  const books: string[] = []
  for (const [nsId, namespace] of Object.entries(props.package.namespaces)) {
    for (const rbName of Object.keys(namespace.rulebooks || {})) {
      books.push(`${nsId}:${rbName}`)
    }
  }
  return books
})

// Auto-select first item based on mode
watch(() => props.package, () => {
  if (renderMode.value === 'promptsection' && promptSections.value.length > 0 && !promptSection.value) {
    promptSection.value = promptSections.value[0]
  }
  if (renderMode.value === 'rulebook' && rulebooks.value.length > 0 && !rulebook.value) {
    rulebook.value = rulebooks.value[0]
  }
}, { immediate: true })

// Switch selection when mode changes
watch(renderMode, (newMode) => {
  if (newMode === 'promptsection' && promptSections.value.length > 0 && !promptSection.value) {
    promptSection.value = promptSections.value[0]
  }
  if (newMode === 'rulebook' && rulebooks.value.length > 0 && !rulebook.value) {
    rulebook.value = rulebooks.value[0]
  }
})

// Get template info for selected prompt section
const templateInfo = computed(() => {
  // Only show template info for prompt sections, not rulebooks
  if (renderMode.value !== 'promptsection') return null

  if (!props.package || !promptSection.value) return null

  const [nsId, sectionName] = promptSection.value.split(':')
  const namespace = props.package.namespaces[nsId]
  if (!namespace) return null

  const section = namespace.prompt_sections?.[sectionName]
  if (!section) return null

  return {
    template: section.template,
    references: section.references || {},
    hasFilters: section.template.includes('#{')
  }
})

// Get rulebook info for selected rulebook
const rulebookInfo = computed(() => {
  // Only show rulebook info when in rulebook mode
  if (renderMode.value !== 'rulebook') return null

  if (!props.package || !rulebook.value) return null

  const [nsId, rbName] = rulebook.value.split(':')
  const namespace = props.package.namespaces[nsId]
  if (!namespace) return null

  const rb = namespace.rulebooks?.[rbName]
  if (!rb) return null

  return {
    name: rb.name,
    description: rb.description,
    entryPoints: rb.entry_points || [],
    batchVariety: rb.batch_variety,
    contextDefaults: rb.context_defaults || {}
  }
})

// Format reference details for display
const referenceDetails = computed(() => {
  if (!templateInfo.value) return []

  const refs: Array<{
    name: string
    target: string
    details: string[]
  }> = []

  for (const [name, ref] of Object.entries(templateInfo.value.references as Record<string, Reference>)) {
    const details: string[] = []

    if (ref.filter) {
      details.push(`filter: ${ref.filter}`)
    }

    if (ref.min !== undefined && ref.max !== undefined && (ref.min !== 1 || ref.max !== 1)) {
      details.push(`min: ${ref.min}, max: ${ref.max}`)
    }

    if (ref.separator) {
      details.push(`separator: ${ref.separator}`)
    }

    if (ref.unique) {
      details.push('unique')
    }

    refs.push({
      name,
      target: ref.target,
      details
    })
  }

  return refs
})

async function render() {
  if (!props.package) {
    error.value = "No package loaded"
    return
  }

  if (renderMode.value === 'promptsection' && !promptSection.value) {
    error.value = "No prompt section selected"
    return
  }

  if (renderMode.value === 'rulebook' && !rulebook.value) {
    error.value = "No rulebook selected"
    return
  }

  try {
    rendering.value = true
    error.value = null
    renderResult.value = null

    let result: RenderResult

    // M9 Phase 3: Use commands with dependencies if available
    if (renderMode.value === 'rulebook') {
      if (props.dependencies && Object.keys(props.dependencies).length > 0) {
        result = await invoke<RenderResult>('render_from_rulebook_with_dependencies', {
          package: props.package,
          dependencies: props.dependencies,
          rulebookRef: rulebook.value,
          seed: seed.value
        })
      } else {
        result = await invoke<RenderResult>('render_from_rulebook', {
          package: props.package,
          rulebookRef: rulebook.value,
          seed: seed.value
        })
      }
    } else {
      if (props.dependencies && Object.keys(props.dependencies).length > 0) {
        result = await invoke<RenderResult>('render_prompt_with_dependencies', {
          package: props.package,
          dependencies: props.dependencies,
          promptsection: promptSection.value,
          seed: seed.value
        })
      } else {
        result = await invoke<RenderResult>('render_prompt', {
          package: props.package,
          promptsection: promptSection.value,
          seed: seed.value
        })
      }
    }

    renderResult.value = result
  } catch (e) {
    error.value = String(e)
    console.error('Render error:', e)
  } finally {
    rendering.value = false
  }
}

async function renderBatch() {
  if (!props.package) {
    error.value = "No package loaded"
    return
  }

  if (renderMode.value === 'promptsection' && !promptSection.value) {
    error.value = "No prompt section selected"
    return
  }

  if (renderMode.value === 'rulebook' && !rulebook.value) {
    error.value = "No rulebook selected"
    return
  }

  try {
    batchRendering.value = true
    error.value = null
    batchResults.value = []

    if (renderMode.value === 'rulebook') {
      // Use batch API for rulebooks
      if (props.dependencies && Object.keys(props.dependencies).length > 0) {
        const response = await invoke<{ results: BatchRenderResult[] }>('render_from_rulebook_batch_with_dependencies', {
          package: props.package,
          dependencies: props.dependencies,
          rulebookRef: rulebook.value,
          count: batchCount.value,
          startSeed: seed.value
        })
        batchResults.value = response.results
      } else {
        const response = await invoke<{ results: BatchRenderResult[] }>('render_from_rulebook_batch', {
          package: props.package,
          rulebookRef: rulebook.value,
          count: batchCount.value,
          startSeed: seed.value
        })
        batchResults.value = response.results
      }
    } else {
      // Manual batch for prompt sections
      const results: RenderResult[] = []

      for (let i = 0; i < batchCount.value; i++) {
        let currentSeed: number

        if (batchSeedMode.value === 'increment') {
          currentSeed = seed.value + i
        } else {
          currentSeed = Math.floor(Math.random() * 1000000)
        }

        if (props.dependencies && Object.keys(props.dependencies).length > 0) {
          const result = await invoke<RenderResult>('render_prompt_with_dependencies', {
            package: props.package,
            dependencies: props.dependencies,
            promptsection: promptSection.value,
            seed: currentSeed
          })
          results.push(result)
        } else {
          const result = await invoke<RenderResult>('render_prompt', {
            package: props.package,
            promptsection: promptSection.value,
            seed: currentSeed
          })
          results.push(result)
        }
      }

      batchResults.value = results
    }
  } catch (e) {
    error.value = String(e)
    console.error('Batch render error:', e)
  } finally {
    batchRendering.value = false
  }
}

function randomizeSeed() {
  seed.value = Math.floor(Math.random() * 1000000)
}

// Copy to clipboard
async function copyToClipboard(text: string) {
  try {
    await navigator.clipboard.writeText(text)
  } catch (err) {
    console.error('Failed to copy:', err)
  }
}

// Copy all batch results
async function copyAllBatchResults() {
  const allText = batchResults.value
    .map((result, idx) => `#${idx + 1} (Seed: ${result.seed})\n${result.output}`)
    .join('\n\n')
  await copyToClipboard(allText)
}

// Clear results when switching between batch and single mode
watch(batchMode, () => {
  batchResults.value = []
  renderResult.value = null
})
</script>

<template>
  <div class="live-preview">
    <h2>Live Preview</h2>

    <div v-if="!package" class="empty-state">
      <p>Load a package to start rendering prompts</p>
    </div>

    <div v-else class="preview-controls">
      <!-- Render Mode Selection -->
      <div class="mode-selection">
        <label>
          <input type="radio" v-model="renderMode" value="promptsection" />
          Prompt Section
        </label>
        <label>
          <input type="radio" v-model="renderMode" value="rulebook" />
          Rulebook
        </label>
      </div>

      <!-- Prompt Section Selection -->
      <div v-if="renderMode === 'promptsection'" class="control-group">
        <label for="prompt-section">Prompt Section:</label>
        <select
          id="prompt-section"
          v-model="promptSection"
          class="select-input"
        >
          <option v-for="section in promptSections" :key="section" :value="section">
            {{ section }}
          </option>
        </select>
      </div>

      <!-- Rulebook Selection -->
      <div v-if="renderMode === 'rulebook'" class="control-group">
        <label for="rulebook">Rulebook:</label>
        <select
          id="rulebook"
          v-model="rulebook"
          class="select-input"
        >
          <option v-for="rb in rulebooks" :key="rb" :value="rb">
            {{ rb }}
          </option>
        </select>
      </div>

      <div class="control-group">
        <label for="seed">Seed:</label>
        <div class="seed-input-group">
          <input
            id="seed"
            v-model.number="seed"
            type="number"
            class="number-input"
            min="0"
          />
          <button
            @click="randomizeSeed"
            class="btn-secondary"
            title="Random seed"
          >
            🎲
          </button>
        </div>
      </div>

      <div class="batch-toggle">
        <label>
          <input type="checkbox" v-model="batchMode" />
          Batch Generation
        </label>
      </div>

      <div v-if="batchMode" class="batch-controls">
        <div class="control-group">
          <label for="batch-count">Number of Prompts:</label>
          <input
            id="batch-count"
            v-model.number="batchCount"
            type="number"
            class="number-input"
            min="1"
            max="100"
          />
        </div>

        <div class="control-group">
          <label>Seed Mode:</label>
          <div class="radio-group">
            <label>
              <input type="radio" v-model="batchSeedMode" value="increment" />
              Increment from {{ seed }}
            </label>
            <label>
              <input type="radio" v-model="batchSeedMode" value="random" />
              Random seeds
            </label>
          </div>
        </div>
      </div>

      <button
        v-if="!batchMode"
        @click="render"
        :disabled="rendering || (renderMode === 'promptsection' && !promptSection) || (renderMode === 'rulebook' && !rulebook)"
        class="btn-primary"
      >
        {{ rendering ? 'Rendering...' : 'Render' }}
      </button>

      <button
        v-else
        @click="renderBatch"
        :disabled="batchRendering || (renderMode === 'promptsection' && !promptSection) || (renderMode === 'rulebook' && !rulebook)"
        class="btn-primary"
      >
        {{ batchRendering ? 'Rendering...' : `Render ${batchCount} Prompts` }}
      </button>
    </div>

    <!-- Template display for Prompt Sections -->
    <div v-if="templateInfo" class="template-display">
      <h4>Template</h4>
      <code class="template-code">{{ templateInfo.template }}</code>
      <div v-if="templateInfo.hasFilters" class="filter-badge">
        🔍 Contains tag filters
      </div>

      <!-- Reference definitions -->
      <div v-if="referenceDetails.length > 0" class="references-section">
        <h4>References</h4>
        <div class="references-list">
          <div
            v-for="ref in referenceDetails"
            :key="ref.name"
            class="reference-item"
          >
            <div class="ref-header">
              <span class="ref-name">{{ ref.name }}</span>
              <span class="ref-arrow">→</span>
              <span class="ref-target">{{ ref.target }}</span>
            </div>
            <div v-if="ref.details.length > 0" class="ref-details">
              <span v-for="(detail, idx) in ref.details" :key="idx" class="detail-badge">
                {{ detail }}
              </span>
            </div>
          </div>
        </div>
      </div>
    </div>

    <!-- Rulebook info display for Rulebooks -->
    <div v-if="rulebookInfo" class="rulebook-display">
      <h4>{{ rulebookInfo.name }}</h4>
      <p v-if="rulebookInfo.description" class="rulebook-description">{{ rulebookInfo.description }}</p>

      <div class="rulebook-section">
        <h5>Entry Points ({{ rulebookInfo.entryPoints.length }})</h5>
        <div class="entry-points-list">
          <div v-for="(ep, idx) in rulebookInfo.entryPoints" :key="idx" class="entry-point">
            <span class="ep-number">{{ idx + 1 }}</span>
            <span class="ep-ref">{{ ep.prompt_section }}</span>
            <span class="ep-weight">weight: {{ ep.weight }}</span>
          </div>
        </div>
      </div>

      <div v-if="rulebookInfo.batchVariety" class="variety-badge">
        📊 Batch Variety Enabled
      </div>

      <div v-if="Object.keys(rulebookInfo.contextDefaults).length > 0" class="rulebook-section">
        <h5>Context Defaults</h5>
        <div class="context-list">
          <div v-for="(value, key) in rulebookInfo.contextDefaults" :key="key" class="context-item">
            <span class="context-key">{{ key }}:</span>
            <span class="context-value">{{ value }}</span>
          </div>
        </div>
      </div>
    </div>

    <div v-if="error" class="error-message">
      <h3>Error</h3>
      <pre>{{ error }}</pre>
    </div>

    <!-- Single render output -->
    <div v-if="!batchMode && renderResult" class="render-output">
      <div class="output-header">
        <h3>Output</h3>
        <button
          @click="copyToClipboard(renderResult.output)"
          class="btn-copy"
          title="Copy to clipboard"
        >
          📋 Copy
        </button>
      </div>
      <div class="output-text">
        {{ renderResult.output }}
      </div>

      <div class="output-meta">
        <span class="meta-item">Seed: {{ renderResult.seed }}</span>
        <span v-if="renderResult.selected_values" class="meta-item">
          Selections: {{ Object.keys(renderResult.selected_values).length }}
        </span>
      </div>

      <details v-if="renderResult.selected_values" class="selected-values">
        <summary>Selected Values</summary>
        <div class="values-list">
          <div
            v-for="(value, name) in renderResult.selected_values"
            :key="name"
            class="value-item"
          >
            <span class="value-name">{{ name }}:</span>
            <span class="value-text">{{ value }}</span>
          </div>
        </div>
      </details>
    </div>

    <!-- Batch render output -->
    <div v-if="batchMode && batchResults && batchResults.length > 0" class="batch-output">
      <div class="output-header">
        <h3>Generated Prompts ({{ batchResults.length }})</h3>
        <button
          @click="copyAllBatchResults"
          class="btn-copy"
          title="Copy all to clipboard"
        >
          📋 Copy All
        </button>
      </div>
      <div class="batch-results">
        <div
          v-for="(result, idx) in batchResults"
          :key="idx"
          class="batch-result-item"
        >
          <div class="batch-header">
            <div>
              <span class="batch-number">#{{ idx + 1 }}</span>
              <span class="batch-seed">Seed: {{ result.seed }}</span>
            </div>
            <button
              @click="copyToClipboard(result.output)"
              class="btn-copy-small"
              title="Copy this prompt"
            >
              📋
            </button>
          </div>
          <div class="batch-output-text">
            {{ result.output }}
          </div>
        </div>
      </div>
    </div>
  </div>
</template>

<style scoped>
.live-preview {
  background: white;
  border-radius: 0.5rem;
  padding: 2rem;
  box-shadow: 0 1px 3px rgba(0,0,0,0.1);
}

.live-preview h2 {
  margin-top: 0;
  color: #2d3748;
}

.empty-state {
  text-align: center;
  padding: 3rem;
  color: #718096;
}

.preview-controls {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  margin: 1.5rem 0;
}

.mode-selection {
  display: flex;
  gap: 2rem;
  padding: 1rem;
  background: #f7fafc;
  border: 1px solid #e2e8f0;
  border-radius: 0.5rem;
}

.mode-selection label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
  font-weight: 600;
  color: #4a5568;
}

.mode-selection input[type="radio"] {
  cursor: pointer;
}

.control-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.control-group label {
  font-weight: 600;
  color: #4a5568;
  font-size: 0.875rem;
}

.select-input,
.number-input {
  padding: 0.5rem;
  border: 1px solid #cbd5e0;
  border-radius: 0.25rem;
  font-size: 1rem;
}

.seed-input-group {
  display: flex;
  gap: 0.5rem;
}

.seed-input-group .number-input {
  flex: 1;
}

.btn-primary,
.btn-secondary {
  padding: 0.75rem 1.5rem;
  font-size: 1rem;
  border: none;
  border-radius: 0.5rem;
  cursor: pointer;
  transition: transform 0.2s, box-shadow 0.2s;
}

.btn-primary {
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
}

.btn-primary:hover:not(:disabled) {
  transform: translateY(-2px);
  box-shadow: 0 4px 8px rgba(102, 126, 234, 0.3);
}

.btn-primary:disabled {
  opacity: 0.6;
  cursor: not-allowed;
}

.btn-secondary {
  background: #e2e8f0;
  color: #2d3748;
  padding: 0.5rem 1rem;
}

.btn-secondary:hover {
  background: #edf2f7;
}

.template-display {
  background: #f7fafc;
  border: 1px solid #e2e8f0;
  border-radius: 0.5rem;
  padding: 1rem;
  margin-top: 1.5rem;
  margin-bottom: 1.5rem;
}

.template-display h4 {
  margin: 0 0 0.5rem 0;
  color: #4a5568;
  font-size: 0.875rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.template-code {
  display: block;
  background: white;
  border: 1px solid #cbd5e0;
  border-radius: 0.25rem;
  padding: 0.75rem;
  font-family: 'Courier New', monospace;
  font-size: 0.875rem;
  color: #2d3748;
  overflow-x: auto;
}

.filter-badge {
  margin-top: 0.5rem;
  display: inline-block;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  padding: 0.25rem 0.75rem;
  border-radius: 1rem;
  font-size: 0.75rem;
  font-weight: 600;
}

/* Rulebook Display */
.rulebook-display {
  background: #f7fafc;
  border: 1px solid #e2e8f0;
  border-radius: 0.5rem;
  padding: 1rem;
  margin-top: 1.5rem;
  margin-bottom: 1.5rem;
}

.rulebook-display h4 {
  margin: 0 0 0.5rem 0;
  color: #2d3748;
  font-size: 1.125rem;
}

.rulebook-description {
  color: #4a5568;
  margin: 0 0 1rem 0;
  font-size: 0.875rem;
}

.rulebook-section {
  margin-top: 1rem;
}

.rulebook-section h5 {
  margin: 0 0 0.5rem 0;
  color: #4a5568;
  font-size: 0.75rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.entry-points-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.entry-point {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  padding: 0.5rem;
  background: white;
  border: 1px solid #e2e8f0;
  border-radius: 0.25rem;
}

.ep-number {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 24px;
  height: 24px;
  background: #667eea;
  color: white;
  border-radius: 50%;
  font-size: 0.75rem;
  font-weight: 600;
  flex-shrink: 0;
}

.ep-ref {
  flex: 1;
  font-family: 'Courier New', monospace;
  font-size: 0.875rem;
  color: #2d3748;
}

.ep-weight {
  font-size: 0.75rem;
  color: #718096;
}

.variety-badge {
  margin-top: 0.75rem;
  display: inline-block;
  background: #d4edda;
  color: #155724;
  padding: 0.5rem 1rem;
  border-radius: 0.5rem;
  font-size: 0.875rem;
  font-weight: 600;
}

.context-list {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.context-item {
  padding: 0.5rem;
  background: white;
  border: 1px solid #e2e8f0;
  border-radius: 0.25rem;
  font-family: 'Courier New', monospace;
  font-size: 0.875rem;
}

.context-key {
  color: #6f42c1;
  font-weight: 600;
  margin-right: 0.5rem;
}

.context-value {
  color: #2d3748;
}

.error-message {
  background: #fed7d7;
  border: 1px solid #fc8181;
  border-radius: 0.5rem;
  padding: 1.5rem;
  margin-top: 1rem;
}

.error-message h3 {
  color: #c53030;
  margin-top: 0;
}

.error-message pre {
  color: #742a2a;
  white-space: pre-wrap;
  word-wrap: break-word;
  margin: 0;
}

.render-output {
  margin-top: 2rem;
}

.output-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 1rem;
}

.output-header h3 {
  color: #2d3748;
  margin: 0;
}

.btn-copy {
  background: #667eea;
  color: white;
  border: none;
  border-radius: 0.5rem;
  padding: 0.5rem 1rem;
  font-size: 0.875rem;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-copy:hover {
  background: #5a67d8;
  transform: translateY(-1px);
  box-shadow: 0 2px 4px rgba(102, 126, 234, 0.3);
}

.btn-copy:active {
  transform: translateY(0);
}

.btn-copy-small {
  background: #e2e8f0;
  color: #4a5568;
  border: none;
  border-radius: 0.25rem;
  padding: 0.25rem 0.5rem;
  font-size: 0.875rem;
  cursor: pointer;
  transition: all 0.2s;
}

.btn-copy-small:hover {
  background: #cbd5e0;
}

.render-output h3 {
  color: #2d3748;
  margin-bottom: 1rem;
}

.output-text {
  background: #f7fafc;
  border: 2px solid #667eea;
  border-radius: 0.5rem;
  padding: 1.5rem;
  font-size: 1.25rem;
  font-weight: 500;
  color: #2d3748;
  margin-bottom: 1rem;
}

.output-meta {
  display: flex;
  gap: 1rem;
  font-size: 0.875rem;
  color: #718096;
  margin-bottom: 1rem;
}

.meta-item {
  background: #f7fafc;
  padding: 0.25rem 0.75rem;
  border-radius: 0.25rem;
}

.selected-values {
  background: #f7fafc;
  border: 1px solid #e2e8f0;
  border-radius: 0.5rem;
  padding: 1rem;
  margin-top: 1rem;
}

.selected-values summary {
  cursor: pointer;
  font-weight: 600;
  color: #4a5568;
  user-select: none;
}

.selected-values summary:hover {
  color: #2d3748;
}

.values-list {
  margin-top: 1rem;
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.value-item {
  display: flex;
  gap: 0.5rem;
  padding: 0.5rem;
  background: white;
  border-radius: 0.25rem;
}

.value-name {
  font-weight: 600;
  color: #4a5568;
  font-family: 'Courier New', monospace;
}

.value-text {
  color: #2d3748;
}

/* NEW: Batch generation styles */
.batch-toggle {
  padding: 0.75rem;
  background: #edf2f7;
  border-radius: 0.5rem;
}

.batch-toggle label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
  font-weight: 600;
  color: #4a5568;
}

.batch-toggle input[type="checkbox"] {
  cursor: pointer;
}

.batch-controls {
  display: flex;
  flex-direction: column;
  gap: 1rem;
  padding: 1rem;
  background: #f7fafc;
  border: 1px solid #e2e8f0;
  border-radius: 0.5rem;
}

.radio-group {
  display: flex;
  flex-direction: column;
  gap: 0.5rem;
}

.radio-group label {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  cursor: pointer;
  color: #4a5568;
  font-weight: normal;
}

.radio-group input[type="radio"] {
  cursor: pointer;
}

.batch-output {
  margin-top: 2rem;
}

.batch-output h3 {
  color: #2d3748;
  margin-bottom: 1rem;
}

.batch-results {
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.batch-result-item {
  background: #f7fafc;
  border: 1px solid #e2e8f0;
  border-radius: 0.5rem;
  padding: 1rem;
}

.batch-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 0.5rem;
  font-size: 0.875rem;
  color: #718096;
}

.batch-header > div {
  display: flex;
  gap: 1rem;
  align-items: center;
}

.batch-number {
  font-weight: 700;
  color: #667eea;
}

.batch-seed {
  font-family: 'Courier New', monospace;
}

.batch-output-text {
  background: white;
  border: 1px solid #cbd5e0;
  border-radius: 0.25rem;
  padding: 0.75rem;
  font-size: 1rem;
  color: #2d3748;
}

/* NEW: Reference definitions styles */
.references-section {
  margin-top: 1.5rem;
  padding-top: 1.5rem;
  border-top: 1px solid #e2e8f0;
}

.references-section h4 {
  margin: 0 0 1rem 0;
  color: #4a5568;
  font-size: 0.875rem;
  font-weight: 600;
  text-transform: uppercase;
  letter-spacing: 0.05em;
}

.references-list {
  display: flex;
  flex-direction: column;
  gap: 0.75rem;
}

.reference-item {
  background: white;
  border: 1px solid #cbd5e0;
  border-radius: 0.25rem;
  padding: 0.75rem;
}

.ref-header {
  display: flex;
  align-items: center;
  gap: 0.5rem;
  font-family: 'Courier New', monospace;
  font-size: 0.875rem;
}

.ref-name {
  color: #667eea;
  font-weight: 700;
}

.ref-arrow {
  color: #cbd5e0;
}

.ref-target {
  color: #2d3748;
  font-weight: 600;
}

.ref-details {
  display: flex;
  flex-wrap: wrap;
  gap: 0.5rem;
  margin-top: 0.5rem;
}

.detail-badge {
  display: inline-block;
  background: #edf2f7;
  color: #4a5568;
  padding: 0.25rem 0.5rem;
  border-radius: 0.25rem;
  font-size: 0.75rem;
  font-family: 'Courier New', monospace;
}
</style>

