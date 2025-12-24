<script setup lang="ts">
// App now uses router for navigation
</script>

<template>
  <div class="app">
    <router-view />
  </div>
</template>

<style scoped>
.app {
  height: 100vh;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
  background-color: var(--bg-primary, #ffffff);
  color: var(--text-primary, #1a1a1a);
}

/* Dark theme */
@media (prefers-color-scheme: dark) {
  .app {
    --bg-primary: #1a1a1a;
    --bg-secondary: #2a2a2a;
    --text-primary: #e0e0e0;
    --text-secondary: #a0a0a0;
    --border-color: #404040;

    background-color: var(--bg-primary);
    color: var(--text-primary);
  }
}
</style>


