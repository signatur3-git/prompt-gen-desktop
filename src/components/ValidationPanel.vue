<template>
  <div v-if="errors.length > 0 || warnings.length > 0" class="validation-panel">
    <div class="panel-header">
      <h4>
        <span class="error-icon">⚠️</span>
        <span v-if="errors.length > 0">
          {{ errors.length }} Validation {{ errors.length === 1 ? 'Error' : 'Errors' }}
        </span>
        <span v-if="errors.length > 0 && warnings.length > 0"> • </span>
        <span v-if="warnings.length > 0" class="warning-count">
          {{ warnings.length }} {{ warnings.length === 1 ? 'Warning' : 'Warnings' }}
        </span>
      </h4>
      <button @click="$emit('close')" class="close-btn" title="Close">×</button>
    </div>
    <div class="panel-content">
      <!-- Errors -->
      <div
        v-for="(error, index) in errors"
        :key="'error-' + index"
        class="error-item"
        :class="{ clickable: error.location }"
        @click="error.location && $emit('jump-to', error)"
      >
        <div class="error-header">
          <span class="error-number">{{ index + 1 }}.</span>
          <span class="error-message">{{ error.message }}</span>
        </div>

        <div v-if="error.location" class="error-location">
          <span class="location-icon">📍</span>
          <span class="location-text">{{ error.location }}</span>
        </div>

        <div v-if="error.suggestion" class="error-suggestion">
          <span class="suggestion-icon">💡</span>
          <span class="suggestion-text">{{ error.suggestion }}</span>
        </div>
      </div>

      <!-- Warnings -->
      <div
        v-for="(warning, index) in warnings"
        :key="'warning-' + index"
        class="warning-item"
      >
        <div class="warning-header">
          <span class="warning-icon">⚠️</span>
          <span class="warning-message">{{ warning }}</span>
        </div>
      </div>
    </div>
  </div>
</template>

<script setup>
defineProps({
  errors: {
    type: Array,
    required: true
  },
  warnings: {
    type: Array,
    default: () => []
  }
})

defineEmits(['jump-to', 'close'])
</script>

<style scoped>
.validation-panel {
  position: fixed;
  bottom: 0;
  left: 0;
  right: 0;
  background: #3c1f1e;
  border-top: 2px solid #f48771;
  max-height: 200px;
  display: flex;
  flex-direction: column;
  z-index: 100;
}

.panel-header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 12px 16px;
  border-bottom: 1px solid #5a2e2b;
}

.panel-header h4 {
  margin: 0;
  font-size: 14px;
  font-weight: 600;
  color: #f48771;
  display: flex;
  align-items: center;
  gap: 8px;
}

.error-icon {
  font-size: 16px;
}

.close-btn {
  background: none;
  border: none;
  color: #858585;
  font-size: 24px;
  cursor: pointer;
  padding: 0;
  width: 24px;
  height: 24px;
  line-height: 1;
}

.close-btn:hover {
  color: #d4d4d4;
}

.panel-content {
  flex: 1;
  overflow-y: auto;
  padding: 8px 0;
}

.error-item {
  padding: 12px 16px;
  border-bottom: 1px solid #5a2e2b;
  cursor: default;
  transition: background 0.2s;
}

.error-item.clickable {
  cursor: pointer;
}

.error-item.clickable:hover {
  background: #4a2624;
}

.error-header {
  display: flex;
  gap: 8px;
}

.error-number {
  color: #858585;
  font-weight: 600;
  flex-shrink: 0;
}

.error-message {
  color: #f48771;
  line-height: 1.5;
  flex: 1;
}

.error-location {
  display: flex;
  align-items: center;
  gap: 6px;
  margin: 6px 0 0 32px;
  font-size: 12px;
  color: #d4a574;
}

.location-icon {
  font-size: 14px;
}

.location-text {
  font-family: 'Courier New', monospace;
}

.error-suggestion {
  display: flex;
  align-items: center;
  gap: 6px;
  margin: 6px 0 0 32px;
  font-size: 12px;
  color: #89d185;
}

.suggestion-icon {
  font-size: 14px;
}

.suggestion-text {
  font-style: italic;
}

.warning-count {
  color: #ffa726;
}

.warning-item {
  padding: 12px;
  margin-bottom: 8px;
  background: rgba(255, 167, 38, 0.1);
  border-left: 3px solid #ffa726;
  border-radius: 4px;
}

.warning-header {
  display: flex;
  gap: 8px;
  align-items: flex-start;
}

.warning-icon {
  font-size: 16px;
  flex-shrink: 0;
}

.warning-message {
  color: #ffb74d;
  line-height: 1.5;
  flex: 1;
}
</style>

