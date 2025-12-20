<template>
  <div class="promptsection-editor">
    <div class="editor-header">
      <h2>
        <span class="component-icon">📝</span>
        Prompt Section: {{ promptSectionName }}
      </h2>
      <button @click="$emit('close')" class="btn-secondary">Close</button>
    </div>

    <div class="editor-content">
      <!-- Name Field (for creation) -->
      <div class="section" v-if="promptSectionName === 'new-promptsection'">
        <div class="section-header">
          <h3>Prompt Section Name *</h3>
        </div>
        <input
          v-model="promptData.name"
          @input="emitUpdate"
          type="text"
          placeholder="Enter a unique name (e.g., 'character_description')"
          class="name-input"
        />
        <small class="help-text">This will be used as the ID for referencing this prompt section</small>
      </div>

      <!-- Template Editor -->
      <div class="section">
        <div class="section-header">
          <h3>Template</h3>
          <button @click="insertReference" class="btn-add-small">+ Insert Reference</button>
        </div>

        <textarea
          v-model="promptData.template"
          @input="emitUpdate"
          class="template-input"
          placeholder="Enter template text with {references} in curly braces..."
          rows="8"
        />

        <div class="template-help">
          <p class="help-text">
            Use <code>{reference_name}</code> to insert references.
            Configure each reference below.
          </p>
        </div>
      </div>

      <!-- References Configuration -->
      <div class="section">
        <div class="section-header">
          <h3>References ({{ Object.keys(promptData.references).length }})</h3>
        </div>

        <div v-if="Object.keys(promptData.references).length === 0" class="empty-state">
          <p>No references defined yet. Add {reference_name} to your template above.</p>
        </div>

        <div v-else class="references-list">
          <div
            v-for="(ref, refName) in promptData.references"
            :key="refName"
            class="reference-item"
            :class="{ selected: selectedRef === refName }"
            @click="selectedRef = refName"
          >
            <div class="ref-header">
              <span class="ref-icon">🔗</span>
              <span class="ref-name">{{ refName }}</span>
              <button @click.stop="removeReference(refName)" class="btn-remove-small" title="Remove">×</button>
            </div>

            <div v-if="selectedRef === refName" class="ref-details">
              <!-- Target -->
              <div class="form-group">
                <label>Target Datatype/PromptSection *</label>
                <input
                  v-model="ref.target"
                  @input="emitUpdate"
                  type="text"
                  placeholder="namespace:name or just name"
                  class="ref-input"
                />
                <small>The datatype or promptsection to reference</small>
              </div>

              <!-- Min/Max -->
              <div class="form-row">
                <div class="form-group">
                  <label>Min</label>
                  <input
                    v-model.number="ref.min"
                    @input="emitUpdate"
                    type="number"
                    min="0"
                    class="ref-input-small"
                  />
                  <small>Minimum values (0 = optional)</small>
                </div>
                <div class="form-group">
                  <label>Max</label>
                  <input
                    v-model.number="ref.max"
                    @input="emitUpdate"
                    type="number"
                    min="1"
                    class="ref-input-small"
                  />
                  <small>Maximum values</small>
                </div>
              </div>

              <!-- Separator -->
              <div class="form-group">
                <label>Separator Set</label>
                <input
                  v-model="ref.separator"
                  @input="emitUpdate"
                  type="text"
                  placeholder="namespace:separator or just separator"
                  class="ref-input"
                />
                <small>Separator set to use when joining multiple values (leave empty for space)</small>
              </div>

              <!-- Unique Checkbox -->
              <div class="form-group">
                <label class="checkbox-label">
                  <input
                    v-model="ref.unique"
                    @input="emitUpdate"
                    type="checkbox"
                  />
                  <span>Unique values only (no duplicates)</span>
                </label>
              </div>

              <!-- Tag Filter -->
              <div class="form-group">
                <label>Tag Filter</label>
                <input
                  v-model="ref.filter"
                  @input="emitUpdate"
                  type="text"
                  placeholder="tags.can_fly or tags.type == 'melee'"
                  class="ref-input"
                />
                <small>Filter expression to limit which values can be selected</small>
              </div>

              <!-- Filter Examples -->
              <div class="filter-examples">
                <details>
                  <summary>Filter Expression Examples</summary>
                  <ul>
                    <li><code>tags.can_fly</code> - Boolean tag is true</li>
                    <li><code>tags.type == "melee"</code> - Tag equals value</li>
                    <li><code>tags.type != "ranged"</code> - Tag not equals value</li>
                    <li><code>tags.can_fly AND tags.is_fast</code> - Both conditions</li>
                    <li><code>tags.can_fly OR tags.can_swim</code> - Either condition</li>
                    <li><code>NOT tags.is_rare</code> - Negation</li>
                  </ul>
                </details>
              </div>
            </div>
          </div>
        </div>
      </div>

      <!-- Add New Reference -->
      <div class="section">
        <div class="section-header">
          <h3>Add Reference</h3>
        </div>

        <div class="add-reference-form">
          <input
            v-model="newRefName"
            @keyup.enter="addReference"
            type="text"
            placeholder="reference_name"
            class="ref-input"
          />
          <button @click="addReference" class="btn-primary">Add Reference</button>
        </div>

        <small>Add a new reference, then use it in the template as <code>{reference_name}</code></small>
      </div>

      <!-- Preview Section -->
      <div class="section preview-section">
        <div class="section-header">
          <h3>Preview</h3>
        </div>

        <div class="template-preview">
          <div class="preview-label">Template with references highlighted:</div>
          <div class="preview-output" v-html="highlightedTemplate"></div>
        </div>

        <div class="reference-summary">
          <div class="preview-label">Reference Summary:</div>
          <ul>
            <li v-for="(ref, refName) in promptData.references" :key="refName">
              <strong>{{ refName }}</strong> →
              {{ ref.target }}
              <span v-if="ref.min !== 1 || ref.max !== 1">({{ ref.min }}-{{ ref.max }})</span>
              <span v-if="ref.filter" class="filter-badge">filtered</span>
              <span v-if="ref.unique" class="unique-badge">unique</span>
            </li>
          </ul>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch } from 'vue'

