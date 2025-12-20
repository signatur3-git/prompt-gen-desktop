<template>
  <div class="datatype-editor">
    <div class="editor-header">
      <h2>
        <span class="component-icon">🎲</span>
        Datatype: {{ datatypeName }}
      </h2>
      <button @click="$emit('close')" class="btn-secondary">Close</button>
    </div>

    <div class="editor-content">
      <!-- Name Field (for creation) -->
      <div class="section" v-if="datatypeName === 'new-datatype'">
        <div class="section-header">
          <h3>Datatype Name *</h3>
        </div>
        <input
          v-model="datatypeName_editable"
          @input="emitUpdate"
          type="text"
          placeholder="Enter a unique name (e.g., 'colors', 'creatures')"
          class="name-input"
        />
        <small class="help-text">This will be used as the ID for referencing this datatype</small>
      </div>

      <!-- Values List -->
      <div class="section">
        <div class="section-header">
          <h3>Values ({{ values.length }})</h3>
          <button @click="addValue" class="btn-add">+ Add Value</button>
        </div>

        <div class="values-list">
          <div
            v-for="(value, index) in values"
            :key="index"
            class="value-item"
            :class="{ selected: selectedIndex === index }"
            @click="selectValue(index)"
          >
            <span class="drag-handle">☰</span>
            <input
              v-model="value.text"
              @input="emitUpdate"
              class="value-text-input"
              placeholder="Value text..."
            />
            <div class="weight-control">
              <input
                v-model.number="value.weight"
                @input="emitUpdate"
                type="range"
                min="1"
                max="100"
                class="weight-slider"
              />
              <span class="weight-value">{{ value.weight }}</span>
            </div>
            <button @click="removeValue(index)" class="btn-remove" title="Remove">×</button>
          </div>

          <div v-if="values.length === 0" class="empty-state">
            <p>No values yet. Click "Add Value" to get started.</p>
          </div>
        </div>
      </div>

      <!-- Selected Value Details -->
      <div v-if="selectedValue" class="section">
        <div class="section-header">
          <h3>Value Details: "{{ selectedValue.text }}"</h3>
        </div>

        <div class="tags-editor">
          <div class="tags-header">
            <h4>Tags</h4>
            <button @click="addTag" class="btn-add-small">+ Add Tag</button>
          </div>

          <div class="tags-list">
            <div
              v-for="(tagValue, tagKey) in selectedValue.tags"
              :key="tagKey"
              class="tag-item"
            >
              <input
                v-model="tagKeys[tagKey]"
                @blur="renameTag(tagKey, tagKeys[tagKey])"
                class="tag-key-input"
                placeholder="key"
              />
              <span class="tag-separator">:</span>
              <input
                v-model="selectedValue.tags[tagKey]"
                @input="emitUpdate"
                class="tag-value-input"
                placeholder="value"
              />
              <button @click="removeTag(tagKey)" class="btn-remove-small" title="Remove">×</button>
            </div>

            <div v-if="Object.keys(selectedValue.tags).length === 0" class="empty-tags">
              <p>No tags. Click "Add Tag" to add metadata.</p>
            </div>
          </div>
        </div>
      </div>

      <div v-else class="section">
        <div class="placeholder">
          <p>Select a value to edit its tags</p>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed, watch } from 'vue'

