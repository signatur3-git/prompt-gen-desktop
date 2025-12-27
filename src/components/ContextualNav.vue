<template>
  <div class="contextual-nav">
    <div class="context-info">
      <slot name="info"></slot>
    </div>

    <div class="context-actions">
      <slot name="actions"></slot>
    </div>
  </div>
</template>

<script setup lang="ts">
// Contextual navigation bar for page-specific tools and actions
// Appears below the main navigation on pages that need it
</script>

<style scoped>
.contextual-nav {
  display: flex;
  justify-content: space-between;
  align-items: center;
  min-height: 48px;
  padding: 8px 24px;
  background: var(--bg-primary);
  border-bottom: 1px solid var(--border-color);
}

.context-info {
  display: flex;
  align-items: center;
  gap: 12px;
}

.context-actions {
  display: flex;
  align-items: center;
  gap: 8px;
}
</style>