const props = defineProps({
  promptSectionName: {
    type: String,
    required: true
  },
  data: {
    type: Object,
    required: true
  }
})

const emit = defineEmits(['update', 'close'])

// Local state
const promptData = ref({
  name: props.data.name,
  template: props.data.template || '',
  references: { ...props.data.references } || {}
})

const selectedRef = ref(null)
const newRefName = ref('')

// Watch for prop changes
watch(() => props.data, (newData) => {
  promptData.value = {
    name: newData.name,
    template: newData.template || '',
    references: { ...newData.references } || {}
  }
}, { deep: true })

// Computed
const highlightedTemplate = computed(() => {
  let template = promptData.value.template || ''

  // Escape HTML
  template = template.replace(/</g, '&lt;').replace(/>/g, '&gt;')

  // Highlight references
  template = template.replace(/\{([^}]+)\}/g, (match, refName) => {
    const exists = promptData.value.references[refName]
    const color = exists ? '#4fc1ff' : '#f48771'
    return `<span style="color: ${color}; font-weight: bold;">${match}</span>`
  })

  return template
})

// Methods
function addReference() {
  const name = newRefName.value.trim()
  if (!name) return

  if (promptData.value.references[name]) {
    alert(`Reference "${name}" already exists`)
    return
  }

  promptData.value.references[name] = {
    target: '',
    filter: null,
    min: 1,
    max: 1,
    separator: null,
    unique: false
  }

  newRefName.value = ''
  selectedRef.value = name
  emitUpdate()
}

function removeReference(refName) {
  if (confirm(`Remove reference "${refName}"?\n\nMake sure to remove {${refName}} from the template too.`)) {
    delete promptData.value.references[refName]
    if (selectedRef.value === refName) {
      selectedRef.value = null
    }
    emitUpdate()
  }
}

function insertReference() {
  const name = prompt('Enter reference name:')
  if (!name) return

  // Add to references if not exists
  if (!promptData.value.references[name]) {
    promptData.value.references[name] = {
      target: '',
      filter: null,
      min: 1,
      max: 1,
      separator: null,
      unique: false
    }
  }

  // Insert into template at cursor (simplified - just append for now)
  promptData.value.template += `{${name}}`
  selectedRef.value = name
  emitUpdate()
}

function emitUpdate() {
  console.log('🔔 PromptSectionEditor emitting update:', {
    name: promptData.value.name,
    template: promptData.value.template,
    refCount: Object.keys(promptData.value.references).length
  })
  emit('update', {
    name: promptData.value.name,
    template: promptData.value.template,
    references: promptData.value.references
  })
}
</script>

<style scoped>
.promptsection-editor {
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

.section-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 16px;
}

.section-header h3 {
  margin: 0;
  font-size: 16px;
  font-weight: 600;
}

.template-input {
  width: 100%;
  background: #1e1e1e;
  border: 1px solid #3e3e42;
  border-radius: 4px;
  color: #d4d4d4;
  padding: 12px;
  font-size: 14px;
  font-family: 'Courier New', monospace;
  resize: vertical;
  min-height: 120px;
}

.template-input:focus {
  outline: none;
  border-color: #0e639c;
}

