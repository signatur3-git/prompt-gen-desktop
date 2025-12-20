<script setup lang="ts">
import { ref, computed, watch } from 'vue'

interface EntryPoint {
  prompt_section: string
  weight: number
}

interface Rulebook {
  name: string
  description: string
  entry_points: EntryPoint[]
  batch_variety: boolean
  context_defaults: Record<string, string>
}

const props = defineProps<{
  rulebookName: string
  data: Rulebook
}>()

const emit = defineEmits<{
  update: [data: Rulebook]
  close: []
}>()

// Local editable state
const localData = ref<Rulebook>({
  name: '',
  description: '',
  entry_points: [],
  batch_variety: false,
  context_defaults: {}
})

// Initialize from props
watch(() => props.data, (newData) => {
  if (newData) {
    localData.value = JSON.parse(JSON.stringify(newData))
  }
}, { immediate: true })

// Add new entry point
function addEntryPoint() {
  localData.value.entry_points.push({
    prompt_section: '',
    weight: 1.0
  })
  emitUpdate()
}

// Remove entry point
function removeEntryPoint(index: number) {
  localData.value.entry_points.splice(index, 1)
  emitUpdate()
}

// Add context default
const newContextKey = ref('')
const newContextValue = ref('')

function addContextDefault() {
  if (newContextKey.value.trim()) {
    localData.value.context_defaults[newContextKey.value.trim()] = newContextValue.value
    newContextKey.value = ''
    newContextValue.value = ''
    emitUpdate()
  }
}

// Remove context default
function removeContextDefault(key: string) {
  delete localData.value.context_defaults[key]
  emitUpdate()
}

// Calculate total weight
const totalWeight = computed(() => {
  return localData.value.entry_points.reduce((sum, ep) => sum + (ep.weight || 0), 0)
})

// Emit update
function emitUpdate() {
  emit('update', JSON.parse(JSON.stringify(localData.value)))
}

// Watch for changes and emit
watch(localData, () => {
  emitUpdate()
}, { deep: true })
</script>

<template>
  <div class="rulebook-editor">
    <div class="editor-header">
      <h2>{{ rulebookName }}</h2>
      <button @click="$emit('close')" class="btn-close">×</button>
    </div>

    <div class="editor-content">
      <!-- Basic Info -->
      <div class="section">
        <h3>Basic Information</h3>

        <div class="form-group">
          <label>Name</label>
          <input
            v-model="localData.name"
            type="text"
            placeholder="Rulebook name"
          />
        </div>

        <div class="form-group">
          <label>Description</label>
          <textarea
            v-model="localData.description"
            rows="3"
            placeholder="Describe what this rulebook does..."
          />
        </div>

        <div class="form-group">
          <label>
            <input
              v-model="localData.batch_variety"
              type="checkbox"
            />
            Enable Batch Variety
            <span class="help-text">Avoid repeating the same entry point in batch renders</span>
          </label>
        </div>
      </div>

      <!-- Entry Points -->
      <div class="section">
        <div class="section-header-row">
          <h3>Entry Points ({{ localData.entry_points.length }})</h3>
          <button @click="addEntryPoint" class="btn-add">+ Add Entry Point</button>
        </div>

        <div v-if="localData.entry_points.length === 0" class="empty-state">
          No entry points defined. Add at least one entry point.
        </div>

        <div v-else class="entry-points-list">
          <div
            v-for="(ep, index) in localData.entry_points"
            :key="index"
            class="entry-point-item"
          >
            <div class="ep-number">{{ index + 1 }}</div>

            <div class="ep-fields">
              <div class="form-group">
                <label>Prompt Section Reference</label>
                <input
                  v-model="ep.prompt_section"
                  type="text"
                  placeholder="namespace:promptsection"
                />
              </div>

              <div class="form-group">
                <label>Weight</label>
                <input
                  v-model.number="ep.weight"
                  type="number"
                  min="0"
                  step="0.1"
                />
              </div>
            </div>

            <button
              @click="removeEntryPoint(index)"
              class="btn-remove"
              title="Remove entry point"
            >
              🗑️
            </button>
          </div>

          <div class="total-weight">
            Total Weight: {{ totalWeight.toFixed(2) }}
          </div>
        </div>
      </div>

      <!-- Context Defaults -->
      <div class="section">
        <h3>Context Defaults ({{ Object.keys(localData.context_defaults).length }})</h3>

        <div class="context-list">
          <div
            v-for="(value, key) in localData.context_defaults"
            :key="key"
            class="context-item"
          >
            <span class="context-key">{{ key }}:</span>
            <span class="context-value">{{ value }}</span>
            <button
              @click="removeContextDefault(String(key))"
              class="btn-remove-small"
              title="Remove"
            >
              ×
            </button>
          </div>

          <div v-if="Object.keys(localData.context_defaults).length === 0" class="empty-state-small">
            No context defaults. These are optional.
          </div>
        </div>

        <div class="add-context-form">
          <input
            v-model="newContextKey"
            type="text"
            placeholder="Key (e.g., mood or global:time)"
            @keyup.enter="addContextDefault"
          />
          <input
            v-model="newContextValue"
            type="text"
            placeholder="Value"
            @keyup.enter="addContextDefault"
          />
          <button
            @click="addContextDefault"
            class="btn-add-small"
            :disabled="!newContextKey.trim()"
          >
            Add
          </button>
        </div>

        <p class="help-text">
          Context defaults are applied before rendering. Use format "key" for prompt scope
          or "scope:key" for other scopes (e.g., "global:time", "session:theme").
        </p>
      </div>
    </div>
  </div>
