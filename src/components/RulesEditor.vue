<template>
  <div class="rules-editor">
    <div class="editor-header">
      <h2>
        <span class="component-icon">⚙️</span>
        Rule: {{ ruleName }}
      </h2>
      <button @click="$emit('close')" class="btn-secondary">Close</button>
    </div>

    <div class="editor-content">
      <!-- Name Field (for creation and reference) -->
      <div class="section" v-if="ruleName === 'new-rule'">
        <div class="section-header">
          <h3>Rule Name *</h3>
        </div>
        <input
          v-model="ruleData.name"
          @input="emitUpdate"
          type="text"
          placeholder="Enter a unique name (e.g., 'article_rule', 'pluralization')"
          class="name-input"
        />
        <small class="help-text">This will be used as the ID for referencing this rule</small>
      </div>

      <!-- Rule Description -->
      <div class="section">
        <h3>Rule Description</h3>
        <p class="description">
          Rules execute during the enrichment phase and can read selected values or tags
          to write data to the context store.
        </p>
      </div>

      <!-- When Field -->
      <div class="section">
        <div class="section-header">
          <h3>When (Trigger)</h3>
        </div>

        <div class="form-group">
          <label for="when-field">Field to Check</label>
          <input
            id="when-field"
            v-model="ruleData.when"
            @input="emitUpdate"
            type="text"
            placeholder="ref:creature.tags.article or ref:creature.text"
            class="rule-input"
          />
          <small>The field that triggers this rule (e.g., ref:name.tags.key or ref:name.text)</small>
        </div>
      </div>

      <!-- Logic Expression -->
      <div class="section">
        <div class="section-header">
          <h3>Logic (Condition)</h3>
        </div>

        <div class="form-group">
          <label for="logic-expr">Logic Expression</label>
          <input
            id="logic-expr"
            v-model="ruleData.logic"
            @input="emitUpdate"
            type="text"
            placeholder="Leave empty for 'field exists' or use expression"
            class="rule-input"
          />
          <small>Optional: expression to evaluate (empty = field must exist and not be null)</small>
        </div>

        <!-- Logic Examples -->
        <div class="logic-examples">
          <details>
            <summary>Logic Expression Examples</summary>
            <ul>
              <li><strong>Empty</strong> - Field exists and is not null</li>
              <li><code>== "value"</code> - Field equals value</li>
              <li><code>!= "value"</code> - Field not equals value</li>
              <li><code>&gt; 5</code> - Field greater than 5</li>
              <li><code>&lt; 10</code> - Field less than 10</li>
            </ul>
          </details>
        </div>
      </div>

      <!-- Set Field & Value -->
      <div class="section">
        <div class="section-header">
          <h3>Set (Action)</h3>
        </div>

        <div class="form-group">
          <label for="set-field">Context Field</label>
          <input
            id="set-field"
            v-model="ruleData.set"
            @input="emitUpdate"
            type="text"
            placeholder="context.prompt.article or context.global.count"
            class="rule-input"
          />
          <small>Context field to write to (e.g., context.prompt.article, context.global.value)</small>
        </div>

        <div class="form-group">
          <label for="set-value">Value</label>
          <input
            id="set-value"
            v-model="ruleData.value"
            @input="emitUpdate"
            type="text"
            placeholder="Value to set (can use expressions)"
            class="rule-input"
          />
          <small>Value to write (can be a literal or expression like ref:name.tags.key)</small>
        </div>

        <!-- Set Examples -->
        <div class="set-examples">
          <details>
            <summary>Set Value Examples</summary>
            <ul>
              <li><code>"a"</code> - Literal string value</li>
              <li><code>ref:creature.tags.article</code> - Copy from tag</li>
              <li><code>ref:creature.text</code> - Copy from selected text</li>
              <li><code>true</code> - Boolean true</li>
              <li><code>42</code> - Number value</li>
            </ul>
          </details>
        </div>
      </div>

      <!-- Complete Example -->
      <div class="section example-section">
        <div class="section-header">
          <h3>Complete Example</h3>
        </div>

        <div class="example-card">
          <h4>Article Selection Rule:</h4>
          <div class="example-fields">
            <div class="example-field">
              <span class="field-label">When:</span>
              <code>ref:creature.tags.article</code>
            </div>
            <div class="example-field">
              <span class="field-label">Logic:</span>
              <code>(empty = exists)</code>
            </div>
            <div class="example-field">
              <span class="field-label">Set:</span>
              <code>context.prompt.article</code>
            </div>
            <div class="example-field">
              <span class="field-label">Value:</span>
              <code>ref:creature.tags.article</code>
            </div>
          </div>
          <div class="example-explanation">
            This rule reads the "article" tag from the selected creature
            and writes it to the context so templates can use it.
          </div>
        </div>
      </div>

      <!-- Current Rule Preview -->
      <div class="section preview-section">
        <div class="section-header">
          <h3>Current Rule Preview</h3>
        </div>

        <div class="rule-preview">
          <div class="preview-line">
            <strong>IF</strong>
            <code>{{ ruleData.when || '(not set)' }}</code>
            <span v-if="ruleData.logic">{{ ruleData.logic }}</span>
            <span v-else>exists</span>
          </div>
          <div class="preview-line">
            <strong>THEN</strong>
            set <code>{{ ruleData.set || '(not set)' }}</code>
            to <code>{{ ruleData.value || '(not set)' }}</code>
          </div>
        </div>

        <div v-if="!isRuleValid" class="validation-warning">
          ⚠️ Rule is incomplete. Fill in all required fields.
        </div>
      </div>

      <!-- Rule Behavior Notes -->
      <div class="section info-section">
        <h3>How Rules Work</h3>
        <ul class="info-list">
          <li><strong>Execution:</strong> Rules run during the enrichment phase (after selection, before rendering)</li>
          <li><strong>Order:</strong> Rules execute in the order they appear in the package</li>
          <li><strong>First Wins:</strong> Once a context field is set, subsequent rules won't overwrite it</li>
          <li><strong>Scope:</strong> Use <code>context.prompt.*</code> for prompt-level or <code>context.global.*</code> for global</li>
          <li><strong>References:</strong> Use <code>ref:name.tags.key</code> to read from selections</li>
        </ul>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch } from 'vue'

