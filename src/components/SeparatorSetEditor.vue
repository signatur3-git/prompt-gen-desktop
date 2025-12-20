<template>
  <div class="separator-editor">
    <div class="editor-header">
      <h2>
        <span class="component-icon">➗</span>
        Separator Set: {{ separatorName }}
      </h2>
      <button @click="$emit('close')" class="btn-secondary">Close</button>
    </div>

    <div class="editor-content">
      <!-- Name Field (for creation) -->
      <div class="section" v-if="separatorName === 'new-separator'">
        <div class="section-header">
          <h3>Separator Set Name *</h3>
        </div>
        <input
          v-model="separatorData.name"
          @input="emitUpdate"
          type="text"
          placeholder="Enter a unique name (e.g., 'comma_and', 'comma_or')"
          class="name-input"
        />
        <small class="help-text">This will be used as the ID for referencing this separator set</small>
      </div>

      <div class="section">
        <h3>Separator Configuration</h3>
        <p class="description">
          Define how multiple values are joined together. For example: "red, blue and green"
        </p>

        <!-- Primary Separator -->
        <div class="form-group">
          <label for="primary">Primary Separator *</label>
          <input
            id="primary"
            v-model="separatorData.primary"
            @input="emitUpdate"
            type="text"
            placeholder=", "
            class="separator-input"
          />
          <small>Used between most items (e.g., ", " for "A, B, C")</small>
        </div>

        <!-- Secondary Separator -->
        <div class="form-group">
          <label for="secondary">Secondary Separator</label>
          <input
            id="secondary"
            v-model="separatorData.secondary"
            @input="emitUpdate"
            type="text"
            placeholder=" and "
            class="separator-input"
          />
          <small>Used before the last item (e.g., " and " for "A, B and C")</small>
        </div>

        <!-- Tertiary Separator -->
        <div class="form-group">
          <label for="tertiary">Tertiary Separator (Optional)</label>
          <input
            id="tertiary"
            v-model="separatorData.tertiary"
            @input="emitUpdate"
            type="text"
            placeholder=""
            class="separator-input"
          />
          <small>Used for exactly two items (if defined, overrides primary/secondary for pairs)</small>
        </div>
      </div>

      <!-- Preview Section -->
      <div class="section preview-section">
        <h3>Preview</h3>
        <p class="description">See how your separator set will format lists:</p>

        <div class="preview-examples">
          <div class="preview-item">
            <span class="preview-label">One item:</span>
            <span class="preview-output">{{ formatPreview(['A']) }}</span>
          </div>
          <div class="preview-item">
            <span class="preview-label">Two items:</span>
            <span class="preview-output">{{ formatPreview(['A', 'B']) }}</span>
          </div>
          <div class="preview-item">
            <span class="preview-label">Three items:</span>
            <span class="preview-output">{{ formatPreview(['A', 'B', 'C']) }}</span>
          </div>
          <div class="preview-item">
            <span class="preview-label">Four items:</span>
            <span class="preview-output">{{ formatPreview(['A', 'B', 'C', 'D']) }}</span>
          </div>
        </div>

        <!-- Example with real words -->
        <div class="real-example">
          <h4>Example with colors:</h4>
          <p class="example-output">{{ formatPreview(['red', 'blue', 'green', 'yellow']) }}</p>
        </div>
      </div>

      <!-- Common Presets -->
      <div class="section">
        <h3>Common Presets</h3>
        <p class="description">Quick shortcuts for common separator patterns:</p>

        <div class="presets">
          <button @click="applyPreset('comma_and')" class="preset-btn">
            Comma And
            <span class="preset-example">"A, B and C"</span>
          </button>
          <button @click="applyPreset('comma_or')" class="preset-btn">
            Comma Or
            <span class="preset-example">"A, B or C"</span>
          </button>
          <button @click="applyPreset('comma_only')" class="preset-btn">
            Comma Only
            <span class="preset-example">"A, B, C"</span>
          </button>
          <button @click="applyPreset('space')" class="preset-btn">
            Space
            <span class="preset-example">"A B C"</span>
          </button>
          <button @click="applyPreset('slash')" class="preset-btn">
            Slash
            <span class="preset-example">"A / B / C"</span>
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, watch } from 'vue'

