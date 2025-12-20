<template>
  <div class="new-package-dialog-overlay">
    <div class="dialog" @click.stop>
      <div class="dialog-header">
        <h2>Create New Package</h2>
        <button @click="handleClose" class="close-btn" title="Close">&times;</button>
      </div>

      <div class="dialog-content">
        <form @submit.prevent="createPackage">
          <!-- Package ID -->
          <div class="form-group">
            <label for="package-id">Package ID *</label>
            <input
              id="package-id"
              v-model="formData.id"
              type="text"
              placeholder="com.example.mypackage"
              pattern="[a-z][a-z0-9._-]*"
              required
            />
            <small>Lowercase alphanumeric with dots, hyphens, underscores</small>
          </div>

          <!-- Package Version -->
          <div class="form-group">
            <label for="package-version">Version *</label>
            <input
              id="package-version"
              v-model="formData.version"
              type="text"
              placeholder="1.0.0"
              pattern="\d+\.\d+\.\d+"
              required
            />
            <small>Semantic version (X.Y.Z)</small>
          </div>

          <!-- Package Name -->
          <div class="form-group">
            <label for="package-name">Package Name *</label>
            <input
              id="package-name"
              v-model="formData.name"
              type="text"
              placeholder="My Awesome Package"
              required
            />
            <small>Display name for your package</small>
          </div>

          <!-- Description -->
          <div class="form-group">
            <label for="package-description">Description</label>
            <textarea
              id="package-description"
              v-model="formData.description"
              rows="3"
              placeholder="Brief description of what this package does..."
            />
          </div>

          <!-- Authors -->
          <div class="form-group">
            <label for="package-authors">Authors</label>
            <input
              id="package-authors"
              v-model="authorsInput"
              type="text"
              placeholder="Author Name, Another Author"
            />
            <small>Comma-separated list of author names</small>
          </div>

          <!-- Default Namespace ID -->
          <div class="form-group">
            <label for="namespace-id">Default Namespace ID *</label>
            <input
              id="namespace-id"
              v-model="formData.namespaceId"
              type="text"
              placeholder="main"
              pattern="[a-z][a-z0-9_-]*"
              required
            />
            <small>First namespace for your package</small>
          </div>

          <!-- Template Selection -->
          <div class="form-group">
            <label>Start with template:</label>
            <div class="template-options">
              <label class="radio-option">
                <input type="radio" v-model="formData.template" value="empty" />
                <span>Empty Package</span>
              </label>
              <label class="radio-option">
                <input type="radio" v-model="formData.template" value="basic" />
                <span>Basic Template (sample datatype & promptsection)</span>
              </label>
            </div>
          </div>

          <!-- Actions -->
          <div class="dialog-actions">
            <button type="button" @click="handleClose" class="btn-secondary">
              Cancel
            </button>
            <button type="submit" class="btn-primary">
              Create Package
            </button>
          </div>
        </form>
      </div>
    </div>

    <!-- Confirmation Dialog -->
    <div v-if="showConfirmClose" class="confirm-overlay" @click.stop>
      <div class="confirm-dialog" @click.stop>
        <h3>Discard Changes?</h3>
        <p>You have unsaved data in the form. Are you sure you want to close?</p>
        <div class="confirm-actions">
          <button @click="showConfirmClose = false" class="btn-secondary">
            Continue Editing
          </button>
          <button @click="confirmClose" class="btn-danger">
            Discard
          </button>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref, computed } from 'vue'

const emit = defineEmits(['create', 'cancel'])

const formData = ref({
  id: '',
  version: '1.0.0',
  name: '',
  description: '',
  namespaceId: 'main',
  template: 'basic'
})

const authorsInput = ref('')
const showConfirmClose = ref(false)

// Check if form has any user input
const hasData = computed(() => {
  return formData.value.id !== '' ||
         formData.value.name !== '' ||
         formData.value.description !== '' ||
         formData.value.namespaceId !== 'main' ||
         authorsInput.value !== '' ||
         formData.value.version !== '1.0.0'
})

function handleClose() {
  if (hasData.value) {
    showConfirmClose.value = true
  } else {
    emit('cancel')
  }
}

function confirmClose() {
  showConfirmClose.value = false
  emit('cancel')
}