const props = defineProps({
  ruleName: {
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
const ruleData = ref({
  name: props.data.name || '',
  when: props.data.when || '',
  logic: props.data.logic || '',
  set: props.data.set || '',
  value: props.data.value || ''
})

// Watch for prop changes
watch(() => props.data, (newData) => {
  ruleData.value = {
    name: newData.name || '',
    when: newData.when || '',
    logic: newData.logic || '',
    set: newData.set || '',
    value: newData.value || ''
  }
}, { deep: true })

// Computed
const isRuleValid = computed(() => {
  return ruleData.value.when &&
         ruleData.value.set &&
         ruleData.value.value
})

// Methods
function emitUpdate() {
  emit('update', {
    name: ruleData.value.name || '',
    when: ruleData.value.when || '',
    logic: ruleData.value.logic || '',
    set: ruleData.value.set || '',
    value: ruleData.value.value || ''
  })
}
</script>

<style scoped>
.rules-editor {
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

.section h3 {
  margin: 0 0 8px 0;
  font-size: 16px;
  font-weight: 600;
}

.section h4 {
  margin: 0 0 12px 0;
  font-size: 14px;
  font-weight: 600;
  color: #d4d4d4;
}

.description {
  margin: 0;
  color: #858585;
  font-size: 14px;
  line-height: 1.5;
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

.rule-input {
  width: 100%;
  background: #3c3c3c;
  border: 1px solid #3e3e42;
  border-radius: 4px;
  color: #d4d4d4;
  padding: 8px 12px;
  font-size: 13px;
  font-family: 'Courier New', monospace;
}

.rule-input:focus {
  outline: none;
  border-color: #0e639c;
}

.form-group small {
  display: block;
  margin-top: 4px;
  color: #858585;
  font-size: 11px;
}

.logic-examples,
.set-examples {
  margin-top: 12px;
}

.logic-examples details,
.set-examples details {
  background: #1e1e1e;
  padding: 8px 12px;
  border-radius: 4px;
  border: 1px solid #3e3e42;
}

.logic-examples summary,
.set-examples summary {
  color: #858585;
  cursor: pointer;
  font-size: 12px;
  user-select: none;
}

.logic-examples ul,
.set-examples ul {
  margin: 8px 0 0 0;
  padding-left: 20px;
}

.logic-examples li,
.set-examples li {
  color: #858585;
  font-size: 12px;
  margin: 4px 0;
}

.logic-examples code,
.set-examples code {
  background: #3c3c3c;
  padding: 2px 4px;
  border-radius: 2px;
  color: #4fc1ff;
  font-family: 'Courier New', monospace;
}

.example-section {
  background: #1e1e1e;
  border: 1px solid #3e3e42;
}

.example-card {
  background: #252526;
  padding: 16px;
  border-radius: 4px;
  border: 1px solid #3e3e42;
}

.example-fields {
  display: flex;
  flex-direction: column;
  gap: 8px;
  margin-bottom: 12px;
}

.example-field {
  display: flex;
  align-items: baseline;
  gap: 8px;
}

.field-label {
  min-width: 60px;
  color: #858585;
  font-size: 12px;
  font-weight: 600;
}

.example-explanation {
  color: #858585;
  font-size: 12px;
  font-style: italic;
  line-height: 1.5;
  padding-top: 12px;
  border-top: 1px solid #3e3e42;
}

.preview-section {
  background: #1e1e1e;
  border: 1px solid #0e639c;
}

.rule-preview {
  background: #252526;
  padding: 16px;
  border-radius: 4px;
  font-family: 'Courier New', monospace;
}

.preview-line {
  margin: 8px 0;
  color: #d4d4d4;
  font-size: 14px;
}

.preview-line strong {
  color: #c586c0;
  margin-right: 8px;
}

.preview-line code {
  background: #3c3c3c;
  padding: 2px 6px;
  border-radius: 3px;
  color: #4fc1ff;
}

.validation-warning {
  margin-top: 12px;
  padding: 12px;
  background: #3c1f1e;
  border: 1px solid #f48771;
  border-radius: 4px;
  color: #f48771;
  font-size: 13px;
}

.info-section {
  background: #2d2d30;
  border-left: 3px solid #0e639c;
}

.info-list {
  margin: 8px 0 0 0;
  padding-left: 20px;
}

.info-list li {
  color: #d4d4d4;
  font-size: 13px;
  margin: 8px 0;
  line-height: 1.5;
}

.info-list code {
  background: #3c3c3c;
  padding: 2px 4px;
  border-radius: 2px;
  color: #4fc1ff;
  font-family: 'Courier New', monospace;
  font-size: 12px;
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