.template-help {
  margin-top: 12px;
}

.help-text {
  margin: 0;
  color: #858585;
  font-size: 13px;
}

.help-text code {
  background: #3c3c3c;
  padding: 2px 6px;
  border-radius: 3px;
  color: #4fc1ff;
  font-family: 'Courier New', monospace;
}

.references-list {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.reference-item {
  background: #1e1e1e;
  border: 1px solid #3e3e42;
  border-radius: 4px;
  padding: 12px;
  cursor: pointer;
  transition: all 0.2s;
}

.reference-item:hover {
  border-color: #0e639c;
}

.reference-item.selected {
  border-color: #0e639c;
  background: #2a2d2e;
}

.ref-header {
  display: flex;
  align-items: center;
  gap: 8px;
}

.ref-icon {
  font-size: 16px;
}

.ref-name {
  flex: 1;
  font-weight: 600;
  color: #4fc1ff;
  font-family: 'Courier New', monospace;
}

.ref-details {
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px solid #3e3e42;
}

.form-group {
  margin-bottom: 16px;
}

.form-group:last-child {
  margin-bottom: 0;
}

.form-group label {
  display: block;
  margin-bottom: 6px;
  font-weight: 500;
  color: #d4d4d4;
  font-size: 13px;
}

.form-row {
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 16px;
}

.ref-input,
.ref-input-small {
  width: 100%;
  background: #3c3c3c;
  border: 1px solid #3e3e42;
  border-radius: 4px;
  color: #d4d4d4;
  padding: 6px 10px;
  font-size: 13px;
}

.ref-input:focus,
.ref-input-small:focus {
  outline: none;
  border-color: #0e639c;
}

.form-group small {
  display: block;
  margin-top: 4px;
  color: #858585;
  font-size: 11px;
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

.filter-examples {
  margin-top: 8px;
}

.filter-examples details {
  background: #1e1e1e;
  padding: 8px 12px;
  border-radius: 4px;
  border: 1px solid #3e3e42;
}

.filter-examples summary {
  color: #858585;
  cursor: pointer;
  font-size: 12px;
  user-select: none;
}

.filter-examples ul {
  margin: 8px 0 0 0;
  padding-left: 20px;
}

.filter-examples li {
  color: #858585;
  font-size: 12px;
  margin: 4px 0;
}

.filter-examples code {
  background: #3c3c3c;
  padding: 2px 4px;
  border-radius: 2px;
  color: #4fc1ff;
  font-family: 'Courier New', monospace;
}

.add-reference-form {
  display: flex;
  gap: 12px;
  align-items: center;
}

.preview-section {
  background: #1e1e1e;
  border: 1px solid #3e3e42;
}

.template-preview {
  margin-bottom: 16px;
}

.preview-label {
  color: #858585;
  font-size: 12px;
  margin-bottom: 8px;
  text-transform: uppercase;
  font-weight: 600;
}

.preview-output {
  background: #252526;
  padding: 12px;
  border-radius: 4px;
  font-family: 'Courier New', monospace;
  font-size: 14px;
  line-height: 1.6;
  white-space: pre-wrap;
  word-wrap: break-word;
}

.reference-summary ul {
  margin: 8px 0 0 0;
  padding-left: 20px;
}

.reference-summary li {
  color: #d4d4d4;
  font-size: 13px;
  margin: 6px 0;
}

.filter-badge,
.unique-badge {
  display: inline-block;
  padding: 2px 6px;
  border-radius: 3px;
  font-size: 10px;
  font-weight: 600;
  text-transform: uppercase;
  margin-left: 4px;
}

.filter-badge {
  background: #c586c0;
  color: #1e1e1e;
}

.unique-badge {
  background: #4fc1ff;
  color: #1e1e1e;
}

.btn-primary {
  background: #0e639c;
  color: white;
  border: none;
  border-radius: 4px;
  padding: 8px 16px;
  cursor: pointer;
  font-size: 13px;
  font-weight: 500;
}

.btn-primary:hover {
  background: #1177bb;
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

.btn-add-small {
  background: #0e639c;
  color: white;
  border: none;
  border-radius: 4px;
  padding: 6px 12px;
  cursor: pointer;
  font-size: 13px;
  font-weight: 500;
}

.btn-add-small:hover {
  background: #1177bb;
}

.btn-remove-small {
  background: none;
  border: none;
  color: #858585;
  font-size: 18px;
  cursor: pointer;
  padding: 0 6px;
  margin-left: auto;
}

.btn-remove-small:hover {
  color: #f48771;
}

.empty-state {
  text-align: center;
  padding: 40px 20px;
  color: #858585;
}

.empty-state p {
  margin: 0;
}
</style>

