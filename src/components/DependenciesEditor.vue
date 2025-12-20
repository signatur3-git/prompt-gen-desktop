<script setup lang="ts">
import { ref, computed } from 'vue'

interface Dependency {
  package: string
  version: string
  path?: string
}

const props = defineProps<{
  modelValue: Dependency[]
}>()

const emit = defineEmits<{
  (e: 'update:modelValue', value: Dependency[]): void
}>()

const dependencies = computed({
  get: () => props.modelValue || [],
  set: (value) => emit('update:modelValue', value),
})

const newDep = ref<Dependency>({
  package: '',
  version: '',
  path: '',
})

const validationError = ref('')

const canAdd = computed(() => {
  return newDep.value.package && newDep.value.version
})

function addDependency() {
  validationError.value = ''
  
  // Validate package ID format (namespace.name)
  if (!/^[a-z0-9-]+\.[a-z0-9-]+$/.test(newDep.value.package)) {
    validationError.value = 'Invalid package ID format. Use: namespace.name (e.g., "prompt-gen.common")'
    return
  }
  
  // Validate semver format (flexible or exact)
  const versionPattern = /^(\^|~|>=|<=|>|<)?\d+\.\d+\.\d+$/
  if (!versionPattern.test(newDep.value.version)) {
    validationError.value = 'Invalid semver format. Examples: 1.0.0, ^1.0.0, ~1.2.0'
    return
  }
  
  // Check for duplicates
  if (dependencies.value.some(d => d.package === newDep.value.package)) {
    validationError.value = `Dependency "${newDep.value.package}" already exists`
    return
  }
  
  // Add dependency
  const newDepCopy: Dependency = {
    package: newDep.value.package,
    version: newDep.value.version,
  }
  
  if (newDep.value.path && newDep.value.path.trim()) {
    newDepCopy.path = newDep.value.path.trim()
  }
  
  dependencies.value = [...dependencies.value, newDepCopy]
  
  // Reset form
  newDep.value = { package: '', version: '', path: '' }
}

function removeDependency(index: number) {
  dependencies.value = dependencies.value.filter((_, i) => i !== index)
}
</script>

<template>
  <div class="dependencies-editor">
    <h3>Package Dependencies</h3>

    <!-- Dependencies List -->
    <div v-if="dependencies.length > 0" class="dependencies-list">
      <div v-for="(dep, index) in dependencies" :key="index" class="dependency-item">
        <div class="dependency-info">
          <strong>{{ dep.package }}</strong>
          <code class="version">{{ dep.version }}</code>
          <span v-if="dep.path" class="path">📁 {{ dep.path }}</span>
        </div>
        <button @click="removeDependency(index)" class="btn-remove" title="Remove dependency">
          🗑️
        </button>
      </div>
    </div>

    <div v-else class="no-dependencies">
      <p>No dependencies defined</p>
      <p class="hint">Dependencies allow you to reference datatypes and prompt sections from other packages.</p>
    </div>

    <!-- Add Dependency Form -->
    <div class="add-dependency-form">
      <h4>Add Dependency</h4>

      <div class="form-field">
        <label for="dep-package">Package ID *</label>
        <input
          id="dep-package"
          v-model="newDep.package"
          type="text"
          placeholder="namespace.name (e.g., prompt-gen.common)"
          @keyup.enter="canAdd && addDependency()"
        />
        <span class="help-text">Format: namespace.name</span>
      </div>

      <div class="form-field">
        <label for="dep-version">Version *</label>
        <input
          id="dep-version"
          v-model="newDep.version"
          type="text"
          placeholder="1.0.0, ^1.0.0, or ~1.2.0"
          @keyup.enter="canAdd && addDependency()"
        />
        <span class="help-text">
          <strong>Semver formats:</strong><br>
          • Exact: <code>1.0.0</code> - Only this version<br>
          • Caret: <code>^1.0.0</code> - Compatible (1.x.y, not 2.0.0)<br>
          • Tilde: <code>~1.2.0</code> - Patch updates (1.2.x, not 1.3.0)<br>
          <em style="font-size: 0.75rem; color: #9ca3af;">Marketplace locks to exact versions at publish time.</em>
        </span>
      </div>

      <div class="form-field">
        <label for="dep-path">Local Path (optional)</label>
        <input
          id="dep-path"
          v-model="newDep.path"
          type="text"
          placeholder="./packages/common.yaml"
          @keyup.enter="canAdd && addDependency()"
        />
        <span class="help-text">Path to local package file (if not in search paths)</span>
      </div>

      <button
        @click="addDependency"
        :disabled="!canAdd"
        class="btn-add"
      >
        Add Dependency
      </button>
    </div>

    <!-- Validation Messages -->
    <div v-if="validationError" class="validation-error">
      ⚠️ {{ validationError }}
    </div>
  </div>