function createPackage() {
  // Parse authors
  const authors = authorsInput.value
    ? authorsInput.value.split(',').map(a => a.trim()).filter(a => a)
    : []

  // Create package structure
  const newPackage = {
    id: formData.value.id,
    version: formData.value.version,
    metadata: {
      name: formData.value.name,
      description: formData.value.description || undefined,
      authors: authors.length > 0 ? authors : ['Unknown Author'],
      bypass_filters: false
    },
    namespaces: {},
    dependencies: []
  }

  // Create default namespace
  const namespace = {
    id: formData.value.namespaceId,
    datatypes: {},
    prompt_sections: {},
    separator_sets: {},
    rules: []
  }

  // Add template content if basic template selected
  if (formData.value.template === 'basic') {
    // Add sample datatype
    namespace.datatypes.colors = {
      name: 'colors',
      values: [
        { text: 'red', weight: 1, tags: {} },
        { text: 'blue', weight: 1, tags: {} },
        { text: 'green', weight: 1, tags: {} }
      ],
      extends: null,
      override_tags: {}
    }

    // Add sample promptsection
    namespace.prompt_sections.simple = {
      name: 'simple',
      template: 'A {color} ball',
      references: {
        color: {
          target: 'colors',
          filter: null,
          min: 1,
          max: 1,
          separator: null,
          unique: false
        }
      }
    }

    // Add sample separator set
    namespace.separator_sets.comma_and = {
      name: 'comma_and',
      primary: ', ',
      secondary: ' and ',
      tertiary: null
    }
  }

  newPackage.namespaces[formData.value.namespaceId] = namespace

  emit('create', newPackage)
}
</script>

<style scoped>
.new-package-dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.7);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.dialog {
  background: #252526;
  border-radius: 8px;
  width: 90%;
  max-width: 600px;
  max-height: 90vh;
  display: flex;
  flex-direction: column;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.5);
}

.dialog-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 20px 24px;
  border-bottom: 1px solid #3e3e42;
}

.dialog-header h2 {
  margin: 0;
  font-size: 20px;
  font-weight: 600;
}

.close-btn {
  background: none;
  border: none;
  color: #858585;
  font-size: 28px;
  cursor: pointer;
  padding: 0;
  width: 32px;
  height: 32px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 4px;
}

.close-btn:hover {
  background: #3e3e42;
  color: #d4d4d4;
}

.dialog-content {
  padding: 24px;
  overflow-y: auto;
}

.form-group {
  margin-bottom: 20px;
}

.form-group label {
  display: block;
  margin-bottom: 8px;
  font-weight: 500;
  color: #d4d4d4;
}

.form-group input[type="text"],
.form-group textarea {
  width: 100%;
  padding: 8px 12px;
  background: #3c3c3c;
  border: 1px solid #3e3e42;
  border-radius: 4px;
  color: #d4d4d4;
  font-size: 14px;
  font-family: inherit;
}

.form-group input[type="text"]:focus,
.form-group textarea:focus {
  outline: none;
  border-color: #0e639c;
}

.form-group small {
  display: block;
  margin-top: 4px;
  color: #858585;
  font-size: 12px;
}

.template-options {
  display: flex;
  flex-direction: column;
  gap: 8px;
}

.radio-option {
  display: flex;
  align-items: center;
  gap: 8px;
  padding: 8px;
  border-radius: 4px;
  cursor: pointer;
}

.radio-option:hover {
  background: #3e3e42;
}

.radio-option input[type="radio"] {
  cursor: pointer;
}

.dialog-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
  margin-top: 24px;
  padding-top: 20px;
  border-top: 1px solid #3e3e42;
}

.btn-primary,
.btn-secondary,
.btn-danger {
  padding: 10px 20px;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 14px;
  font-weight: 500;
  transition: all 0.2s;
}

.btn-primary {
  background: #0e639c;
  color: white;
}

.btn-primary:hover {
  background: #1177bb;
}

.btn-secondary {
  background: #3e3e42;
  color: #d4d4d4;
}

.btn-secondary:hover {
  background: #505050;
}

.btn-danger {
  background: #c5392a;
  color: white;
}

.btn-danger:hover {
  background: #e53935;
}

/* Confirmation Dialog */
.confirm-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.85);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1001;
}

.confirm-dialog {
  background: #2d2d30;
  border-radius: 8px;
  padding: 24px;
  max-width: 400px;
  box-shadow: 0 8px 32px rgba(0, 0, 0, 0.7);
}

.confirm-dialog h3 {
  margin: 0 0 12px 0;
  color: #f48771;
  font-size: 18px;
}

.confirm-dialog p {
  margin: 0 0 24px 0;
  color: #d4d4d4;
  line-height: 1.5;
}

.confirm-actions {
  display: flex;
  justify-content: flex-end;
  gap: 12px;
}
</style>

