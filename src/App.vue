<script setup lang="ts">
import { ref } from 'vue'
import PackageEditor from './components/PackageEditor.vue'

const appName = 'RPG Desktop'
const version = 'Internal v1.1.0-dev (M9: Rulebooks)'
</script>

<template>
  <div class="app">
    <PackageEditor />
  </div>
</template>

<style scoped>
.app {
  height: 100vh;
  font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
}
</style>