</template>

<style scoped>
.dependencies-editor {
  background: white;
  border-radius: 0.5rem;
  padding: 1.5rem;
  border: 1px solid #e2e8f0;
}

.dependencies-editor h3 {
  margin-top: 0;
  margin-bottom: 1.5rem;
  color: #2d3748;
}

.dependencies-list {
  margin-bottom: 1.5rem;
}

.dependency-item {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 0.75rem 1rem;
  border: 1px solid #cbd5e0;
  border-radius: 0.375rem;
  margin-bottom: 0.5rem;
  background: #f7fafc;
  transition: all 0.2s;
}

.dependency-item:hover {
  border-color: #4299e1;
  box-shadow: 0 2px 4px rgba(66, 153, 225, 0.1);
}

.dependency-info {
  display: flex;
  gap: 0.75rem;
  align-items: center;
  flex: 1;
}

.dependency-info strong {
  color: #2d3748;
  font-size: 0.95rem;
}

.dependency-info .version {
  background: #edf2f7;
  padding: 0.25rem 0.5rem;
  border-radius: 0.25rem;
  font-size: 0.875rem;
  font-family: 'Courier New', monospace;
  border: 1px solid #cbd5e0;
  color: #2d3748;
}

.dependency-info .path {
  color: #718096;
  font-size: 0.875rem;
  font-family: 'Courier New', monospace;
}

.btn-remove {
  background: none;
  border: none;
  font-size: 1.25rem;
  cursor: pointer;
  padding: 0.25rem 0.5rem;
  opacity: 0.6;
  transition: opacity 0.2s;
}

.btn-remove:hover {
  opacity: 1;
}

.no-dependencies {
  padding: 2rem;
  text-align: center;
  color: #a0aec0;
  background: #f7fafc;
  border-radius: 0.375rem;
  border: 1px dashed #cbd5e0;
  margin-bottom: 1.5rem;
}

.no-dependencies p {
  margin: 0.5rem 0;
}

.no-dependencies .hint {
  font-size: 0.875rem;
  color: #718096;
}

.add-dependency-form {
  background: #f7fafc;
  border: 1px solid #e2e8f0;
  padding: 1.5rem;
  border-radius: 0.5rem;
}

.add-dependency-form h4 {
  margin-top: 0;
  margin-bottom: 1rem;
  color: #2d3748;
}

.form-field {
  margin-bottom: 1rem;
}

.form-field label {
  display: block;
  margin-bottom: 0.375rem;
  font-weight: 600;
  color: #4a5568;
  font-size: 0.875rem;
}

.form-field input {
  width: 100%;
  padding: 0.625rem;
  border: 1px solid #cbd5e0;
  border-radius: 0.375rem;
  font-family: inherit;
  font-size: 0.9375rem;
  transition: all 0.2s;
}

.form-field input:focus {
  outline: none;
  border-color: #4299e1;
  box-shadow: 0 0 0 3px rgba(66, 153, 225, 0.1);
}

.help-text {
  display: block;
  margin-top: 0.25rem;
  font-size: 0.8125rem;
  color: #718096;
}

.btn-add {
  background: linear-gradient(135deg, #4299e1 0%, #3182ce 100%);
  color: white;
  border: none;
  padding: 0.75rem 1.5rem;
  border-radius: 0.375rem;
  cursor: pointer;
  font-weight: 600;
  font-size: 0.9375rem;
  transition: all 0.2s;
  margin-top: 0.5rem;
}

.btn-add:hover:not(:disabled) {
  transform: translateY(-1px);
  box-shadow: 0 4px 8px rgba(66, 153, 225, 0.3);
}

.btn-add:disabled {
  background: #cbd5e0;
  cursor: not-allowed;
  transform: none;
}

.validation-error {
  margin-top: 1rem;
  padding: 0.75rem 1rem;
  background: #fed7d7;
  color: #c53030;
  border: 1px solid #fc8181;
  border-radius: 0.375rem;
  font-size: 0.9375rem;
}
</style>

