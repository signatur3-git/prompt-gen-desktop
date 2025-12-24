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
  background-color: var(--bg-primary);
  color: var(--text-primary);
}
</style>