const props = defineProps({
  datatypeName: {
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
const values = ref([...props.data.values])
const selectedIndex = ref(null)
const tagKeys = ref({})
const datatypeName_editable = ref(props.data.name || '')

// Computed
const selectedValue = computed(() => {
  return selectedIndex.value !== null ? values.value[selectedIndex.value] : null
})

// Watch for prop changes
watch(() => props.data, (newData) => {
  values.value = [...newData.values]
}, { deep: true })

// Initialize tag keys for tracking renames
watch(selectedValue, (newVal) => {
  if (newVal) {
    tagKeys.value = { ...newVal.tags }
    Object.keys(newVal.tags).forEach(key => {
      tagKeys.value[key] = key
    })
  }
})

// Methods
function addValue() {
  values.value.push({
    text: '',
    weight: 1,
    tags: {}
  })
  emitUpdate()
  // Select the new value
  selectedIndex.value = values.value.length - 1
}

function removeValue(index) {
  if (confirm(`Remove value "${values.value[index].text}"?`)) {
    values.value.splice(index, 1)
    if (selectedIndex.value === index) {
      selectedIndex.value = null
    } else if (selectedIndex.value > index) {
      selectedIndex.value--
    }
    emitUpdate()
  }
}

function selectValue(index) {
  selectedIndex.value = index
}

function addTag() {
  if (selectedValue.value) {
    const newKey = `tag${Object.keys(selectedValue.value.tags).length + 1}`
    selectedValue.value.tags[newKey] = ''
    tagKeys.value[newKey] = newKey
    emitUpdate()
  }
}

function removeTag(tagKey) {
  if (selectedValue.value && confirm(`Remove tag "${tagKey}"?`)) {
    delete selectedValue.value.tags[tagKey]
    delete tagKeys.value[tagKey]
    emitUpdate()
  }
}

function renameTag(oldKey, newKey) {
  if (oldKey !== newKey && selectedValue.value) {
    const value = selectedValue.value.tags[oldKey]
    delete selectedValue.value.tags[oldKey]
    selectedValue.value.tags[newKey] = value
    tagKeys.value[newKey] = newKey
    delete tagKeys.value[oldKey]
    emitUpdate()
  }
}

function emitUpdate() {
  emit('update', {
    ...props.data,
    name: datatypeName_editable.value,
    values: values.value
  })
}
</script>

<style scoped>
.datatype-editor {
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
  padding: 16px;
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

.values-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.value-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  background: #1e1e1e;
  border-radius: 4px;
  border: 1px solid #3e3e42;
  cursor: pointer;
  transition: all 0.2s;
}

.value-item:hover {
  border-color: #0e639c;
  background: #2a2d2e;
}

.value-item.selected {
  border-color: #0e639c;
  background: #094771;
}

.drag-handle {
  color: #858585;
  cursor: grab;
  user-select: none;
}

.value-text-input {
  flex: 1;
  background: #3c3c3c;
  border: 1px solid #3e3e42;
  border-radius: 4px;
  color: #d4d4d4;
  padding: 6px 10px;
  font-size: 14px;
}

.value-text-input:focus {
  outline: none;
  border-color: #0e639c;
}

.weight-control {
  display: flex;
  align-items: center;
  gap: 8px;
  min-width: 150px;
}

.weight-slider {
  flex: 1;
}

.weight-value {
  min-width: 30px;
  text-align: right;
  color: #858585;
  font-size: 12px;
}

.btn-remove {
  background: none;
  border: none;
  color: #858585;
  font-size: 20px;
  cursor: pointer;
  padding: 0 8px;
  line-height: 1;
}

.btn-remove:hover {
  color: #f48771;
}

.tags-editor {
  margin-top: 12px;
}

.tags-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  margin-bottom: 12px;
}

.tags-header h4 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: #858585;
  text-transform: uppercase;
}

.tags-list {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.tag-item {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px;
  background: #1e1e1e;
  border-radius: 4px;
}

.tag-key-input,
.tag-value-input {
  background: #3c3c3c;
  border: 1px solid #3e3e42;
  border-radius: 4px;
  color: #d4d4d4;
  padding: 4px 8px;
  font-size: 13px;
}

.tag-key-input {
  width: 120px;
  font-weight: 500;
}

.tag-value-input {
  flex: 1;
}

.tag-key-input:focus,
.tag-value-input:focus {
  outline: none;
  border-color: #0e639c;
}

.tag-separator {
  color: #858585;
}

.btn-remove-small {
  background: none;
  border: none;
  color: #858585;
  font-size: 18px;
  cursor: pointer;
  padding: 0 4px;
}

.btn-remove-small:hover {
  color: #f48771;
}

.btn-add,
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

.btn-add:hover,
.btn-add-small:hover {
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

.empty-state,
.empty-tags,
.placeholder {
  text-align: center;
  padding: 40px 20px;
  color: #858585;
}

.empty-state p,
.empty-tags p,
.placeholder p {
  margin: 0;
}
</style>