</template>

<style scoped>
.rulebook-editor {
  height: 100%;
  display: flex;
  flex-direction: column;
  background: #1e1e1e;
  color: #cccccc;
}

.editor-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 16px 20px;
  border-bottom: 1px solid #3e3e42;
}

.editor-header h2 {
  margin: 0;
  font-size: 18px;
  color: #ffffff;
}

.btn-close {
  background: transparent;
  border: none;
  color: #858585;
  font-size: 32px;
  line-height: 1;
  cursor: pointer;
  padding: 0;
  width: 32px;
  height: 32px;
}

.btn-close:hover {
  color: #ffffff;
}

.editor-content {
  flex: 1;
  overflow-y: auto;
  padding: 20px;
}

.section {
  margin-bottom: 32px;
  padding: 20px;
  background: #252526;
  border-radius: 4px;
}

.section h3 {
  margin: 0 0 16px 0;
  font-size: 16px;
  color: #ffffff;
}

.section-header-row {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.section-header-row h3 {
  margin: 0;
}

.form-group {
  margin-bottom: 16px;
}

.form-group label {
  display: block;
  margin-bottom: 6px;
  font-size: 13px;
  color: #858585;
}

.form-group input[type="text"],
.form-group input[type="number"],
.form-group textarea {
  width: 100%;
  padding: 8px 12px;
  background: #3c3c3c;
  border: 1px solid #3e3e42;
  border-radius: 4px;
  color: #cccccc;
  font-family: inherit;
  font-size: 14px;
}

.form-group input[type="text"]:focus,
.form-group input[type="number"]:focus,
.form-group textarea:focus {
  outline: none;
  border-color: #007acc;
}

.form-group input[type="checkbox"] {
  margin-right: 8px;
}

.help-text {
  display: block;
  margin-top: 4px;
  font-size: 12px;
  color: #858585;
  font-style: italic;
}

.btn-add {
  padding: 8px 16px;
  background: #0e639c;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
}

.btn-add:hover {
  background: #1177bb;
}

.btn-add-small {
  padding: 6px 12px;
  background: #0e639c;
  color: white;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 13px;
}

.btn-add-small:hover {
  background: #1177bb;
}

.btn-add-small:disabled {
  background: #3e3e42;
  cursor: not-allowed;
}

.empty-state {
  padding: 24px;
  text-align: center;
  color: #858585;
  font-style: italic;
}

.empty-state-small {
  padding: 12px;
  text-align: center;
  color: #858585;
  font-size: 13px;
  font-style: italic;
}

/* Entry Points */
.entry-points-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.entry-point-item {
  display: flex;
  gap: 12px;
  align-items: flex-start;
  padding: 16px;
  background: #1e1e1e;
  border: 1px solid #3e3e42;
  border-radius: 4px;
}

.ep-number {
  display: flex;
  align-items: center;
  justify-content: center;
  width: 32px;
  height: 32px;
  background: #0e639c;
  color: white;
  border-radius: 50%;
  font-weight: 600;
  flex-shrink: 0;
}

.ep-fields {
  flex: 1;
  display: grid;
  grid-template-columns: 2fr 1fr;
  gap: 12px;
}

.btn-remove {
  background: transparent;
  border: none;
  color: #858585;
  font-size: 18px;
  cursor: pointer;
  padding: 4px;
  flex-shrink: 0;
}

.btn-remove:hover {
  color: #f48771;
}

.total-weight {
  margin-top: 8px;
  padding: 12px;
  background: #2d2d30;
  border-radius: 4px;
  font-weight: 600;
  color: #ffffff;
  text-align: right;
}

/* Context Defaults */
.context-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: 16px;
}

.context-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px 12px;
  background: #1e1e1e;
  border: 1px solid #3e3e42;
  border-radius: 4px;
  font-family: 'Courier New', monospace;
  font-size: 13px;
}

.context-key {
  color: #9cdcfe;
  font-weight: 600;
}

.context-value {
  flex: 1;
  color: #ce9178;
}

.btn-remove-small {
  background: transparent;
  border: none;
  color: #858585;
  font-size: 20px;
  line-height: 1;
  cursor: pointer;
  padding: 0 4px;
}

.btn-remove-small:hover {
  color: #f48771;
}

.add-context-form {
  display: grid;
  grid-template-columns: 1fr 1fr auto;
  gap: 8px;
}

.add-context-form input {
  padding: 8px 12px;
  background: #3c3c3c;
  border: 1px solid #3e3e42;
  border-radius: 4px;
  color: #cccccc;
  font-size: 13px;
}

.add-context-form input:focus {
  outline: none;
  border-color: #007acc;
}
</style>