const props = defineProps({
  separatorName: {
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
const separatorData = ref({
  name: props.data.name,
  primary: props.data.primary || '',
  secondary: props.data.secondary || null,
  tertiary: props.data.tertiary || null
})

// Watch for prop changes
watch(() => props.data, (newData) => {
  separatorData.value = {
    name: newData.name,
    primary: newData.primary || '',
    secondary: newData.secondary || null,
    tertiary: newData.tertiary || null
  }
}, { deep: true })

// Methods
function formatPreview(items) {
  if (items.length === 0) return ''
  if (items.length === 1) return items[0]

  // Two items with tertiary separator defined
  if (items.length === 2 && separatorData.value.tertiary) {
    return items[0] + separatorData.value.tertiary + items[1]
  }

  // Two items without tertiary
  if (items.length === 2) {
    const sep = separatorData.value.secondary || separatorData.value.primary
    return items[0] + sep + items[1]
  }

  // Three or more items
  const primary = separatorData.value.primary || ''
  const secondary = separatorData.value.secondary || primary

  const allButLast = items.slice(0, -1).join(primary)
  const last = items[items.length - 1]

  return allButLast + secondary + last
}

function applyPreset(presetName) {
  const presets = {
    comma_and: {
      primary: ', ',
      secondary: ' and ',
      tertiary: null
    },
    comma_or: {
      primary: ', ',
      secondary: ' or ',
      tertiary: null
    },
    comma_only: {
      primary: ', ',
      secondary: ', ',
      tertiary: null
    },
    space: {
      primary: ' ',
      secondary: ' ',
      tertiary: null
    },
    slash: {
      primary: ' / ',
      secondary: ' / ',
      tertiary: null
    }
  }

  if (presets[presetName]) {
    separatorData.value.primary = presets[presetName].primary
    separatorData.value.secondary = presets[presetName].secondary
    separatorData.value.tertiary = presets[presetName].tertiary
    emitUpdate()
  }
}

function emitUpdate() {
  emit('update', {
    name: separatorData.value.name,
    primary: separatorData.value.primary,
    secondary: separatorData.value.secondary || null,
    tertiary: separatorData.value.tertiary || null
  })
}
</script>

<style scoped>
.separator-editor {
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
  margin: 0 0 8px 0;
  font-size: 16px;
  font-weight: 600;
}

.section h4 {
  margin: 16px 0 8px 0;
  font-size: 14px;
  font-weight: 600;
  color: #858585;
}

.description {
  margin: 0 0 20px 0;
  color: #858585;
  font-size: 14px;
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
}

.separator-input {
  width: 100%;
  background: #3c3c3c;
  border: 1px solid #3e3e42;
  border-radius: 4px;
  color: #d4d4d4;
  padding: 8px 12px;
  font-size: 14px;
  font-family: 'Courier New', monospace;
}

.separator-input:focus {
  outline: none;
  border-color: #0e639c;
}

.form-group small {
  display: block;
  margin-top: 4px;
  color: #858585;
  font-size: 12px;
}

.preview-section {
  background: #1e1e1e;
  border: 1px solid #3e3e42;
}

.preview-examples {
  display: flex;
  flex-direction: column;
  gap: 12px;
}

.preview-item {
  display: flex;
  align-items: baseline;
  gap: 12px;
  padding: 8px 12px;
  background: #252526;
  border-radius: 4px;
}

.preview-label {
  min-width: 100px;
  color: #858585;
  font-size: 13px;
}

.preview-output {
  color: #4fc1ff;
  font-family: 'Courier New', monospace;
  font-size: 14px;
}

.real-example {
  margin-top: 16px;
  padding-top: 16px;
  border-top: 1px solid #3e3e42;
}

.example-output {
  margin: 8px 0 0 0;
  padding: 12px;
  background: #252526;
  border-radius: 4px;
  color: #89d185;
  font-family: 'Courier New', monospace;
  font-size: 16px;
}

.presets {
  display: grid;
  grid-template-columns: repeat(auto-fill, minmax(200px, 1fr));
  gap: 12px;
}

.preset-btn {
  display: flex;
  flex-direction: column;
  align-items: flex-start;
  gap: 4px;
  padding: 12px;
  background: #3c3c3c;
  border: 1px solid #3e3e42;
  border-radius: 4px;
  color: #d4d4d4;
  cursor: pointer;
  transition: all 0.2s;
  font-size: 14px;
  font-weight: 500;
}

.preset-btn:hover {
  background: #505050;
  border-color: #0e639c;
}

.preset-example {
  color: #858585;
  font-size: 12px;
  font-weight: normal;
  font-family: 'Courier New', monospace;
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

