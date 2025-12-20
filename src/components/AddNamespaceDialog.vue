<template>
  <div class="add-namespace-dialog-overlay">
    <div class="dialog" @click.stop>
      <div class="dialog-header">
        <h2>Add Namespace</h2>
        <button @click="$emit('cancel')" class="close-btn" title="Close">&times;</button>
      </div>

      <div class="dialog-content">
        <form @submit.prevent="addNamespace">
          <!-- Namespace ID -->
          <div class="form-group">
            <label for="namespace-id">Namespace ID *</label>
            <input
              id="namespace-id"
              v-model="namespaceId"
              type="text"
              placeholder="e.g., common, featured.common, myapp.custom"
              pattern="[a-z][a-z0-9._-]*"
              required
              class="input"
            />
            <small>Lowercase alphanumeric with dots, underscores, hyphens (e.g., "featured.common")</small>
          </div>

          <!-- Actions -->
          <div class="dialog-actions">
            <button type="button" @click="$emit('cancel')" class="btn-secondary">
              Cancel
            </button>
            <button type="submit" class="btn-primary">
              Add Namespace
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup>
import { ref } from 'vue'

const emit = defineEmits(['add', 'cancel'])

const namespaceId = ref('')

function addNamespace() {
  if (!namespaceId.value.trim()) return

  if (!/^[a-z][a-z0-9._-]*$/.test(namespaceId.value)) {
    alert('Namespace ID must start with lowercase letter and contain only lowercase letters, numbers, dots, underscores, and hyphens')
    return
  }

  emit('add', namespaceId.value.trim())
}
</script>

<style scoped>
.add-namespace-dialog-overlay {
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
  max-width: 500px;
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

.input {
  width: 100%;
  padding: 8px 12px;
  background: #3c3c3c;
  border: 1px solid #3e3e42;
  border-radius: 4px;
  color: #d4d4d4;
  font-size: 14px;
  font-family: inherit;
}

.input:focus {
  outline: none;
  border-color: #0e639c;
}

.form-group small {
  display: block;
  margin-top: 4px;
  color: #858585;
  font-size: 12px;
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
.btn-secondary {
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
</style>

